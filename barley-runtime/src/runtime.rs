use futures::future::join_all;
use tokio::sync::RwLock;
use tokio::sync::Barrier;
use anyhow::Result;
use tokio::task::JoinHandle;
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};

use std::{
    sync::Arc,
    collections::HashMap
};
use crate::{
    ActionObject, Id,
    ActionOutput,
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
    barriers: HashMap<Id, Arc<Barrier>>,
    outputs: Arc<RwLock<HashMap<Id, ActionOutput>>>,
    progress: Arc<RwLock<MultiProgress>>
}

impl Runtime {
    /// Run the workflow.
    pub async fn run(mut self) -> Result<()> {
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

        let mut handles: Vec<JoinHandle<Result<()>>> = Vec::new();
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

            handles.push(tokio::spawn(async move {
                let self_barrier = self_barriers.get(&action.id).cloned();

                for barrier in barriers {
                    barrier.wait().await;
                }

                if action.check(runtime_clone.clone()).await? {
                    return Ok(())
                }

                let display_name = action.display_name();

                let progress = runtime_clone.progress.write().await.add(ProgressBar::new_spinner());
                progress.set_style(ProgressStyle::default_spinner().template(" {spinner} [{elapsed_precise}] {wide_msg}")?);
                progress.set_message(display_name.clone());
                bars.write().await.push(progress.clone());

                let output = action.perform(runtime_clone.clone()).await;

                if output.is_err() {
                    progress.finish_with_message(format!("Error: {}", display_name));
                    return Err(output.err().unwrap())
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
            }));
        }

        let results = join_all(handles).await;

        for result in results {
            result??;
        }

        Ok(())
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
            barriers: HashMap::new(),
            outputs: Arc::new(RwLock::new(HashMap::new())),
            progress: Arc::new(RwLock::new(MultiProgress::new()))
        }
    }
}

impl Default for RuntimeBuilder {
    fn default() -> Self {
        Self::new()
    }
}