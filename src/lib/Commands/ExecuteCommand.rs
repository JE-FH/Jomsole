use std::env::current_dir;
use std::io::{stdout, Stdout, Write};
use std::path::Path;
use std::process;
use std::process::Stdio;
use std::rc::Rc;
use crate::lib::Command::{Command, CommandError};
use crate::lib::PathResolver::PathResolver;

pub enum CommandScope {
    LOCAL,
    ANY
}

pub struct ExecuteCommand {
    command_executable: String,
    command_scope: CommandScope,
    arguments: Vec<String>,
    path_resolver: Rc<dyn PathResolver>
}

impl ExecuteCommand {
	pub fn new(
        command_name: String,
        command_scope: CommandScope,
        arguments: Vec<String>,
        path_resolver: Rc<dyn PathResolver>
    ) -> ExecuteCommand {
		return ExecuteCommand {
            command_executable: command_name,
            command_scope: command_scope,
            arguments: arguments,
            path_resolver: path_resolver
        };
	}

    fn create_command(&self) -> Result<process::Command, CommandError> {
        let proc_path = match self.command_scope {
            CommandScope::LOCAL => {
                self.path_resolver.resolve_command_local(
                    Path::new(current_dir()
                        .expect("Should be able to get current directory").as_path()
                    ),
                    &self.command_executable
                )?
            },
            CommandScope::ANY => {
                self.path_resolver.resolve_command_global(
                    &self.command_executable
                )?
            }
        };

        let mut proc = process::Command::new(proc_path);

        proc.args(self.arguments.clone());

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
            Ok(exit_status) => Ok(exit_status.code().unwrap())
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