use async_trait::async_trait;
pub use anyhow::Result;
use std::sync::Arc;
use std::collections::{VecDeque, HashMap};
pub use uuid::Uuid as Id;

pub use barley_proc::barley_action;

#[async_trait]
pub trait Action: Send + Sync {
  async fn check(&self, ctx: &mut Context) -> Result<bool>;
  async fn check_deps(&self, ctx: &mut Context) -> Result<bool>;
  async fn perform(&self, ctx: &mut Context) -> Result<()>;
  async fn rollback(&self, ctx: &mut Context) -> Result<()>;

  fn id(&self) -> Id;
}

pub struct Context<'ctx> {
  actions: VecDeque<Arc<dyn Action + 'ctx>>,
  variables: HashMap<String, String>,
  callbacks: ContextCallbacks
}

impl<'ctx> Context<'ctx> {
  pub fn new(callbacks: ContextCallbacks) -> Self {
    Self {
      actions: VecDeque::new(),
      variables: HashMap::new(),
      callbacks
    }
  }

  pub fn add_action<A: Action + 'ctx>(&mut self, action: A) {
    self.actions.push_back(Arc::new(action));
  }

  pub async fn run(&mut self) -> Result<()> {
    while let Some(action) = self.actions.pop_front() {
      if action.check(self).await? {
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

  pub fn set_variable(&mut self, name: &str, value: &str) {
    self.variables.insert(name.to_string(), value.to_string());
  }

  pub fn get_variable(&self, name: &str) -> Option<&str> {
    self.variables.get(name).map(|s| s.as_str())
  }
}

#[derive(Default)]
pub struct ContextCallbacks {
  pub on_action_started: Option<fn(&dyn Action)>,
  pub on_action_finished: Option<fn(&dyn Action)>,
  pub on_action_failed: Option<fn(&dyn Action, &anyhow::Error)>
}