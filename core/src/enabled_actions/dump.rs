use super::{EnabledEntry, EnabledList};
use crate::problem;
use crate::utils::{path, Result};
use std::fs::File;
use std::io::Write;

type IoError = std::result::Result<(), std::io::Error>;

pub fn dump_enabled_actions(list: &EnabledList) -> Result<()> {
    let mut file = File::create(path::ENABLED_ACTIONS_FILE)
        .map_err(|io_err| problem!("unable to write the enabled actions list").because(io_err))?;

    write_enabled_actions_file(&mut file, list)?;

    Ok(())
}

fn write_enabled_actions_file<O>(output: &mut O, list: &EnabledList) -> IoError
where
    O: Write,
{
    write_enabled_action_comment(output, "This file is managed by maidono")?;
    write_enabled_action_comment(output, "please do not modify it manually")?;

    for entry in &list.enabled {
        write_enabled_action_entry(output, entry)?
    }

    Ok(())
}

fn write_enabled_action_comment<O>(output: &mut O, comment: &str) -> IoError
where
    O: Write,
{
    writeln!(output, "#: {}", comment)
}

fn write_enabled_action_entry<O>(output: &mut O, entry: &EnabledEntry) -> IoError
where
    O: Write,
{
    match entry {
        EnabledEntry::Action(path) => writeln!(output, "action:{}", path.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::ActionPath;

    use super::*;
    use indoc::indoc;

    #[test]
    fn action_entry() {
        let mut output = Vec::new();
        let result = write_enabled_action_entry(
            &mut output,
            &EnabledEntry::Action(ActionPath::from_parts("my_group", "my_action_♥")),
        );

        assert!(result.is_ok());

        assert_eq!(
            String::from_utf8(output).unwrap(),
            "action:my_group/my_action_♥\n"
        );
    }

    #[test]
    fn comment_line() {
        let mut output = Vec::new();
        let result = write_enabled_action_comment(&mut output, "check this out");

        assert!(result.is_ok());

        assert_eq!(String::from_utf8(output).unwrap(), "#: check this out\n");
    }

    #[test]
    fn write_whole_file() {
        let mut output = Vec::new();
        let result = write_enabled_actions_file(
            &mut output,
            &EnabledList {
                enabled: vec![EnabledEntry::Action(ActionPath::from_parts("hoshi", "mi"))],
            },
        );

        assert!(result.is_ok());

        assert_eq!(
            String::from_utf8(output).unwrap(),
            indoc! {"
                #: This file is managed by maidono
                #: please do not modify it manually
                action:hoshi/mi
            "}
        );
    }
}
