use std::rc::Rc;
use pest::{Parser, iterators::Pairs, iterators::Pair};
use crate::lib::Commands::BuiltinCommand::BuiltinCommand;

use crate::lib::Commands::ExecuteCommand::{CommandScope, ExecuteCommand};
use crate::lib::Commands::PipeCommand::PipeCommand;
use crate::lib::JshCommandRepository::JshCommandRepository;
use crate::lib::Trait::BuiltinCommandRepository::BuiltinCommandRepository;
use crate::lib::Trait::Command::Command;
use crate::lib::Trait::CommandParser::{CommandParser, CommandParserError};
use crate::lib::Trait::PathResolver::PathResolver;

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

pub struct JshCommandParser<TBuiltinCommandRepository: BuiltinCommandRepository> {
	path_resolver: Rc<dyn PathResolver>,
	builtin_command_repository: Rc<TBuiltinCommandRepository>
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

impl<TBuiltinCommandRepository: BuiltinCommandRepository> JshCommandParser<TBuiltinCommandRepository> {
	pub fn new(
		path_resolver: Rc<dyn PathResolver>,
		builtin_command_repository: Rc<TBuiltinCommandRepository>
	) -> JshCommandParser<TBuiltinCommandRepository> {
		return JshCommandParser {
			path_resolver: path_resolver,
			builtin_command_repository: builtin_command_repository
		};
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

		if next.as_rule() == Rule::SerialCommand {
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
		assert_rule_type(&command, Rule::ExecuteCommand, "Expected execute command")?;

		let mut inner = command.into_inner();
		let next = get_next_or_err!(inner, Rule::ScopedCommand, "Expected command part");

		let (command_name, command_scope) = self.scoped_command(next)?;

		let mut arguments = Vec::<String>::new();

		for command in inner {
			if (command.as_rule() != Rule::Argument) {
				return Err(JshCommandParserError::new("Expected command part".to_string()));
			}
			arguments.push(self.parse_argument(command)?);
		}
		if command_scope == CommandScope::ANY {
			match self.builtin_command_repository.lookup_command(&command_name) {
				Some(command) => {
					return Ok(Box::new(BuiltinCommand::new(command, arguments)));
				},
				None => ()
			};
		}
		return Ok(Box::new(ExecuteCommand::new(command_name, command_scope, arguments, self.path_resolver.clone())));
	}

	fn parse_argument(&self, argument: Pair<Rule>) -> Result<String, JshCommandParserError> {
		let mut inner = argument.into_inner();
		let next = inner.next();
		match next {
			Some(argument) => {
				match argument.as_rule() {
					Rule::PlainArgument => Ok(argument.as_str().to_string()),
					Rule::QuotedArgument => Ok(self.parse_quoted_argument(argument)?),
					_ => Err(JshCommandParserError::new("Expected plain argument or quoted argument".to_string()))
				}
			},
			None => Err(JshCommandParserError::new("Expected plain argument or quoted argument".to_string()))
		}
	}

	fn parse_quoted_argument(&self, plain_argument: Pair<Rule>) -> Result<String, JshCommandParserError> {
		if (plain_argument.as_rule() != Rule::QuotedArgument) {
			return Err(JshCommandParserError::new("Expected quoted argument".to_string()));
		}
		let mut inner = plain_argument.into_inner();
		let next = get_next_or_err!(inner, Rule::QuotedContent, "Expected quoted argument");

		return Ok(next.as_str().to_string());
	}

	fn scoped_command(&self, scoped_command: Pair<Rule>) -> Result<(String, CommandScope), JshCommandParserError> {
		assert_rule_type(&scoped_command, Rule::ScopedCommand, "Expected execute command")?;
		let mut inner = scoped_command.into_inner();

		let next = match inner.next() {
			Some(command) => command,
			None => { return Err(JshCommandParserError::new("Expected local scope command or any scope command".to_string()));}
		};

		return match next.as_rule() {
			Rule::LocalScopeCommand => Ok((self.parse_argument(next)?, CommandScope::LOCAL)),
			Rule::AnyScopeCommand => Ok((self.parse_any_scoped_command(next)?, CommandScope::ANY)),
			_ => Err(JshCommandParserError::new("Expected local scope command or any scope command".to_string()))
		};
	}

	fn parse_any_scoped_command(&self, command: Pair<Rule>) -> Result<String, JshCommandParserError> {
		let mut inner = command.into_inner();
		let next = get_next_or_err!(inner, Rule::Argument, "Expected argument");

		return self.parse_argument(next);
	}
}

impl<
	TBuiltinCommandRepository: BuiltinCommandRepository
> CommandParser for JshCommandParser<TBuiltinCommandRepository> {
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