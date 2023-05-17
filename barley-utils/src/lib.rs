#![deny(missing_docs)]

//! `barley-utils`
//! 
//! This crate provides various utilities for the `barley` workflow
//! engine. Most available utilities are behind feature flags. See
//! each module's documentation for more information.

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