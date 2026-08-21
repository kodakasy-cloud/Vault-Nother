use crate::message::Message;
use crate::state::AppState;
use iced::widget::{button, column, container, row, text, text_input, Space};
use iced::{Alignment, Element, Length, Padding};

pub fn view(state: &AppState) -> Element<'_, Message> {
    // 1. Cabeçalho com o título e botões de ação rápida (Diário, Nota Rápida, Link)
    let title_section = column![
        text("Central de Anotações e Diário 📝").size(28),
        text("Organize pensamentos, salve links de sites, crie diários e agende compromissos.").size(15),
    ]
    .spacing(4);

    let btn_diary = button(text("📖 Novo Diário").size(13)).padding(10);
    let btn_quick = button(text("⚡ Nota Rápida").size(13)).padding(10);
    let btn_link = button(text("🔗 Salvar Site").size(13)).padding(10);

    let actions_row = row![
        btn_diary,
        Space::with_width(8),
        btn_quick,
        Space::with_width(8),
        btn_link,
    ];

    let header = row![
        title_section,
        Space::with_width(Length::Fill),
        actions_row,
    ]
    .align_y(Alignment::Center);

    // 2. Barra de pesquisa de anotações
    let search_bar = text_input("Pesquisar notas, diários, sites ou tarefas...", &state.search_query)
        .on_input(Message::SearchChanged)
        .padding(12);

    // 3. Sistema de Personalização de Notas (Emoji, Cor, Tamanho)
    let customization_header = text("🎨 Personalização Rápida").size(18);
    let custom_bar = row![
        text("Emoji: 💡").size(13),
        Space::with_width(15),
        text("Cor: 🟦 Azul").size(13),
        Space::with_width(15),
        text("Fonte: 16px").size(13),
    ];
    let custom_section = container(
        column![customization_header, custom_bar].spacing(6)
    )
    .padding(12)
    .width(Length::Fill);

    // 4. Seção de Calendário e Agendamentos
    let calendar_title = text("📅 Calendário e Agendamentos").size(18);
    let calendar_card = container(
        column![
            text("🗓️ Agosto de 2026 • 19 Quarta-feira").size(14),
            text("Próximo evento: Reunião do projeto às 15:00").size(12),
        ]
        .spacing(4)
    )
    .padding(14)
    .width(Length::Fill);

    let calendar_section = column![calendar_title, calendar_card].spacing(8);

    // 5. Seção de Links / URLs Favoritas
    let links_title = text("🔗 Sites e URLs Salvos").size(18);
    let link_card = container(
        row![
            text("🌐 Documentação Oficial do Framework Iced").size(14),
            Space::with_width(Length::Fill),
            text("Acessar ↗").size(13)
        ]
        .align_y(Alignment::Center)
    )
    .padding(12)
    .width(Length::Fill);

    let links_section = column![links_title, link_card].spacing(8);

    // 6. Bloco de Notícias do Dia a Dia
    let news_title = text("📰 Notícias e Atualidades").size(18);
    let news_card = container(
        text("Dicas de produtividade: Como manter um diário de estudos eficiente e organizado.").size(13)
    )
    .padding(12)
    .width(Length::Fill);

    let news_section = column![news_title, news_card].spacing(8);

    // 7. Layout Principal Unindo Tudo
    column![
        header,
        Space::with_height(5),
        search_bar,
        Space::with_height(5),
        custom_section,
        Space::with_height(5),
        calendar_section,
        Space::with_height(5),
        links_section,
        Space::with_height(5),
        news_section,
    ]
    .spacing(15)
    .padding(Padding::new(30.0))
    .into()
}