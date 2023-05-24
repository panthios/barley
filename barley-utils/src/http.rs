use reqwest::get;
use barley_runtime::prelude::*;

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
  async fn check(&self, ctx: Arc<RwLock<Context>>) -> Result<bool> {
    let var_name = format!("http_get__{}", self.url);

    if let Some(_) = ctx.get_variable(&var_name).await {
      return Ok(true);
    }

    Ok(false)
  }

  async fn perform(&self, ctx: Arc<RwLock<Context>>) -> Result<Option<ActionOutput>> {
    let res = get(&self.url).await?;
    let body = res.text().await?;

    Ok(Some(ActionOutput::String(body)))
  }

  async fn rollback(&self, _ctx: Arc<RwLock<Context>>) -> Result<()> {
    Ok(())
  }

  fn display_name(&self) -> String {
    format!("HTTP GET \"{}\"", self.url)
  }
}