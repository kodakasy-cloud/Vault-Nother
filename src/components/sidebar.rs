use crate::message::Message;
use crate::state::AppState;
use iced::widget::{button, column, text, container, row, Column};
use iced::{Element, Length, Color};

pub fn view(state: &AppState) -> Element<'_, Message> {
    let views = vec![
        ("Início", "Início"),
        ("Notas", "Notas"),
        ("Imagens", "Imagens"),
        ("Arquivos", "Arquivos"),
        ("Configurações", "Configurações"),
    ];

    let mut nav_column = Column::new().spacing(8).width(Length::Fill);

    for (name, label_text) in views {
        let _is_active = state.current_view == name;
        
        let label = text(label_text).size(16);
        
        let btn = button(label)
            .on_press(Message::SidebarNavPressed(name))
            .width(Length::Fill)
            .padding(15);

        nav_column = nav_column.push(btn);
    }

    let header = row![
        text("VaultNote").size(20),
        button(text(" X ").size(15))
            .on_press(Message::ToggleSidebar)
            .padding(10)
    ]
    .spacing(40)
    .align_y(iced::Alignment::Center);

    let content = column![
        header,
        nav_column
    ]
    .spacing(25)
    .padding(20)
    .width(220);

    container(content)
        .width(220)
        .height(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(Color::from_rgb(0.95, 0.96, 0.98).into()),
            ..Default::default()
        })
        .into()
}