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
    async fn probe(&self, _runtime: Runtime) -> Result<Probe, ActionError> {
        Ok(Probe {
            needs_run: false,
            can_rollback: false
        })
    }

    async fn run(&self, _runtime: Runtime, _op: Operation) -> Result<Option<ActionOutput>, ActionError> {
        Ok(None)
    }

    fn display_name(&self) -> String {
        "".to_string()
    }
}
