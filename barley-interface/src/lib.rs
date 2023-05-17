#![deny(missing_docs)]

//! `barley-interface`
//! 
//! This crate provides a simple CLI interface for the `barley`
//! workflow engine. It should be used instead of the [`Context`]
//! struct from the `barley-runtime` crate, since it provides
//! debug callbacks for progress tracking.

use barley_runtime::*;
use std::sync::Arc;

/// A simple CLI interface for the `barley` workflow engine.
/// 
/// This interface is not yet complete, but should be used instead
/// of the [`Context`] struct from the `barley-runtime` crate,
/// since it will require no extra modifications when stable.
pub struct Interface<'me> {
    ctx: Context<'me>
}

impl<'me> Interface<'me> {
    /// Create a new `Interface`.
    pub fn new() -> Self {
        let callbacks = ContextCallbacks {
            on_action_started: Some(Self::on_action_started),
            on_action_finished: Some(Self::on_action_finished),
            on_action_failed: Some(Self::on_action_failed)
        };

        Self {
            ctx: Context::new(callbacks)
        }
    }

    /// Add an action to the context.
    pub fn add_action<A: Action + 'me>(&mut self, action: A) -> Arc<dyn Action + 'me> {
        self.ctx.add_action(action)
    }

    /// Run the context.
    pub async fn run(&mut self) -> Result<()> {
        self.ctx.run().await
    }

    pub(crate) fn on_action_started(action: &dyn Action) {
        println!("Started: {}", action.id());
    }

    pub(crate) fn on_action_finished(action: &dyn Action) {
        println!("Finished: {}", action.id());
    }

    pub(crate) fn on_action_failed(action: &dyn Action, _err: &Error) {
        println!("Failed: {}", action.id());
    }
}