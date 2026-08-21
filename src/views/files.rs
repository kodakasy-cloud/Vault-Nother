use crate::message::Message;
use crate::state::AppState;
use iced::widget::{button, column, container, row, text, text_input, Space};
use iced::{Alignment, Element, Length, Padding};

pub fn view(state: &AppState) -> Element<'_, Message> {
    // 1. Cabeçalho e Botões de Armazenamento (Nuvem vs PC)
    let title_section = column![
        text("Central de Arquivos e Mídia 📁").size(28),
        text("Gerencie documentos, aplicativos, jogos e músicas com segurança local ou na nuvem.").size(15),
    ]
    .spacing(4);

    let btn_pc = button(text("💻 Armazenar no PC").size(13)).padding(10);
    let btn_cloud = button(text("☁️ Salvar na Nuvem").size(13)).padding(10);

    let storage_row = row![
        btn_pc,
        Space::with_width(8),
        btn_cloud,
    ];

    let header = row![
        title_section,
        Space::with_width(Length::Fill),
        storage_row,
    ]
    .align_y(Alignment::Center);

    // 2. Barra de Pesquisa de Arquivos
    let search_bar = text_input("Pesquisar arquivos, jogos, apps ou documentos...", &state.search_query)
        .on_input(Message::SearchChanged)
        .padding(12);

    // 3. Categorias de Arquivos (Apps, Jogos, Docs, Músicas)
    let categories_title = text("🗂️ Categorias de Armazenamento").size(18);
    
    let cat_apps = container(column![text("📦 Aplicativos").size(14), text("4 instalados").size(11)].spacing(2)).padding(12);
    let cat_games = container(column![text("🎮 Jogos").size(14), text("2 salvos").size(11)].spacing(2)).padding(12);
    let cat_docs = container(column![text("📄 Documentos").size(14), text("15 arquivos").size(11)].spacing(2)).padding(12);
    let cat_music = container(column![text("🎵 Músicas").size(14), text("8 áudios").size(11)].spacing(2)).padding(12);

    let categories_row = row![
        cat_apps,
        Space::with_width(10),
        cat_games,
        Space::with_width(10),
        cat_docs,
        Space::with_width(10),
        cat_music,
    ];

    let categories_section = column![categories_title, categories_row].spacing(10);

    // 4. Seção de Arquivos Recentes e Proteção por Senha
    let recent_header = text("🕒 Arquivos Recentes e Segurança").size(18);
    
    let recent_card = container(
        row![
            column![
                text("📄 Relatório_Final_v2.pdf").size(14),
                text("Modificado hoje • Local: PC • 🔒 Protegido por Senha").size(11),
            ]
            .spacing(3),
            Space::with_width(Length::Fill),
            button(text("Alterar Senha").size(11)).padding(6)
        ]
        .align_y(Alignment::Center)
    )
    .padding(14)
    .width(Length::Fill);

    let recent_section = column![recent_header, recent_card].spacing(10);

    // 5. Feed de Notícias e Tecnologia
    let news_title = text("📰 Notícias e Tecnologia").size(18);
    let news_card = container(
        text("Novas tecnologias de criptografia para armazenamento seguro de arquivos em 2026.").size(13)
    )
    .padding(12)
    .width(Length::Fill);

    let news_section = column![news_title, news_card].spacing(8);

    // 6. Layout Principal Combinando Tudo
    column![
        header,
        Space::with_height(5),
        search_bar,
        Space::with_height(5),
        categories_section,
        Space::with_height(5),
        recent_section,
        Space::with_height(5),
        news_section,
    ]
    .spacing(15)
    .padding(Padding::new(30.0))
    .into()
}