use crate::commands::command::*;
use crate::utils::display::{DisplayFormat, display};


lazy_static! {
    pub static ref EXAMPLES: Vec<CommandExample<'static>> = {
        vec![
            command_example!(include_str!("find-in-cwd-extension-sed.json")),
        ]
    };
}

pub fn examples(display_format: &DisplayFormat) -> () {
    EXAMPLES.iter()
    .for_each(|cmd_ex| display(cmd_ex, &display_format))
}