#[derive(Debug)]
pub enum CommandError {
	CouldNotExecute {
		reason: String
	}
}

pub trait Command {
	fn execute(&self) -> Result<i32, CommandError>;
	fn execute_redirected_output(&self) -> Result<(i32, String), CommandError>;
	fn execute_redirected_input(&self, input: &str) -> Result<i32, CommandError>;
	fn execute_redirected_io(&self, input: &str) -> Result<(i32, String), CommandError>;
}
