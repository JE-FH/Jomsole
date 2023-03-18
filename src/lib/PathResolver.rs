use std::ffi::OsString;
use std::path::Path;
use crate::lib::Command::CommandError;

pub trait PathResolver {
    fn resolve_command_global(&self, command_name: &str) -> Result<OsString, CommandError>;
    fn resolve_command_local(&self, current_directory: &Path, given_path: &str) -> Result<OsString, CommandError>;
}
