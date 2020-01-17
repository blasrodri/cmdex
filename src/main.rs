#[macro_use]

mod commands;
use commands::command::*;

fn main() {
        println!(
            "{:#?}",
            command_example!(
                command!("find", "search for files in a directory hierarchy"),
                synopsis!(
                    &[
                        command_options!(
                            &[
                                flag_plus_value!(flag!("", "find"), ""),
                                flag_plus_value!(flag!("--", "find"), "*.rs"),
                            ]
                        ),
                    ]
                )
            )
        );
}
