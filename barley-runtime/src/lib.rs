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
use tokio::sync::{RwLock, Barrier};
use std::collections::HashMap;
use uuid::Uuid;
use futures::future::join_all;
use std::sync::Arc;
use async_trait::async_trait;

/// The prelude for the `barley-runtime` crate.
/// 
/// This module contains all of the important types
/// and traits for the `barley-runtime` crate. It
/// should be used instead of importing the types
/// directly.
pub mod prelude;

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
  async fn check(&self, ctx: Arc<RwLock<Context>>) -> Result<bool>;

  /// Check if the action's dependencies need to be run.
  /// 
  /// This method is called internally, and should not
  /// be called directly. It is used to check if any
  /// of the action's dependencies need to be run.
  async fn check_deps(&self, ctx: Arc<RwLock<Context>>) -> Result<bool>;

  /// Run the action.
  async fn perform(&self, ctx: Arc<RwLock<Context>>) -> Result<Option<ActionOutput>>;

  /// Undo the action.
  /// 
  /// This is not currently possible, and will not
  /// do anything. This will be usable in a future
  /// version of Barley.
  async fn rollback(&self, ctx: Arc<RwLock<Context>>) -> Result<()>;

  /// Get the action's ID.
  fn id(&self) -> Id;

  /// Add a direct dependency to the action.
  /// 
  /// This action will not run until the dependency
  /// has been run. This behavior is 100% guaranteed
  /// by the engine.
  fn requires(&mut self, action: Arc<dyn Action>);

  /// Get the display name of the action.
  fn display_name(&self) -> String;

  /// Get a list of dependencies.
  /// 
  /// This method is used internally, and should not
  /// be called directly. Dependencies are automatically
  /// handled by the engine.
  fn deps(&self) -> Vec<Arc<dyn Action>>;
}

/// An action object.
/// 
/// This is just a type alias, and does
/// not add any functionality.
pub type ActionObject = Arc<dyn Action + 'static>;

/// A context for running actions.
/// 
/// There should only be one of these per workflow
#[derive(Default)]
pub struct Context {
  actions: Vec<Arc<dyn Action + 'static>>,
  variables: HashMap<String, String>,
  callbacks: ContextCallbacks,
  outputs: HashMap<Id, ActionOutput>,
  barriers: HashMap<Id, Arc<Barrier>>
}

/// The abstract interface for a context.
/// 
/// This should always be used in any program using
/// Barley, but it should never be implemented
/// directly. Use the `barley-interface` crate
/// instead.
#[async_trait]
pub trait ContextAbstract {
  /// Add an action to the context.
  /// 
  /// This method adds an action to the context, and
  /// returns a reference to the action. The action
  /// will be run when the context is run.
  /// 
  /// You can use the returned reference as a
  /// dependency for other actions.
  async fn add_action<A: Action + 'static>(self, action: A) -> Arc<dyn Action + 'static>;

  /// Run the context.
  /// 
  /// While processing the actions, it will
  /// call the callbacks if they are set.
  async fn run(self) -> Result<()>;

  /// Run an individual action.
  /// 
  /// This is called internally, and should not
  /// be called directly. It is used to run
  /// individual actions, and to check if their
  /// outputs are available and successful.
  async fn run_action(self, action: Arc<dyn Action>) -> Result<Option<ActionOutput>>;

  /// Sets a variable in the context.
  /// 
  /// This can be used to send information between
  /// actions. For example, you could set a return code
  /// in one action, and check it in another.
  async fn set_variable(self, name: &str, value: &str);

  /// Gets a variable from the context.
  /// 
  /// If the variable doesn't exist, this method
  /// returns `None`.
  async fn get_variable(self, name: &str) -> Option<String>;

  /// Sets a local variable for the action.
  /// 
  /// This variable will be namespaced to the action,
  /// and will not be visible to other actions in any
  /// reasonable way. Actions could fetch the ID of the
  /// current action, and use that to access the variable,
  /// but that's voodoo magic and you shouldn't do it.
  async fn set_local(self, action: &dyn Action, name: &str, value: &str);

  /// Gets a local variable for the action.
  /// 
  /// This variable will be namespaced to the action,
  /// and will not be visible to other actions in any
  /// reasonable way. Actions could fetch the ID of the
  /// current action, and use that to access the variable,
  /// but that's voodoo magic and you shouldn't do it.
  async fn get_local(self, action: &dyn Action, name: &str) -> Option<String>;

  /// Gets the output of the action.
  /// 
  /// If the action has not been run yet, this will return
  /// `None`, regardless of the action's value after running
  /// it. If you are using this outside of an action, you
  /// should only use it after the context has been run.
  async fn get_output(self, action: &dyn Action) -> Option<ActionOutput>;

  /// Gets the output of an action Arc
  /// 
  /// This should be used instead of [`Context::get_output`]
  /// if you have an [`Arc`] to the action.
  /// 
  /// [`Context::get_output`]: struct.Context.html#method.get_output
  /// [`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
  async fn get_output_arc(self, action: Arc<dyn Action>) -> Option<ActionOutput>;
}

