#[derive(Debug, Clone)]
pub struct AppState {
    pub current_view: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_view: String::from("Início"),
        }
    }
}