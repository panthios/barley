use async_trait::async_trait;
pub use anyhow::Result;
use std::rc::Rc;


#[async_trait]
pub trait Action {
  async fn check(&self, ctx: &mut Context) -> Result<bool>;
  async fn perform(&self, ctx: &mut Context) -> Result<()>;
}

pub struct Context<'ctx> {
  actions: Vec<Rc<dyn Action + 'ctx>>
}

impl<'ctx> Context<'ctx> {
  pub fn new() -> Self {
    Self {
      actions: vec![]
    }
  }

  pub fn add_action<A: Action + 'ctx>(&mut self, action: A) {
    self.actions.push(Rc::new(action));
  }

  pub async fn run(&mut self) -> Result<()> {
    let actions = self.actions.clone();

    for action in actions {
      if !action.check(self).await? {
        action.perform(self).await?;
      }
    }

    Ok(())
  }
}