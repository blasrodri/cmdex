use crate::commands::command::*;
use crate::utils::display::{display, DisplayFormat};

lazy_static! {
    pub static ref EXAMPLES: Vec<CommandExample> = {
        vec![command_example!(include_str!(
            "examples-data/grep/grep-recursively-in-cwd.json"
        ))]
    };
}

pub fn examples(display_format: &DisplayFormat) -> Vec<String> {
    EXAMPLES
        .iter()
        .map(|cmd_ex| display(cmd_ex, &display_format))
        .collect::<Vec<String>>()
}
