pub use std::sync::Arc;
pub use tokio::sync::RwLock;
pub use async_trait::async_trait;
pub use crate::{
    Action, Runtime,
    RuntimeBuilder, Probe,
    Operation, Scope
};

#[cfg(not(feature = "next"))]
pub use crate::{
    error::Error as ActionError,
    output::Output as ActionOutput,
    input::Input as ActionInput,
    action::Node as ActionObject
};

#[cfg(feature = "next")]
pub use crate::{
    error::Error,
    output::Output,
    input::Input,
    action::Node as ActionObject
};
