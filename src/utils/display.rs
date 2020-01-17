use crate::commands::command::{Command, CommandExample, Synopsis, CommandOptions, FlagPlusValue, Flag};

pub enum DisplayFormat {
    ASCII,
    JSON,
}

fn display_ascii<'a>(ce: &CommandExample<'a>) -> String {
    let CommandExample { 
        command: Command {name, description}, 
        synopsis: Synopsis { command_options }
    } = ce;
    let command_options_str = command_options_display_ascii(&command_options[..]);
    format!(
        r#"{} - {}
{} {}"#
        , name, description, name, command_options_str)
}

fn display_json<'a>(ce: &CommandExample<'a>) -> String {
    serde_json::to_string_pretty(ce).unwrap()
}

pub fn display<'a>(ce: &CommandExample<'a>, display_format: &DisplayFormat) -> () {
        match display_format {
            DisplayFormat::ASCII => println!("{}", display_ascii(ce)),
            DisplayFormat::JSON => println!("{}", display_json(ce)),
        }
}

fn command_options_display_ascii(command_options: &[CommandOptions]) -> String {
    command_options.iter()
        .map(|co| {
            co.flag_values.iter()
            .map(flag_plus_value_display_ascii)
            .fold("".to_string(), |mut acc, x| { acc.push_str(&x[..]); acc})
        })
        .fold("".to_string(), |mut acc, x| { acc.push_str(&x[..]); acc})
        .trim_end()
        .to_string()
}

fn flag_plus_value_display_ascii(flag_plus_value: &FlagPlusValue) -> String {
    let FlagPlusValue { 
        flag: Flag {prefix, name}, 
        value
    }  = flag_plus_value;
    format!("{}{}{}", 
        prefix.map_or_else(|| "".to_string(), |p| format!("{}", p)),
        name.map_or_else(|| "".to_string(), |p| format!("{} ", p)),
        value.map_or_else(|| "".to_string(), |p| format!("{} ", p))
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json;
    #[test]
    fn test_display_ascii() {
        let command_example = r#"{
                "command": {
                    "name": "tar",
                    "description": "compress an entire directory"
                },
                "synopsis": {
                    "command_options": [
                        {
                            "flag_values": [
                                { "flag": {"prefix": "-", "name": "zvcf"}, "value": "[result-filename.tar.gz]"},
                                { "flag": {}, "value": "[path-of-directory-to-compress]"}
                            ]
                        }
                    ]
                }
            }"#;

        let expected_result = r#"tar - compress an entire directory
tar -zvcf [result-filename.tar.gz] [path-of-directory-to-compress]"#;
        assert_eq!(
            display_ascii(&serde_json::from_str(command_example).unwrap())
            ,
            expected_result
        );
    }
}