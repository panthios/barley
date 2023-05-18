use reqwest::get;
use async_trait::async_trait;
use barley_runtime::*;

/// An HTTP GET request.
/// 
/// The data from the request is set in
/// the `http_get__<url>` variable.
#[barley_action]
#[derive(Default)]
pub struct HttpGet {
  url: String
}

impl HttpGet {
  /// Create a new `HttpGet` action.
  pub fn new(url: String) -> Self {
    Self { url, ..Default::default() }
  }
}

#[barley_action]
#[async_trait]
impl Action for HttpGet {
  async fn check(&self, ctx: &mut Context) -> Result<bool> {
    let var_name = format!("http_get__{}", self.url);

    if let Some(_) = ctx.get_variable(&var_name) {
      return Ok(true);
    }

    Ok(false)
  }

  async fn perform(&self, ctx: &mut Context) -> Result<()> {
    let res = get(&self.url).await?;
    let body = res.text().await?;

    let var_name = format!("http_get__{}", self.url);

    ctx.set_variable(&var_name, &body);

    Ok(())
  }

  async fn rollback(&self, _ctx: &mut Context) -> Result<()> {
    Ok(())
  }

  fn display_name(&self) -> String {
    format!("HTTP GET \"{}\"", self.url)
  }
}