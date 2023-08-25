use graphics::iced_wgpu::Renderer;
use graphics::iced_widget::core::Element;
use graphics::iced_winit::style::Theme;

pub trait Ui {
    type Message;
    fn update(&mut self, msg: Self::Message) -> Option<Box<dyn Ui<Message = Self::Message>>>;
    fn view(&self) -> Element<Self::Message, Renderer<Theme>>;
    fn title(&self) -> &str;
}
