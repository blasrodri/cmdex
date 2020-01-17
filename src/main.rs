extern crate lazy_static;

use command_examples::examples::*;

fn main() {
    let user_input = "tar"; //mock
    match user_input {
        "find" => find::examples(),
        "tar" => tar::examples(),
        _ => println!("Not matched")
    }
}
