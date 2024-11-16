use crate::problem;
use crate::utils::error::Result;
use crate::utils::split_in_two;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub struct ActionPath {
    group_part: String,
    action_part: String,
}

impl ActionPath {
    pub fn parse(path: &str) -> Result<ActionPath> {
        match split_in_two(path, '/') {
            (_, None) => Err(problem!("Missing action part in path '{}'", path)),
            (group, Some(action)) => Ok(ActionPath {
                group_part: group.to_owned(),
                action_part: action.to_owned(),
            }),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}/{}", self.group_part, self.action_part)
    }

    pub fn from_parts<G: Into<String>, A: Into<String>>(group_part: G, action_part: A) -> Self {
        Self {
            group_part: group_part.into(),
            action_part: action_part.into(),
        }
    }

    pub fn into_parts(self) -> (String, String) {
        (self.group_part, self.action_part)
    }

    pub fn matches(&self, pattern: &ActionPathPattern) -> bool {
        pattern.test(self)
    }
}

impl Display for ActionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.group_part, self.action_part)
    }
}

#[derive(Debug)]
pub struct ActionPathPattern {
    group_part: String,
    action_part: Option<String>,
}

impl ActionPathPattern {
    pub fn parse(path: &str) -> Result<Self> {
        match split_in_two(path, '/') {
            (group, None) => Ok(Self {
                group_part: group.to_owned(),
                action_part: None,
            }),
            (group, Some(action)) => Ok(Self {
                group_part: group.to_owned(),
                action_part: Some(action.to_owned()),
            }),
        }
    }

    fn test(&self, path: &ActionPath) -> bool {
        self.group_part == path.group_part
            && match (self.action_part.as_ref(), &path.action_part) {
                (None, _) => true,
                (Some(action_pattern), action_path) => action_pattern == action_path,
            }
    }
}

impl Display for ActionPathPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.action_part {
            None => write!(f, "{}", self.group_part),
            Some(action_part) => write!(f, "{}/{}", self.group_part, action_part),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_action_path() {
        let result = ActionPath::parse("my_group/my_action");
        assert!(result.is_ok());

        assert_eq!(result.as_ref().unwrap().group_part, "my_group");
        assert_eq!(result.as_ref().unwrap().action_part, "my_action");
    }

    #[test]
    fn parse_fixed_group_pattern() {
        let result = ActionPathPattern::parse("my_group");
        assert!(result.is_ok());

        assert_eq!(result.as_ref().unwrap().group_part, "my_group");
        assert_eq!(result.as_ref().unwrap().action_part, None);
    }

    #[test]
    fn parse_fixed_action_pattern() {
        let result = ActionPathPattern::parse("my_group/my_action");
        assert!(result.is_ok());

        assert_eq!(result.as_ref().unwrap().group_part, "my_group");
        assert_eq!(
            result.as_ref().unwrap().action_part,
            Some(String::from("my_action"))
        );
    }
}
