use std::fmt::Debug;
use super::Commands;
use super::Command::Command;
use super::Commands::ExecuteCommand::ExecuteCommand;
extern crate pest;

use pest::Parser;

pub trait CommandParserError {
	fn describe(&self) -> String;
}

pub trait CommandParser {
	type TCommandParserError: CommandParserError + Debug + Clone;

	fn parse_command(&self, command: &str) -> Result<Box<dyn Command>, Self::TCommandParserError>;
}

