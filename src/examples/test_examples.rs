#[allow(unused_imports)]
use crate::commands::command::CommandExample;
#[allow(unused_imports)]
use std::convert::From;
#[allow(unused_imports)]
use std::fs;
#[allow(unused_imports)]
use std::io::Read;

#[cfg(test)]
#[test]
fn load_examples_and_check_no_panic_on_deserialization() {
    let entries = fs::read_dir("./src/examples/")
        .unwrap()
        .map(|res| res.unwrap().path())
        .collect::<Vec<_>>();
    let json_files = entries
        .iter()
        .filter(|p| p.to_str().unwrap().ends_with(".json"))
        .collect::<Vec<_>>();
    json_files.iter().for_each(|json| {
        let mut contents = String::new();
        let file = fs::File::open(json).unwrap();
        let mut buf_reader = std::io::BufReader::new(file);
        buf_reader.read_to_string(&mut contents).unwrap();
        command_example!(contents);
    });
}
