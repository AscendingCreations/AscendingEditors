use iced::Element;

pub trait UiRenderer {
    type Message;
    fn update(&mut self, msg: Self::Message);
    fn view(&self) -> Element<Self::Message>;
}
