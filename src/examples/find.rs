use crate::commands::command::*;
use crate::utils::display::{display, DisplayFormat};

lazy_static! {
    pub static ref EXAMPLES: Vec<CommandExample<'static>> = {
        vec![
            command_example!(include_str!(
                "examples-data/find/find-in-cwd-extension-sed.json"
            )),
            command_example!(include_str!(
                "examples-data/find/find-contains-filename-in-cwd.json"
            )),
        ]
    };
}

pub fn examples(display_format: &DisplayFormat) {
    EXAMPLES
        .iter()
        .for_each(|cmd_ex| display(cmd_ex, &display_format))
}
