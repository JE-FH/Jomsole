use std::collections::{HashMap, HashSet};
use std::io::{Read, Write};
use std::rc::Rc;
use crate::lib::Commands::BuiltinCommand::BuiltinCommandHandler;
use crate::lib::Trait::BuiltinCommandRepository::BuiltinCommandRepository;

pub struct JshCommandRepository {
    commands: HashMap<String, Rc<dyn BuiltinCommandHandler>>
}

struct SimpleCommand {
    handler: fn(arguments: &[String], in_stream: &mut dyn Read, out_stream: &mut dyn Write) -> i32
}

impl SimpleCommand {
    pub fn new(handler: fn(arguments: &[String], in_stream: &mut dyn Read, out_stream: &mut dyn Write) -> i32) -> SimpleCommand {
        return SimpleCommand {
            handler: handler
        }
    }
}

impl BuiltinCommandHandler for SimpleCommand {
    fn run(&self, arguments: &[String], in_stream: &mut dyn Read, out_stream: &mut dyn Write) -> i32 {
        (self.handler)(arguments, in_stream, out_stream)
    }
}

impl JshCommandRepository {
    pub fn new() -> JshCommandRepository {
        return JshCommandRepository {
            commands: HashMap::new()
        };
    }
}

impl BuiltinCommandRepository for JshCommandRepository {
    fn lookup_command(&self, command_name: &str) -> Option<Rc<dyn BuiltinCommandHandler>> {
        match self.commands.get(command_name) {
            Some(command) => Some(command.clone()),
            None => None
        }
    }

    fn add_command(&mut self, name: String, command: Rc<dyn BuiltinCommandHandler>) {
        self.commands.insert(name, command);
    }

    fn add_simple_command(&mut self, name: String, handler: fn(arguments: &[String], in_stream: &mut dyn Read, out_stream: &mut dyn Write) -> i32) {
        self.commands.insert(name, Rc::new(SimpleCommand::new(handler)));
    }
}
