use std::collections::{HashMap, HashSet};
use std::env;
use std::ffi::{OsStr, OsString};
use std::fs::read_dir;
use std::io::BufRead;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::{absolute, Path, PathBuf};
use std::thread::current;
use log::{debug, info};
use crate::lib::Trait::Command::CommandError;
use crate::lib::Trait::PathResolver::PathResolver;

pub struct WindowsPathResolver {
    path: Vec<OsString>,
    extensions: Vec<OsString>
}

fn split_vec(target: &[u16], splitter: u16) -> Vec<&[u16]> {
    let mut next_slice_begin: usize = 0;
    let mut rv = Vec::new();
    for i in 0..target.len() {
        if target[i] == splitter {
            if next_slice_begin < i {
                rv.push(&target[next_slice_begin..i]);
            }
            next_slice_begin = i + 1;
        }
    }
    if next_slice_begin < target.len() {
        rv.push(&target[next_slice_begin..target.len()]);
    }
    return rv;
}

const SEMICOLON_ENCODING: u16 = 0x3B;

pub fn new() -> WindowsPathResolver {
    let path = match env::var_os("Path") {
        Some(path) => {
            let character_array: Vec<u16> = path.encode_wide().collect();
            split_vec( character_array.as_slice(), SEMICOLON_ENCODING)
                .iter()
                .map(|pp| OsString::from_wide(pp))
                .collect()
        },
        None => Vec::new()
    };

    let mut extensions = match env::var_os("PATHEXT") {
        Some(path) => {
            let character_array: Vec<u16> = path.encode_wide().collect();
                split_vec( character_array.as_slice(), SEMICOLON_ENCODING)
                    .iter()
                    .map(|pp| OsString::from_wide(pp))
                    .collect()
        },
        None => Vec::new()
    };

    extensions.push(OsString::new());

    return WindowsPathResolver {
        path: path,
        extensions: extensions
    };
}

impl WindowsPathResolver {
    fn look_in_directory(&self, directory: &OsString, command_name: &str) -> Option<OsString> {
        let testing_path = Path::new(directory).join(command_name).into_os_string();
        for extension in &self.extensions {
            let mut path_with_extension = testing_path.clone();
            path_with_extension.push(extension);
            match Path::new(&path_with_extension).canonicalize() {
                Ok(p) => {
                    return Some(p.into_os_string())
                },
                _ => {}
            };
        }

        return None;
    }
}

impl PathResolver for WindowsPathResolver {
    fn resolve_command_global(&self, command_name: &str) -> Result<OsString, CommandError> {
        let paths: Vec<OsString> = self.path.iter()
            .filter_map(|p| match self.look_in_directory(p, command_name) {
                Some(p) => Some(p),
                None => None
            })
            .take(1)
            .collect();

        if paths.len() == 0 {
            return Err(CommandError::CouldNotExecute {
                reason: "Command could not be found".to_string()
            });
        }

        return Ok(paths[0].clone());
    }

    fn resolve_command_local(&self, current_directory: &Path, given_path: &str) -> Result<OsString, CommandError> {
        return Ok(current_directory
            .join(given_path)
            .as_os_str().to_os_string());
    }
}