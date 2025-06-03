use iced::widget::{button, column, pick_list, row, text, Column};
use iced::Task;
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::models::ModelOptions;
use ollama_rs::Ollama;

#[derive(Default)]
pub struct Caiculator {
    previous: String,
    current: String,
    models: Vec<String>,
    current_model: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    None,
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
}

impl Caiculator {
    pub const SIZE: (f32, f32) = (500.0, 500.0);

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
                self.previous = self.current.clone();
                self.current = result;
            }
            Message::Calculate => {
                return Task::perform(
                    Self::get_result(self.current_model.clone().unwrap(), self.current.clone()),
                    Message::Result,
                );
            }
            Message::ModelSelected(mode) => {
                self.current_model = Some(mode);
            }
            Message::None => unreachable!(),
        }
        Task::none()
    }

    pub fn view(&self) -> Column<Message> {
        column![
            text(&self.previous).size(20),
            text(&self.current).size(50),
            row![
                button("C").on_press(Message::Clear),
                button("X").on_press(Message::None),
                button("<-").on_press(Message::Back),
                button("/").on_press(Message::Div),
            ]
            .spacing(10)
            .padding(10),
            row![
                button("7").on_press(Message::Digit('7')),
                button("8").on_press(Message::Digit('8')),
                button("9").on_press(Message::Digit('9')),
                button("*").on_press(Message::Mul),
            ]
            .spacing(10)
            .padding(10),
            row![
                button("4").on_press(Message::Digit('4')),
                button("5").on_press(Message::Digit('5')),
                button("6").on_press(Message::Digit('6')),
                button("-").on_press(Message::Sub),
            ]
            .spacing(10)
            .padding(10),
            row![
                button("1").on_press(Message::Digit('1')),
                button("2").on_press(Message::Digit('2')),
                button("3").on_press(Message::Digit('3')),
                button("+").on_press(Message::Add),
            ]
            .spacing(10)
            .padding(10),
            row![
                button(" ").on_press(Message::None),
                button("0").on_press(Message::Digit('0')),
                button(" ").on_press(Message::None),
                button("=").on_press(Message::Calculate),
            ]
            .spacing(10)
            .padding(10),
            pick_list(
                self.models.as_ref(),
                self.current_model.as_ref(),
                Message::ModelSelected
            )
        ]
        .padding(10)
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
}
