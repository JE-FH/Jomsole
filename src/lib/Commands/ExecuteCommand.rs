use crate::lib::Command::Command;

pub struct ExecuteCommand {
    command_name: String,
    arguments: Vec<String>
}

impl ExecuteCommand {
	pub fn new(command_name: string, arguments: Vec<String>) -> ExecuteCommand {
		return ExecuteCommand {
            command_name: command_name,
            arguments: arguments
        };
	}
}

impl Command for ExecuteCommand {
    fn execute(&self) {
        println!("{}", self.command_name);
    }

    fn execute_redirected_output(&self) -> String {
        println!("{} out", self.command_name);
        return "".to_string();
    }

    fn execute_redirected_input(&self, input: &str) {
        println!("{} in", self.command_name);
    }

    fn execute_redirected_io(&self, input: &str) -> String {
        println!("{} io", self.command_name);
        return "".to_string();
    }
}