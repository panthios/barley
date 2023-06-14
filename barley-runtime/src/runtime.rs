use tokio::sync::RwLock;
use tokio::sync::Barrier;
use tokio::task::JoinSet;

use std::any::{Any, TypeId};
use tracing::{debug, info, error};
use std::{
    sync::Arc,
    collections::HashMap
};

use crate::Operation;
use crate::{
    ActionObject, Id,
    ActionOutput,
    ActionError,
    context::Context,
    scope::Scope
};


/// The runtime for a workflow.
/// 
/// This struct is used to run a workflow. It contains
/// all of the actions that need to be run, and it
/// ensures that all dependencies are run before the
/// actions that depend on them.
/// 
/// # Example
/// 
/// ```
/// use barley_runtime::prelude::*;
/// 
/// let runtime = RuntimeBuilder::new().build();
/// ```
#[derive(Clone)]
pub struct Runtime {
    ctx: Context,
    barriers: HashMap<Id, Arc<Barrier>>,
    outputs: Arc<RwLock<HashMap<Id, ActionOutput>>>,
    state: HashMap<TypeId, Arc<dyn Any + Send + Sync>>
}

impl Runtime {
    /// Run the workflow.
    /// 
    /// # Errors
    /// 
    /// This function will return an error if any of
    /// the actions fail, or if there is an internal
    /// error with the runtime itself.
    pub async fn perform(mut self) -> Result<(), ActionError> {
        let actions = self.ctx.actions.clone();
        let mut dependents: HashMap<Id, usize> = HashMap::new();

        // Get the dependents for each action. For
        // example, if action A depends on action B,
        // then 1 action is dependent on B (A) and 0
        // actions are dependent on A.
        for action in &actions {
            dependents.insert(action.id, 0);

            action.deps()
                .iter()
                .map(ActionObject::id)
                .for_each(|id| {
                    let count = dependents.entry(id).or_insert(0);
                    *count += 1;
                });
        }

        // Create a barrier for each action that has
        // any dependents. The barrier will be used
        // to wait for the dependent actions to finish.
        for (id, dependents) in dependents.clone() {
            if dependents == 0 {
                continue;
            }

            let barrier = Arc::new(Barrier::new(dependents + 1));
            self.barriers.insert(id, barrier);
        }

        let mut join_set: JoinSet<Result<(), ActionError>> = JoinSet::new();

        debug!("Starting actions");
        for action in actions {
            let runtime_clone = self.clone();

            let action = action.clone();

            let deps = action.deps();

            let barriers = deps
                .iter()
                .map(ActionObject::id);

            let barriers = barriers
                .filter_map(|id| self.barriers.get(&id).cloned())
                .collect::<Vec<_>>();

            let self_barriers = self.barriers.clone();

            join_set.spawn(async move {
                let self_barrier = self_barriers.get(&action.id).cloned();

                for barrier in barriers {
                    barrier.wait().await;
                }

                let probe = action.probe(runtime_clone.clone()).await?;
                if !probe.needs_run {
                    return Ok(())
                }

                let display_name = action.display_name();
                info!("Starting action: {}", display_name);

                let output = action.run(runtime_clone.clone(), Operation::Perform).await;

                if let Err(err) = &output {
                    error!("Action failed: {}", display_name);
                    error!("Error: {}", err);

                    return Err(err.clone())
                }
                
                info!("Action finished: {}", display_name);

                if let Some(barrier) = self_barrier {
                    barrier.wait().await;
                }

                if let Some(output) = output? {
                    runtime_clone.outputs.write().await.insert(action.id, output);
                }

                Ok(())
            });
        }

        while let Some(result) = join_set.join_next().await {
            match result {
                Ok(Ok(())) => {},
                Ok(Err(err)) => {
                    join_set.abort_all();

                    if let ActionError::ActionFailed(_, long) = err.clone() {
                        println!("{long}");
                    }

                    return Err(err)
                },
                Err(_) => {
                    join_set.abort_all();

                    return Err(ActionError::InternalError("JOIN_SET_ERROR"))
                }
            }
        }

        Ok(())
    }

