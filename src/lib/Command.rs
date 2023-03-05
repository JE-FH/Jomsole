pub trait Command {
	fn execute(&self);
	fn execute_redirected_output(&self) -> String;
	fn execute_redirected_input(&self, input: &str);
	fn execute_redirected_io(&self, input: &str) -> String;
}
