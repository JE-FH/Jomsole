use std::env;
use std::ffi::OsString;
use std::path::Path;
use crate::lib::Command::CommandError;
use super::super::PathResolver::{PathResolver};

pub struct WindowsPathResolver {
    path: Vec<&Path>,

}

fn new() -> WindowsPathResolver {
    let path: Vec<&Path> = match env::var("PATH") {
        Ok(paths) => {
            paths
                .split(";")
                .map(|path|  Path::new(path))
                .collect()
        },
        Err(err) => {
            Vec::new()
        }
    };

    return WindowsPathResolver {
        path: path
    };
}

impl PathResolver for WindowsPathResolver {
    fn resolve(given_path: &str) -> Result<OsString, CommandError> {

    }
}