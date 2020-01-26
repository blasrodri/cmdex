use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CommandExample<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub value: String,
    pub platforms: Option<Vec<String>>,
}

impl<'a> From<&'a str> for CommandExample<'a> {
    fn from(s: &'a str) -> Self {
        match serde_json::from_str::<CommandExample<'a>>(s) {
            Ok(command_example) => command_example,
            Err(err) => panic!(err),
        }
    }
}

#[macro_export]
macro_rules! command_example {
    ($e: expr) => {
        CommandExample::from(&$e[..])
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_command_example() {
        let expected_value = CommandExample {
            name: "find",
            description: "search for files in a directory hierarchy",
            platforms: None,
            value: "find . --name *.rs".to_string(),
        };

        let command_example: CommandExample = serde_json::from_str(
            r#"{
                    "name": "find",
                    "description": "search for files in a directory hierarchy",
                    "value": "find . --name *.rs"
                }"#,
        )
        .unwrap();
        assert_eq!(command_example, expected_value);
    }
}
