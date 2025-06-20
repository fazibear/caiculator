mod caiculator;
use caiculator::Caiculator;

pub fn main() -> iced::Result {
    iced::application("cAIculator", Caiculator::update, Caiculator::view)
        .theme(Caiculator::theme)
        .window_size(Caiculator::SIZE)
        .run_with(Caiculator::new)
}
