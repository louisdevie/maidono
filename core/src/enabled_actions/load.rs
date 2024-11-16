use super::{EnabledEntry, EnabledList};
use crate::problem;
use crate::utils::{path, split_in_two, ActionPath, Result};
use std::fs::read_to_string;

pub fn load_enabled_actions() -> Result<EnabledList> {
    let contents = read_to_string(path::ENABLED_ACTIONS_FILE)
        .map_err(|io_err| problem!("unable to read the enabled actions list").because(io_err))?;

    parse_enabled_actions_file(contents)
}

fn parse_enabled_actions_file(contents: String) -> Result<EnabledList> {
    let mut enabled = Vec::new();

    for result in contents.lines().map(parse_enabled_action_entry) {
        match result? {
            Some(entry) => enabled.push(entry),
            None => {}
        }
    }

    Ok(EnabledList { enabled })
}

fn parse_enabled_action_entry(line: &str) -> Result<Option<EnabledEntry>> {
    match split_in_two(line, ':') {
        ("action", Some(path)) => ActionPath::parse(path).map(|p| Some(EnabledEntry::Action(p))),
        ("#", _) => Ok(None),
        (_, None) => Err(problem!("invalid entry '{}'", line)),
        (other, _) => Err(problem!("invalid scope '{}'", other)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn action_entry() {
        let result = parse_enabled_action_entry("action:my_group/my_action");
        assert!(result.is_ok());

        assert!(
            matches!(result.unwrap(), Some(EnabledEntry::Action(a)) if a == ActionPath::from_parts("my_group", "my_action"))
        )
    }

    #[test]
    fn comment_line() {
        let result = parse_enabled_action_entry("#: check this out");
        assert!(result.is_ok());

        assert!(matches!(result.unwrap(), None))
    }

    #[test]
    fn invalid_line() {
        let result = parse_enabled_action_entry("what:is:this");
        assert!(result.is_err());
    }

    #[test]
    fn read_whole_file() {
        let result = parse_enabled_actions_file(String::from(indoc! {"
            #: misaka
            action:hoshi/mi
            action:miko/to
        "}));
        assert!(result.is_ok());

        let ena = result.unwrap();
        assert_eq!(ena.enabled.len(), 2);
        assert!(
            matches!(&ena.enabled[0], EnabledEntry::Action(a) if *a == ActionPath::from_parts("hoshi", "mi"))
        );
        assert!(
            matches!(&ena.enabled[1], EnabledEntry::Action(a) if *a == ActionPath::from_parts("miko", "to"))
        );
    }
}
