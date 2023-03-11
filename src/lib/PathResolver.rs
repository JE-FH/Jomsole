use std::ffi::OsString;
use std::path::Path;
use crate::lib::Command::CommandError;

pub trait PathResolver {
    //Should provide a direct path to the
    fn resolve(given_path: &Path) -> Result<OsString, CommandError> {

    }
}
