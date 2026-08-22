use crate::message::DesempenhoApp;
use crate::message::TemaApp;
use crate::message::IdiomaApp;

#[derive(Debug, Clone)]
pub struct AppState {
    pub current_view: String,
    pub sidebar_open: bool,
    pub search_query: String,
    pub notes: Vec<String>,


    pub profile_name: String,
    pub profile_email: String,
    pub profile_password: String,

    pub desempenho_atual: DesempenhoApp,
    pub tema_atual: TemaApp,
    pub idioma_atual: IdiomaApp,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_view: String::from("Início"),
            sidebar_open: false,
            search_query: String::new(),
            notes: Vec::new(),

            // PERFIL

            profile_name: String::new(),
            profile_email: String::new(),
            profile_password: String::new(),  

            // CONDIGURAÇÕES
            
            desempenho_atual: DesempenhoApp::Medio, 
            tema_atual: TemaApp::White,    
            idioma_atual: IdiomaApp::Portugues,     
        }
    }
}