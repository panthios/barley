use crate::Error;


/// The output of an action.
/// 
/// When an [`Action`] is run, it can return a value
/// back to the context. This value can be used by
/// other actions depending on said value.
/// 
/// [`Action`]: trait.Action.html
#[derive(Debug, Clone)]
pub enum Output {
    /// A string.
    String(String),
    /// An integer (i64).
    Integer(i64),
    /// A floating-point number (f64).
    Float(f64),
    /// A boolean.
    Boolean(bool)
}

impl TryFrom<Output> for String {
    type Error = Error;

    fn try_from(value: Output) -> Result<Self, Self::Error> {
        match value {
            Output::String(value) => Ok(value),
            _ => Err(Error::OutputConversionFailed("String".to_string()))
        }
    }
}

impl TryFrom<Output> for i64 {
    type Error = Error;

    fn try_from(value: Output) -> Result<Self, Self::Error> {
        match value {
            Output::Integer(value) => Ok(value),
            _ => Err(Error::OutputConversionFailed("i64".to_string()))
        }
    }
}

impl TryFrom<Output> for f64 {
    type Error = Error;

    fn try_from(value: Output) -> Result<Self, Self::Error> {
        match value {
            Output::Float(value) => Ok(value),
            _ => Err(Error::OutputConversionFailed("f64".to_string()))
        }
    }
}

impl TryFrom<Output> for bool {
    type Error = Error;

    fn try_from(value: Output) -> Result<Self, Self::Error> {
        match value {
            Output::Boolean(value) => Ok(value),
            _ => Err(Error::OutputConversionFailed("bool".to_string()))
        }
    }
}

impl From<String> for Output {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<i64> for Output {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<f64> for Output {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<bool> for Output {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

impl From<&str> for Output {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}