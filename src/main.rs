mod app;
mod message;
mod state;
mod components;
mod views;
mod theme;


fn main() -> iced::Result {
    iced::application(
        app::App::new,
        app::App::update,
        app::App::view,
    )
    .title("VaultNote")
    .run()
}