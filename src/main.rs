extern crate lazy_static;

use command_examples::examples::find;

fn main() {
    let user_input = "find"; //mock
    match user_input {
        "find" => find::find(),
        _ => println!("Not matched")
    }
}
