use crate::commands::command::CommandExample;
arg_enum! {
    #[derive(PartialEq, Debug)]
    pub enum DisplayFormat {
        ASCII,
        JSON,
    }
}

fn display_ascii(ce: &CommandExample) -> String {
    let CommandExample {
        name,
        description,
        value,
        platforms,
        authors,
    } = ce;
    let platforms_str = if platforms.is_none() {
        "all".to_string()
    } else {
        platforms
            .clone()
            .unwrap()
            .iter()
            .fold(String::from(""), |mut acc, s| {
                acc.push_str(" ");
                acc.push_str(s.as_str());
                acc
            })
    };
    format!(
        r#"{} - {}
Platforms: {}
{}
Authors: {}
"#,
        name, description, platforms_str, value, authors
    )
}

fn display_json(ce: &CommandExample) -> String {
    serde_json::to_string_pretty(ce).unwrap()
}

pub fn display(ce: &CommandExample, display_format: &DisplayFormat) {
    match display_format {
        DisplayFormat::ASCII => println!("{}", display_ascii(ce)),
        DisplayFormat::JSON => println!("{}", display_json(ce)),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json;
    #[test]
    fn test_display_ascii() {
        let command_example = r#"{
                    "name": "tar",
                    "description": "compress an entire directory",
                    "value": "tar -zvcf [result-filename.tar.gz] [path-of-directory-to-compress]"
            }"#;

        let expected_result = r#"tar - compress an entire directory
Platforms: all
tar -zvcf [result-filename.tar.gz] [path-of-directory-to-compress]
"#;
        assert_eq!(
            display_ascii(&serde_json::from_str(command_example).unwrap()),
            expected_result
        );
    }
}
