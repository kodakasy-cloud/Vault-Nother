use crate::message::Message;
use crate::state::AppState;
use iced::widget::{button, column, container, row, text, text_input, Space};
use iced::{Alignment, Element, Length, Padding};

pub fn view(state: &AppState) -> Element<'_, Message> {
    // 1. Banner de Atualizações do App
    let update_banner = container(
        row![
            text("Nova atualização disponível (v1.2.0): Melhorias de desempenho e novos temas!").size(13),
            Space::with_width(Length::Fill),
            button(text("Ver Detalhes").size(12)).padding(6)
        ]
        .align_y(Alignment::Center)
    )
    .padding(10)
    .width(Length::Fill);

    // 2. Cabeçalho com boas-vindas e botão de ação
    let title_section = column![
        text("Painel Principal").size(28),
        text("Sua central de notas, mídias, músicas e atualizações.").size(15),
    ]
    .spacing(4);

    let new_note_button = button(text(" Nova "))
        .on_press(Message::CreateNewNote)
        .padding(12);

    let header = row![
        title_section,
        Space::with_width(Length::Fill),
        new_note_button,
    ]
    .align_y(Alignment::Center);

    // 3. Barra de pesquisa de arquivos, notas e mídias
    let search_bar = text_input("Pesquisar notas, fotos, arquivos e músicas...", &state.search_query)
        .on_input(Message::SearchChanged)
        .padding(12);

    // 4. Clima do Dia e Próximos Dias (Widget de Tempo)
    let weather_today = container(column![text("Hoje: 24°C").size(15), text("Parcialmente Nublado").size(12)].spacing(3)).padding(12);
    let weather_forecast = container(column![text("Próximos Dias").size(15), text("Qui: 26°C • Sex: 22°C • Sáb: 25°C").size(12)].spacing(3)).padding(12);
    
    let weather_row = row![
        weather_today,
        Space::with_width(12),
        weather_forecast,
    ];

    // 5. Seção de Músicas (Player / Favoritas)
    let music_title = text("🎵 Música em Destaque / Vibe").size(18);
    let music_card = container(
        row![
            text("Lofi Beats para Foco & Relaxar").size(14),
            Space::with_width(Length::Fill),
            text("▶ Reproduzir").size(13)
        ]
        .align_y(Alignment::Center)
    )
    .padding(14)
    .width(Length::Fill);

    let music_section = column![music_title, music_card].spacing(8);

    // 6. Bloco de Notícias / Feed Rápido
    let news_title = text("Notícias e Destaques").size(18);
    let news_card = container(
        column![
            text("VaultNote ganha suporte completo a organização de mídias pesadas.").size(14),
            text("Confira as dicas de produtividade da semana para aproveitar ao máximo sua central.").size(12)
        ]
        .spacing(4)
    )
    .padding(14)
    .width(Length::Fill);

    let news_section = column![news_title, news_card].spacing(8);

    // 7. Layout principal unindo todos os blocos
    column![
        update_banner,
        Space::with_height(5),
        header,
        Space::with_height(5),
        search_bar,
        Space::with_height(5),
        weather_row,
        Space::with_height(5),
        music_section,
        Space::with_height(5),
        news_section,
    ]
    .spacing(15)
    .padding(Padding::new(30.0))
    .into()
}