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
use thiserror::Error;

/// The prelude for the `barley-runtime` crate.
/// 
/// This module contains all of the important types
/// and traits for the `barley-runtime` crate. It
/// should be used instead of importing the types
/// directly.
#[cfg(feature = "async")]
pub mod prelude;

/// Synchronous versions of the default runtime.
/// 
/// `barley-runtime` is asynchronous by default. This
/// module contains a synchronous version of the
/// default runtime. Certain use cases may require
/// or benefit from a synchronous runtime. When/if this
/// crate supports `no_std`, only the synchronous
/// runtime will be available under `no_std`.
#[cfg(feature = "blocking")]
pub mod blocking;

cfg_if::cfg_if! {
    if #[cfg(feature = "async")] {
        mod context;
        mod runtime;
        mod scope;
        mod action;
        mod input;

        pub use runtime::{Runtime, RuntimeBuilder};
        pub use action::{Action, Node};
        pub use input::Input;
        pub use scope::Scope;
    }
}

mod error;
pub use error::Error;

mod output;
pub use output::Output;

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

/// The operation to perform.
/// 
/// This enum is used to determine what an action
/// should do. It is used by the [`run`] method.
/// 
/// [`run`]: trait.Action.html#method.run
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
