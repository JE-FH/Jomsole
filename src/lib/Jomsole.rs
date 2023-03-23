use std::{thread, time};
use std::rc::Rc;
use crate::lib::Trait::Command::CommandError;
use crate::lib::Trait::CommandInterface::CommandInterface;
use crate::lib::Trait::CommandParser::{CommandParser, CommandParserError};
use crate::lib::Trait::ContextGenerator::ContextGenerator;
use crate::lib::Trait::PathResolver::PathResolver;
use crate::lib::Trait::UserSettingProvider::UserSettingProvider;

pub struct Jomsole<
	TCommandParser: CommandParser,
	TCommandInterface: CommandInterface,
	TPathResolver: PathResolver,
	TContextGenerator: ContextGenerator,
	TUserSettingProvider: UserSettingProvider
> {
	command_parser: TCommandParser,
	command_interface: TCommandInterface,
	path_resolver: TPathResolver,
	context_generator: TContextGenerator,
	user_setting_provider: Rc<TUserSettingProvider>
}

impl<
	TCommandParser: CommandParser, 
	TCommandInterface: CommandInterface,
	TPathResolver: PathResolver,
	TContextGenerator: ContextGenerator,
	TUserSettingProvider: UserSettingProvider
> Jomsole<TCommandParser, TCommandInterface, TPathResolver, TContextGenerator, TUserSettingProvider> {
	pub fn new(
		command_parser: TCommandParser,
		command_interface: TCommandInterface,
		path_resolver: TPathResolver,
		context_generator: TContextGenerator,
		user_setting_provider: Rc<TUserSettingProvider>
	) -> Jomsole<TCommandParser, TCommandInterface, TPathResolver, TContextGenerator, TUserSettingProvider> {
		return Jomsole {
			command_parser: command_parser,
			command_interface: command_interface,
			path_resolver: path_resolver,
			context_generator: context_generator,
			user_setting_provider: user_setting_provider
		};
	}

	pub fn run(&self) {
		loop {
			self.do_one_command();
		}
	}

	fn do_one_command(&self) {
		let command_text = self.command_interface.read_command(&format!("{}> ", self.context_generator.generate_context_text()));
		if command_text.len() == 0 {
			return;
		}
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
		} else {
			println!("Program exited with code {}", execution_result.unwrap());
		}
	}
}
