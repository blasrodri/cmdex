use crate::commands::command::*;
use crate::utils::display::{display, DisplayFormat};

lazy_static! {
    pub static ref EXAMPLES: Vec<CommandExample<'static>> = {
        vec![command_example!(include_str!(
            "grep-recursively-in-cwd.json"
        ))]
    };
}

pub fn examples(display_format: &DisplayFormat) {
    EXAMPLES
        .iter()
        .for_each(|cmd_ex| display(cmd_ex, &display_format))
}
