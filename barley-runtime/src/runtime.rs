use tokio::sync::RwLock;
use tokio::sync::Barrier;
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use tokio::task::JoinSet;

use std::any::{Any, TypeId};
use std::{
    sync::Arc,
    collections::HashMap
};
use crate::Operation;
use crate::{
    ActionObject, Id,
    ActionOutput,
    ActionError,
    context::Context
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
    progress: Arc<RwLock<MultiProgress>>,
    state: HashMap<TypeId, Arc<dyn Any + Send + Sync>>
}

impl Runtime {
    /// Run the workflow.
    pub async fn perform(mut self) -> Result<(), ActionError> {
        let actions = self.ctx.actions.clone();
        let mut dependents: HashMap<Id, usize> = HashMap::new();

        // Get the dependents for each action. For
        // example, if action A depends on action B,
        // then 1 action is dependent on B (A) and 0
        // actions are dependent on A.
        for action in actions.iter() {
            dependents.insert(action.id, 0);

            action.deps()
                .iter()
                .map(|dep| dep.id())
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
        let bars = Arc::new(RwLock::new(Vec::new()));
        let bars_clone = bars.clone();

        let tick_loop = tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                bars_clone.write().await.iter().for_each(|bar: &ProgressBar| bar.tick());
            }
        });

        for action in actions {
            let runtime_clone = self.clone();
            let bars = bars.clone();

            let action = action.clone();

            let deps = action.deps();

            let barriers = deps
                .iter()
                .map(|dep| dep.id());

            let barriers = barriers
                .map(|id| self.barriers.get(&id).unwrap().clone())
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

                let progress = runtime_clone.progress.write().await.add(ProgressBar::new_spinner());
                progress.set_style(
                    ProgressStyle::default_spinner().template(" {spinner} [{elapsed_precise}] {wide_msg}")
                        .map_err(|_| ActionError::InternalError("PROGRESS_TEMPLATE_FAIL"))?
                );

                progress.set_message(display_name.clone());
                bars.write().await.push(progress.clone());

                let output = action.run(runtime_clone.clone(), Operation::Perform).await;

                if let Err(errmsg) = output {
                    progress.finish_with_message(format!("ERROR: {}", match errmsg.clone() {
                        ActionError::ActionFailed(msg, _) => msg,
                        ActionError::InternalError(msg) => msg.to_string(),
                        ActionError::NoActionReturn => "No action return".into(),
                        ActionError::OutputConversionFailed(msg) => msg,
                        ActionError::OperationNotSupported => "Operation not supported".into(),
                        ActionError::StateNotLoaded => "State not loaded".into(),
                    }));

                    return Err(errmsg)
                }

                progress.finish_and_clear();

                let output = output.unwrap();

                if let Some(barrier) = self_barrier {
                    barrier.wait().await;
                }

                if let Some(output) = output {
                    runtime_clone.outputs.write().await.insert(action.id, output);
                }

                Ok(())
            });
        }

        while let Some(result) = join_set.join_next().await {
            match result {
                Ok(Ok(())) => {},
                Ok(Err(err)) => {
                    tick_loop.abort();
                    join_set.abort_all();

                    if let ActionError::ActionFailed(_, long) = err.clone() {
                        println!("{}", long);
                    }

                    return Err(err)
                },
                Err(_) => {
                    tick_loop.abort();
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
    pub async fn rollback(self) -> Result<(), ActionError> {
        let actions = self.ctx.actions.clone();
        let mut dependencies: HashMap<Id, Vec<Id>> = HashMap::new();

        // Check if all of the actions have a rollback
        // function. If not, then the rollback cannot
        // be performed.
        for action in actions.iter() {
            if !action.probe(self.clone()).await?.can_rollback {
                return Err(ActionError::InternalError("NO_ROLLBACK"))
            }
        }

        // Get the dependencies for each action. For
        // example, if action A depends on action B,
        // then B is a dependency of A.
        for action in actions.iter() {
            dependencies.insert(action.id, Vec::new());

            action.deps()
                .iter()
                .map(|dep| dep.id())
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
                        println!("{}", long);
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
    pub fn get_state<T: Send + Sync + 'static>(&self) -> Option<Arc<T>> {
        self.state.get(&TypeId::of::<T>()).cloned().map(|state| {
            state.downcast::<T>().unwrap()
        })
    }
}

/// A builder for a runtime.
pub struct RuntimeBuilder {
    ctx: Context,
    state: HashMap<TypeId, Arc<dyn Any + Send + Sync>>
}

impl RuntimeBuilder {
    /// Create a new runtime builder.
    pub fn new() -> Self {
        Self {
            ctx: Context::new(),
            state: HashMap::new()
        }
    }

    /// Add an action to the runtime.
    pub fn add_action(mut self, action: ActionObject) -> Self {
        self.ctx.add_action(action);
        self
    }

    /// Build the runtime.
    pub fn build(self) -> Runtime {
        Runtime {
            ctx: self.ctx,
            barriers: HashMap::new(),
            outputs: Arc::new(RwLock::new(HashMap::new())),
            progress: Arc::new(RwLock::new(MultiProgress::new())),
            state: HashMap::new()
        }
    }

    /// Add a state object to the runtime.
    pub fn add_state<T: Send + Sync + 'static>(mut self, state: T) -> Self {
        self.state.insert(TypeId::of::<T>(), Arc::new(state));
        self
    }
}

impl Default for RuntimeBuilder {
    fn default() -> Self {
        Self::new()
    }
}