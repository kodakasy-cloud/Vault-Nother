use crate::message::Message;
use crate::state::AppState;
use iced::widget::{column, text};
use iced::{Element, Padding};

pub fn view(_state: &AppState) -> Element<'_, Message> {
    column![
        text("Configurações").size(32),
        text("Ajustes gerais do aplicativo, tema e preferências locais.").size(16),
    ]
    .spacing(15)
    .padding(Padding::new(40.0))
    .into()
}