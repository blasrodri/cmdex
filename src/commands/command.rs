use serde::{self, Deserialize, Serialize};
use std::convert::From;

use crate::query::fuzzy_search::FuzzySearchCategory;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CommandExample {
    pub name: String,
    pub description: String,
    pub value: String,
    pub platforms: Option<Vec<String>>,
    #[serde(default = "default_authors")]
    pub authors: String,
}

fn default_authors() -> String {
    "Blas Rodriguez Irizar <rodrigblas@gmail.com>".to_string()
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
            FuzzySearchCategory::Command => self.value.as_str(),
            FuzzySearchCategory::Description => self.description.as_str(),
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
