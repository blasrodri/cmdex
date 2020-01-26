use crate::commands::command::*;
use crate::utils::display::{display, DisplayFormat};

lazy_static! {
    pub static ref EXAMPLES: Vec<CommandExample> = { vec![] };
}

pub fn examples(display_format: &DisplayFormat) {
    EXAMPLES
        .iter()
        .for_each(|cmd_ex| display(cmd_ex, &display_format))
}
