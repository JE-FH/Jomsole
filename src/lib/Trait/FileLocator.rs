use std::path::{Path, PathBuf};

pub trait FileLocator {
    fn get_config_folder(&self) -> PathBuf;
}