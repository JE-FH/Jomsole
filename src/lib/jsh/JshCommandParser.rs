use pest::{Parser, iterators::Pairs, iterators::Pair};

use crate::lib::{CommandParser::{CommandParser, CommandParserError}, Command::Command, Commands::ExecuteCommand::ExecuteCommand};
use crate::lib::Commands::PipeCommand::PipeCommand;

#[derive(Debug, Clone)]
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

macro_rules! get_next_or_err {
	($it:expr, $expected_rule:expr, $err_msg:expr) => {
		{
		let Some(next) = $it.next() else
			{return Err(JshCommandParserError::new($err_msg.to_string()))};

		if (next.as_rule() != $expected_rule) {
			return Err(JshCommandParserError::new($err_msg.to_string()));
		}
		next
		}
	}
}

#[derive(Parser)]
#[grammar = "lib/jsh.pest"]
struct JshParser;


fn assert_rule_type(pair: &Pair<Rule>, rule: Rule, error: &str) -> Result<(), JshCommandParserError> {
	if (pair.as_rule() != rule) {
		return Err(JshCommandParserError::new(error.to_string()));
	}
	return Ok(());
}

impl JshCommandParser {
	pub fn new() -> JshCommandParser {
		return JshCommandParser {};
	}

	pub fn compose_command_from_composition(&self, target: Pair<Rule>) -> Result<Box<dyn Command>, JshCommandParserError> {
		assert_rule_type(&target, Rule::CommandComposition, "Expected composition")?;

		return self.compose_command_from_command_composition(target);
	}

	fn compose_command_from_command_composition(&self, command: Pair<Rule>) -> Result<Box<dyn Command>, JshCommandParserError> {
		assert_rule_type(&command, Rule::CommandComposition, "Expected composition command")?;

		let mut inner = command.into_inner();
		let next = get_next_or_err!(inner, Rule::ExecuteCommand, "Expected execute command");

		let first_command = self.compose_command_from_execute_command(next)?;

		let optional_next = inner.next();
		if (optional_next.is_none()) {
			return Ok(first_command);
		}

		let next = optional_next.unwrap();

		if (next.as_rule() == Rule::SerialCommand) {
			let left_command = self.compose_serial_command(next)?;
			return Ok(Box::new(PipeCommand::new(left_command, first_command)));
		}

		return Err(JshCommandParserError::new("Expected serial command".to_string()))
	}

	fn compose_serial_command(&self, command: Pair<Rule>) -> Result<Box<dyn Command>, JshCommandParserError> {
		assert_rule_type(&command, Rule::SerialCommand, "Expected serial command")?;

		let mut inner = command.into_inner();
		let next = get_next_or_err!(inner, Rule::CommandComposition, "Expected command composition");
		return self.compose_command_from_composition(next);
	}

	fn compose_command_from_execute_command(&self, command: Pair<Rule>) -> Result<Box<dyn Command>, JshCommandParserError> {
		assert_rule_type(&command, Rule::ExecuteCommand, "Expected execute command");

		let mut inner = command.into_inner();
		let next = get_next_or_err!(inner, Rule::CommandPart, "Expected command part");

		let command_name = next.as_str().to_string();
		let mut arguments = Vec::<String>::new();

		for command in inner {
			if (command.as_rule() != Rule::CommandPart) {
				return Err(JshCommandParserError::new("Expected command part".to_string()));
			}
			arguments.push(command.as_str().to_string());
		}

		return Ok(Box::new(ExecuteCommand::new(command_name, arguments)));
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