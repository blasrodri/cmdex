use crate::commands::command::*;
use crate::utils::display::{display, DisplayFormat};

type Examples = Vec<CommandExample>;

pub fn examples(examples: &Examples, display_format: &DisplayFormat) {
    examples
        .iter()
        .for_each(|cmd_ex| display(cmd_ex, &display_format))
}
