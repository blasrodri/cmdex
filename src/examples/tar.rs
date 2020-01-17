use crate::commands::command::*;
use crate::utils::display::{DisplayFormat, display};


lazy_static! {
    pub static ref EXAMPLES: Vec<CommandExample<'static>> = {
        vec![
            command_example!(
                command!("tar", "compress an entire directory"),
                synopsis!(
                    vec![
                        command_options!(
                            vec![
                                flag_plus_value!(flag!("-", "zcvf"), "[result-filename.tar.gz]"),
                                flag_plus_value!(flag!("", ""), "[path-of-directory-to-compress]"),
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