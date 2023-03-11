use std::io::{stdout, Stdout, Write};
use std::path::Path;
use std::process;
use std::process::Stdio;
use crate::lib::Command::{Command, CommandError};

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

    fn create_command(&self) -> Result<process::Command, CommandError> {
        let canonicalized_path = match Path::new(&self.command_name).canonicalize() {
            Ok(path) => Ok(path),
            Err(err) => Err(CommandError::CouldNotExecute {
                reason: err.to_string()
            })
        }?.into_os_string();

        let mut proc = process::Command::new(canonicalized_path);

        proc.args(self.arguments.clone());

        println!("{:?}", proc.get_program());

        return Ok(proc);
    }
}

impl Command for ExecuteCommand {
    fn execute(&self) -> Result<i32, CommandError> {
        let result = self.create_command()?
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .spawn();

        if let Err(err) = result {
            return Err(CommandError::CouldNotExecute {
                reason: err.to_string()
            });
        }

        return match result.unwrap().wait() {
            Err(err) => Err(CommandError::CouldNotExecute {
                reason: err.to_string()
            }),
            Ok(exitStatus) => Ok(exitStatus.code().unwrap())
        }
    }

    fn execute_redirected_output(&self) -> Result<(i32, String), CommandError> {
        let result = self.create_command()?
            .stdin(Stdio::inherit())
            .output();

        return match result {
            Err(err) => Err(
                CommandError::CouldNotExecute {
                    reason: err.to_string()
                }
            ),
            Ok(output) => Ok((
                output.status.code().expect("Process code should be available since the process exited"),
                String::from_utf8(output.stdout).expect("invalid utf8")
            ))
        }
    }

    fn execute_redirected_input(&self, input: &str) -> Result<i32, CommandError> {
        let mut child_result = self.create_command()?
            .stdin(Stdio::piped())
            .stdout(Stdio::inherit())
            .spawn();

        if let Err(err) = child_result {
            return Err(CommandError::CouldNotExecute {
                reason: err.to_string()
            });
        }

        let mut child = child_result.unwrap();

        let mut stdin = child.stdin
            .take()
            .expect("stdin should be available since stdin has been set to piped");

        if let Err(err) = stdin.write(input.as_bytes()) {
            return Err(CommandError::CouldNotExecute {
                reason: err.to_string()
            });
        }

        return match child.wait() {
            Err(err) => Err(
                CommandError::CouldNotExecute {
                    reason: err.to_string()
                }
            ),
            Ok(exit_status) => Ok(exit_status.code()
                .expect("exit code should be available since the process exited")
            )
        }
    }

    fn execute_redirected_io(&self, input: &str) -> Result<(i32, String), CommandError> {
        let mut child_result = self.create_command()?
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn();

        if let Err(err) = child_result {
            return Err(CommandError::CouldNotExecute {
                reason: err.to_string()
            });
        }

        let mut child = child_result.unwrap();

        let mut stdin = child.stdin
            .take()
            .expect("stdin should be available since stdin has been set to piped");

        if let Err(err) = stdin.write(input.as_bytes()) {
            return Err(CommandError::CouldNotExecute {
                reason: err.to_string()
            });
        }

        let input_clone = String::from(input);

        std::thread::spawn(move || {
            stdin.write_all(input_clone.as_bytes()).expect("failed to write to stdin");
        });

        return match child.wait_with_output() {
            Err(err) => Err(
                CommandError::CouldNotExecute {
                    reason: err.to_string()
                }
            ),
            Ok(output) => Ok((
                output.status.code().expect("Process code should be available since the process exited"),
                String::from_utf8(output.stdout).expect("invalid utf8")
            ))
        }
    }
}