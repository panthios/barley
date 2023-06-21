use crate::action::Node;


/// A collection of actions.
/// 
/// This does not provide any extra action
/// execution context, but it is useful
/// for simplifying the creation of
/// runtimes with many actions.
#[derive(Default, Clone)]
pub struct Scope {
    actions: Vec<Node>
}

impl Scope {
    /// Create a new scope.
    #[must_use]
    pub fn new() -> Self {
        Self {
            actions: Vec::new()
        }
    }

    /// Add an action to the scope.
    /// 
    /// The action object will be returned
    /// so that it can be used to add
    /// dependencies.
    pub fn add_action<A: Into<Node>>(&mut self, action: A) -> Node {
        let action = action.into();
        self.actions.push(action.clone());
        action
    }

    /// List the actions in the scope.
    #[must_use]
    pub fn actions(&self) -> &[Node] {
        &self.actions
    }
}
