use std::io::Write;
use crate::lib::Trait::CommandInterface::CommandInterface;

pub struct Ecma48CommandInterface {

}

impl Ecma48CommandInterface {
    pub fn new() -> Ecma48CommandInterface {
        return Ecma48CommandInterface {};
    }
}

impl CommandInterface for Ecma48CommandInterface {
    fn read_command(&self, prompt: &str) -> String {
        std::io::stdout().write("\n".as_bytes()).unwrap();
        std::io::stdout().write(prompt.as_bytes()).unwrap();

        std::io::stdout().flush().unwrap();

        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();

        if (line.ends_with("\r\n")) {
            line.truncate(line.len() - 2);
        } else if (line.ends_with("\n")) {
            line.truncate(line.len() - 1);
        }
        return line;
    }
}