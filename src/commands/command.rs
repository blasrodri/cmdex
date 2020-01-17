use serde::{Serialize};

#[derive(Debug, Serialize, PartialEq)]
pub struct CommandExample<'a> {
    pub command: Command<'a>,
    pub synopsis: Synopsis<'a>,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct Command<'a> {
    pub name: &'a str,
    pub description: &'a str,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct Synopsis<'a> {
    pub command_options: Vec<CommandOptions<'a>>,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct CommandOptions<'a> {
    pub flag_values: Vec<FlagPlusValue<'a>>,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct Flag<'a> {
    pub prefix: Option<&'a str>,
    pub name: &'a str
}

#[derive(Debug, Serialize, PartialEq)]
pub struct FlagPlusValue<'a> {
    pub flag: Flag<'a>, 
    pub value: Option<&'a str>,
}

impl<'a> Flag<'a> {
    pub fn new(prefix: Option<&'a str>, name: &'a str) -> Flag<'a> {
        Flag {prefix, name}
    }
}

macro_rules! command {
    ($name: expr, $description: expr) => { Command {name: $name, description: $description}};
}

macro_rules! flag {
    ( "", $name: expr ) =>  { Flag::new(None, $name) };
    ( $prefix: expr, $name: expr ) =>  { Flag::new(Some($prefix), $name) };
}

macro_rules! flag_plus_value {
    ( $flag: expr, "" ) => { FlagPlusValue { flag: $flag, value: None } };
    ( $flag: expr, $value: expr ) => { FlagPlusValue { flag: $flag, value: Some($value) } };
}

macro_rules! command_options {
    ($flag_values: expr) => { CommandOptions { flag_values: $flag_values } };
}

macro_rules! synopsis {
    ($command_options: expr) => { Synopsis { command_options: $command_options } };
}

macro_rules! command_example {
    ($command: expr, $synopsis: expr) => { CommandExample { command: $command, synopsis: $synopsis } };
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_command_macro() {
        assert_eq!(
            command!("find", "search for files in a directory hierarchy"),
            Command {name: "find", description: "search for files in a directory hierarchy"}
        );
    }

    #[test]
    fn test_flag_macro() {
        assert_eq!(flag!("", "find"), Flag {prefix: None, name: &"find"[..]});
        assert_eq!(flag!("--", "find"), Flag {prefix: Some(&"--"[..]), name: &"find"[..]});
    }

    #[test]
    fn test_flag_value_macro() {
        assert_eq!(flag_plus_value!(flag!("", "find"), ""), FlagPlusValue{flag: flag!("", "find"), value: None});
        assert_eq!(flag_plus_value!(flag!("--", "find"), ""), FlagPlusValue{flag: flag!("--", "find"), value: None});
        assert_eq!(flag_plus_value!(flag!("--", "find"), "*.rs"), FlagPlusValue{flag: flag!("--", "find"), value: Some(&"*.rs"[..])});
    }

    #[test]
    fn test_command_options_macro() {
        assert_eq!(
            command_options!(
                vec![
                    flag_plus_value!(flag!("", "find"), ""),
                    flag_plus_value!(flag!("--", "find"), "*.rs"),
                ]
            ),
            CommandOptions {
                flag_values: vec![
                    FlagPlusValue{flag: flag!("", "find"), value: None },
                    FlagPlusValue{flag: flag!("--", "find"), value: Some("*.rs") },
                    ]
            },
        );
    }

    #[test]
    fn test_synopsis_macro() {
        assert_eq!(
            synopsis!(
                vec![
                    command_options!(
                        vec![
                            flag_plus_value!(flag!("", "find"), ""),
                            flag_plus_value!(flag!("--", "find"), "*.rs"),
                        ]
                    ),
                ]
            ),
            Synopsis {
                command_options: vec![
                    CommandOptions {
                        flag_values: vec![
                            FlagPlusValue{flag: flag!("", "find"), value: None },
                            FlagPlusValue{flag: flag!("--", "find"), value: Some("*.rs") },
                            ]
                    },
                ]
            }
        );
    }

    #[test]
    fn test_command_example() {
        assert_eq!(
            command_example!(
                command!("find", "search for files in a directory hierarchy"),
                synopsis!(
                    vec![
                        command_options!(
                            vec![
                                flag_plus_value!(flag!("", "find"), ""),
                                flag_plus_value!(flag!("--", "find"), "*.rs"),
                            ]
                        ),
                    ]
                )
            ),
            CommandExample {
                command: Command {name: "find", description: "search for files in a directory hierarchy"},
                synopsis:
                    Synopsis {
                        command_options: vec![
                            CommandOptions {
                                flag_values: vec![
                                    FlagPlusValue{flag: flag!("", "find"), value: None },
                                    FlagPlusValue{flag: flag!("--", "find"), value: Some("*.rs") },
                                    ]
                            },
                        ]
                    }
            },
        );
    }
}
// command_example!("find", argument!("."), flag_value!("-exec", "grep chrome {} \;")