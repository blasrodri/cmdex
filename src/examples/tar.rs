use crate::commands::command::*;

use serde_json;

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

pub fn examples() -> () {
    EXAMPLES.iter()
    .for_each(
        |cmd_ex| println!("{}", serde_json::to_string_pretty(cmd_ex).unwrap())
    )
}