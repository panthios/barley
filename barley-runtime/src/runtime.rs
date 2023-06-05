use futures::future::join_all;
use tokio::sync::RwLock;
use tokio::sync::Barrier;

use std::{
    sync::Arc,
    collections::HashMap
};
use crate::{
    ActionObject, Id,
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
    ctx: Arc<RwLock<Context>>,
    barriers: HashMap<Id, Arc<Barrier>>
}

impl Runtime {
    /// Run the workflow.
    pub async fn run(mut self) {
        let actions = self.ctx.read().await.actions.clone();
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

        let mut handles = Vec::new();
        let ctx = Arc::new(RwLock::new(self.ctx));

        for action in actions {
            let ctx = ctx.clone();
            let action = action.clone();

            let deps = action.deps();

            let barriers = deps
                .iter()
                .map(|dep| dep.id());

            let barriers = barriers
                .map(|id| self.barriers.get(&id).unwrap().clone())
                .collect::<Vec<_>>();

            let self_barriers = self.barriers.clone();

            handles.push(tokio::spawn(async move {
                let self_barrier = self_barriers.get(&action.id).cloned();

                for barrier in barriers {
                    barrier.wait().await;
                }

                if let Some(barrier) = self_barrier {
                    barrier.wait().await;
                }
            }));
        }

        let results = join_all(handles).await;

        for result in results {
            result.unwrap();
        }
    }
}

/// A builder for a runtime.
pub struct RuntimeBuilder {
    ctx: Context
}

impl RuntimeBuilder {
    /// Create a new runtime builder.
    pub fn new() -> Self {
        Self {
            ctx: Context::new()
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
            ctx: Arc::new(RwLock::new(self.ctx)),
            barriers: HashMap::new()
        }
    }
}