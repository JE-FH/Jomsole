use std::error::Error;
use std::fmt;
use std::fs::{File, read_to_string};
use std::path::Path;
use toml::Table;
use crate::lib::Trait::UserSettingProvider::{UserSettingProvider, WindowsCwdHandling};

#[derive(Debug)]
pub struct ConfigReadingError {
    details: String
}

impl ConfigReadingError {
    fn new(details: &str) -> ConfigReadingError {
        return ConfigReadingError {
            details: details.to_string()
        }
    }
}

impl fmt::Display for ConfigReadingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for ConfigReadingError {
    fn description(&self) -> &str {
        &self.details
    }
}


pub struct FileUserSettingProvider {
    windows_cwd_handling: WindowsCwdHandling
}

impl FileUserSettingProvider {
    pub fn from_file(path: &Path) -> Result<FileUserSettingProvider, ConfigReadingError> {
        let config_text = match read_to_string(path) {
            Ok(fileText) => fileText,
            Err(err) => return Err(ConfigReadingError::new(&err.to_string()))
        };

        let parsed = match config_text.parse::<Table>() {
            Ok(t) => t,
            Err(err) => return Err(ConfigReadingError::new(&err.to_string()))
        };
        let mut rv = FileUserSettingProvider {
            windows_cwd_handling: WindowsCwdHandling::LaunchWithoutUNC
        };

        match parsed.get("windows") {
            Some(windows_settings) => {
                match windows_settings.as_table() {
                    Some(t) => FileUserSettingProvider::read_windows_settings(&mut rv, t)?,
                    None => return Err(ConfigReadingError::new("Expected windows to be a table"))
                }
            },
            None => ()
        };

        return Ok(rv);
    }

    fn read_windows_settings(&mut self, table: &Table) -> Result<(), ConfigReadingError> {
        match table.get("cwd_handling") {
            Some(v) => match v.as_str() {
                Some(s) => {
                    self.windows_cwd_handling = match s {
                        "FullUNC" => WindowsCwdHandling::FullUNC,
                        "LaunchWithoutUNC" => WindowsCwdHandling::LaunchWithoutUNC,
                        "NeverUNC" => WindowsCwdHandling::NeverUNC,
                        _ => return Err(ConfigReadingError::new("Unknown cwd_handling value"))
                    }
                },
                None => return Err(ConfigReadingError::new("cwd_handling needs to be a string"))
            },
            None => ()
        };
        return Ok(());
    }
}

impl UserSettingProvider for FileUserSettingProvider {
    fn windows_cwd_handling(&self) -> WindowsCwdHandling {
        return self.windows_cwd_handling.clone();
    }
}
