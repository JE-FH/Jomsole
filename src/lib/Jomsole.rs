use std::{thread, time};
use crate::lib::Command::CommandError;
use crate::lib::PathResolver::PathResolver;

use super::{CommandParser::{CommandParser, CommandParserError}, CommandInterface::CommandInterface};

pub struct Jomsole< 
	TCommandParser: CommandParser,
	TCommandInterface: CommandInterface,
	TPathResolver: PathResolver,
> {
	command_parser: TCommandParser,
	command_interface: TCommandInterface,
	path_resolver: TPathResolver
}

impl<
	TCommandParser: CommandParser, 
	TCommandInterface: CommandInterface,
	TPathResolver: PathResolver
> Jomsole<TCommandParser, TCommandInterface, TPathResolver> {
	pub fn new(
		command_parser: TCommandParser,
		command_interface: TCommandInterface,
		path_resolver: TPathResolver
	) -> Jomsole<TCommandParser, TCommandInterface, TPathResolver> {
		return Jomsole {
			command_parser: command_parser,
			command_interface: command_interface,
			path_resolver: path_resolver
		};
	}

	pub fn run(&self) {
		loop {
			self.do_one_command();
		}
	}

	fn do_one_command(&self) {
		let command_text = self.command_interface.read_command("> ");
		let command_result = self.command_parser.parse_command(&command_text);
		let command = match command_result {
			Err(err) => {
				println!("Error occured: {}", err.describe());
				return;
			},
			Ok(command) => {
				command
			}
		};

		let execution_result = command.execute();
		if let Err(err) = execution_result {
			match err {
				CommandError::CouldNotExecute {reason} => {
					println!("Error: {}", reason);
				}
			}
		}

		thread::sleep(time::Duration::from_millis(1000));
	}
}
