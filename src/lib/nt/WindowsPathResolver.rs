use std::collections::{HashMap, HashSet};
use std::env;
use std::ffi::{OsStr, OsString};
use std::fs::read_dir;
use std::io::BufRead;
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};
use std::thread::current;
use log::{debug, info};
use crate::lib::Command::CommandError;
use super::super::PathResolver::{PathResolver};

pub struct WindowsPathResolver {
    path: Vec<OsString>,
    extensions: HashMap<OsString, usize>
}

fn split_vec(target: &[u16], splitter: u16) -> Vec<&[u16]> {
    let mut last_slice_end: usize = 0;
    let mut rv = Vec::new();
    for i in 0..target.len() {
        if target[i] == splitter {
            if last_slice_end + 1 < i {
                rv.push(&target[last_slice_end + 1..i]);
            }
            last_slice_end = i;
        }
    }
    if last_slice_end + 1 < target.len() {
        rv.push(&target[last_slice_end + 1..target.len()]);
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

    let extension_ordering = match env::var_os("PATHEXT") {
        Some(path) => {
            let character_array: Vec<u16> = path.encode_wide().collect();
                split_vec( character_array.as_slice(), SEMICOLON_ENCODING)
                    .iter()
                    .map(|pp| OsString::from_wide(pp).make_ascii_lowercase())
                    .collect()
        },
        None => Vec::new()
    };

    let extensions = HashMap::from_iter(
        extension_ordering.into_iter().enumerate()
                .map(|(i, ext)| (ext, i))
    );

    return WindowsPathResolver {
        path: path,
        extensions: extensions
    };
}

impl WindowsPathResolver {
    fn look_in_directory(&self, directory: &OsString, command_name: &str) -> Option<OsString> {
        match read_dir(directory) {
            Err(e) => None,
            Ok(dir) => {
                let candidate_extension = dir
                    .filter_map(|r| match r {
                        Ok(t) => Some(t.path()),
                        Err(t) => None
                    })
                    .filter_map(|p| match p.extension() {
                        Some(ext) => Some(p),
                        None => None
                    })
                    .filter_map(|p| {
                        info!("{:?}", p.file_stem().expect(""));
                        if p.file_stem().expect("Should be able to get file name") == command_name && p.is_file() {
                            let ext =  p.extension().expect("Was already filtered").to_os_string();
                            return Some((p, ext));
                        }
                        return None;
                    })
                    .fold(None, |current_opt: Option<(usize, (PathBuf, OsString))>, ext| {
                        info!("{:?}", ext);
                        let newMin = self.extensions.get(&ext.1).expect("Already checked").clone();
                        if let Some(current) = current_opt {
                            if newMin < current.0 {
                                return Some((newMin, ext));
                            }
                            return Some(current);
                        } else {
                            return Some((newMin, ext));
                        }
                    });
                match candidate_extension {
                    Some((_, p)) => Some(p.0.into_os_string()),
                    None => None,
                }
            }
        }
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