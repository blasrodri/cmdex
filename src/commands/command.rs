use serde::{Deserialize, Serialize};
use std::convert::From;

use crate::query::fuzzy_search::FuzzySearchCategory;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CommandExample {
    pub name: String,
    pub description: String,
    pub value: String,
    pub platforms: Option<Vec<String>>,
}

impl<'a> From<&'a str> for CommandExample {
    fn from(s: &'a str) -> Self {
        match serde_json::from_str::<CommandExample>(s) {
            Ok(command_example) => command_example,
            Err(err) => panic!(err),
        }
    }
}

impl CommandExample {
    pub fn get_by_category(&self, category: &FuzzySearchCategory) -> &str {
        match category {
            &FuzzySearchCategory::Command => self.value.as_str(),
            &FuzzySearchCategory::Description => self.description.as_str(),
        }
    }
}
/*
pub fn validate(new_cmd_str: &str) -> Result<CommandExample, String> {
        let cmd_example = serde_json::from_str::<CommandExample>(new_cmd_str)
            .map_err(|e| e.to_string())?;

}
            */

// pub fn new_command_cli() -> CommandExample {

// }

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
            name: "find".to_string(),
            description: "search for files in a directory hierarchy".to_string(),
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
