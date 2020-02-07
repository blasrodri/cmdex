use crate::commands::command::*;
use crate::utils::display::{display, DisplayFormat};

lazy_static! {
    pub static ref EXAMPLES: Vec<CommandExample> = {
        vec![
            command_example!(include_str!(
                "../../examples-data/npm/npm-update-to-latest.json"
            )),
            command_example!(include_str!(
                "../../examples-data/npm/npm-installing-package-global-mode.json"
            )),
            command_example!(include_str!(
                "../../examples-data/npm/npm-listing-global-packages.json"
            )),
        ]
    };
}

pub fn examples(display_format: &DisplayFormat) {
    EXAMPLES
        .iter()
        .for_each(|cmd_ex| display(cmd_ex, &display_format))
}
