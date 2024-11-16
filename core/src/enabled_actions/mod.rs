mod dump;
mod load;

pub use dump::dump_enabled_actions;
pub use load::load_enabled_actions;

use crate::utils::ActionPath;

pub enum ActionStatus {
    Enabled,
    Disabled,
}

impl ActionStatus {
    pub fn from_flags(enabled: bool, disabled: bool) -> Option<Self> {
        match (enabled, disabled) {
            (true, false) => Some(Self::Enabled),
            (false, true) => Some(Self::Disabled),
            (_, _) => None,
        }
    }
}

#[derive(Debug)]
pub enum EnabledEntry {
    Action(ActionPath),
}

#[derive(Debug)]
pub struct EnabledList {
    enabled: Vec<EnabledEntry>,
}

impl EnabledList {
    pub fn is_action_enabled(&self, group: &str, action: &str) -> bool {
        self.enabled.iter().any(|e| match e {
            EnabledEntry::Action(a) => a == &ActionPath::from_parts(group, action),
        })
    }

    pub fn is_path_enabled(&self, path: &ActionPath) -> bool {
        self.enabled.iter().any(|e| match e {
            EnabledEntry::Action(a) => a == path,
        })
    }

    pub fn enable_path(&mut self, path: ActionPath) -> bool {
        let already_enabled = self.enabled.iter().any(|e| match e {
            EnabledEntry::Action(a) => a == &path,
        });
        if !already_enabled {
            self.enabled.push(EnabledEntry::Action(path));
        }
        !already_enabled
    }

    pub fn disable_path(&mut self, path: ActionPath) -> bool {
        let indexes_to_remove: Vec<usize> = self
            .enabled
            .iter()
            .enumerate()
            .filter_map(|(i, e)| match e {
                EnabledEntry::Action(a) => {
                    if a == &path {
                        Some(i)
                    } else {
                        None
                    }
                }
            })
            .collect();
        for i in indexes_to_remove.iter().rev() {
            self.enabled.remove(*i);
        }
        !indexes_to_remove.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn are_actions_enabled() {
        let ena = EnabledList {
            enabled: vec![EnabledEntry::Action(ActionPath::from_parts("hoshi", "mi"))],
        };

        assert!(ena.is_action_enabled("hoshi", "mi"));
        assert!(!ena.is_action_enabled("hoshi", "no"));
        assert!(!ena.is_action_enabled("noe", "mi"));
        assert!(!ena.is_action_enabled("miko", "to"));
    }
}
