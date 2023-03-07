use std::io::{stdout, Stdout, Write};
use std::process;
use std::process::Stdio;
use crate::lib::Command::Command;

pub struct ExecuteCommand {
    command_name: String,
    arguments: Vec<String>
}

impl ExecuteCommand {
	pub fn new(command_name: String, arguments: Vec<String>) -> ExecuteCommand {
		return ExecuteCommand {
            command_name: command_name,
            arguments: arguments
        };
	}
}

impl Command for ExecuteCommand {
    fn execute(&self) {
        process::Command::new(self.command_name.clone())
            .args(self.arguments.clone())
            .spawn()
            .expect("bob");
    }

    fn execute_redirected_output(&self) -> String {
        let output = process::Command::new(self.command_name.clone())
            .args(self.arguments.clone())
            .stdin(Stdio::inherit())
            .output()
            .expect("bob");
        return String::from_utf8(output.stdout).expect("invalid utf8");
    }

    fn execute_redirected_input(&self, input: &str) {
        let mut process = process::Command::new(self.command_name.clone())
            .args(self.arguments.clone())
            .stdin(Stdio::piped())
            .stdout(Stdio::inherit())
            .spawn()
            .expect("bob");

        let mut stdin = process.stdin.take();
        stdin.unwrap().write(input.as_bytes()).expect("bob");

        process.wait().expect("penis");
    }

    fn execute_redirected_io(&self, input: &str) -> String {
        let mut process = process::Command::new(self.command_name.clone())
            .args(self.arguments.clone())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("bob");


        let input_clone = String::from(input);
        let mut stdin = process.stdin.take().expect("failed to get stdin");
        std::thread::spawn(move || {
            stdin.write_all(input_clone.as_bytes()).expect("failed to write to stdin");
        });

        let output = process
            .wait_with_output()
            .expect("penis");

        return String::from_utf8(output.stdout).expect("invalid utf8")
    }
}