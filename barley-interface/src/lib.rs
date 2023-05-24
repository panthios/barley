#![deny(missing_docs)]

//! `barley-interface`
//! 
//! This crate provides a simple CLI interface for the `barley`
//! workflow engine. It should be used instead of the [`Context`]
//! struct from the `barley-runtime` crate, since it provides
//! debug callbacks for progress tracking.

use barley_runtime::prelude::*;
use std::sync::Arc;
use tokio::sync::RwLock;
use colored::*;

/// A simple CLI interface for the `barley` workflow engine.
/// 
/// This interface is not yet complete, but should be used instead
/// of the [`Context`] struct from the `barley-runtime` crate,
/// since it will require no extra modifications when stable.
pub struct Interface {
    ctx: Arc<RwLock<Context>>
}

impl Interface {
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
    pub async fn add_action<A: Action + 'static>(&self, action: A) -> Arc<dyn Action + 'static> {
        self.ctx.clone().add_action(action).await
    }

    /// Run the context.
    pub async fn run(&self) -> Result<()> {
        self.ctx.clone().run().await
    }

    /// Gets the output of the action.
    /// 
    /// This method will return `None` if the action
    /// has not been run yet. See [`Context::get_output`]
    /// for more information.
    /// 
    /// [`Context::get_output`]: https://docs.rs/barley-runtime/latest/barley_runtime/struct.Context.html#method.get_output
    pub async fn get_output(&self, action: &dyn Action) -> Option<ActionOutput> {
        self.ctx.clone().get_output(action).await
    }

    /// Gets the output of an action Arc.
    /// 
    /// See [`Context::get_output_arc`] for more information.
    /// 
    /// [`Context::get_output_arc`]: https://docs.rs/barley-runtime/latest/barley_runtime/struct.Context.html#method.get_output_arc
    pub async fn get_output_arc(&self, action: Arc<dyn Action + 'static>) -> Option<ActionOutput> {
        self.ctx.clone().get_output_arc(action).await
    }

    pub(crate) fn on_action_started(action: &dyn Action) {
        let display_name = action.display_name();

        if !display_name.is_empty() {
            println!("{} {}", "[STARTED]".yellow(), display_name);
            return;
        }
    }

    pub(crate) fn on_action_finished(action: &dyn Action) {
        let display_name = action.display_name();

        if !display_name.is_empty() {
            println!("{} {}", "[FINISHED]".green(), display_name);
            return;
        }
    }

    pub(crate) fn on_action_failed(action: &dyn Action, _err: &Error) {
        let display_name = action.display_name();

        if !display_name.is_empty() {
            println!("{} {}", "[FAILED]".red(), display_name);
            return;
        }
    }
}