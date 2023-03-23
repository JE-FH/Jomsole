#![feature(absolute_path)]
mod lib;

use std::env::{current_dir, set_current_dir};
use std::fs::read_dir;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::exit;
use std::rc::Rc;
use log::{debug, info, LevelFilter};

use crate::lib::{DefaultContextGenerator, Jomsole::Jomsole, jsh::JshCommandParser::JshCommandParser};
use crate::lib::Ecma48CommandInterface::Ecma48CommandInterface;
use crate::lib::FileUserSettingProvider::FileUserSettingProvider;
use crate::lib::JshCommandRepository::JshCommandRepository;
use crate::lib::nt::WindowsFileLocator::WindowsFileLocator;
use crate::lib::nt::WindowsPathResolver;
use crate::lib::SimpleLogger::SimpleLogger;
use crate::lib::Trait::BuiltinCommandRepository::BuiltinCommandRepository;
use crate::lib::Trait::FileLocator::FileLocator;
use crate::lib::Trait::UserSettingProvider::UserSettingProvider;

static LOGGER: SimpleLogger = SimpleLogger;

#[macro_use]
extern crate pest_derive;

fn main() {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Off));

    let file_locator = if cfg!(windows) {
        WindowsFileLocator::new()
    } else {
        panic!("Unsupported OS");
    };

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

    builtin_command_repo.add_simple_command("ls".to_string(), |arguments, istream, ostream| {
        let given_dir = arguments.join(" ");
        let path_to_set = if Path::new(&given_dir).is_absolute() {
            PathBuf::from(&given_dir)
        } else {
            match current_dir() {
                Ok(dir) => dir.join(given_dir),
                Err(err) => {
                    ostream.write(err.to_string().as_bytes());
                    ostream.write("\nPlease use an absolute path\n".as_bytes());
                    return 1;
                }
            }
        };

        match read_dir(path_to_set) {
            Ok(dirs) => dirs.for_each(|entry| {
                match entry {
                    Ok(entry) => {
                        ostream.write(entry.file_name().to_string_lossy().as_bytes());
                        ostream.write("\n".as_bytes());
                    },
                    Err(err) => {
                        ostream.write(format!("<error: {}>\n", err.to_string()).as_bytes());
                    }
                }
            }),
            Err(err) => {
                ostream.write(err.to_string().as_bytes());
                ostream.write("\n".as_bytes());
                return 1;
            }
        }

        return 0;
    });

    builtin_command_repo.add_simple_command("cd".to_string(), |arguments, istream, ostream| {
        let given_dir = arguments.join(" ");
        let path_to_set = if Path::new(&given_dir).is_absolute() {
            PathBuf::from(&given_dir)
        } else {
            match current_dir() {
                Ok(dir) => dir.join(given_dir),
                Err(err) => {
                    ostream.write(err.to_string().as_bytes());
                    ostream.write("\nPlease use an absolute path\n".as_bytes());
                    return 1;
                }
            }
        };

        match path_to_set.canonicalize() {
            Ok(dir) => {
                match set_current_dir(dir) {
                    Ok(_) => (),
                    Err(err) => {
                        ostream.write(err.to_string().as_bytes());
                        ostream.write("\n".as_bytes());
                        return 1;
                    }
                }
            },
            Err(err) => {
                ostream.write(err.to_string().as_bytes());
                ostream.write("\n".as_bytes());
                return 1;
            }
        };

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
        DefaultContextGenerator::new(),
        Rc::new(FileUserSettingProvider::from_file(file_locator.get_config_folder().as_path()).unwrap())
    );

    jomsole.run();
}
