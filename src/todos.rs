use iced::{executor, widget::text_input, Application, Theme};
use once_cell::sync::Lazy;

use crate::tasks::{Filter, Task};

static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);

#[derive(Debug)]
pub enum Todos {
    Loading,
    Loaded(State),
}

#[derive(Debug, Default)]
pub struct State {
    input_value: String,
    filter: Filter,
    tasks: Vec<Task>,
    dirty: bool,
    saving: bool,
}

#[derive(Debug, Clone)]
pub enum Message {}

impl Application for Todos {
    type Message = Message;

    type Executor = executor::Default;

    type Theme = Theme;

    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        todo!()
    }

    fn title(&self) -> String {
        todo!()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        todo!()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        todo!()
    }
}
