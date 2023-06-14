use crate::ActionObject;


#[derive(Default, Clone)]
pub struct Scope {
    actions: Vec<ActionObject>
}

impl Scope {
    pub fn new() -> Self {
        Self {
            actions: Vec::new()
        }
    }

    pub fn add_action<A: Into<ActionObject>>(&mut self, action: A) -> ActionObject {
        let action = action.into();
        self.actions.push(action.clone());
        action
    }

    pub fn actions(&self) -> &[ActionObject] {
        &self.actions
    }
}
