use iced::widget::{Button, Text};
use iced::{alignment::Horizontal, Length};

pub fn button<Message: Clone>(label: &str) -> Button<Message> {
    Button::new(Text::new(label).horizontal_alignment(Horizontal::Center))
        .padding(12)
        .width(Length::Shrink)
}