impl Context {
  /// Create a new context with the given callbacks.
  /// 
  /// If you don't want any callbacks, it's recommended
  /// to use the [`Default`] implementation instead.
  /// 
  /// [`Default`]: https://doc.rust-lang.org/std/default/trait.Default.html
  pub fn new(callbacks: ContextCallbacks) -> Arc<RwLock<Self>> {
    Arc::new(RwLock::new(Self {
      actions: Vec::new(),
      variables: HashMap::new(),
      callbacks,
      outputs: HashMap::new(),
      barriers: HashMap::new()
    }))
  }
}

#[async_trait]
impl ContextAbstract for Arc<RwLock<Context>> {
  async fn add_action<A: Action + 'static>(self, action: A) -> Arc<dyn Action> {
    let action = Arc::new(action);

    self.write().await.actions.push(action.clone());

    action
  }

  async fn run(self) -> Result<()> {
    let mut actions = self.read().await.actions.clone();
    let mut dependents: HashMap<Id, usize> = HashMap::new();

    for action in actions.clone() {
      dependents.insert(action.id(), 0);

      let deps = action.clone().deps()
        .iter().map(|a| a.id()).collect::<Vec<_>>();

      for dep in deps {
        dependents.insert(dep, dependents.get(&dep).unwrap_or(&0) + 1);
      }
    }

    for (id, revdeps) in dependents.clone() {
      if revdeps == 0 {
        continue
      }

      self.clone().write().await
        .barriers.insert(id, Arc::new(Barrier::new(revdeps + 1)));
    }

    actions.sort_by(|a, b| {
      let a_revdeps = dependents.get(&a.id()).unwrap_or(&0);
      let b_revdeps = dependents.get(&b.id()).unwrap_or(&0);

      a_revdeps.cmp(b_revdeps)
    });

    let mut handles = Vec::new();
    for action in actions {
      let ctx = self.clone();
      let action = action.clone();

      let dep_ids = action.clone().deps()
        .iter().map(|a| a.id()).collect::<Vec<_>>();
      let barriers = ctx.clone().read().await.barriers.clone();
      let dep_barriers = dep_ids.iter()
        .map(|id| barriers.get(id).unwrap().clone())
        .collect::<Vec<_>>();
      let self_barrier = barriers.get(&action.id()).cloned();

      handles.push(tokio::spawn(async move {
        for barrier in dep_barriers {
          barrier.wait().await;
        }

        let res = ctx.run_action(action).await;

        if let Some(barrier) = self_barrier {
          barrier.wait().await;
        }

        res
      }))
    }

    let results = join_all(handles).await;

    for result in results {
      if let Err(e) = result? {
        return Err(e)
      }
    }

    Ok(())
  }

  async fn run_action(self, action: Arc<dyn Action>) -> Result<Option<ActionOutput>> {
    let callbacks = self.clone().read().await.callbacks.clone();

    if !action.check(self.clone()).await? {
      if let Some(callback) = callbacks.on_action_started {
        callback(action.as_ref());
      }

      let output = action.perform(self.clone()).await;

      if let Err(e) = &output {
        if let Some(callback) = callbacks.on_action_failed {
          callback(action.as_ref(), &e);
        }

        return output
      }

      if let Some(callback) = callbacks.on_action_finished {
        callback(action.as_ref());
      }

      if let Some(output) = output.unwrap() {
        self.clone().write().await.outputs.insert(action.id(), output.clone());
        Ok(Some(output))
      } else {
        Ok(None)
      }
    } else {
      Ok(self.clone().write().await.outputs.get(&action.id()).cloned())
    }
  }

  async fn set_variable(self, name: &str, value: &str) {
    self.write().await.variables.insert(name.to_string(), value.to_string());
  }

  async fn get_variable(self, name: &str) -> Option<String> {
    self.read().await.variables.get(name).cloned()
  }

  async fn set_local(self, action: &dyn Action, name: &str, value: &str) {
    self.set_variable(&format!("{}::{}", action.id(), name), value).await;
  }

  async fn get_local(self, action: &dyn Action, name: &str) -> Option<String> {
    self.get_variable(&format!("{}::{}", action.id(), name)).await
  }

  async fn get_output(self, action: &dyn Action) -> Option<ActionOutput> {
    self.read().await.outputs.get(&action.id()).cloned()
  }

  async fn get_output_arc(self, action: Arc<dyn Action>) -> Option<ActionOutput> {
    self.read().await.outputs.get(&action.id()).cloned()
  }
}

/// Callbacks for the context.
/// 
/// These callbacks are set by interfaces, and are
/// usually not set by scripts directly.
#[derive(Default, Clone)]
pub struct ContextCallbacks {
  /// Called when an action is started.
  pub on_action_started: Option<fn(&dyn Action)>,
  /// Called when an action is completed successfully.
  pub on_action_finished: Option<fn(&dyn Action)>,
  /// Called when an action fails.
  pub on_action_failed: Option<fn(&dyn Action, &anyhow::Error)>
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