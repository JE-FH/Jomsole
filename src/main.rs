mod lib;

use std::rc::Rc;
use log::{debug, info, LevelFilter};
use lib::CommandInterface::Ecma48CommandInterface;

use crate::lib::{Jomsole::Jomsole, jsh::JshCommandParser::JshCommandParser};
use crate::lib::nt::WindowsPathResolver;
use crate::lib::SimpleLogger::SimpleLogger;

static LOGGER: SimpleLogger = SimpleLogger;

#[macro_use]
extern crate pest_derive;

fn main() {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Trace));
info!("bomg");
    debug!("fuq");

    let path_resolver = if cfg!(windows) {
        Rc::new(WindowsPathResolver::new())
    } else {
        panic!("Unsupported OS");
    };

    let jomsole = Jomsole::new(
        JshCommandParser::new(path_resolver),
        Ecma48CommandInterface::new(),
        WindowsPathResolver::new()
    );
    jomsole.run();
}
