use crate::message::Message;
use crate::state::AppState;
use iced::widget::{button, column, container, row, text, text_input, Space};
use iced::{Alignment, Element, Length, Padding};

pub fn view(state: &AppState) -> Element<'_, Message> {
    // 1. Cabeçalho Principal
    let title_section = column![
        text("Configurações e Preferências").size(28),
        text("Personalize a aparência, o desempenho e os dados da sua conta no VaultNote.").size(15),
    ]
    .spacing(4);

    let save_settings_btn = button(text("Salvar Alterações").size(13)).padding(10);

    let header = row![
        title_section,
        Space::with_width(Length::Fill),
        save_settings_btn,
    ]
    .align_y(Alignment::Center);

    // 2. Seção de Perfil do Usuário
    let profile_title = text("👤 Perfil de Usuário").size(18);
    let profile_card = container(
        row![
            column![
                text("Nome: Desenvolvedor VaultNote").size(14),
                text("E-mail: user@vaultnote.com").size(12),
            ]
            .spacing(3),
            Space::with_width(Length::Fill),
            button(text("Editar Perfil").size(11)).padding(6)
        ]
        .align_y(Alignment::Center)
    )
    .padding(14)
    .width(Length::Fill);

    let profile_section = column![profile_title, profile_card].spacing(8);

    // 3. Seção de Aparência (Tema, Cores e Tamanho de Letra)
    let appearance_title = text("Aparência e Acessibilidade").size(18);
    let appearance_card = container(
        column![
            text("Cor do Aplicativo: Azul Padrão (Mudar)").size(13),
            text("Tamanho das Letras: Médio (16px)").size(13),
        ]
        .spacing(8)
    )
    .padding(14)
    .width(Length::Fill);

    let appearance_section = column![appearance_title, appearance_card].spacing(8);

    // 4. Seção de Desempenho (Baixa Qualidade e Sem Animações)
    let performance_title = text("Desempenho e Economia").size(18);
    let performance_card = container(
        column![
            text("Modo Sem Animações: [ Ativado / Desativado ]").size(13),
            text("Modo Baixa Qualidade (Economia de Recursos): [ Desativado ]").size(13),
        ]
        .spacing(8)
    )
    .padding(14)
    .width(Length::Fill);

    let performance_section = column![performance_title, performance_card].spacing(8);

    // 5. Seção de Idioma e Localidade
    let language_title = text("Idioma").size(18);
    let language_card = container(
        text("Idioma Atual: Português (Brasil) • Clique para alterar").size(13)
    )
    .padding(14)
    .width(Length::Fill);

    let language_section = column![language_title, language_card].spacing(8);

    // 6. Seção de Suporte e Ajuda
    let support_title = text("🛟 Suporte e Sobre").size(18);
    let support_card = container(
        column![
            text("Central de Ajuda e FAQ").size(13),
            text("Versão do Aplicativo: v1.2.0 (Build 2026)").size(12),
        ]
        .spacing(4)
    )
    .padding(14)
    .width(Length::Fill);

    let support_section = column![support_title, support_card].spacing(8);

    // 7. Layout Principal Unindo Tudo
    column![
        header,
        Space::with_height(5),
        profile_section,
        Space::with_height(5),
        appearance_section,
        Space::with_height(5),
        performance_section,
        Space::with_height(5),
        language_section,
        Space::with_height(5),
        support_section,
    ]
    .spacing(15)
    .padding(Padding::new(30.0))
    .into()
}