use std::fs::create_dir;
use std::path::{Path, PathBuf};
use winsafe::co::{KF, KNOWNFOLDERID};
use crate::lib::Trait::FileLocator::FileLocator;
use winsafe::SHGetKnownFolderPath;
pub struct WindowsFileLocator {

}

impl WindowsFileLocator {
    pub fn new() -> WindowsFileLocator {
        return WindowsFileLocator {};
    }
}

impl FileLocator for WindowsFileLocator {
    fn get_config_folder(&self) -> PathBuf {
        let result = match SHGetKnownFolderPath(&KNOWNFOLDERID::RoamingAppData, KF::default(), Option::None) {
            Ok(s) => s,
            Err(err) => panic!("Could not get special path {}", err.to_string())
        };
        let p = Path::new(&result).join("Jomsole");
        return match create_dir(&p) {
            Ok(_) => p,
            Err(err) => panic!("Could not create dir for config {}", err.to_string())
        }
    }
}
