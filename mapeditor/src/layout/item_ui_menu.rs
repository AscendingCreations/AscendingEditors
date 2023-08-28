use crate::layout::*;

use araiseal_types::*;
use araiseal_ui::*;

use graphics::iced_wgpu::Renderer;
use graphics::iced_widget::{Button, PickList, Row};
use graphics::iced_winit::{
    core::{Element, Length},
    style::Theme,
};

#[derive(Educe)]
#[educe(Default)]
pub struct ItemUiMenu {
    #[educe(Default(expression = "Vec::with_capacity(MAX_ITEMS)"))]
    pub list: Vec<ListData>,
    pub list_selected: Option<ListData>,
}

impl ItemUiMenu {
    pub fn layout(&self) -> Element<Message, Renderer<Theme>> {
        Row::<Message, Renderer<Theme>>::with_children(vec![
            PickList::new(
                &self.list[..],
                self.list_selected.clone(),
                Message::ListSelect,
            )
            .width(Length::Fill)
            .into(),
            Button::new("Revert")
                .padding(12)
                .width(Length::Shrink)
                .on_press(Message::RevertButtonPress)
                .into(),
            Button::new("Save")
                .padding(12)
                .width(Length::Shrink)
                .on_press(Message::SaveButtonPress)
                .into(),
            Button::new("Save All")
                .padding(12)
                .width(Length::Shrink)
                .on_press(Message::SaveAllButtonPress)
                .into(),
        ])
        .width(Length::Fill)
        .spacing(5)
        .into()
    }
}
