use crate::message::Message;
use crate::state::AppState;
use crate::components::sidebar;
use crate::views::{home, notes, images, files, settings};
use iced::widget::{row, column, container, button, text, Space};
use iced::Element;

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
            Message::ToggleSidebar => {
                self.state.sidebar_open = !self.state.sidebar_open;
            }
            Message::CreateNewNote => {
                let new_note_title = format!("Nota {}", self.state.notes.len() + 1);
                self.state.notes.push(new_note_title);
            }
            Message::SearchChanged(query) => {
                self.state.search_query = query;
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let content_view = match self.state.current_view.as_str() {
            "Notas" => notes::view(&self.state),
            "Imagens" => images::view(&self.state),
            "Arquivos" => files::view(&self.state),
            "Configurações" => settings::view(&self.state),
            _ => home::view(&self.state),
        };

        let main_content = if !self.state.sidebar_open {
            let open_btn = button(text("Menu").size(15))
                .on_press(Message::ToggleSidebar)
                .padding(10);
            
            let top_bar = row![
                Space::new(iced::Length::Fill, iced::Length::Shrink),
                open_btn
            ]
            .padding(10);

            column![
                top_bar,
                content_view
            ]
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
        } else {
            column![
                content_view
            ]
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
        };

        let mut layout = row![
            main_content
        ];

        if self.state.sidebar_open {
            let sidebar_view = sidebar::view(&self.state);
            layout = layout.push(sidebar_view);
        }

        container(layout)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            state: AppState::default(),
        }
    }
}