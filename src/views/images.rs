use crate::message::Message;
use crate::state::AppState;
use iced::widget::{button, column, container, row, text, text_input, Space};
use iced::{Alignment, Element, Length, Padding};

pub fn view(state: &AppState) -> Element<'_, Message> {
    // 1. Cabeçalho e Botões de Ação Rápida (Criar Pasta / Enviar Imagem)
    let title_section = column![
        text("Galeria de Imagens e Prints 🖼️").size(28),
        text("Organize suas fotos, capturas de tela e mídias em pastas ou soltas.").size(15),
    ]
    .spacing(4);

    let create_folder_btn = button(text("📁 Nova Pasta").size(13)).padding(10);
    let upload_image_btn = button(text("+ Enviar Imagem").size(13)).padding(10);

    let actions_row = row![
        create_folder_btn,
        Space::with_width(10),
        upload_image_btn,
    ];

    let header = row![
        title_section,
        Space::with_width(Length::Fill),
        actions_row,
    ]
    .align_y(Alignment::Center);

    // 2. Barra de Pesquisa de Imagens (por nome, título ou descrição)
    let search_bar = text_input("Pesquisar por título, nome do arquivo ou descrição...", &state.search_query)
        .on_input(Message::SearchChanged)
        .padding(12);

    // 3. Seção de Pastas (Organização)
    let folders_header = text("Suas Pastas").size(18);
    let folder_card_1 = container(text("📂 Projetos Pessoais (12)").size(14)).padding(12);
    let folder_card_2 = container(text("📂 Capturas de Tela (45)").size(14)).padding(12);
    let folder_card_3 = container(text("📂 Wallpapers (8)").size(14)).padding(12);

    let folders_row = row![
        folder_card_1,
        Space::with_width(10),
        folder_card_2,
        Space::with_width(10),
        folder_card_3,
    ];

    let folders_section = column![
        folders_header,
        folders_row,
    ]
    .spacing(10);

    // 4. Seção de Imagens Recentes / Detalhes (Título, Descrição, Data)
    let gallery_header = text("Imagens Soltas e Recentes").size(18);
    
    // Simulação visual de um card de imagem contendo metadados (Título, Descrição e Data)
    let image_meta_card = container(
        column![
            text("🖼️ [Pré-visualização da Imagem]").size(16),
            text("Título: Print de Configuração do Sistema").size(14),
            text("Descrição: Configuração otimizada para o ambiente de desenvolvimento.").size(12),
            text("Data: 19/08/2026").size(11),
        ]
        .spacing(4)
    )
    .padding(14)
    .width(Length::Fill);

    let gallery_section = column![
        gallery_header,
        image_meta_card,
    ]
    .spacing(10);

    // 5. Feed Rápido / Notícias do Dia relacionadas a design/mídia
    let daily_feed_header = text("📰 Notícias e Destaques do Dia").size(18);
    let daily_feed_card = container(
        text("Tendências de UI/UX e organização de ativos digitais para 2026.").size(13)
    )
    .padding(12)
    .width(Length::Fill);

    let daily_feed_section = column![
        daily_feed_header,
        daily_feed_card,
    ]
    .spacing(8);

    // 6. Layout Principal Combinando Tudo
    column![
        header,
        Space::with_height(5),
        search_bar,
        Space::with_height(5),
        folders_section,
        Space::with_height(5),
        gallery_section,
        Space::with_height(5),
        daily_feed_section,
    ]
    .spacing(15)
    .padding(Padding::new(30.0))
    .into()
}