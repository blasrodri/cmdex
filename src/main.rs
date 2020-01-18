extern crate lazy_static;

use command_examples::examples::*;
use command_examples::utils::display::DisplayFormat;

fn main() {
    let user_input = "find"; //mock
    let display_format = DisplayFormat::ASCII;
    match user_input {
        "find" => find::examples(&display_format),
        "grep" => grep::examples(&display_format),
        "tar" => tar::examples(&display_format),
        _ => println!("Not matched"),
    }
}
