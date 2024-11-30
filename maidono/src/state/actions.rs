use crate::execution::Context;
use maidono_core::actions::{read_all_groups, Action};
use maidono_core::problem;
use maidono_core::utils::{ActionPath, Result};
use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Actions {
    by_path: BTreeMap<ActionPath, Action>,
}

pub struct ActionRef<'a> {
    pub path: Cow<'a, ActionPath>,
    pub action: &'a Action,
}

impl Actions {
    pub(crate) fn lookup_by_trigger(&self, uri_path: &str) -> Option<ActionRef> {
        for (path, action) in &self.by_path {
            if action.trigger() == uri_path {
                return Some(ActionRef {
                    path: Cow::Borrowed(path),
                    action,
                });
            }
        }
        None
    }

    pub(crate) fn load_context_for<'a>(&'a self, action_ref: ActionRef<'a>) -> Result<Context> {
        Ok(self.load_actions_from_ref(action_ref)?.into())
    }

    fn load_actions_from_path(&self, action_path: ActionPath) -> Result<Vec<ActionRef>> {
        match self.by_path.get(&action_path) {
            None => Err(problem!("action '{}' not found", action_path)),
            Some(action_before) => Ok(self.load_actions_from_ref(ActionRef {
                path: Cow::Owned(action_path),
                action: action_before,
            })?),
        }
    }

    fn load_actions_from_ref<'a>(
        &'a self,
        action_ref: ActionRef<'a>,
    ) -> Result<Vec<ActionRef<'a>>> {
        let mut actions = Vec::new();
        for path_before in action_ref.action.before() {
            let path = ActionPath::parse(path_before)?;
            actions.append(&mut self.load_actions_from_path(path)?);
        }
        let this_action_idx = actions.len();
        for path_after in action_ref.action.after() {
            let path = ActionPath::parse(path_after)?;
            actions.append(&mut self.load_actions_from_path(path)?);
        }
        actions.insert(this_action_idx, action_ref);
        Ok(actions)
    }
}

pub fn load_initial_actions() -> Result<Actions> {
    let groups = read_all_groups()?;
    let mut actions = Actions {
        by_path: BTreeMap::new(),
    };

    for (group_name, group) in groups {
        for (action_name, action) in group.into_enumerated_actions() {
            let path = ActionPath::from_parts(group_name.clone(), action_name);
            actions.by_path.insert(path, action);
        }
    }

    Ok(actions)
}
