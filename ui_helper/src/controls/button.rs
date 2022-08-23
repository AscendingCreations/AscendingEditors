use iced::pure::widget::{Button, Text};
use iced::{
    alignment::Horizontal,
    Length,
};

pub fn button<'a, Message: Clone>(label: &str) -> Button<'a, Message> {
    Button::new(Text::new(label).horizontal_alignment(Horizontal::Center))
        .padding(12)
        .width(Length::Shrink)
}