    /// Rollback the workflow.
    /// 
    /// This will undo all of the actions that have
    /// been performed, if possible.
    /// 
    /// # Panics
    /// 
    /// This action can theoretically panic due to an internal
    /// unwrapping of guaranteed `Some` variants. In practice,
    /// this doesn't happen.
    /// 
    /// # Errors
    /// 
    /// This function will return an error if any of
    /// the actions fail, or if there is an internal
    /// error with the runtime itself.
    pub async fn rollback(self) -> Result<(), ActionError> {
        let actions = self.ctx.actions.clone();
        let mut dependencies: HashMap<Id, Vec<Id>> = HashMap::new();

        // Check if all of the actions have a rollback
        // function. If not, then the rollback cannot
        // be performed.
        for action in &actions {
            if !action.probe(self.clone()).await?.can_rollback {
                return Err(ActionError::InternalError("NO_ROLLBACK"))
            }
        }

        // Get the dependencies for each action. For
        // example, if action A depends on action B,
        // then B is a dependency of A.
        for action in &actions {
            dependencies.insert(action.id, Vec::new());

            action.deps()
                .iter()
                .map(ActionObject::id)
                .for_each(|id| {
                    let deps = dependencies.entry(id).or_insert(Vec::new());
                    deps.push(action.id);
                });
        }

        // Sort the actions by their dependencies.
        let mut actions = actions;
        actions.sort_by(|a, b| {
            let a_deps = dependencies.get(&a.id).unwrap();
            let b_deps = dependencies.get(&b.id).unwrap();

            if a_deps.contains(&b.id) {
                return std::cmp::Ordering::Greater
            }

            if b_deps.contains(&a.id) {
                return std::cmp::Ordering::Less
            }

            std::cmp::Ordering::Equal
        });

        // Create spawns
        let mut join_set: JoinSet<Result<(), ActionError>> = JoinSet::new();

        for action in actions {
            let runtime_clone = self.clone();

            join_set.spawn(async move {
                action.run(runtime_clone.clone(), Operation::Rollback).await?;

                Ok(())
            });
        }

        while let Some(result) = join_set.join_next().await {
            match result {
                Ok(Ok(())) => {},
                Ok(Err(err)) => {
                    join_set.abort_all();

                    if let ActionError::ActionFailed(_, long) = err.clone() {
                        println!("{long}");
                    }

                    return Err(err)
                },
                Err(_) => {
                    join_set.abort_all();

                    return Err(ActionError::InternalError("JOIN_SET_ERROR"))
                }
            }
        }

        Ok(())
    }

    /// Get the output of an action.
    pub async fn get_output(&self, obj: ActionObject) -> Option<ActionOutput> {
        self.outputs.read().await.get(&obj.id()).cloned()
    }

    /// Get the state object of a type.
    /// 
    /// # Panics
    /// 
    /// This function will panic if the state object
    /// is not the type that is requested.
    #[must_use]
    pub fn get_state<T: Send + Sync + 'static>(&self) -> Option<Arc<T>> {
        self.state.get(&TypeId::of::<T>()).cloned().map(|state| {
            state.downcast::<T>().unwrap()
        })
    }
}

/// A builder for a runtime.
#[allow(clippy::module_name_repetitions)]
pub struct RuntimeBuilder {
    ctx: Context,
    state: HashMap<TypeId, Arc<dyn Any + Send + Sync>>
}

impl RuntimeBuilder {
    /// Create a new runtime builder.
    #[must_use]
    pub fn new() -> Self {
        Self {
            ctx: Context::new(),
            state: HashMap::new()
        }
    }

    /// Add an action to the runtime.
    pub async fn add_action(mut self, action: ActionObject) -> Self {
        action.load_state(&mut self).await;
        self.ctx.add_action(action);
        self
    }

    /// Add a scope to the runtime.
    pub async fn add_scope(mut self, scope: Scope) -> Self {
        for action in scope.actions() {
            self = self.add_action(action.clone()).await;
        }

        self
    }

    /// Build the runtime.
    #[must_use]
    pub fn build(self) -> Runtime {
        Runtime {
            ctx: self.ctx,
            barriers: HashMap::new(),
            outputs: Arc::new(RwLock::new(HashMap::new())),
            state: self.state
        }
    }

    /// Add a state object to the runtime.
    pub fn add_state<T: Send + Sync + 'static>(&mut self, state: T) -> &mut Self {
        self.state.insert(TypeId::of::<T>(), Arc::new(state));
        self
    }
}

impl Default for RuntimeBuilder {
    fn default() -> Self {
        Self::new()
    }
}