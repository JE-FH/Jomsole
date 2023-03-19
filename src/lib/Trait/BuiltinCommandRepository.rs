use std::io::{Read, Write};
use std::rc::Rc;
use crate::lib::Commands::BuiltinCommand::BuiltinCommandHandler;

pub trait BuiltinCommandRepository {
    fn lookup_command(&self, command_name: &str) -> Option<Rc<dyn BuiltinCommandHandler>>;
    fn add_command(&mut self, name: String, command: Rc<dyn BuiltinCommandHandler>);
    fn add_simple_command(&mut self, name: String, handler: fn(arguments: &[String], in_stream: &mut dyn Read, out_stream: &mut dyn Write) -> i32);
}