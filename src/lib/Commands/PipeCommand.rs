use crate::lib::Trait::Command::{Command, CommandError};

pub struct PipeCommand {
    left_command: Box<dyn Command>,
    right_command: Box<dyn Command>
}

impl PipeCommand {
    pub fn new(left_command: Box<dyn Command>, right_command: Box<dyn Command>) -> PipeCommand {
        return PipeCommand {
            left_command: left_command,
            right_command: right_command,
        };
    }

    fn check_exit_code(&self, exit_code: i32, error_message: &str) -> Result<(), CommandError> {
        if exit_code != 0 {
            return Err(CommandError::CouldNotExecute {
                reason: error_message.to_string()
            });
        }
        return Ok(());
    }
}

impl Command for PipeCommand {
    fn execute(&self) -> Result<i32, CommandError> {
        let out = self.right_command.execute_redirected_output()?;
        self.check_exit_code(out.0, "Exit code from chained process was not 0")?;
        return self.left_command.execute_redirected_input(out.1);
    }

    fn execute_redirected_output(&self) -> Result<(i32, Vec<u8>), CommandError> {
        let out = self.right_command.execute_redirected_output()?;
        self.check_exit_code(out.0, "Exit code from chained process was not 0")?;
        return self.left_command.execute_redirected_io(out.1);
    }

    fn execute_redirected_input(&self, input: Vec<u8>) -> Result<i32, CommandError> {
        let out = self.right_command.execute_redirected_io(input)?;
        self.check_exit_code(out.0, "Exit code from chained process was not 0")?;
        return self.left_command.execute_redirected_input(out.1);
    }

    fn execute_redirected_io(&self, input: Vec<u8>) -> Result<(i32, Vec<u8>), CommandError> {
        let out = self.right_command.execute_redirected_io(input)?;
        self.check_exit_code(out.0, "Exit code from chained process was not 0")?;
        return self.left_command.execute_redirected_io(out.1);
    }
}