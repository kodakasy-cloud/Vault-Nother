#[derive(Debug, Clone)]
pub struct AppState {
    pub current_view: String,
    pub sidebar_open: bool,
    pub search_query: String,
    pub notes: Vec<String>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_view: String::from("Início"),
            sidebar_open: false,
            search_query: String::new(),
            notes: Vec::new(),
        }
    }
}