use async_trait::async_trait;
pub use anyhow::Result;


#[async_trait]
pub trait Action {
  async fn check(&self) -> Result<bool>;
  async fn perform(&self) -> Result<()>;
}

pub struct Context<'ctx> {
  pub actions: Vec<Box<dyn Action + 'ctx>>
}

impl<'ctx> Context<'ctx> {
  pub fn new() -> Self {
    Self { actions: Vec::new() }
  }

  pub fn add_action<A: Action + 'ctx>(&mut self, action: A) {
    self.actions.push(Box::new(action));
  }

  pub async fn run(&self) -> Result<()> {
    for action in self.actions.iter() {
      if !action.check().await? {
        action.perform().await?;
      }
    }

    Ok(())
  }
}