use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Deserialize, Debug, PartialEq)]
#[serde(from = "String")]
pub struct Commands {
    commands: Vec<String>,
}

impl Commands {
    pub fn new(commands: Vec<String>) -> Self {
        Self { commands }
    }

    pub fn has_multiple_commmands(&self) -> bool {
        self.commands.len() > 1
    }
}

impl From<String> for Commands {
    fn from(value: String) -> Self {
        Commands::new(
            value
                .lines()
                .filter_map(|ln| {
                    if ln.is_empty() {
                        None
                    } else {
                        Some(String::from(ln))
                    }
                })
                .collect(),
        )
    }
}

impl Display for Commands {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        for cmd in &self.commands {
            writeln!(f, "{}", cmd)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::{assert_de_tokens, assert_de_tokens_error, Token};

    #[test]
    fn deserialize_string() {
        let cmds = Commands::new(vec![String::from("sudo something")]);

        assert_de_tokens(&cmds, &[Token::BorrowedStr("sudo something")]);
    }

    #[test]
    fn deserialize_bad_type() {
        assert_de_tokens_error::<Commands>(
            &[Token::I32(33)],
            "invalid type: integer `33`, expected a string",
        );
    }

    #[test]
    fn yaml_single_command() {
        let expected = Commands::new(vec![String::from("sudo something")]);

        let parsed = serde_yaml::from_str::<Commands>("sudo something");

        assert_eq!(expected, parsed.unwrap());

        let parsed = serde_yaml::from_str::<Commands>("'sudo something'");

        assert_eq!(expected, parsed.unwrap());

        let parsed = serde_yaml::from_str::<Commands>("\"sudo something\"");

        assert_eq!(expected, parsed.unwrap());
    }

    #[test]
    fn yaml_multiple_commands() {
        let expected = Commands::new(vec![
            String::from("npm ci"),
            String::from("npm test"),
            String::from("npm build"),
        ]);

        let parsed = serde_yaml::from_str::<Commands>("|\n  npm ci\n\n  npm test\n  npm build");

        assert_eq!(expected, parsed.unwrap());
    }
}
