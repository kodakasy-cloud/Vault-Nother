use crate::message::Message;
use crate::state::AppState;
use iced::widget::{column, text};
use iced::{Element, Padding};

pub fn view(_state: &AppState) -> Element<'_, Message> {
    column![
        text("Seção de Imagens e Prints").size(32),
        text("Guarde e visualize suas capturas de tela e fotos organizadas.").size(16),
    ]
    .spacing(15)
    .padding(Padding::new(40.0))
    .into()
}