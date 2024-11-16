use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum HostRef {
    GitHub,
    Any,
    Custom(String),
}

impl<'de> Deserialize<'de> for HostRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(HostVisitor)
    }
}

impl Display for HostRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            HostRef::GitHub => write!(f, "GitHub"),
            HostRef::Any => write!(f, "any"),
            HostRef::Custom(name) => write!(f, "'{}'", name),
        }
    }
}

struct HostVisitor;
impl<'de> Visitor<'de> for HostVisitor {
    type Value = HostRef;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "'gh', 'github', '*' or a custom host name")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match v {
            "gh" | "github" => Ok(HostRef::GitHub),
            "*" => Ok(HostRef::Any),
            _ => Ok(HostRef::Custom(String::from(v))),
        }
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_str(v.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::{assert_de_tokens, assert_de_tokens_error, Token};

    #[test]
    fn deserialize_github() {
        assert_de_tokens(&HostRef::GitHub, &[Token::String("gh")]);
        assert_de_tokens(&HostRef::GitHub, &[Token::String("github")]);
    }

    #[test]
    fn deserialize_any() {
        assert_de_tokens(&HostRef::Any, &[Token::String("*")]);
    }

    #[test]
    fn deserialize_custom() {
        assert_de_tokens(
            &HostRef::Custom(String::from("my-self-hosted-config")),
            &[Token::String("my-self-hosted-config")],
        );
    }

    #[test]
    fn deserialize_bad_type() {
        assert_de_tokens_error::<HostRef>(
            &[Token::I32(33)],
            "invalid type: integer `33`, expected 'gh', 'github', '*' or a custom host name",
        );
    }
}
