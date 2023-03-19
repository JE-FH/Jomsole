use std::env::current_dir;
use crate::lib::Trait::ContextGenerator::ContextGenerator;

pub struct DefaultContextGenerator {

}

pub fn new() -> DefaultContextGenerator {
    return DefaultContextGenerator {};
}

impl DefaultContextGenerator {
    fn generate_current_dir_fragment(&self) -> String {
        match current_dir() {
            Ok(dir) => dir.into_os_string().to_string_lossy().to_string(),
            Err(e) => "<Directory unavailable>".to_string(),
        }
    }

    fn generate_identity_fragment(&self) -> String {
        return format!("{}@{}", whoami::username(), whoami::devicename());
    }
}

impl ContextGenerator for DefaultContextGenerator {
    fn generate_context_text(&self) -> String {
        return format!("\x1B[32m{}\x1B[0m \x1B[34m{}\x1B[0m", self.generate_identity_fragment(), self.generate_current_dir_fragment());
    }
}