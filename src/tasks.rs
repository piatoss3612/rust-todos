use iced::{
    alignment::Horizontal,
    theme::{self},
    widget::{button, checkbox, row, text, text_input, Text},
    Alignment, Element, Font, Length,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub description: String,
    pub completed: bool,

    #[serde(skip)]
    pub state: TaskState,
}

#[derive(Debug, Clone, Default)]
pub enum TaskState {
    #[default]
    Idle,
    Editing,
}

#[derive(Debug, Clone)]
pub enum TaskMessage {
    Completed(bool),
    Edit,
    DescriptionEdited(String),
    FinishEdition,
    Delete,
}

impl Task {
    pub fn text_input_id(i: usize) -> text_input::Id {
        text_input::Id::new(format!("task-{}", i))
    }

    pub fn new(description: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            description,
            completed: false,
            state: TaskState::Idle,
        }
    }

    pub fn update(&mut self, message: TaskMessage) {
        match message {
            TaskMessage::Completed(completed) => {
                self.completed = completed;
            }
            TaskMessage::Edit => {
                self.state = TaskState::Editing;
            }
            TaskMessage::DescriptionEdited(new_description) => {
                self.description = new_description;
            }
            TaskMessage::FinishEdition => {
                if !self.description.is_empty() {
                    self.state = TaskState::Idle;
                }
            }
            TaskMessage::Delete => {}
        }
    }

    pub fn view(&self, i: usize) -> Element<TaskMessage> {
        match &self.state {
            TaskState::Idle => {
                let checkbox = checkbox(&self.description, self.completed, TaskMessage::Completed)
                    .width(Length::Fill)
                    .text_shaping(text::Shaping::Advanced);

                row![
                    checkbox,
                    button(edit_icon())
                        .on_press(TaskMessage::Edit)
                        .padding(10)
                        .style(theme::Button::Text),
                ]
                .spacing(20)
                .align_items(Alignment::Center)
                .into()
            }
            TaskState::Editing => {
                let text_input = text_input("Describe your task...", &self.description)
                    .id(Self::text_input_id(i))
                    .on_input(TaskMessage::DescriptionEdited)
                    .on_submit(TaskMessage::FinishEdition)
                    .padding(10);

                row![
                    text_input,
                    button(
                        row![delete_icon(), "Delete"]
                            .spacing(10)
                            .align_items(Alignment::Center)
                    )
                    .on_press(TaskMessage::Delete)
                    .padding(10)
                    .style(theme::Button::Destructive)
                ]
                .spacing(20)
                .align_items(Alignment::Center)
                .into()
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, Eq, PartialEq)]
pub enum Filter {
    #[default]
    All,
    Active,
    Completed,
}

impl Filter {
    pub fn matches(&self, task: &Task) -> bool {
        match self {
            Filter::All => true,
            Filter::Active => !task.completed,
            Filter::Completed => task.completed,
        }
    }
}

const ICONS: Font = Font::with_name("Iced-Todos-Icons");

fn icon(unicode: char) -> Text<'static> {
    text(unicode.to_string())
        .font(ICONS)
        .width(20)
        .horizontal_alignment(Horizontal::Center)
}

fn edit_icon() -> Text<'static> {
    icon('\u{F303}')
}

fn delete_icon() -> Text<'static> {
    icon('\u{F1F8}')
}
