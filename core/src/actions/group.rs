use super::action::{Action, ActionEntry};
use crate::problem;
use crate::utils::{Report, Result};
use serde::Deserialize;
use std::collections::hash_map;
use std::collections::HashMap;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(transparent)]
pub struct GroupActions {
    actions: Vec<ActionEntry>,
}

#[derive(Debug)]
pub struct Group {
    actions: HashMap<String, Action>,
}

impl Group {
    pub fn from_deserialized(deserialized_actions: GroupActions) -> Result<Self> {
        let mut action_map = HashMap::new();
        let mut report = Report::new();

        for action_entry in deserialized_actions.actions {
            if let Ok((name, action)) = Action::from_deserialized(action_entry) {
                if action_map.contains_key(&name) {
                    report.add(problem!("Duplicate action named '{}'", name))
                } else {
                    action_map.insert(name, action);
                }
            }
        }

        report.wrap(Self {
            actions: action_map,
        })
    }

    pub fn action_count(&self) -> usize {
        self.actions.len()
    }

    pub fn enumerate_actions(&self) -> hash_map::Iter<'_, String, Action> {
        self.actions.iter()
    }

    pub fn enumerate_action_names(&self) -> hash_map::Keys<'_, String, Action> {
        self.actions.keys()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actions::action::fakes::*;
    use indoc::indoc;

    #[test]
    fn yaml_empty() {
        let result = serde_yaml::from_str::<GroupActions>("");

        assert!(result.unwrap().actions.is_empty());
    }

    #[test]
    fn yaml() {
        let expected = GroupActions {
            actions: vec![
                make_test_action_entry("action1"),
                make_test_action_entry("action2"),
                make_test_action_entry("action3"),
            ],
        };

        let parsed = serde_yaml::from_str::<GroupActions>(indoc! {"
            - name: action1
              on: POST /test/action
              run: some-command
            - name: action2
              on: POST /test/action
              run: some-command
            - name: action3
              on: POST /test/action
              run: some-command
        "});

        assert_eq!(expected, parsed.unwrap());
    }

    #[test]
    fn yaml_duplicate() {
        let parsed = serde_yaml::from_str::<GroupActions>(indoc! {"
            - name: action1
              on: POST /test/action
              run: some-command
            - name: action1
              on: POST /test/action
              run: some-command
        "});

        assert!(parsed.is_ok());
        assert!(Group::from_deserialized(parsed.unwrap()).is_err())
    }
}
