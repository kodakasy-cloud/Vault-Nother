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

    // ============================================================
    // TEMA
    // ============================================================

    let cores = obter_cores(state.tema_atual);


    // ============================================================
    // TÍTULO
    // ============================================================

    let title_section = column![

        text("Painel Principal")
            .size(28)
            .color(cores.texto),

        text("Sua central de notas, mídias, músicas e atualizações.")
            .size(15)
            .color(cores.texto_secundario),

    ]
    .spacing(4);


    // ============================================================
    // BOTÃO NOVA NOTA
    // ============================================================

    let new_note_button = button(
        text("Nova")
            .size(13)
            .color(cores.texto)
    )
    .on_press(Message::CreateNewNote)
    .padding(12);


    // ============================================================
    // HEADER
    // ============================================================

    let header = row![

        title_section,

        Space::new()
            .width(Length::Fill),

        new_note_button,

    ]
    .align_y(Alignment::Center);


    // ============================================================
    // BARRA DE PESQUISA
    // ============================================================

    let search_bar = text_input(
        "Pesquisar notas, fotos, arquivos e músicas...",
        &state.search_query,
    )
    .on_input(Message::SearchChanged)
    .padding(12);


    // ============================================================
    // CLIMA - HOJE
    // ============================================================

    let weather_today = container(

        column![

            text("Hoje: 24°C")
                .size(15)
                .color(cores.texto),

            text("Parcialmente Nublado")
                .size(12)
                .color(cores.texto_secundario),

        ]
        .spacing(3)

    )
    .padding(12)
    .width(Length::Fill)
    .style(move |_| iced::widget::container::Style {

        background: Some(
            Background::Color(cores.card)
        ),

        border: Border {
            color: cores.borda,
            width: 1.0,
            radius: 8.0.into(),
        },

        ..Default::default()
    });


    // ============================================================
    // PREVISÃO
    // ============================================================

    let weather_forecast = container(

        column![

            text("Próximos Dias")
                .size(15)
                .color(cores.texto),

            text("Qui: 26°C • Sex: 22°C • Sáb: 25°C")
                .size(12)
                .color(cores.texto_secundario),

        ]
        .spacing(3)

    )
    .padding(12)
    .width(Length::Fill)
    .style(move |_| iced::widget::container::Style {

        background: Some(
            Background::Color(cores.card)
        ),

        border: Border {
            color: cores.borda,
            width: 1.0,
            radius: 8.0.into(),
        },

        ..Default::default()
    });


    // ============================================================
    // LINHA DO CLIMA
    // ============================================================

    let weather_row = row![

        weather_today,

        Space::new()
            .width(12),

        weather_forecast,

    ]
    .width(Length::Fill);


    // ============================================================
    // MÚSICA
    // ============================================================

    let music_title = text("🎵 Música em Destaque / Vibe")
        .size(18)
        .color(cores.texto);


    let music_card = container(

        row![

            text("Lofi Beats para Foco & Relaxar")
                .size(14)
                .color(cores.texto),

            Space::new()
                .width(Length::Fill),

            text("▶ Reproduzir")
                .size(13)
                .color(cores.texto_secundario),

        ]
        .align_y(Alignment::Center)

    )
    .padding(14)
    .width(Length::Fill)
    .style(move |_| iced::widget::container::Style {

        background: Some(
            Background::Color(cores.card)
        ),

        border: Border {
            color: cores.borda,
            width: 1.0,
            radius: 8.0.into(),
        },

        ..Default::default()
    });


    let music_section = column![

        music_title,

        music_card,

    ]
    .spacing(8);


    // ============================================================
    // NOTÍCIAS
    // ============================================================

    let news_title = text("Notícias e Destaques")
        .size(18)
        .color(cores.texto);


    let news_card = container(

        column![

            text(
                "VaultNote ganha suporte completo a organização de mídias pesadas."
            )
            .size(14)
            .color(cores.texto),

            text(
                "Confira as dicas de produtividade da semana para aproveitar ao máximo sua central."
            )
            .size(12)
            .color(cores.texto_secundario),

        ]
        .spacing(4)

    )
    .padding(14)
    .width(Length::Fill)
    .style(move |_| iced::widget::container::Style {

        background: Some(
            Background::Color(cores.card)
        ),

        border: Border {
            color: cores.borda,
            width: 1.0,
            radius: 8.0.into(),
        },

        ..Default::default()
    });


    let news_section = column![

        news_title,

        news_card,

    ]
    .spacing(8);


    // ============================================================
    // PÁGINA PRINCIPAL
    // ============================================================

    container(

        column![

            header,

            Space::new()
                .height(5),

            search_bar,

            Space::new()
                .height(5),

            weather_row,

            Space::new()
                .height(5),

            music_section,

            Space::new()
                .height(5),

            news_section,

        ]
        .spacing(15)
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