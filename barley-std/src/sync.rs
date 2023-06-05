use barley_runtime::prelude::*;
use futures::future::join_all;


#[barley_action]
#[derive(Default)]
pub struct Join {
    actions: Vec<ActionObject>
}

impl Join {
    pub fn new<I>(actions: I) -> Self
    where
        I: IntoIterator<Item = ActionObject>,
    {
        let mut action = Self::default();

        for action_object in actions {
            action.actions.push(action_object.clone());
            action.requires(action_object);
        }

        action
    }
}

#[barley_action]
#[async_trait]
impl Action for Join {
    async fn check(&self, ctx: Arc<RwLock<Context>>) -> Result<bool> {
        Ok(false)
    }

    async fn perform(&self, ctx: Arc<RwLock<Context>>) -> Result<Option<ActionOutput>> {
        Ok(None)
    }

    async fn rollback(&self, _ctx: Arc<RwLock<Context>>) -> Result<()> {
        Ok(())
    }

    fn display_name(&self) -> String {
        "".to_string()
    }
}