#![deny(missing_docs)]

//! `barley-utils`
//! 
//! This crate provides various utilities for the `barley` workflow
//! engine. Most available utilities are behind feature flags. See
//! each item's documentation for more information.

use barley_runtime::*;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Provides time-based utilities.
/// 
/// These utilities do not track their progress. This may
/// be a problem, since a timer will run more than once if
/// another action has it as a dependency.
#[cfg(feature = "time")]
pub mod time;

/// Provides filesystem access.
#[cfg(feature = "fs")]
pub mod fs;

/// Provides HTTP request utilities.
#[cfg(feature = "http")]
pub mod http;

/// Provides process spawning and management utilities.
#[cfg(feature = "process")]
pub mod process;


/// Waits for multiple actions to complete.
/// 
/// This action will wait for all of its dependencies to complete
/// before completing itself. It will not run any of its dependencies;
/// that is the runtime's job.
#[barley_action]
#[derive(Default)]
pub struct Join {}

impl Join {
  /// Creates a new `Join` action with the given dependencies.
  pub fn new(actions: Vec<Arc<dyn Action>>) -> Self {
    let mut join = Self::default();

    for action in actions {
      join.add_dep(action);
    }

    join
  }
}

#[barley_action]
#[async_trait]
impl Action for Join {
  async fn check(&self, _ctx: Arc<RwLock<Context>>) -> Result<bool> {
    Ok(false)
  }

  async fn perform(&self, _ctx: Arc<RwLock<Context>>) -> Result<Option<ActionOutput>> {
    Ok(None)
  }

  async fn rollback(&self, _ctx: Arc<RwLock<Context>>) -> Result<()> {
    Ok(())
  }

  fn display_name(&self) -> String {
    "Join".to_string()
  }
}