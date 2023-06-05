use barley_runtime::prelude::*;


#[derive(Default)]
pub struct Join;

impl Join {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Action for Join {
    async fn check(&self, _ctx: Arc<RwLock<Runtime>>) -> Result<bool> {
        Ok(false)
    }

    async fn perform(&self, _ctx: Arc<RwLock<Runtime>>) -> Result<Option<ActionOutput>> {
        Ok(None)
    }

    async fn rollback(&self, _ctx: Arc<RwLock<Runtime>>) -> Result<()> {
        Ok(())
    }

    fn display_name(&self) -> String {
        "".to_string()
    }
}