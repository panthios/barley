use reqwest::get;
use async_trait::async_trait;
use barley_runtime::*;


#[barley_action]
#[derive(Default)]
pub struct HttpGet {
  url: String
}

impl HttpGet {
  pub fn new(url: String) -> Self {
    Self { url, ..Default::default() }
  }
}

#[barley_action]
#[async_trait]
impl Action for HttpGet {
  async fn check(&self, ctx: &mut Context) -> Result<bool> {
    Ok(false)
  }

  async fn perform(&self, ctx: &mut Context) -> Result<()> {
    let res = get(&self.url).await?;
    let body = res.text().await?;

    ctx.set_variable("http_get", &body);

    Ok(())
  }
}