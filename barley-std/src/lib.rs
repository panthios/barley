#[cfg(feature = "time")]
pub mod time;

#[cfg(feature = "fs")]
pub mod fs;

#[cfg(feature = "process")]
pub mod process;

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
    async fn check(&self, _ctx: Runtime) -> Result<bool, ActionError> {
        Ok(false)
    }

    async fn perform(&self, _ctx: Runtime) -> Result<Option<ActionOutput>, ActionError> {
        Ok(None)
    }

    async fn rollback(&self, _ctx: Runtime) -> Result<(), ActionError> {
        Ok(())
    }

    fn display_name(&self) -> String {
        "".to_string()
    }
}
