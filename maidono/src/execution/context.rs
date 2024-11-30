use crate::state::ActionRef;
use maidono_core::actions::Commands;
use maidono_core::utils::ActionPath;

pub struct Context {
    actions: Vec<ContextAction>,
}

pub struct ContextAction {
    path: ActionPath,
    commands: Commands,
}

impl Context {
    pub fn actions(&self) -> &[ContextAction] {
        &self.actions
    }
}

impl From<Vec<ActionRef<'_>>> for Context {
    fn from(value: Vec<ActionRef<'_>>) -> Self {
        Self {
            actions: value.iter().map(|action_ref| action_ref.into()).collect(),
        }
    }
}

impl From<&ActionRef<'_>> for ContextAction {
    fn from(value: &ActionRef<'_>) -> Self {
        Self {
            path: value.path.as_ref().clone(),
            commands: value.action.action().clone(),
        }
    }
}

impl ContextAction {
    pub fn path(&self) -> &ActionPath {
        &self.path
    }

    pub fn commands(&self) -> &Commands {
        &self.commands
    }
}
