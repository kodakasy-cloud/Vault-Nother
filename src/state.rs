#[derive(Debug, Clone)]
pub struct AppState {
    pub current_view: String,
    pub sidebar_open: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_view: String::from("Início"),
            sidebar_open: false, // Começa fechada por padrão
        }
    }
}