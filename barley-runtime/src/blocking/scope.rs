use super::action::Node;


/// A collection of actions.
/// 
/// This does not provide any extra action
/// execution context, but it is useful
/// for simplifying the creation of
/// runtimes with many actions.
#[derive(Default)]
pub struct Scope<'scope> {
    actions: Vec<Node<'scope>>
}

impl<'scope> Scope<'scope> {
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
    pub fn add_action<A: Into<Node<'scope>>>(&mut self, action: A) {
        let action = action.into();
        self.actions.push(action);
    }

    /// List the actions in the scope.
    #[must_use]
    pub fn actions(&self) -> &[Node] {
        &self.actions
    }

    pub(crate) fn actions_owned(self) -> Vec<Node<'scope>> {
        self.actions
    }
}
