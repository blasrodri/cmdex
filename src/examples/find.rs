use crate::commands::command::*;
use crate::utils::display::{DisplayFormat, display};


lazy_static! {
    pub static ref EXAMPLES: Vec<CommandExample<'static>> = {
        vec![
            command_example!(
                command!("find", "find all rust files and apply rust fmt"),
                synopsis!(
                    vec![
                        command_options!(
                            vec![
                                flag_plus_value!(flag!("--", "name"), "*.rs"),
                                flag_plus_value!(flag!("-", "exec"), "cargo fmt {} \\;"),
                            ]
                        ),
                    ]
                )
            ),

        ]
    };
}

pub fn examples(display_format: &DisplayFormat) -> () {
    EXAMPLES.iter()
    .for_each(|cmd_ex| display(cmd_ex, &display_format))
}