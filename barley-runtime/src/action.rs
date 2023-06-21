use async_trait::async_trait;
use std::sync::Arc;
use crate::{
    Runtime, RuntimeBuilder, Error,
    Output, Probe, Operation, Id
};


/// A measurable, reversible task.
/// 
/// Any `Action` can test its environment to see if
/// it needs to run at all, and can undo any changes
/// it has made. Any `Action` can also depend on
/// other `Action`s, and the engine will ensure that
/// all dependencies are run before the `Action` itself.
#[async_trait]
pub trait Action: Send + Sync {
    /// Run the action.
    /// 
    /// This method takes a [`Runtime`] node, which
    /// contains the context for the action. It also
    /// takes an [`Operation`], which is used to
    /// determine what the action should do.
    async fn run(&self, runtime: Runtime, operation: Operation) -> Result<Option<Output>, Error>;

    /// Probe the action for specific information.
    async fn probe(&self, runtime: Runtime) -> Result<Probe, Error>;

    /// Load required state.
    async fn load_state(&self, _builder: &mut RuntimeBuilder) {}

    /// Get the display name of the action.
    fn display_name(&self) -> String;
}

/// A usable action node.
/// 
/// This struct is used by actions to store their
/// dependencies and identification. It should
/// not be constructed directly, unless you are
/// writing a custom Action.
#[derive(Clone)]
pub struct Node {
    action: Arc<dyn Action>,
    deps: Vec<Node>,
    pub(crate) id: Id
}


impl Node {
    /// Create a new action node.
    /// 
    /// This method should not be called directly,
    /// unless you are writing a custom Action.
    pub fn new(action: Arc<dyn Action>) -> Self {
        Self {
            action,
            deps: Vec::new(),
            id: Id::default()
        }
    }
  
    /// Get the display name of the action.
    #[must_use]
    pub fn display_name(&self) -> String {
        self.action.display_name()
    }
  
    pub(crate) fn id(&self) -> Id {
        self.id
    }
  
    pub(crate) fn deps(&self) -> Vec<Node> {
        self.deps.clone()
    }
  
    pub(crate) async fn probe(&self, ctx: Runtime) -> Result<Probe, Error> {
        self.action.probe(ctx).await
    }
  
    pub(crate) async fn run(&self, ctx: Runtime, operation: Operation) -> Result<Option<Output>, Error> {
        self.action.run(ctx, operation).await
    }
  
    /// Add a dependency to the action.
    pub fn requires(&mut self, action: Node) {
        self.deps.push(action);
    }
  
    /// Load the state
    pub async fn load_state(&self, builder: &mut RuntimeBuilder) {
        self.action.load_state(builder).await;
    }
}

impl<A> From<A> for Node
where
A: Action + 'static {
    fn from(action: A) -> Self {
        Self::new(Arc::new(action))
    }
}