use crate::lib::Command::Command;

pub struct PipeCommand {
    left_command: Box<dyn Command>,
    right_command: Box<dyn Command>
}

impl PipeCommand {
    pub fn new(left_command: Box<dyn Command>, right_command: Box<dyn command>) -> PipeCommand {
        return PipeCommand {
            left_command: left_command,
            right_command: right_command,
        };
    }
}

impl Command for PipeCommand {
    fn execute(&self) {
        let out = self.right_command.execute_redirected_output();
        self.left_command.execute_redirected_input(&out);
    }

    fn execute_redirected_output(&self) -> String {
        let out = self.right_command.execute_redirected_output();
        return self.left_command.execute_redirected_io(&out);
    }

    fn execute_redirected_input(&self, input: &str) {
        let out = self.right_command.execute_redirected_io(input);
        self.left_command.execute_redirected_input(&out);
    }

    fn execute_redirected_io(&self, input: &str) -> String {
        let out = self.right_command.execute_redirected_io(input);
        return self.left_command.execute_redirected_io(&out);
    }
}