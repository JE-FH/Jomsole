mod lib;

use std::rc::Rc;
use lib::CommandInterface::Ecma48CommandInterface;

use crate::lib::{Jomsole::Jomsole, jsh::JshCommandParser::JshCommandParser};
use crate::lib::nt::WindowsPathResolver;

#[macro_use]
extern crate pest_derive;

fn main() {
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
