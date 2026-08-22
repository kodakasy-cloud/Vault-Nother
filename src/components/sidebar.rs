use crate::message::Message;
use crate::state::AppState;
use crate::theme::obter_cores;

use iced::widget::{button, column, container, text, Space};
use iced::{Background, Border, Element, Length};

pub fn view(state: &AppState) -> Element<'_, Message> {

    let cores = obter_cores(state.tema_atual);

    // =========================================================
    // TÍTULO
    // =========================================================

    let logo = text("VaultNote")
        .size(22)
        .color(cores.texto);


    let subtitulo = text("Seu cofre digital")
        .size(11)
        .color(cores.texto_secundario);


    // =========================================================
    // BOTÕES
    // =========================================================

    let btn_principal = button(
        text("🏠  Principal")
            .size(14)
            .color(cores.texto)
    )
    .on_press(
        Message::SidebarNavPressed("Principal")
    )
    .width(Length::Fill)
    .padding(10);


    let btn_notas = button(
        text("📝  Notas")
            .size(14)
            .color(cores.texto)
    )
    .on_press(
        Message::SidebarNavPressed("Notas")
    )
    .width(Length::Fill)
    .padding(10);


    let btn_imagens = button(
        text("🖼  Imagens")
            .size(14)
            .color(cores.texto)
    )
    .on_press(
        Message::SidebarNavPressed("Imagens")
    )
    .width(Length::Fill)
    .padding(10);


    let btn_arquivos = button(
        text("📁  Arquivos")
            .size(14)
            .color(cores.texto)
    )
    .on_press(
        Message::SidebarNavPressed("Arquivos")
    )
    .width(Length::Fill)
    .padding(10);


    // =========================================================
    // CONFIGURAÇÕES
    // =========================================================

    let btn_configuracoes = button(
        text("⚙  Configurações")
            .size(14)
            .color(cores.texto)
    )
    .on_press(
        Message::SidebarNavPressed("Configurações")
    )
    .width(Length::Fill)
    .padding(10);


    // =========================================================
    // FECHAR SIDEBAR
    // =========================================================

    let btn_fechar = button(
        text("→  Fechar menu")
            .size(13)
            .color(cores.texto_secundario)
    )
    .on_press(Message::ToggleSidebar)
    .width(Length::Fill)
    .padding(10);


    // =========================================================
    // SIDEBAR
    // =========================================================

    container(
        column![

            logo,

            subtitulo,

            Space::new()
                .height(25),

            btn_principal,

            btn_notas,

            btn_imagens,

            btn_arquivos,

            Space::new()
                .height(Length::Fill),

            btn_configuracoes,

            Space::new()
                .height(8),

            btn_fechar,

        ]
        .spacing(6)
        .padding(15)
    )
    .width(230)
    .height(Length::Fill)
    .style(move |_| iced::widget::container::Style {

        background: Some(
            Background::Color(cores.card)
        ),

        border: Border {
            color: cores.borda,
            width: 1.0,
            radius: 0.0.into(),
        },

        ..Default::default()
    })
    .into()
}