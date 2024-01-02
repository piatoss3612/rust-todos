use iced::{window, Application, Settings};
use todos::Todos;

pub mod persistence;
pub mod tasks;
pub mod todos;

fn main() -> iced::Result {
    Todos::run(Settings {
        window: window::Settings {
            size: (500, 800),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}
