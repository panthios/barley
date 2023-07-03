use std::collections::HashMap;
use std::any::{Any, TypeId};

use crate::{Id, Operation};
use crate::output::Output;
use crate::error::Error;
use super::action::Node;
use super::scope::Scope;


/// The runtime for a workflow.
/// 
/// This struct is used to run a workflow. It contains
/// all of the actions that need to be run, and it
/// ensures that all dependencies are run before the
/// actions that depend on them.
pub struct Runtime<'run> {
    ctx: Vec<Node<'run>>,
    outputs: HashMap<Id, Output>,
    state: HashMap<TypeId, Box<dyn Any>>
}

impl<'run> Runtime<'run> {
    /// Run the workflow.
    /// 
    /// # Errors
    /// 
    /// All action errors are handled internally. This
    /// function will return an error if there is an
    /// internal error with the runtime itself.
    pub fn perform(mut self) -> Result<(), Error> {
        let actions = &mut self.ctx;
        let mut dependents: HashMap<Id, usize> = HashMap::new();

        for action in &mut *actions {
            dependents.insert(action.id, 0);

            action.deps()
                .iter()
                .for_each(|dep| {
                    let count = dependents.entry(dep.id()).or_insert(0);
                    *count += 1;
                });
        }

        actions.sort_by(|a, b| {
            let a_deps = dependents.get(&a.id).unwrap_or(&0);
            let b_deps = dependents.get(&b.id).unwrap_or(&0);

            a_deps.cmp(b_deps)
        });

        let actions = &self.ctx;

        for action in actions {
            let output = action.run(&self, Operation::Perform)?;

            if let Some(output) = output {
                self.outputs.insert(action.id, output);
            }
        }

        Ok(())
    }

    /// Reverse the workflow.
    /// 
    /// # Errors
    /// 
    /// All action errors are handled internally. This
    /// function will return an error if there is an
    /// internal error with the runtime itself.
    /// 
    /// # Panics
    /// 
    /// This function uses unwrap, but panics are impossible.
    /// If a panic occurs, please report it as a bug.
    pub fn rollback(mut self) -> Result<(), Error> {
        let actions = &self.ctx;
        let mut dependencies: HashMap<Id, Vec<Id>> = HashMap::new();

        for action in actions {
            if !action.probe(&self)?.can_rollback {
                return Err(Error::OperationNotSupported);
            }

            dependencies.insert(action.id, Vec::new());

            action.deps()
                .iter()
                .map(|dep| dep.id())
                .for_each(|id| {
                    let deps = dependencies.entry(id).or_insert(Vec::new());
                    deps.push(action.id);
                });
        }

        let mut order = dependencies
            .iter()
            .map(|(id, deps)| (*id, deps.len()))
            .collect::<Vec<_>>();

        order.sort_by(|a, b| a.1.cmp(&b.1));
        let order = order.iter().map(|(id, _)| id).collect::<Vec<_>>();

        for id in order {
            let action = actions
                .iter()
                .find(|action| action.id == *id)
                .unwrap();

            let output = action.run(&self, Operation::Rollback)?;

            if let Some(output) = output {
                self.outputs.insert(action.id, output);
            }
        }

        Ok(())
    }

    /// Get the output of an action.
    #[must_use]
    pub fn get_output(&self, obj: &Node) -> Option<&Output> {
        self.outputs.get(&obj.id)
    }

    /// Get the state object of a given type.
    #[must_use]
    pub fn get_state<T: Any>(&self) -> Option<&T> {
        self.state
            .get(&TypeId::of::<T>())
            .and_then(|state| state.downcast_ref::<T>())
    }
}

/// A builder for a runtime.
/// 
/// This struct is used to build a runtime. Once
/// you have added all of your actions, you can
/// call [`build`] to create the runtime.\
#[allow(clippy::module_name_repetitions)]
pub struct RuntimeBuilder<'build> {
    ctx: Vec<Node<'build>>,
    state: HashMap<TypeId, Box<dyn Any>>
}

impl<'build> RuntimeBuilder<'build> {
    /// Create a new builder.
    #[must_use]
    pub fn new() -> Self {
        Self {
            ctx: Vec::new(),
            state: HashMap::new()
        }
    }

    /// Add an action to the runtime.
    #[must_use]
    pub fn add_action(mut self, action: Node<'build>) -> Self {
        action.load_state(&mut self);
        self.ctx.push(action);
        self
    }

    /// Add a scope to the runtime.
    #[must_use]
    pub fn add_scope(mut self, scope: Scope<'build>) -> Self {
        for action in scope.actions_owned() {
            self = self.add_action(action);
        }

        self
    }

    /// Build the runtime.
    #[must_use]
    pub fn build(self) -> Runtime<'build> {
        Runtime {
            ctx: self.ctx,
            outputs: HashMap::new(),
            state: self.state
        }
    }

    /// Add a state object to the runtime.
    pub fn add_state<T: Any>(&mut self, state: T) -> &mut Self {
        self.state.insert(TypeId::of::<T>(), Box::new(state));
        self
    }
}

impl Default for RuntimeBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}
