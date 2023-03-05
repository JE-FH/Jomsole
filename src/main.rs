mod lib;
use lib::CommandInterface::Ecma48CommandInterface;

use crate::lib::{Jomsole::Jomsole, jsh::JshCommandParser::JshCommandParser};
#[macro_use]
extern crate pest_derive;

fn main() {
    let jomsole = Jomsole::new(
        JshCommandParser::new(), 
        Ecma48CommandInterface::new()
    );
    jomsole.run();
}
