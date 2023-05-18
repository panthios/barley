#![deny(missing_docs)]

//! `barley-interface`
//! 
//! This crate provides a simple CLI interface for the `barley`
//! workflow engine. It should be used instead of the [`Context`]
//! struct from the `barley-runtime` crate, since it provides
//! debug callbacks for progress tracking.

use barley_runtime::*;
use std::sync::Arc;
use colored::*;

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

    /// Gets the output of the action.
    /// 
    /// This method will return `None` if the action
    /// has not been run yet. See [`Context::get_output`]
    /// for more information.
    /// 
    /// [`Context::get_output`]: https://docs.rs/barley-runtime/latest/barley_runtime/struct.Context.html#method.get_output
    pub fn get_output(&self, action: &dyn Action) -> Option<&ActionOutput> {
        self.ctx.get_output(action)
    }

    /// Gets the output of an action Arc.
    /// 
    /// See [`Context::get_output_arc`] for more information.
    /// 
    /// [`Context::get_output_arc`]: https://docs.rs/barley-runtime/latest/barley_runtime/struct.Context.html#method.get_output_arc
    pub fn get_output_arc(&self, action: Arc<dyn Action + 'me>) -> Option<&ActionOutput> {
        self.ctx.get_output_arc(action)
    }

    pub(crate) fn on_action_started(action: &dyn Action) {
        println!("{} {}", "[STARTED]".yellow() ,action.display_name());
    }

    pub(crate) fn on_action_finished(action: &dyn Action) {
        println!("{} {}", "[FINISHED]".green(), action.display_name());
    }

    pub(crate) fn on_action_failed(action: &dyn Action, _err: &Error) {
        println!("{} {}", "[FAILED]".red(), action.display_name());
    }
}