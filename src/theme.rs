use iced::Color;

use crate::message::TemaApp;

#[derive(Debug, Clone, Copy)]
pub struct Cores {
    pub fundo: Color,
    pub card: Color,
    pub texto: Color,
    pub texto_secundario: Color,
    pub borda: Color,
}

pub fn obter_cores(tema: TemaApp) -> Cores {

    match tema {

        // ============================================================
        // TEMA CLARO
        // ============================================================

        TemaApp::White => Cores {

            // Fundo principal
            fundo: Color::from_rgb(
                0.92,
                0.93,
                0.95,
            ),

            // Texto principal
            texto: Color::from_rgb(
                0.05,
                0.06,
                0.08,
            ),

            // Texto secundário
            texto_secundario: Color::from_rgb(
                0.25,
                0.27,
                0.30,
            ),

            // Cards
            card: Color::from_rgb(
                0.98,
                0.98,
                0.99,
            ),

            // Bordas
            borda: Color::from_rgb(
                0.78,
                0.80,
                0.83,
            ),
        },


        // ============================================================
        // TEMA ESCURO
        // ============================================================

        TemaApp::Dark => Cores {

            // Fundo principal
            fundo: Color::from_rgb(
                0.07,
                0.07,
                0.09,
            ),

            // Texto principal
            texto: Color::from_rgb(
                0.95,
                0.95,
                0.97,
            ),

            // Texto secundário
            texto_secundario: Color::from_rgb(
                0.60,
                0.60,
                0.65,
            ),

            // Cards
            card: Color::from_rgb(
                0.12,
                0.12,
                0.15,
            ),

            // Bordas
            borda: Color::from_rgb(
                0.22,
                0.22,
                0.27,
            ),
        },
    }
}