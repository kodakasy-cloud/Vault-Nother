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

        text("Imagens")
            .size(28)
            .color(cores.texto),

        text("Organize suas imagens, prints e fotografias.")
            .size(14)
            .color(cores.texto_secundario),

    ]
    .spacing(4);

    let import_button = button(
        text("+ Importar")
            .size(13)
            .color(cores.texto)
    )
    .padding(12);

    let header = row![

        title,

        Space::new()
            .width(Length::Fill),

        import_button,

    ]
    .align_y(Alignment::Center);


    // ============================================================
    // PESQUISA
    // ============================================================

    let search = text_input(
        "🔍 Pesquisar imagens...",
        &state.search_query,
    )
    .on_input(Message::SearchChanged)
    .padding(12)
    .width(Length::Fill);


    // ============================================================
    // TÍTULO
    // ============================================================

    let recent_title = text("Imagens recentes")
        .size(18)
        .color(cores.texto);


    // ============================================================
    // GALERIA
    // ============================================================

    let image_1 = image_placeholder(
        "Imagem 01",
        cores,
    );

    let image_2 = image_placeholder(
        "Imagem 02",
        cores,
    );

    let image_3 = image_placeholder(
        "Imagem 03",
        cores,
    );

    let image_4 = image_placeholder(
        "Imagem 04",
        cores,
    );

    let gallery = row![

        image_1,
        image_2,
        image_3,
        image_4,

    ]
    .spacing(12);


    // ============================================================
    // FAVORITOS
    // ============================================================

    let favorites_title = text("Favoritos")
        .size(18)
        .color(cores.texto);

    let favorites = row![

        image_placeholder(
            "Favorito 01",
            cores,
        ),

        image_placeholder(
            "Favorito 02",
            cores,
        ),

        image_placeholder(
            "Favorito 03",
            cores,
        ),

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

            gallery,

            Space::new()
                .height(10),

            favorites_title,

            favorites,

        ]
        .spacing(10)
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
// PLACEHOLDER DE IMAGEM
// ================================================================

fn image_placeholder<'a>(
    name: &'a str,
    cores: crate::theme::Cores,
) -> Element<'a, Message> {

    container(

        column![

            Space::new()
                .height(55),

            text("🖼")
                .size(30),

            text(name)
                .size(12)
                .color(cores.texto_secundario),

            Space::new()
                .height(55),

        ]
        .align_x(Alignment::Center)

    )
    .padding(10)
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
    })
    .into()
}