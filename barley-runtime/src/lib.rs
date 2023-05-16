use async_trait::async_trait;
pub use anyhow::Result;


#[async_trait]
pub trait Action {
  async fn perform(&self) -> Result<()>;
}

pub struct Context {
  pub actions: Vec<Box<dyn Action>>
}