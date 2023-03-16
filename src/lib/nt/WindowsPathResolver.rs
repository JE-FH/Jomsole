use std::env;
use std::ffi::OsString;
use std::path::Path;
use crate::lib::Command::CommandError;
use super::super::PathResolver::{PathResolver};

pub struct WindowsPathResolver {
    path: Vec<OsString>,
}

pub fn new() -> WindowsPathResolver {
    let path: Vec<OsString> = match env::var("PATH") {
        Ok(paths) => {
            paths
                .split(";")
                .map(|path| Path::new(path).as_os_str().to_os_string())
                .collect()
        }
        Err(err) => {
            Vec::new()
        }
    };

    return WindowsPathResolver {
        path: path
    };
}

impl PathResolver for WindowsPathResolver {
    fn resolve_command_global(&self, given_path: &str) -> Result<OsString, CommandError> {
        let os_string_paths = self.path
            .iter()
            .map(|p| Path::new(p).join(Path::new(given_path)))
            .filter(|p| p.exists())
            .map(|p| p.as_os_str().to_os_string())
            .take(1)
            .collect::<Vec<OsString>>();

        if os_string_paths.len() == 0 {
            return Err(CommandError::CouldNotExecute {
                reason: "Command could not be found".to_string()
            });
        }

        return Ok(os_string_paths[0].clone());
    }

    fn resolve_command_local(&self, current_directory: &Path, given_path: &str) -> Result<OsString, CommandError> {
        return Ok(current_directory
            .join(given_path)
            .as_os_str().to_os_string());
    }
}