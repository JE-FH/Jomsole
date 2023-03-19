use std::io::{Cursor, Read, sink, stdin, stdout, Write};
use std::process::exit;
use std::rc::Rc;
use crate::lib::Trait::Command::{Command, CommandError};

pub trait BuiltinCommandHandler {
    fn run(&self, arguments: &[String], in_stream: &mut dyn Read, out_stream: &mut dyn Write) -> i32;
}

pub struct BuiltinCommand {
    handler: Rc<dyn BuiltinCommandHandler>,
    arguments: Vec<String>
}

impl BuiltinCommand {
    pub fn new(handler: Rc<dyn BuiltinCommandHandler>, arguments: Vec<String>) -> BuiltinCommand {
        return BuiltinCommand {
            handler: handler,
            arguments: arguments
        }
    }
}

impl Command for BuiltinCommand {
    fn execute(&self) -> Result<i32, CommandError> {
        return Ok(self.handler.run(
            self.arguments.as_slice(),
            &mut stdin(),
            &mut stdout()
        ));
    }

    fn execute_redirected_output(&self) -> Result<(i32, Vec<u8>), CommandError> {
        let mut write_stream: Vec<u8> = Vec::new();
        let exit_code = self.handler.run(
            self.arguments.as_slice(),
            &mut stdin(),
            &mut write_stream
        );
        return Ok((exit_code, write_stream));
    }

    fn execute_redirected_input(&self, input: Vec<u8>) -> Result<i32, CommandError> {
        return Ok(self.handler.run(
            self.arguments.as_slice(),
             &mut Cursor::new(input),
            &mut stdout()
        ));
    }

    fn execute_redirected_io(&self, input: Vec<u8>) -> Result<(i32, Vec<u8>), CommandError> {
        let mut write_stream: Vec<u8> = Vec::new();
        let exit_code = self.handler.run(
            self.arguments.as_slice(),
            &mut Cursor::new(input),
            &mut write_stream
        );
        return Ok((exit_code, write_stream));
    }
}