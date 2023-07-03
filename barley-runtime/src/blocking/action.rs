use crate::{Id, Probe, Operation};
use crate::error::Error;
use crate::output::Output;
use super::runtime::{Runtime, Builder};

/// An action that can be run by the Barley runtime.
/// 
/// This trait is similar to the version at the crate
/// root, but this one is synchronous. Using this trait
/// may be useful when multithreading is not desired.
pub trait Action {
    /// Run the action.
    /// 
    /// This function should never be called directly.
    /// Use the [`Runtime`] to run the workflow.
    /// 
    /// # Errors
    /// 
    /// This function will return an error if the action
    /// fails. All error codes are handled internally.
    /// 
    /// [`Runtime`]: https://docs.rs/barley-runtime/latest/barley_runtime/blocking/struct.Runtime.html
    fn run(&self, runtime: &Runtime, operation: Operation) -> Result<Option<Output>, Error>;

    /// Get metadata about the action.
    /// 
    /// This function should never be called directly.
    /// Use the [`Runtime`] to run the workflow.
    /// 
    /// # Errors
    /// 
    /// Some actions need to run other code before they
    /// can get their metadata. This function will return
    /// an error if that code fails. All error codes are
    /// handled internally.
    /// 
    /// [`Runtime`]: https://docs.rs/barley-runtime/latest/barley_runtime/blocking/struct.Runtime.html
    fn probe(&self) -> Result<Probe, Error>;

    /// Load the state of the action.
    /// 
    /// This function should never be called directly.
    /// Use the [`Runtime`] to run the workflow.
    /// 
    /// [`Runtime`]: https://docs.rs/barley-runtime/latest/barley_runtime/blocking/struct.Runtime.html
    fn load_state(&self, _builder: &mut Builder) {}

    /// Get the display name of the action.
    /// 
    /// This function should never be called directly.
    /// Use the [`Runtime`] to run the workflow.
    /// 
    /// [`Runtime`]: https://docs.rs/barley-runtime/latest/barley_runtime/blocking/struct.Runtime.html
    fn display_name(&self) -> String;
}

/// A node in the workflow graph.
/// 
/// Nodes have a list of dependencies, and they can
/// be run by the runtime.
pub struct Node<'node> {
    action: Box<dyn Action>,
    deps: Vec<&'node Node<'node>>,
    pub(crate) id: Id
}

impl<'node> Node<'node> {
    /// Create a node from a boxed action.
    /// 
    /// You should convert your action into a
    /// node using [`From`] when possible.
    /// 
    /// [`From`]: https://doc.rust-lang.org/std/convert/trait.From.html
    #[must_use]
    pub fn new(action: Box<dyn Action>) -> Self {
        Self {
            action,
            deps: Vec::new(),
            id: Id::default()
        }
    }

    /// Get the display name of the action.
    /// 
    /// This function should never be called directly.
    /// Use the [`Runtime`] to run the workflow.
    /// 
    /// [`Runtime`]: https://docs.rs/barley-runtime/latest/barley_runtime/blocking/struct.Runtime.html
    #[must_use]
    pub fn display_name(&self) -> String {
        self.action.display_name()
    }

    pub(crate) fn id(&self) -> Id {
        self.id
    }

    pub(crate) fn deps(&self) -> Vec<&'node Node<'node>> {
        self.deps.clone()
    }

    pub(crate) fn probe(&self) -> Result<Probe, Error> {
        self.action.probe()
    }

    pub(crate) fn run(&self, runtime: &Runtime, operation: Operation) -> Result<Option<Output>, Error> {
        self.action.run(runtime, operation)
    }

    /// Add a dependency to the node.
    ///
    /// Unlike in the asynchronous version of this
    /// library, this function takes a reference to
    /// the node instead of an owned [`Arc`]. This
    /// is because the blocking runtime does not
    /// need to worry about multithreading.
    /// 
    /// [`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
    pub fn requires(&mut self, action: &'node Node) {
        self.deps.push(action);
    }

    /// Load the state of the action.
    /// 
    /// This function should never be called directly.
    /// Use the [`Runtime`] to run the workflow.
    /// 
    /// [`Runtime`]: https://docs.rs/barley-runtime/latest/barley_runtime/blocking/struct.Runtime.html
    pub fn load_state(&self, builder: &mut Builder) {
        self.action.load_state(builder);
    }
}
