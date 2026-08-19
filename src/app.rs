use crate::message::Message;
use crate::state::AppState;
use crate::components::sidebar;
use crate::views::home;
use iced::widget::{row, container, Element};
use iced::{Length, Theme};

pub struct App {
    state: AppState,
}

impl App {
    pub fn new() -> (Self, iced::Task<Message>) {
        (
            Self {
                state: AppState::default(),
            },
            iced::Task::none(),
        )
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::SidebarNavPressed(view_name) => {
                self.state.current_view = String::from(view_name);
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let sidebar_view = sidebar::view(&self.state);
        let content_view = home::view(&self.state);

        let layout = row![
            sidebar_view,
            content_view
        ]
        .width(Length::Fill)
        .height(Length::Fill);

        container(layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

// Implementação necessária para o Iced executar o App sem dados iniciais complexos
impl Default for App {
    fn default() -> Self {
        Self {
            state: AppState::default(),
        }
    }
}