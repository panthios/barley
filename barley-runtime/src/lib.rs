#![deny(missing_docs)]
#![warn(
  clippy::pedantic
)]

//! `barley-runtime`
//! 
//! This crate contains the runtime for the `barley` workflow engine. It
//! provides the [`Action`] trait, which is the main interface for defining
//! actions that can be executed by the engine. It also provides the
//! [`Runtime`] struct, which is used to run workflows.
//! 
//! [`Action`]: trait.Action.html
//! [`Runtime`]: struct.Runtime.html

use uuid::Uuid;
use std::sync::Arc;
use thiserror::Error;
use async_trait::async_trait;

/// The prelude for the `barley-runtime` crate.
/// 
/// This module contains all of the important types
/// and traits for the `barley-runtime` crate. It
/// should be used instead of importing the types
/// directly.
pub mod prelude;

mod context;
mod runtime;
mod scope;

pub use runtime::{Runtime, RuntimeBuilder};
pub use scope::Scope;

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
  /// This method takes a [`Runtime`] object, which
  /// contains the context for the action. It also
  /// takes an [`Operation`], which is used to
  /// determine what the action should do.
  async fn run(&self, runtime: Runtime, operation: Operation) -> Result<Option<ActionOutput>, ActionError>;

  /// Probe the action for specific information.
  async fn probe(&self, runtime: Runtime) -> Result<Probe, ActionError>;

  /// Load required state.
  async fn load_state(&self, _builder: &mut RuntimeBuilder) {}

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
  #[must_use]
  pub fn display_name(&self) -> String {
    self.action.display_name()
  }

  pub(crate) fn id(&self) -> Id {
    self.id
  }

  pub(crate) fn deps(&self) -> Vec<ActionObject> {
    self.deps.clone()
  }

  pub(crate) async fn probe(&self, ctx: Runtime) -> Result<Probe, ActionError> {
    self.action.probe(ctx).await
  }

  pub(crate) async fn run(&self, ctx: Runtime, operation: Operation) -> Result<Option<ActionOutput>, ActionError> {
    self.action.run(ctx, operation).await
  }

  /// Add a dependency to the action.
  pub fn requires(&mut self, action: ActionObject) {
    self.deps.push(action);
  }

  /// Load the state
  pub async fn load_state(&self, builder: &mut RuntimeBuilder) {
    self.action.load_state(builder).await;
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
  pub on_action_failed: Option<fn(ActionObject, &ActionError)>
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

impl TryFrom<ActionOutput> for String {
  type Error = ActionError;

  fn try_from(value: ActionOutput) -> Result<Self, Self::Error> {
    match value {
      ActionOutput::String(value) => Ok(value),
      _ => Err(ActionError::OutputConversionFailed("String".to_string()))
    }
  }
}

impl TryFrom<ActionOutput> for i64 {
  type Error = ActionError;

  fn try_from(value: ActionOutput) -> Result<Self, Self::Error> {
    match value {
      ActionOutput::Integer(value) => Ok(value),
      _ => Err(ActionError::OutputConversionFailed("i64".to_string()))
    }
  }
}

impl TryFrom<ActionOutput> for f64 {
  type Error = ActionError;

  fn try_from(value: ActionOutput) -> Result<Self, Self::Error> {
    match value {
      ActionOutput::Float(value) => Ok(value),
      _ => Err(ActionError::OutputConversionFailed("f64".to_string()))
    }
  }
}

impl TryFrom<ActionOutput> for bool {
  type Error = ActionError;

  fn try_from(value: ActionOutput) -> Result<Self, Self::Error> {
    match value {
      ActionOutput::Boolean(value) => Ok(value),
      _ => Err(ActionError::OutputConversionFailed("bool".to_string()))
    }
  }
}

impl From<String> for ActionOutput {
  fn from(value: String) -> Self {
    Self::String(value)
  }
}

impl From<i64> for ActionOutput {
  fn from(value: i64) -> Self {
    Self::Integer(value)
  }
}

impl From<f64> for ActionOutput {
  fn from(value: f64) -> Self {
    Self::Float(value)
  }
}

impl From<bool> for ActionOutput {
  fn from(value: bool) -> Self {
    Self::Boolean(value)
  }
}

impl From<&str> for ActionOutput {
  fn from(value: &str) -> Self {
    Self::String(value.to_string())
  }
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
  Dynamic(ActionObject)
}

impl<T> ActionInput<T> {
  /// Creates a new input from an action.
  #[must_use]
  pub fn new_dynamic(value: ActionObject) -> Self {
    Self::Dynamic(value)
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
      Self::Dynamic(_) => None
    }
  }

  /// Returns the action, or `None` if the input is
  /// static.
  pub fn dynamic(&self) -> Option<ActionObject> {
    match self {
      Self::Dynamic(action) => Some(action.clone()),
      Self::Static(_) => None
    }
  }

  /// Returns `true` if the input is static.
  pub fn is_static(&self) -> bool {
    self.static_value().is_some()
  }

  /// Returns `true` if the input is an action.
  pub fn is_dynamic(&self) -> bool {
    self.dynamic().is_some()
  }

  /// Returns the static value, and panics if the
  /// input is an action.
  /// 
  /// # Panics
  /// 
  /// This method panics if the input is a
  /// dynamic value.
  #[deprecated(since = "0.7.0", note = "Use a direct unwrapper like `is_X`, `match`, or `if let` instead")]
  pub fn unwrap_static(&self) -> &T {
    self.static_value().unwrap()
  }

  /// Returns the action, and panics if the input is
  /// static.
  /// 
  /// # Panics
  /// 
  /// This method panics if the input is a static
  /// value.
  #[deprecated(since = "0.7.0", note = "Use a direct unwrapper like `is_X`, `match`, or `if let` instead")]
  pub fn unwrap_dynamic(&self) -> ActionObject {
    self.dynamic().unwrap()
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

/// Any error that can occur during an action.
#[derive(Debug, Error, Clone)]
#[non_exhaustive]
pub enum ActionError {
  /// An error occured internally in the action.
  #[error("{0}")]
  ActionFailed(String, String),
  /// Action output conversion failed.
  #[error("Could not convert ActionOutput to {0}")]
  OutputConversionFailed(String),
  /// An internal error occured, and should be reported.
  #[error("An internal error occured, please report this error code: {0}")]
  InternalError(&'static str),
  /// An action which should have returned a value did not.
  #[error("Dependency did not return a value")]
  NoActionReturn,
  /// The operation is not supported by the action.
  #[error("Operation not supported")]
  OperationNotSupported,
  /// Required state was not loaded.
  #[error("Required state was not loaded")]
  StateNotLoaded
}

/// The operation to perform.
/// 
/// This enum is used to determine what an action
/// should do. It is used by the [`run`] method.
/// 
/// [`run`]: trait.Action.html#method.run
pub enum Operation {
  /// Perform the action.
  Perform,
  /// Rollback the action.
  Rollback
}

/// A probe for an action.
/// 
/// This struct is returned by the [`probe`] method
/// of an [`Action`]. It contains information about
/// the action, such as whether it needs to be run
/// or not.
/// 
/// [`probe`]: trait.Action.html#method.probe
/// [`Action`]: trait.Action.html
#[derive(Debug, Clone)]
pub struct Probe {
  /// Whether the action needs to be run.
  pub needs_run: bool,
  /// Whether `rollback` is available.
  pub can_rollback: bool
}

impl Default for Probe {
  fn default() -> Self {
    Self {
      needs_run: true,
      can_rollback: false
    }
  }
}
