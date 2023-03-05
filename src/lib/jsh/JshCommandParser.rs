use pest::{Parser, iterators::Pairs, iterators::Pair};

use crate::lib::{CommandParser::{CommandParser, CommandParserError}, Command::Command, Commands::ExecuteCommand::ExecuteCommand};

pub struct JshCommandParserError {
	reason: String,
}

impl JshCommandParserError {
	pub fn new(reason: String) -> JshCommandParserError {
		return JshCommandParserError { reason:  reason };
	}
}

impl CommandParserError for JshCommandParserError {
    fn describe(&self) -> String {
        return self.reason.clone();
    }
}

pub struct JshCommandParser {

}

#[derive(Parser)]
#[grammar = "lib/jsh.pest"]
struct JshParser;

impl JshCommandParser {
	pub fn new() -> JshCommandParser {
		return JshCommandParser {};
	}

	fn compose_command_from_composition(&self, target: Pair<Rule>) -> Result<Box<dyn Command>, JshCommandParserError> {
		if (target.as_rule() != Rule::CommandComposition) {
			return Err(JshCommandParserError::new("Expected composition".to_string()));
		}

		match target.as_rule() {
			Rule::CommandComposition => {

			}
		}
		return Err(JshCommandParserError::new("Expected execute command".to_string()))
	}

	fn compose_command_from_command_composition(&self, command: Pair<Rule>) -> Result<Box<dyn Command>, JshCommandParserError> {
		if (target.as_rule() != Rule::CommandComposition) {
			return Err(JshCommandParserError::new("Expected composition command".to_string()));
		}

		let mut inner = command.into_inner();
		let Some(next) = inner.next() else
			{return Err(JshCommandParserError::new("Expected composition command".to_string()))};

		if (next.as_rule() != Rule::ExecuteCommand) {
			Err(JshCommandParserError::new("Expected execute command".to_string()));
		}

		let first_command_result = self.compose_command_from_execute_command(next);
		if (first_command_result.is_err()) {
			return first_command_result;
		}
		let first_command = first_command_result.unwrap();

		let optional_next = inner.next();
		if (optional_next.is_none()) {
			return Ok(first_command);
		}


	}

	fn compose_serial_command(&self, command: Pair<Rule>) -> Result<(ExecuteCommand, Optional<Box<dyn Command>>), JshCommandParserError> {
		//Return the execute command and compose the optional serial command after

	}

	fn compose_command_from_execute_command(&self, command: Pair<Rule>) -> Result<Box<dyn Command>, JshCommandParserError> {
		if (target.as_rule() != Rule::ExecuteCommand) {
			return Err(JshCommandParserError::new("Expected execute command".to_string()));
		}

		let mut inner = command.into_inner();
		let Some(next) = inner.next() else
			{return Err(JshCommandParserError::new("Expected execute command".to_string()))};

		if (next.as_rule() != Rule::CommandPart) {
			Err(JshCommandParserError::new("Expected command part".to_string()));
		}

		let command_name = next.to_string();
		let mut arguments = Vec::<String>::new();

		for command in inner {
			if (command.as_rule() != Rule::CommandPart) {
				return Err(JshCommandParserError::new("Expected command part".to_string()));
			}
			arguments.push(command.to_string());
		}

		return Ok(Box::new(ExecuteCommand::new(command_name, arguments)));
	}

	fn compose_command_from_serial(&self, command: Pair<Rule>) -> Result<Box<dyn Command>, JshCommandParserError> {
		if (target.as_rule() != Rule::SerialCommand) {
			return Err(JshCommandParserError::new("Expected serial command".to_string()));
		}

		let mut inner = command.into_inner();
		let Some(next) = inner.next() else
			{ return Err(JshCommandParserError::new("Expected command composition".to_string())); }

		let first_command =
			self.compose_command_from_composition(next);


	}
}

impl CommandParser for JshCommandParser {
	type TCommandParserError = JshCommandParserError;

	fn parse_command(&self, command: &str) -> Result<Box<dyn Command>, JshCommandParserError> {
		let parse_result = JshParser::parse(Rule::CommandComposition, command);
		if (!parse_result.is_ok()) {
			return Result::Err(JshCommandParserError::new("Syntax error".to_string()));
		}

		let first_pair = parse_result.unwrap().peek();
		if (first_pair.is_none()) {
			return Result::Err(JshCommandParserError::new("No command".to_string()));
		}

		return self.compose_command_from_composition(first_pair.unwrap());
	}


	
}