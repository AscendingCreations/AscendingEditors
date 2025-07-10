use iced::Element;

pub trait UiRenderer {
    type Message;
    fn update(&mut self, msg: Self::Message);
    fn view(&'_ self) -> Element<'_, Self::Message>;
}
