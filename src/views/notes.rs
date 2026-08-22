use crate::message::Message;
use crate::state::AppState;
use crate::theme::obter_cores;

use iced::widget::{
    button,
    column,
    container,
    row,
    text,
    text_input,
    Space,
};

use iced::{
    Alignment,
    Background,
    Border,
    Element,
    Length,
    Padding,
};

pub fn view(state: &AppState) -> Element<'_, Message> {

    let cores = obter_cores(state.tema_atual);

    // ============================================================
    // CABEÇALHO
    // ============================================================

    let title = column![

        text("Notas")
            .size(28)
            .color(cores.texto),

        text("Suas ideias, informações e anotações.")
            .size(14)
            .color(cores.texto_secundario),

    ]
    .spacing(4);

    let new_note_button = button(
        text("+ Nova Nota")
            .size(13)
            .color(cores.texto)
    )
    .on_press(Message::CreateNewNote)
    .padding(12);

    let header = row![

        title,

        Space::new()
            .width(Length::Fill),

        new_note_button,

    ]
    .align_y(Alignment::Center);


    // ============================================================
    // PESQUISA
    // ============================================================

    let search = text_input(
        "🔍 Pesquisar notas...",
        &state.search_query,
    )
    .on_input(Message::SearchChanged)
    .padding(12)
    .width(Length::Fill);


    // ============================================================
    // SEÇÃO
    // ============================================================

    let recent_title = text("Notas recentes")
        .size(18)
        .color(cores.texto);


    // ============================================================
    // NOTAS
    // ============================================================

    let note_1 = note_card(
        "Projeto VaultNote",
        "Ideias para o próximo update do aplicativo...",
        "Hoje",
        cores,
    );

    let note_2 = note_card(
        "Estudos Rust",
        "Ownership, borrowing e gerenciamento de memória...",
        "Ontem",
        cores,
    );

    let note_3 = note_card(
        "Projeto RPG",
        "Sistema de combate e progressão do personagem...",
        "18/08",
        cores,
    );

    let note_4 = note_card(
        "Banco de dados",
        "Estrutura SQLite e organização das tabelas...",
        "17/08",
        cores,
    );


    let notes_row_1 = row![

        note_1,
        note_2,

    ]
    .spacing(12);

    let notes_row_2 = row![

        note_3,
        note_4,

    ]
    .spacing(12);


    // ============================================================
    // PÁGINA
    // ============================================================

    container(

        column![

            header,

            Space::new()
                .height(8),

            search,

            Space::new()
                .height(8),

            recent_title,

            notes_row_1,

            notes_row_2,

        ]
        .spacing(12)
        .padding(Padding::new(30.0))

    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(move |_| iced::widget::container::Style {

        background: Some(
            Background::Color(cores.fundo)
        ),

        ..Default::default()
    })
    .into()
}


// ================================================================
// CARD DE NOTA
// ================================================================

fn note_card<'a>(
    title: &'a str,
    description: &'a str,
    date: &'a str,
    cores: crate::theme::Cores,
) -> Element<'a, Message> {

    container(

        column![

            text(title)
                .size(16)
                .color(cores.texto),

            text(description)
                .size(12)
                .color(cores.texto_secundario),

            Space::new()
                .height(8),

            row![

                text(date)
                    .size(11)
                    .color(cores.texto_secundario),

                Space::new()
                    .width(Length::Fill),

                text("•••")
                    .size(14)
                    .color(cores.texto_secundario),

            ]

        ]
        .spacing(5)

    )
    .padding(16)
    .width(Length::Fill)
    .style(move |_| iced::widget::container::Style {

        background: Some(
            Background::Color(cores.card)
        ),

        border: Border {
            color: cores.borda,
            width: 1.0,
            radius: 10.0.into(),
        },

        ..Default::default()
    })
    .into()
}