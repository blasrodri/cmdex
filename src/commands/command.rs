#[derive(Debug, PartialEq)]
pub struct CommandExample<'a> {
    command: &'a Command<'a>,
    synopsis: &'a Synopsis<'a>,
}

#[derive(Debug, PartialEq)]
pub struct Command<'a> {
    name: &'a str,
    description: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct Synopsis<'a> {
    command_options: &'a [CommandOptions<'a>],
}

#[derive(Debug, PartialEq)]
pub struct CommandOptions<'a> {
    flag_values: &'a [FlagPlusValue<'a>],
}

#[derive(Debug, PartialEq)]
pub struct Flag<'a> {
    prefix: Option<&'a str>,
    name: &'a str
}

#[derive(Debug, PartialEq)]
struct FlagPlusValue<'a> {
    flag: Flag<'a>, 
    value: Option<&'a str>,
}

impl<'a> Flag<'a> {
    pub fn new(prefix: Option<&'a str>, name: &'a str) -> Flag<'a> {
        Flag {prefix, name}
    }
}

impl<'a> Into<(Flag<'a>, Option<&'a str>)> for FlagPlusValue<'a> {
    fn into(self) -> (Flag<'a>, Option<&'a str>) {
        (self.flag, self.value)
    }
}

impl<'a> FlagPlusValue<'a> {
    pub fn new(flag: Flag<'a>, value: Option<&'a str>) -> FlagPlusValue<'a> {
        FlagPlusValue {flag, value}
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
    ($command: expr, $synopsis: expr) => { CommandExample { command: &$command, synopsis: &$synopsis } };
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
                &[
                    flag_plus_value!(flag!("", "find"), ""),
                    flag_plus_value!(flag!("--", "find"), "*.rs"),
                ]
            ),
            CommandOptions {
                flag_values: &[
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
                &[
                    command_options!(
                        &[
                            flag_plus_value!(flag!("", "find"), ""),
                            flag_plus_value!(flag!("--", "find"), "*.rs"),
                        ]
                    ),
                ]
            ),
            Synopsis {
                command_options: &[
                    CommandOptions {
                        flag_values: &[
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
                    &[
                        command_options!(
                            &[
                                flag_plus_value!(flag!("", "find"), ""),
                                flag_plus_value!(flag!("--", "find"), "*.rs"),
                            ]
                        ),
                    ]
                )
            ),
            CommandExample {
                command: &Command {name: "find", description: "search for files in a directory hierarchy"},
                synopsis:
                    &Synopsis {
                        command_options: &[
                            CommandOptions {
                                flag_values: &[
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