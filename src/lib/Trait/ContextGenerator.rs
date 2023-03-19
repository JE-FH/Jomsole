pub trait ContextGenerator {
    fn generate_context_text(&self) -> String;
}