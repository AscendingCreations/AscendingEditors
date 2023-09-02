use crate::layout::{ItemUI, Message};
use crate::widgets::*;
//use araiseal_ui::*;
//use graphics::iced_wgpu::Renderer;
use graphics::iced_wgpu::Renderer;
use graphics::iced_widget::{Column, Container};
use graphics::iced_winit::{
    core::{Element, Length},
    runtime::{Command, Program},
    style::Theme,
};
pub struct Pages {
    page: Box<dyn Ui<Message = Message>>,
}

impl Pages {
    pub fn new() -> Self {
        Self {
            page: Box::new(ItemUI::new()),
        }
    }
}
impl Program for Pages {
    type Renderer = Renderer<Theme>;
    type Message = Message;

    fn update(&mut self, message: Message) -> Command<Message> {
        self.page.update(message);

        Command::none()
    }

    fn view(&self) -> Element<Message, Renderer<Theme>> {
        let page = self.page.view();

        let content = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(20)
            .padding(20)
            .push(page);

        Container::new(content).height(Length::Fill).into()
    }
}
