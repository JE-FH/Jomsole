use std::io::Write;

pub trait CommandInterface {
	fn read_command(&self, prompt: &str) -> String;
}
