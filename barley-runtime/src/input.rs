use crate::action::Node;


/// An input for an action.
/// 
/// Action inputs are not required to use this
/// enum, but it is recommended to do so. It allows
/// users to pass both static values and dependency
/// outputs to actions.
pub enum Input<T> {
    /// A static value.
    Static(T),
    /// A value from an action.
    Dynamic(Node)
}

impl<T> Input<T> {
    /// Creates a new input from an action.
    #[must_use]
    pub fn new_dynamic(value: Node) -> Self {
        Self::Dynamic(value)
    }

    /// Creates a new input from a static value.
    pub fn new_static(value: T) -> Self {
        Self::Static(value)
    }

    /// Returns the static value, or `None` if the input
    /// is an action.
    pub fn static_value(&self) -> Option<&T> {
        match self {
            Self::Static(value) => Some(value),
            Self::Dynamic(_) => None
        }
    }

    /// Returns the action, or `None` if the input is
    /// static.
    pub fn dynamic(&self) -> Option<Node> {
        match self {
            Self::Dynamic(action) => Some(action.clone()),
            Self::Static(_) => None
        }
    }

    /// Returns `true` if the input is static.
    pub fn is_static(&self) -> bool {
        self.static_value().is_some()
    }

    /// Returns `true` if the input is an action.
    pub fn is_dynamic(&self) -> bool {
        self.dynamic().is_some()
    }

    /// Returns the static value, and panics if the
    /// input is an action.
    /// 
    /// # Panics
    /// 
    /// This method panics if the input is a
    /// dynamic value.
    #[deprecated(since = "0.6.1", note = "Use a direct unwrapper like `is_X`, `match`, or `if let` instead")]
    pub fn unwrap_static(&self) -> &T {
        self.static_value().unwrap()
    }

    /// Returns the action, and panics if the input is
    /// static.
    /// 
    /// # Panics
    /// 
    /// This method panics if the input is a static
    /// value.
    #[deprecated(since = "0.6.1", note = "Use a direct unwrapper like `is_X`, `match`, or `if let` instead")]
    pub fn unwrap_dynamic(&self) -> Node {
        self.dynamic().unwrap()
    }
}

impl<T> From<T> for Input<T> {
    fn from(value: T) -> Self {
        Self::new_static(value)
    }
}

impl<T: Default> Default for Input<T> {
    fn default() -> Self {
        Self::new_static(T::default())
    }
}