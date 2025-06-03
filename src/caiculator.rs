use iced::widget::{column, pick_list, row, text, Button, Column, Container};
use iced::{Alignment, Length, Task, Theme};
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::models::ModelOptions;
use ollama_rs::Ollama;

#[derive(Default)]
pub struct Caiculator {
    theme: Theme,
    previous: String,
    current: String,
    models: Vec<String>,
    current_model: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Digit(char),
    ModelsList(Vec<String>),
    Add,
    Sub,
    Div,
    Mul,
    Result(String),
    Calculate,
    Clear,
    Back,
    ModelSelected(String),
    ThemeChanged(Theme),
}

impl Caiculator {
    pub const SIZE: (f32, f32) = (310.0, 600.0);

    pub fn new() -> (Self, Task<Message>) {
        (
            Self::default(),
            Task::perform(Self::get_models(), Message::ModelsList),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ModelsList(models) => {
                self.models = models;
                let first: String = self.models.first().unwrap().clone();
                self.current_model = Some(first);
            }
            Message::Digit(digit) => self.current.push(digit),
            Message::Add => self.current.push('+'),
            Message::Sub => self.current.push('-'),
            Message::Mul => self.current.push('*'),
            Message::Div => self.current.push('/'),
            Message::Back => {
                self.current.pop();
            }
            Message::Clear => {
                self.previous = "".to_string();
                self.current = "".to_string();
            }
            Message::Result(result) => {
                self.current = result;
            }
            Message::Calculate => {
                self.previous = self.current.clone();
                self.current = "Thinking...".to_string();
                return Task::perform(
                    Self::get_result(self.current_model.clone().unwrap(), self.previous.clone()),
                    Message::Result,
                );
            }
            Message::ModelSelected(mode) => {
                self.current_model = Some(mode);
            }
            Message::ThemeChanged(theme) => {
                self.theme = theme;
            }
        }
        Task::none()
    }

    pub fn view(&self) -> Column<Message> {
        column![
            text(&self.previous)
                .size(20)
                .width(Length::Fill)
                .align_x(Alignment::End),
            text(&self.current)
                .size(30)
                .width(Length::Fill)
                .align_x(Alignment::End),
            row![
                Self::button("C", Message::Clear),
                Self::button("<-", Message::Back),
                Self::empty(),
                Self::button("/", Message::Div),
            ]
            .spacing(10)
            .padding(10),
            row![
                Self::button("7", Message::Digit('7')),
                Self::button("8", Message::Digit('8')),
                Self::button("9", Message::Digit('9')),
                Self::button("*", Message::Mul),
            ]
            .spacing(10)
            .padding(10),
            row![
                Self::button("4", Message::Digit('4')),
                Self::button("5", Message::Digit('5')),
                Self::button("6", Message::Digit('6')),
                Self::button("-", Message::Sub),
            ]
            .spacing(10)
            .padding(10),
            row![
                Self::button("1", Message::Digit('1')),
                Self::button("2", Message::Digit('2')),
                Self::button("3", Message::Digit('3')),
                Self::button("+", Message::Add),
            ]
            .spacing(10)
            .padding(10),
            row![
                Self::empty(),
                Self::button("0", Message::Digit('0')),
                Self::empty(),
                Self::button("=", Message::Calculate),
            ]
            .spacing(10)
            .padding(10),
            text("select theme:").center().width(Length::Fill),
            pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged).width(Length::Fill),
            text("select model:").center().width(Length::Fill),
            pick_list(
                self.models.as_ref(),
                self.current_model.as_ref(),
                Message::ModelSelected
            )
            .width(Length::Fill)
        ]
        .padding(10)
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }

    async fn get_models() -> Vec<String> {
        let ollama = Ollama::default();
        let models = ollama.list_local_models().await.unwrap();
        let mut result = Vec::new();
        for model in models {
            result.push(model.name);
        }

        result
    }

    async fn get_result(model: String, equation: String) -> String {
        let ollama = Ollama::default();

        let options = ModelOptions::default();
        let prompt = format!("Solve following equation: #{equation}. Give me just a result as a number without any comment additional text or explanation.");

        let res = ollama
            .generate(GenerationRequest::new(model, prompt).options(options))
            .await;

        if let Ok(res) = res {
            res.response
        } else {
            "Something went wrong!".to_string()
        }
    }

    pub fn button(content: &str, message: Message) -> Button<Message> {
        Button::new(content)
            .on_press(message)
            .width(60)
            .height(60)
            .padding(10)
    }

    pub fn empty() -> Container<'static, Message> {
        Container::new("").width(60).height(60).padding(10)
    }
}
