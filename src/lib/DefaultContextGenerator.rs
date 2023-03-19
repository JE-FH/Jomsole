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
}

impl ContextGenerator for DefaultContextGenerator {
    fn generate_context_text(&self) -> String {
        return format!("{}", self.generate_current_dir_fragment());
    }
}