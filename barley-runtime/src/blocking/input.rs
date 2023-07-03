use super::action::Node;


/// An input to an action.
/// 
/// This enum is used to represent input data to
/// an action in a workflow. It will be resolved
/// to a specific type once the workflow is run.
pub enum Input<'node, T> {
    /// A static value.
    Static(T),
    /// An action.
    /// 
    /// The output of this action will be used as
    /// the input to the action that this input
    /// belongs to.
    Dynamic(&'node Node<'node>)
}

impl<'node, T> Input<'node, T> {
    /// Create a new static input.
    #[must_use]
    pub fn new_static(value: T) -> Self {
        Self::Static(value)
    }

    /// Create a new dynamic input.
    #[must_use]
    pub fn new_dynamic(value: &'node Node<'node>) -> Self {
        Self::Dynamic(value)
    }

    /// Get the static value of the input.
    /// 
    /// If the input is dynamic, this will return
    /// `None`.
    pub fn static_value(&self) -> Option<&T> {
        match self {
            Self::Static(value) => Some(value),
            Self::Dynamic(_) => None
        }
    }

    /// Get the dynamic value of the input.
    /// 
    /// If the input is static, this will return
    /// `None`.
    pub fn dynamic(&self) -> Option<&'node Node<'node>> {
        match self {
            Self::Dynamic(action) => Some(action),
            Self::Static(_) => None
        }
    }

    /// Check if the input is static.
    pub fn is_static(&self) -> bool {
        self.static_value().is_some()
    }

    /// Check if the input is dynamic.
    pub fn is_dynamic(&self) -> bool {
        self.dynamic().is_some()
    }
}
