use std::fmt::Debug;
use super::Command::Command;
extern crate pest;

use pest::Parser;
use crate::lib::Trait::BuiltinCommandRepository::BuiltinCommandRepository;

pub trait CommandParserError {
	fn describe(&self) -> String;
}

pub trait CommandParser {
	type TCommandParserError: CommandParserError + Debug + Clone;
	fn parse_command(&self, command: &str) -> Result<Box<dyn Command>, Self::TCommandParserError>;
}

