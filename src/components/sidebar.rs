use crate::message::Message;
use crate::state::AppState;
use iced::widget::{button, column, text, Element};
use iced::{Length, Spacing};

pub fn view(_state: &AppState) -> Element<'_, Message> {
    let btn_inicio = button(text("🏠 Início"))
        .on_press(Message::SidebarNavPressed("Início"))
        .width(Length::Fill);

    let btn_notas = button(text("📝 Notas"))
        .on_press(Message::SidebarNavPressed("Notas"))
        .width(Length::Fill);

    let content = column![
        text("VaultNote").size(24),
        btn_inicio,
        btn_notas,
    ]
    .spacing(10)
    .padding(20)
    .width(200);

    content.into()
}