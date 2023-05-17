#![deny(missing_docs)]

//! `barley-runtime`
//! 
//! This crate contains the runtime for the `barley` workflow engine. It
//! provides the [`Action`] trait, which is the main interface for defining
//! actions that can be executed by the engine. It also provides the
//! [`Context`] struct, which is used to pass information between actions.
//! 
//! [`Action`]: trait.Action.html
//! [`Context`]: struct.Context.html

use async_trait::async_trait;
pub use anyhow::{Result, Error};
use std::sync::Arc;
use std::collections::{VecDeque, HashMap};
pub use uuid::Uuid as Id;

pub use barley_proc::barley_action;

/// A measurable, reversible task.
/// 
/// Any `Action` can test its environment to see if
/// it needs to run at all, and can undo any changes
/// it has made. Any `Action` can also depend on
/// other `Action`s, and the engine will ensure that
/// all dependencies are run before the `Action` itself.
#[async_trait]
pub trait Action: Send + Sync {
  /// Check if the action needs to be run.
  /// 
  /// This method is called before the action is run,
  /// and can be used to check if the action needs to
  /// run at all. If this method returns `false`, the
  /// action has not run yet, and the engine will
  /// proceed to run it. If this method returns `true`,
  /// the action has already run, and the engine will
  /// skip it.
  async fn check(&self, ctx: &mut Context) -> Result<bool>;

  /// Check if the action's dependencies need to be run.
  /// 
  /// This method is called internally, and should not
  /// be called directly. It is used to check if any
  /// of the action's dependencies need to be run.
  async fn check_deps(&self, ctx: &mut Context) -> Result<bool>;

  /// Run the action.
  async fn perform(&self, ctx: &mut Context) -> Result<()>;

  /// Undo the action.
  async fn rollback(&self, ctx: &mut Context) -> Result<()>;

  /// Get the action's ID.
  fn id(&self) -> Id;

  /// Add a dependency to the action.
  fn add_dep(&mut self, action: Arc<dyn Action>);
}

/// A context for running actions.
/// 
/// There should only be one of these per workflow
#[derive(Default)]
pub struct Context<'ctx> {
  actions: VecDeque<Arc<dyn Action + 'ctx>>,
  variables: HashMap<String, String>,
  callbacks: ContextCallbacks
}

impl<'ctx> Context<'ctx> {
  /// Create a new context with the given callbacks.
  /// 
  /// If you don't want any callbacks, it's recommended
  /// to use the [`Default`] implementation instead.
  /// 
  /// [`Default`]: https://doc.rust-lang.org/std/default/trait.Default.html
  pub fn new(callbacks: ContextCallbacks) -> Self {
    Self {
      actions: VecDeque::new(),
      variables: HashMap::new(),
      callbacks
    }
  }

  /// Add an action to the context.
  /// 
  /// This method adds an action to the context, and
  /// returns a reference to the action. The action
  /// will be run when the context is run.
  /// 
  /// You can use the returned reference as a
  /// dependency for other actions.
  pub fn add_action<A: Action + 'ctx>(&mut self, action: A) -> Arc<dyn Action + 'ctx> {
    let action = Arc::new(action);
    self.actions.push_back(action.clone());
    action
  }

  /// Run the context.
  /// 
  /// While processing the actions, it will
  /// call the callbacks if they are set.
  pub async fn run(&mut self) -> Result<()> {
    while let Some(action) = self.actions.pop_front() {
      if !action.check(self).await? {
        if let Some(callback) = self.callbacks.on_action_started {
          callback(action.as_ref());
        }

        match action.perform(self).await {
          Ok(_) => {
            if let Some(callback) = self.callbacks.on_action_finished {
              callback(action.as_ref());
            }
          },
          Err(err) => {
            if let Some(callback) = self.callbacks.on_action_failed {
              callback(action.as_ref(), &err);
            }

            action.rollback(self).await?;
          }
        }
      }
    }

    Ok(())
  }

  /// Sets a variable in the context.
  /// 
  /// This can be used to send information between
  /// actions. For example, you could set a return code
  /// in one action, and check it in another.
  pub fn set_variable(&mut self, name: &str, value: &str) {
    self.variables.insert(name.to_string(), value.to_string());
  }

  /// Gets a variable from the context.
  /// 
  /// If the variable doesn't exist, this method
  /// returns `None`.
  pub fn get_variable(&self, name: &str) -> Option<&str> {
    self.variables.get(name).map(|s| s.as_str())
  }
}

/// Callbacks for the context.
/// 
/// These callbacks are set by interfaces, and are
/// usually not set by scripts directly.
#[derive(Default)]
pub struct ContextCallbacks {
  /// Called when an action is started.
  pub on_action_started: Option<fn(&dyn Action)>,
  /// Called when an action is completed successfully.
  pub on_action_finished: Option<fn(&dyn Action)>,
  /// Called when an action fails.
  pub on_action_failed: Option<fn(&dyn Action, &anyhow::Error)>
}