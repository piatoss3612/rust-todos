use iced::{
    alignment::Horizontal,
    executor, font,
    theme::{self},
    widget::{self, button, column, container, row, scrollable, text, text_input},
    window, Alignment, Application, Color, Command, Element, Length, Theme,
};
use once_cell::sync::Lazy;

use crate::{
    persistence::{LoadError, SaveError, SavedState},
    tasks::{Filter, Task, TaskMessage},
};

static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);

#[derive(Debug)]
pub enum Todos {
    Loading,
    Loaded(State),
}

#[derive(Debug, Default)]
pub struct State {
    pub input_value: String,
    pub filter: Filter,
    pub tasks: Vec<Task>,
    pub dirty: bool,
    pub saving: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<SavedState, LoadError>),
    FontLoaded(Result<(), font::Error>),
    Saved(Result<(), SaveError>),
    InputChanged(String),
    CreateTask,
    FilterChanged(Filter),
    TaskMessage(usize, TaskMessage),
    TabPressed { shift: bool },
    ToggleFullscreen(window::Mode),
}

impl Application for Todos {
    type Message = Message;

    type Executor = executor::Default;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Todos::Loading,
            Command::batch(vec![
                font::load(include_bytes!("../fonts/icons.ttf").as_slice())
                    .map(Message::FontLoaded),
                Command::perform(SavedState::load(), Message::Loaded),
            ]),
        )
    }

    fn title(&self) -> String {
        let dirty = match self {
            Todos::Loading => false,
            Todos::Loaded(state) => state.dirty,
        };

        format!("Todos{} - Iced", if dirty { "*" } else { "" })
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match self {
            Todos::Loading => {
                match message {
                    Message::Loaded(Ok(state)) => {
                        *self = Todos::Loaded(State {
                            input_value: state.input_value,
                            filter: state.filter,
                            tasks: state.tasks,
                            ..State::default()
                        });
                    }
                    Message::Loaded(Err(_)) => {
                        *self = Todos::Loaded(State::default());
                    }
                    _ => {}
                }

                text_input::focus(INPUT_ID.clone())
            }
            Todos::Loaded(state) => {
                let mut saved = false;

                let command = match message {
                    Message::InputChanged(value) => {
                        state.input_value = value;
                        Command::none()
                    }
                    Message::CreateTask => {
                        if !state.input_value.is_empty() {
                            state.tasks.push(Task::new(state.input_value.clone()));
                            state.input_value.clear();
                        }

                        Command::none()
                    }
                    Message::FilterChanged(filter) => {
                        state.filter = filter;
                        Command::none()
                    }
                    Message::TaskMessage(i, TaskMessage::Delete) => {
                        state.tasks.remove(i);
                        Command::none()
                    }
                    Message::TaskMessage(i, task_message) => {
                        if let Some(task) = state.tasks.get_mut(i) {
                            let should_focus = matches!(task_message, TaskMessage::Edit);

                            task.update(task_message);

                            if should_focus {
                                let id = Task::text_input_id(i);
                                Command::batch(vec![
                                    text_input::focus(id.clone()),
                                    text_input::select_all(id),
                                ])
                            } else {
                                Command::none()
                            }
                        } else {
                            Command::none()
                        }
                    }
                    Message::Saved(_) => {
                        state.saving = false;
                        saved = true;
                        Command::none()
                    }
                    Message::TabPressed { shift } => {
                        if shift {
                            widget::focus_previous()
                        } else {
                            widget::focus_next()
                        }
                    }
                    Message::ToggleFullscreen(mode) => window::change_mode(mode),
                    _ => Command::none(),
                };

                if !saved {
                    state.dirty = true;
                }

                let save = if state.dirty && !state.saving {
                    state.dirty = false;
                    state.saving = true;

                    Command::perform(
                        SavedState {
                            input_value: state.input_value.clone(),
                            filter: state.filter,
                            tasks: state.tasks.clone(),
                        }
                        .save(),
                        Message::Saved,
                    )
                } else {
                    Command::none()
                };

                Command::batch(vec![command, save])
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        match self {
            Todos::Loading => loading_message(),
            Todos::Loaded(State {
                input_value,
                filter,
                tasks,
                ..
            }) => {
                let title = text("todos")
                    .width(Length::Fill)
                    .size(100)
                    .style(Color::from([0.5, 0.5, 0.5]))
                    .horizontal_alignment(Horizontal::Center);

                let input = text_input("What needs to be done?", input_value)
                    .id(INPUT_ID.clone())
                    .on_input(Message::InputChanged)
                    .on_submit(Message::CreateTask)
                    .padding(15)
                    .size(30);

                let controls = view_controls(tasks, *filter);
                let filtered_tasks = tasks.iter().filter(|task| filter.matches(task));

                let tasks: Element<_> = if filtered_tasks.count() > 0 {
                    column(
                        tasks
                            .iter()
                            .enumerate()
                            .filter(|(_, task)| filter.matches(task))
                            .map(|(i, task)| {
                                task.view(i)
                                    .map(move |message| Message::TaskMessage(i, message))
                            })
                            .collect(),
                    )
                    .spacing(10)
                    .into()
                } else {
                    empty_message(match filter {
                        Filter::All => "You have not created a task yet...",
                        Filter::Active => "All your tasks are done! :D",
                        Filter::Completed => "You have not completed a task yet...",
                    })
                };

                let content = column![title, input, controls, tasks]
                    .spacing(20)
                    .max_width(800);

                scrollable(
                    container(content)
                        .width(Length::Fill)
                        .padding(40)
                        .center_x(),
                )
                .into()
            }
        }
    }
}

fn view_controls(tasks: &[Task], current_filter: Filter) -> Element<Message> {
    let tasks_left = tasks.iter().filter(|task| !task.completed).count();

    let filter_button = |label, filter, current_filter| {
        let label = text(label);

        let button = button(label).style(if filter == current_filter {
            theme::Button::Primary
        } else {
            theme::Button::Text
        });

        button.on_press(Message::FilterChanged(filter)).padding(8)
    };

    row![
        text(format!(
            "{} {} left",
            tasks_left,
            if tasks_left == 1 { "task" } else { "tasks" }
        ))
        .width(Length::Fill),
        row![
            filter_button("All", Filter::All, current_filter),
            filter_button("Active", Filter::Active, current_filter),
            filter_button("Completed", Filter::Completed, current_filter,),
        ]
        .width(Length::Shrink)
        .spacing(10)
    ]
    .spacing(20)
    .align_items(Alignment::Center)
    .into()
}

fn loading_message<'a>() -> Element<'a, Message> {
    container(
        text("Loading...")
            .horizontal_alignment(Horizontal::Center)
            .size(50),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_y()
    .into()
}

fn empty_message(message: &str) -> Element<'_, Message> {
    container(
        text(message)
            .width(Length::Fill)
            .size(25)
            .horizontal_alignment(Horizontal::Center)
            .style(Color::from([0.7, 0.7, 0.7])),
    )
    .width(Length::Fill)
    .height(200)
    .center_y()
    .into()
}
