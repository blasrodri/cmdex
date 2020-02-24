use std::collections::HashMap;

use crate::commands::command::CommandExample;
use crate::utils::load_files::{get_command_names, load_json_file_paths};

pub struct CommandsDB {
    state: HashMap<String, Vec<CommandExample>>,
}

impl CommandsDB {
    pub fn new() -> CommandsDB {
        let mut state: HashMap<String, Vec<CommandExample>> = HashMap::new();
        for cmd_name in get_command_names() {
            let mut v = vec![];
            for fp in load_json_file_paths(Some(&cmd_name)) {
                let ce_str = std::fs::read_to_string(fp).unwrap();
                v.push(command_example!(ce_str));
            }
            state.insert(cmd_name.clone(), v);
        }
        CommandsDB { state }
    }

    pub fn get_data(&self) -> &HashMap<String, Vec<CommandExample>> {
        &self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_commands_db() {
        CommandsDB::new();
    }

    #[test]
    fn get_data() {
        assert!(CommandsDB::new().get_data().get("git").unwrap().len() > 1);
        assert!(CommandsDB::new()
            .get_data()
            .get("this command does not exist")
            .is_none());
    }
}
