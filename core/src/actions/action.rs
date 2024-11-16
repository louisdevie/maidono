use super::commands::Commands;
use super::host::HostRef;
use super::refs::{flatten_optional_refs, ActionRefs};
use crate::utils::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields, expecting = "an action")]
pub struct ActionEntry {
    name: String,
    #[serde(rename = "on")]
    trigger: String,
    #[serde(rename = "from", default)]
    origin: Option<HostRef>,
    #[serde(default)]
    secret: Option<String>,
    #[serde(default)]
    before: Option<ActionRefs>,
    #[serde(default)]
    after: Option<ActionRefs>,
    #[serde(rename = "run")]
    action: Commands,
}

#[derive(Debug)]
pub struct Action {
    trigger: String,
    origin: Option<HostRef>,
    secret: Option<String>,
    before: Vec<String>,
    after: Vec<String>,
    action: Commands,
}

impl Action {
    pub fn from_deserialized(deserialized_action: ActionEntry) -> Result<(String, Self)> {
        Ok((
            deserialized_action.name,
            Self {
                trigger: deserialized_action.trigger,
                origin: deserialized_action.origin,
                secret: deserialized_action.secret,
                before: flatten_optional_refs(deserialized_action.before),
                after: flatten_optional_refs(deserialized_action.after),
                action: deserialized_action.action,
            },
        ))
    }

    pub fn trigger(&self) -> &str {
        &self.trigger
    }

    pub fn origin(&self) -> Option<&HostRef> {
        self.origin.as_ref()
    }

    pub fn secret(&self) -> Option<&str> {
        self.secret.as_ref().map(|x| x.as_str())
    }

    pub fn before(&self) -> &Vec<String> {
        &self.before
    }

    pub fn after(&self) -> &Vec<String> {
        &self.after
    }

    pub fn action(&self) -> &Commands {
        &self.action
    }
}

#[cfg(test)]
pub mod fakes {
    use super::*;

    pub fn make_test_action_entry(name: &'_ str) -> ActionEntry {
        ActionEntry {
            name: name.to_owned(),
            trigger: String::from("POST /test/action"),
            origin: None,
            secret: None,
            before: None,
            after: None,
            action: Commands::new(vec![String::from("some-command")]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn yaml_all() {
        let expected = ActionEntry {
            name: String::from("my-action"),
            trigger: String::from("POST /my/action"),
            origin: Some(HostRef::Any),
            secret: Some(String::from("my_secret_key")),
            before: Some(ActionRefs::Single(String::from("other-action"))),
            after: Some(ActionRefs::Multiple(vec![
                String::from("something-else"),
                String::from("other-group/action"),
            ])),
            action: Commands::new(vec![String::from("some-command")]),
        };

        let parsed = serde_yaml::from_str::<ActionEntry>(indoc! {"
            name: my-action
            on: POST /my/action
            from: '*'
            secret: my_secret_key
            before: other-action
            after: [ something-else, other-group/action ]
            run: some-command
        "});

        assert_eq!(expected, parsed.unwrap());
    }

    #[test]
    fn yaml_minimal() {
        let expected = ActionEntry {
            name: String::from("my-action"),
            trigger: String::from("POST /my/action"),
            origin: None,
            secret: None,
            before: None,
            after: None,
            action: Commands::new(vec![String::from("some-command")]),
        };

        let parsed = serde_yaml::from_str::<ActionEntry>(indoc! {"
            name: my-action
            on: POST /my/action
            run: some-command
        "});

        assert_eq!(expected, parsed.unwrap());
    }
}
