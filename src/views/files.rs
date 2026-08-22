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

        text("Arquivos")
            .size(28)
            .color(cores.texto),

        text("Gerencie seus documentos e arquivos.")
            .size(14)
            .color(cores.texto_secundario),

    ]
    .spacing(4);

    let new_file_button = button(
        text("+ Novo Arquivo")
            .size(13)
            .color(cores.texto)
    )
    .padding(12);

    let header = row![

        title,

        Space::new()
            .width(Length::Fill),

        new_file_button,

    ]
    .align_y(Alignment::Center);


    // ============================================================
    // PESQUISA
    // ============================================================

    let search = text_input(
        "🔍 Pesquisar arquivos...",
        &state.search_query,
    )
    .on_input(Message::SearchChanged)
    .padding(12)
    .width(Length::Fill);


    // ============================================================
    // PASTAS
    // ============================================================

    let folders_title = text("Pastas")
        .size(18)
        .color(cores.texto);

    let folder_projects = folder_card(
        "📁",
        "Projetos",
        cores,
    );

    let folder_studies = folder_card(
        "📁",
        "Estudos",
        cores,
    );

    let folder_work = folder_card(
        "📁",
        "Trabalho",
        cores,
    );

    let folder_personal = folder_card(
        "📁",
        "Pessoal",
        cores,
    );

    let folders = row![

        folder_projects,
        folder_studies,
        folder_work,
        folder_personal,

    ]
    .spacing(10);


    // ============================================================
    // ARQUIVOS RECENTES
    // ============================================================

    let files_title = text("Arquivos recentes")
        .size(18)
        .color(cores.texto);

    let file_1 = file_row(
        "📄",
        "documento.pdf",
        "PDF",
        "2.4 MB",
        cores,
    );

    let file_2 = file_row(
        "📦",
        "projeto.zip",
        "ZIP",
        "18 MB",
        cores,
    );

    let file_3 = file_row(
        "📊",
        "orçamento.xlsx",
        "XLSX",
        "1.2 MB",
        cores,
    );

    let file_4 = file_row(
        "📝",
        "anotacoes.txt",
        "TXT",
        "8 KB",
        cores,
    );

    let files_list = column![

        file_1,
        file_2,
        file_3,
        file_4,

    ]
    .spacing(2);


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

            folders_title,

            folders,

            Space::new()
                .height(5),

            files_title,

            files_list,

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
// CARD DE PASTA
// ================================================================

fn folder_card<'a>(
    icon: &'a str,
    name: &'a str,
    cores: crate::theme::Cores,
) -> Element<'a, Message> {

    container(

        column![

            text(icon)
                .size(25),

            text(name)
                .size(13)
                .color(cores.texto),

        ]
        .spacing(6)

    )
    .padding(15)
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


// ================================================================
// LINHA DE ARQUIVO
// ================================================================

fn file_row<'a>(
    icon: &'a str,
    name: &'a str,
    extension: &'a str,
    size: &'a str,
    cores: crate::theme::Cores,
) -> Element<'a, Message> {

    container(

        row![

            text(icon)
                .size(22),

            column![

                text(name)
                    .size(14)
                    .color(cores.texto),

                text(extension)
                    .size(11)
                    .color(cores.texto_secundario),

            ]
            .spacing(2),

            Space::new()
                .width(Length::Fill),

            text(size)
                .size(12)
                .color(cores.texto_secundario),

        ]
        .align_y(Alignment::Center)

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
            radius: 6.0.into(),
        },

        ..Default::default()
    })
    .into()
}