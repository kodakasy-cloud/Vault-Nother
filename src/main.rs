mod app;
mod message;
mod state;
mod components;
mod views;

use app::App;

fn main() -> iced::Result {
    // Inicia o aplicativo Iced
    iced::run("VaultNote", App::update, App::view)
}