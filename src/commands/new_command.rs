use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::commands::command::CommandExample;

fn load_new_command(filename: &Path) -> Result<String, String> {
    let mut buffer = String::new();
    let mut f = File::open(filename).map_err(|e| e.to_string())?;
    f.read_to_string(&mut buffer)
        .map(move |_| buffer)
        .map_err(|e| e.to_string())
}

fn validate_command(command_example_str: &str) -> Result<CommandExample, String> {
    match serde_json::from_str::<CommandExample>(command_example_str) {
        Ok(command_example) => Ok(command_example),
        Err(err) => Err(err.to_string()),
    }
}

pub fn publish_new_command(filename: &str) -> Result<String, String> {
    let path = Path::new(filename);
    let new_command_str = load_new_command(&path)?;
    let _new_command = validate_command(new_command_str.as_str())?;
    Ok("the url is".to_string())
}
