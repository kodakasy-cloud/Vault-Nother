use crate::message::Message;
use crate::state::AppState;
use crate::theme::{escala_fonte, obter_cores};

use iced::widget::{
    button,
    column,
    container,
    row,
    text,
    text_input,
    Space,
};

use iced::{Alignment, Element, Length, Padding};

pub fn view(state: &AppState) -> Element<'_, Message> {

    // ========================================================
    // TEMA
    // ========================================================

    let cores = obter_cores(state.tema_atual);

    // ========================================================
    // TAMANHO DA FONTE
    // ========================================================

    let escala = escala_fonte(state.fonte_tamanho_atual);


    // ========================================================
    // CABEÇALHO
    // ========================================================

    let title_section = column![

        text("Painel Principal")
            .size(28.0 * escala)
            .color(cores.texto),

        text("Sua central de notas, mídias, músicas e atualizações.")
            .size(15.0 * escala)
            .color(cores.texto_secundario),

    ]
    .spacing(4);


    let new_note_button = button(
        text("Nova")
            .size(13.0 * escala)
            .color(cores.texto)
    )
    .on_press(Message::CreateNewNote)
    .padding(12);


    let header = row![

        title_section,

        Space::new()
            .width(Length::Fill),

        new_note_button,

    ]
    .align_y(Alignment::Center);


    // ========================================================
    // PESQUISA
    // ========================================================

    let search_bar = text_input(
        "Pesquisar notas, fotos, arquivos e músicas...",
        &state.search_query
    )
    .on_input(Message::SearchChanged)
    .padding(12);


    // ========================================================
    // CLIMA
    // ========================================================

    let weather_today = container(
        column![

            text("Hoje: 24°C")
                .size(15.0 * escala)
                .color(cores.texto),

            text("Parcialmente Nublado")
                .size(12.0 * escala)
                .color(cores.texto_secundario),

        ]
        .spacing(3)
    )
    .padding(12);


    let weather_forecast = container(
        column![

            text("Próximos Dias")
                .size(15.0 * escala)
                .color(cores.texto),

            text("Qui: 26°C • Sex: 22°C • Sáb: 25°C")
                .size(12.0 * escala)
                .color(cores.texto_secundario),

        ]
        .spacing(3)
    )
    .padding(12);


    let weather_row = row![

        weather_today,

        Space::new()
            .width(12),

        weather_forecast,

    ];


    // ========================================================
    // MÚSICA
    // ========================================================

    let music_title = text("🎵 Música em Destaque / Vibe")
        .size(18.0 * escala)
        .color(cores.texto);


    let music_card = container(

        row![

            text("Lofi Beats para Foco & Relaxar")
                .size(14.0 * escala)
                .color(cores.texto),

            Space::new()
                .width(Length::Fill),

            text("▶ Reproduzir")
                .size(13.0 * escala)
                .color(cores.texto_secundario),

        ]
        .align_y(Alignment::Center)

    )
    .padding(14)
    .width(Length::Fill);


    let music_section = column![

        music_title,
        music_card,

    ]
    .spacing(8);


    // ========================================================
    // NOTÍCIAS
    // ========================================================

    let news_title = text("Notícias e Destaques")
        .size(18.0 * escala)
        .color(cores.texto);


    let news_card = container(

        column![

            text("VaultNote ganha suporte completo a organização de mídias pesadas.")
                .size(14.0 * escala)
                .color(cores.texto),

            text("Confira as dicas de produtividade da semana para aproveitar ao máximo sua central.")
                .size(12.0 * escala)
                .color(cores.texto_secundario),

        ]
        .spacing(4)

    )
    .padding(14)
    .width(Length::Fill);


    let news_section = column![

        news_title,
        news_card,

    ]
    .spacing(8);


    // ========================================================
    // PÁGINA
    // ========================================================

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
            iced::Background::Color(cores.fundo)
        ),

        ..Default::default()
    })

    .into()
}