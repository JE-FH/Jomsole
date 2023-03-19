mod lib;

use std::io::{Read, Write};
use std::process::exit;
use std::rc::Rc;
use log::{debug, info, LevelFilter};

use crate::lib::{DefaultContextGenerator, Jomsole::Jomsole, jsh::JshCommandParser::JshCommandParser};
use crate::lib::Ecma48CommandInterface::Ecma48CommandInterface;
use crate::lib::JshCommandRepository::JshCommandRepository;
use crate::lib::nt::WindowsPathResolver;
use crate::lib::SimpleLogger::SimpleLogger;
use crate::lib::Trait::BuiltinCommandRepository::BuiltinCommandRepository;

static LOGGER: SimpleLogger = SimpleLogger;

#[macro_use]
extern crate pest_derive;

fn main() {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Off));

    let path_resolver = if cfg!(windows) {
        Rc::new(WindowsPathResolver::new())
    } else {
        panic!("Unsupported OS");
    };

    let mut builtin_command_repo = JshCommandRepository::new();

    builtin_command_repo.add_simple_command("echo".to_string(), |arguments, istream, ostream| {
        for arg in arguments {
            ostream.write(arg.as_bytes());
            ostream.write(" ".as_bytes());
        }
        ostream.write("\n".as_bytes());
        return 0;
    });

    builtin_command_repo.add_simple_command("pipereader".to_string(), |arguments, istream, ostream| {
        let mut buf: Vec<u8> = Vec::new();
        istream.read_to_end(&mut buf);
        ostream.write(format!("Read {} bytes\n", buf.len()).as_bytes());
        ostream.write(buf.as_slice());
        ostream.write("EOF\n".as_bytes());
        return 0;
    });

    builtin_command_repo.add_simple_command("exit".to_string(), |arguments, istream, ostream| {
        ostream.write("Goodbye\n".as_bytes());
        exit(0);
    });

    let mut jomsole = Jomsole::new(
        JshCommandParser::new(path_resolver, Rc::new(builtin_command_repo)),
        Ecma48CommandInterface::new(),
        WindowsPathResolver::new(),
        DefaultContextGenerator::new()
    );

    jomsole.run();
}
