use crate::message::Message;
use crate::state::AppState;
use iced::widget::{column, text};
use iced::{Element, Padding};

pub fn view(_state: &AppState) -> Element<'_, Message> {
    column![
        text("Seção de Arquivos").size(32),
        text("Armazene documentos importantes e arquivos de referência.").size(16),
    ]
    .spacing(15)
    .padding(Padding::new(40.0))
    .into()
}