use iced::Element;

pub trait UiRenderer {
    type Message;
    fn update(
        &mut self,
        msg: Self::Message,
    ) -> Option<Box<dyn UiRenderer<Message = Self::Message>>>;
    fn view(&self) -> Element<Self::Message>;
    fn title(&self) -> &str;
}
