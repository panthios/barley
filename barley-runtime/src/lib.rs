use async_trait::async_trait;
pub use anyhow::Result;
use std::rc::Rc;
use std::collections::VecDeque;


#[async_trait]
pub trait Action {
  async fn check(&self, ctx: &mut Context) -> Result<bool>;
  async fn perform(&self, ctx: &mut Context) -> Result<()>;
}

pub struct Context<'ctx> {
  actions: VecDeque<Rc<dyn Action + 'ctx>>
}

impl<'ctx> Context<'ctx> {
  pub fn new() -> Self {
    Self {
      actions: VecDeque::new()
    }
  }

  pub fn add_action<A: Action + 'ctx>(&mut self, action: A) {
    self.actions.push_back(Rc::new(action));
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