use crate::message::Message;
use crate::state::AppState;
use iced::widget::{column, text, Element};
use iced::{Length, Padding};

pub fn view(state: &AppState) -> Element<'_, Message> {
    let content = column![
        text(format!("Bem-vindo ao VaultNote!"))
            .size(28),
        text(format!("Seção atual: {}", state.current_view))
            .size(16),
    ]
    .spacing(15)
    .padding(Padding::new(40));

    content.into()
}