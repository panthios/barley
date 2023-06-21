use super::action::Node;


/// A context for running actions.
/// 
/// There should only be one of these per workflow
#[derive(Default, Clone)]
pub struct Context {
  pub(crate) actions: Vec<Node>
}

impl Context {
  /// Create a new context with the given callbacks.
  /// 
  /// If you don't want any callbacks, it's recommended
  /// to use the [`Default`] implementation instead.
  /// 
  /// [`Default`]: https://doc.rust-lang.org/std/default/trait.Default.html
  pub fn new() -> Self {
    Self {
      actions: Vec::new(),
    }
  }

  /// Add an action to the context.
  pub fn add_action(&mut self, action: Node) {
    self.actions.push(action);
  }
}