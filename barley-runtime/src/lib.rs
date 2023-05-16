use async_trait::async_trait;
pub use anyhow::Result;
use std::sync::Arc;
use std::collections::VecDeque;

pub use barley_proc::barley_action;

#[async_trait]
pub trait Action: Send + Sync {
  async fn check(&self, ctx: &mut Context) -> Result<bool>;
  async fn perform(&self, ctx: &mut Context) -> Result<()>;
}

pub struct Context<'ctx> {
  actions: VecDeque<Arc<dyn Action + 'ctx>>
}

impl<'ctx> Context<'ctx> {
  pub fn new() -> Self {
    Self {
      actions: VecDeque::new()
    }
  }

  pub fn add_action<A: Action + 'ctx>(&mut self, action: A) {
    self.actions.push_back(Arc::new(action));
  }

  pub async fn run(&mut self) -> Result<()> {
    while let Some(action) = self.actions.pop_front() {
      if action.check(self).await? {
        action.perform(self).await?;
      }
    }

    Ok(())
  }
}