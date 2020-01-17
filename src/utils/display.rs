use crate::commands::command::{Command, CommandExample, Synopsis, CommandOptions, FlagPlusValue};

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
"#
        , name, description)
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
        .map(|_co| "".to_string())
        .fold("".to_string(), |mut acc, x| { acc.push_str(&x[..]); acc})
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::commands::command::*;
    #[test]
    fn test_display_ascii() {
        let command_example = command_example!(
                command!("tar", "compress an entire directory"),
                synopsis!(
                    vec![
                        command_options!(
                            vec![
                                flag_plus_value!(flag!("-", "zcvf"), "[result-filename.tar.gz]"),
                                flag_plus_value!(flag!("", ""), "[path-of-directory-to-compress]"),
                            ]
                        ),
                    ]
                )
        );
        let expected_result = r#"tar - compress an entire directory
---
tar -zcvf [result-filename.tar.gz] [path-of-directory-to-compress]

"#;
        assert_eq!(
            display_ascii(&command_example)
            ,
            expected_result
        )
    }
}