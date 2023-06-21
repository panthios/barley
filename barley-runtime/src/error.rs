use thiserror::Error;


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