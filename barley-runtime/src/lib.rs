#![deny(missing_docs)]

//! `barley-runtime`
//! 
//! This crate contains the runtime for the `barley` workflow engine. It
//! provides the [`Action`] trait, which is the main interface for defining
//! actions that can be executed by the engine. It also provides the
//! [`Context`] struct, which is used to pass information between actions.
//! 
//! [`Action`]: trait.Action.html
//! [`Context`]: struct.Context.html

use anyhow::Result;
use uuid::Uuid;
use std::sync::Arc;
use async_trait::async_trait;
use async_recursion::async_recursion;

/// The prelude for the `barley-runtime` crate.
/// 
/// This module contains all of the important types
/// and traits for the `barley-runtime` crate. It
/// should be used instead of importing the types
/// directly.
pub mod prelude;

mod context;
mod runtime;

pub use runtime::{Runtime, RuntimeBuilder};

/// A measurable, reversible task.
/// 
/// Any `Action` can test its environment to see if
/// it needs to run at all, and can undo any changes
/// it has made. Any `Action` can also depend on
/// other `Action`s, and the engine will ensure that
/// all dependencies are run before the `Action` itself.
#[async_trait]
pub trait Action: Send + Sync {
  /// Check if the action needs to be run.
  /// 
  /// This method is called before the action is run,
  /// and can be used to check if the action needs to
  /// run at all. If this method returns `false`, the
  /// action has not run yet, and the engine will
  /// proceed to run it. If this method returns `true`,
  /// the action has already run, and the engine will
  /// skip it.
  async fn check(&self, runtime: Runtime) -> Result<bool>;

  /// Run the action.
  async fn perform(&self, runtime: Runtime) -> Result<Option<ActionOutput>>;

  /// Undo the action.
  /// 
  /// This is not currently possible, and will not
  /// do anything. This will be usable in a future
  /// version of Barley.
  async fn rollback(&self, runtime: Runtime) -> Result<()>;

  /// Get the display name of the action.
  fn display_name(&self) -> String;
}

/// A usable action object.
/// 
/// This struct is used by actions to store their
/// dependencies and identification. It should
/// not be constructed directly, unless you are
/// writing a custom Action.
#[derive(Clone)]
pub struct ActionObject {
  action: Arc<dyn Action>,
  deps: Vec<ActionObject>,
  id: Id
}

impl ActionObject {
  /// Create a new action object.
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
  pub fn display_name(&self) -> String {
    self.action.display_name()
  }

  pub(crate) fn id(&self) -> Id {
    self.id
  }

  pub(crate) fn deps(&self) -> Vec<ActionObject> {
    self.deps.clone()
  }

  pub(crate) async fn check(&self, ctx: Runtime) -> Result<bool> {
    self.action.check(ctx).await
  }

  #[async_recursion]
  pub(crate) async fn check_deps(&self, ctx: Runtime) -> Result<bool> {
    if self.check(ctx.clone()).await? {
      return Ok(true)
    }

    let deps = self.deps.clone();

    for dep in deps.clone() {
      if !dep.check_deps(ctx.clone()).await? {
        return Ok(false)
      }
    }

    Ok(true)
  }

  pub(crate) async fn perform(&self, runtime: Runtime) -> Result<Option<ActionOutput>> {
    if self.check(runtime.clone()).await? {
      return Ok(None)
    }

    self.action.perform(runtime).await
  }

  pub(crate) async fn rollback(&self, ctx: Runtime) -> Result<()> {
    self.action.rollback(ctx).await
  }

  /// Add a dependency to the action.
  pub fn requires(&mut self, action: ActionObject) {
    self.deps.push(action);
  }
}

impl<A> From<A> for ActionObject
where
  A: Action + 'static
{
  fn from(action: A) -> Self {
    Self::new(Arc::new(action))
  }
}

/// Callbacks for the context.
/// 
/// These callbacks are set by interfaces, and are
/// usually not set by scripts directly.
#[derive(Default, Clone)]
pub struct ContextCallbacks {
  /// Called when an action is started.
  pub on_action_started: Option<fn(ActionObject)>,
  /// Called when an action is completed successfully.
  pub on_action_finished: Option<fn(ActionObject)>,
  /// Called when an action fails.
  pub on_action_failed: Option<fn(ActionObject, &anyhow::Error)>
}

/// A unique identifier for an action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(Uuid);

impl Default for Id {
  fn default() -> Self {
    Self(Uuid::new_v4())
  }
}

impl std::fmt::Display for Id {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.0.fmt(f)
  }
}

/// The output of an action.
/// 
/// When an [`Action`] is run, it can return a value
/// back to the context. This value can be used by
/// other actions depending on said value.
/// 
/// [`Action`]: trait.Action.html
#[derive(Debug, Clone)]
pub enum ActionOutput {
  /// A string.
  String(String),
  /// An integer (i64).
  Integer(i64),
  /// A floating-point number (f64).
  Float(f64),
  /// A boolean.
  Boolean(bool)
}

/// An input for an action.
/// 
/// Action inputs are not required to use this
/// enum, but it is recommended to do so. It allows
/// users to pass both static values and dependency
/// outputs to actions.
pub enum ActionInput<T> {
  /// A static value.
  Static(T),
  /// A value from an action.
  Action(Arc<dyn Action>)
}

impl<T> ActionInput<T> {
  /// Creates a new input from an action.
  pub fn new_action(value: Arc<dyn Action>) -> Self {
    Self::Action(value)
  }

  /// Creates a new input from a static value.
  pub fn new_static(value: T) -> Self {
    Self::Static(value)
  }

  /// Returns the static value, or `None` if the input
  /// is an action.
  pub fn static_value(&self) -> Option<&T> {
    match self {
      Self::Static(value) => Some(value),
      _ => None
    }
  }

  /// Returns the action, or `None` if the input is
  /// static.
  pub fn action(&self) -> Option<Arc<dyn Action>> {
    match self {
      Self::Action(action) => Some(action.clone()),
      _ => None
    }
  }

  /// Returns `true` if the input is static.
  pub fn is_static(&self) -> bool {
    self.static_value().is_some()
  }

  /// Returns `true` if the input is an action.
  pub fn is_action(&self) -> bool {
    self.action().is_some()
  }

  /// Returns the static value, or panics if the input
  /// is an action.
  pub fn unwrap_static(&self) -> &T {
    self.static_value().unwrap()
  }

  /// Returns the action, or panics if the input is
  /// static.
  pub fn unwrap_action(&self) -> Arc<dyn Action> {
    self.action().unwrap()
  }
}

impl<T> From<T> for ActionInput<T> {
  fn from(value: T) -> Self {
    Self::new_static(value)
  }
}

impl<T: Default> Default for ActionInput<T> {
  fn default() -> Self {
    Self::new_static(T::default())
  }
}