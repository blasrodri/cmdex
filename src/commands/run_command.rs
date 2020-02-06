use std::process::Command;

use std::io::{self, Write};

pub fn run_command(command_name: &str, command_args: &[&str]) {
    let output = Command::new(command_name)
                     .args(command_args)
                     .output()
                     .expect("failed to execute process");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_run_command() {
        run_command("echo", &["hi"]);
        run_command("find", &["-name", "*.rs"]);
    }

}
