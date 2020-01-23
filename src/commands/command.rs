use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Flag<'a> {
    pub prefix: Option<&'a str>,
    pub name: Option<&'a str>,
}

#[derive(Deserialize, Debug, Serialize, PartialEq)]
pub struct FlagPlusValue<'a> {
    #[serde(borrow)]
    pub flag: Flag<'a>,
    pub value: Option<&'a str>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CommandOptions<'a> {
    #[serde(borrow)]
    pub flag_values: Vec<FlagPlusValue<'a>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Command<'a> {
    pub name: &'a str,
    pub description: &'a str,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Synopsis<'a> {
    #[serde(borrow)]
    pub command_options: Vec<CommandOptions<'a>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CommandExample<'a> {
    #[serde(borrow)]
    pub command: Command<'a>,
    pub synopsis: Synopsis<'a>,
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
            command: Command {
                name: "find",
                description: "search for files in a directory hierarchy",
            },
            synopsis: Synopsis {
                command_options: vec![CommandOptions {
                    flag_values: vec![
                        FlagPlusValue {
                            flag: Flag {
                                prefix: None,
                                name: Some("find"),
                            },
                            value: None,
                        },
                        FlagPlusValue {
                            flag: Flag {
                                prefix: Some("--"),
                                name: Some("find"),
                            },
                            value: Some("*.rs"),
                        },
                    ],
                }],
            },
        };

        let command_example: CommandExample = serde_json::from_str(
            r#"{
                "command": {
                    "name": "find",
                    "description": "search for files in a directory hierarchy"
                },
                "synopsis": {
                    "command_options": [
                        {
                            "flag_values": [
                                { "flag": {"name": "find"}},
                                { "flag": {"prefix": "--", "name": "find"}, "value": "*.rs"}
                            ]
                        }
                    ]
                }
            }"#,
        )
        .unwrap();
        assert_eq!(command_example, expected_value);
    }
}
