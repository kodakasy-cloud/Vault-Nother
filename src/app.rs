use crate::message::Message;
use crate::state::AppState;

use crate::components::sidebar;
use crate::views::{files, home, images, notes, settings};

use iced::widget::{button, column, container, row, text, Space};
use iced::{Element, Length};

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

            // =========================
            // SIDEBAR
            // =========================

            Message::SidebarNavPressed(view_name) => {
                self.state.current_view = String::from(view_name);
            }

            Message::ToggleSidebar => {
                self.state.sidebar_open = !self.state.sidebar_open;
            }


            // =========================
            // NOTAS
            // =========================

            Message::CreateNewNote => {
                let new_note_title =
                    format!("Nota {}", self.state.notes.len() + 1);

                self.state.notes.push(new_note_title);
            }


            // =========================
            // PESQUISA
            // =========================

            Message::SearchChanged(query) => {
                self.state.search_query = query;
            }


            // =========================
            // PERFIL
            // =========================

            Message::ProfileNameChanged(name) => {
                self.state.profile_name = name;
            }

            Message::ProfileEmailChanged(email) => {
                self.state.profile_email = email;
            }

            Message::ProfilePasswordChanged(password) => {
                self.state.profile_password = password;
            }

            Message::SaveProfile => {
                // Futuramente salvar perfil
            }


            // =========================
            // CONFIGURAÇÕES
            // =========================

            Message::MudarTema(novo_tema) => {
                self.state.tema_atual = novo_tema;
            }

            Message::MudarIdioma(novo_idioma) => {
                self.state.idioma_atual = novo_idioma;
            }

            Message::MudarDesempenho(novo_nivel) => {
                self.state.desempenho_atual = novo_nivel;
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {

        let content_view = match self.state.current_view.as_str() {

            "Notas" => notes::view(&self.state),

            "Imagens" => images::view(&self.state),

            "Arquivos" => files::view(&self.state),

            "Configurações" => settings::view(&self.state),

            _ => home::view(&self.state),
        };


        let main_content: Element<'_, Message> =
            if self.state.sidebar_open {

                content_view

            } else {

                let open_button = button(
                    text("☰ Menu")
                        .size(14)
                )
                .on_press(Message::ToggleSidebar)
                .padding(10);


                let top_bar = row![
                    Space::new().width(Length::Fill),
                    open_button
                ]
                .width(Length::Fill);


                column![
                    top_bar,
                    content_view
                ]
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
            };


        let layout: Element<'_, Message> =
            if self.state.sidebar_open {

                let sidebar_view =
                    sidebar::view(&self.state);

                // SIDEBAR À DIREITA
                row![
                    main_content,
                    sidebar_view
                ]
                .width(Length::Fill)
                .height(Length::Fill)
                .into()

            } else {

                row![
                    main_content
                ]
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
            };


        container(layout)
            .width(Length::Fill)
            .height(Length::Fill)
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