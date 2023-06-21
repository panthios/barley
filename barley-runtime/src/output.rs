use crate::ActionError;


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