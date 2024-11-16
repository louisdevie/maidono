use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum ActionRefs {
    Single(String),
    Multiple(Vec<String>),
}

pub fn flatten_optional_refs(refs: Option<ActionRefs>) -> Vec<String> {
    match refs {
        Some(ActionRefs::Single(single)) => vec![single],
        Some(ActionRefs::Multiple(multiple)) => multiple,
        None => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::{assert_de_tokens, assert_de_tokens_error, Token};

    #[test]
    fn deserialize_string() {
        let refs = ActionRefs::Single(String::from("some-action"));

        assert_de_tokens(&refs, &[Token::String("some-action")]);
    }

    #[test]
    fn deserialize_array() {
        let refs = ActionRefs::Multiple(vec![String::from("action-1"), String::from("action-2")]);

        assert_de_tokens(
            &refs,
            &[
                Token::Seq { len: None },
                Token::String("action-1"),
                Token::String("action-2"),
                Token::SeqEnd,
            ],
        );
    }

    #[test]
    fn deserialize_bad_type() {
        assert_de_tokens_error::<ActionRefs>(
            &[Token::I32(33)],
            "data did not match any variant of untagged enum ActionRefs",
        );
    }

    #[test]
    fn deserialize_array_containing_bad_type() {
        assert_de_tokens_error::<ActionRefs>(
            &[
                Token::Seq { len: None },
                Token::String("action-1"),
                Token::I32(33),
                Token::SeqEnd,
            ],
            "data did not match any variant of untagged enum ActionRefs",
        );
    }
}
