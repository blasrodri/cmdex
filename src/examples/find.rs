use crate::commands::command::*;

use serde_json;

lazy_static! {
    pub static ref FIND: Vec<CommandExample<'static>> = {
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
    static ref COUNT: usize = FIND.len();
}

pub fn find() -> () {
    for i in 0..*COUNT {
        println!("{}", serde_json::to_string_pretty(&FIND[i]).unwrap());
    }
}