use super::{ActionObject, Id, ActionOutput};
use std::collections::HashMap;


/// A context for running actions.
/// 
/// There should only be one of these per workflow
#[derive(Default)]
pub struct Context {
  pub(crate) actions: Vec<ActionObject>,
  variables: HashMap<String, String>,
  outputs: HashMap<Id, ActionOutput>
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
      variables: HashMap::new(),
      outputs: HashMap::new(),
    }
  }

  /// Add an action to the context.
  pub fn add_action(&mut self, action: ActionObject) {
    self.actions.push(action);
  }
}