use std::fmt;

// CONFIGURAÇÃO DE DESEMPENHO

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesempenhoApp {
    SemAnimacoes,
    Baixo,
    Medio,
    Alto,
}

impl fmt::Display for DesempenhoApp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DesempenhoApp::SemAnimacoes => {
                write!(f, "Sem animações")
            }

            DesempenhoApp::Baixo => {
                write!(f, "Baixo")
            }

            DesempenhoApp::Medio => {
                write!(f, "Médio")
            }

            DesempenhoApp::Alto => {
                write!(f, "Alto")
            }
        }
    }
}

// CONFIGURAÇÃO DE TEMAS

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemaApp {
    White,
    Dark,
}

impl std::fmt::Display for TemaApp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::White => "Modo Claro (White)",
                Self::Dark => "Modo Escuro (Dark)",
            }
        )
    }
}

// CONFIGURAÇÃO DE IDIOMA

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdiomaApp {
    Portugues,
    Ingles,
    Espanhol,
}

impl std::fmt::Display for IdiomaApp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IdiomaApp::Portugues => write!(f, "Português (Brasil)"),
            IdiomaApp::Ingles => write!(f, "English"),
            IdiomaApp::Espanhol => write!(f, "Español"),
        }
    }
}

// PRINCIPAL

#[derive(Debug, Clone)]
pub enum Message {
    CreateNewNote,
    SearchChanged(String),
    SidebarNavPressed(&'static str),
    ToggleSidebar,

    // PERFIL

    ProfileNameChanged(String),
    ProfileEmailChanged(String),
    ProfilePasswordChanged(String),
    SaveProfile,

    // CONFIGURAÇÕES

    MudarDesempenho(DesempenhoApp),
    MudarTema(TemaApp),
    MudarIdioma(IdiomaApp)

}