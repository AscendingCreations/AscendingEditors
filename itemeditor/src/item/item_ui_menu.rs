use crate::item::*;
use araiseal_types::*;
use araiseal_ui::*;
use iced::{
    theme,
    widget::{PickList, Row},
    Element, Length,
};

#[derive(Educe)]
#[educe(Default)]
pub struct ItemUiMenu {
    #[educe(Default(expression = "Vec::with_capacity(MAX_ITEMS)"))]
    pub list: Vec<ListData>,
    pub list_selected: Option<ListData>,
}

impl ItemUiMenu {
    pub fn layout(&self) -> Element<Message> {
        Row::new()
            .width(Length::Fill)
            .spacing(5)
            .push(
                PickList::new(
                    &self.list[..],
                    self.list_selected.clone(),
                    Message::ListSelect,
                )
                .width(Length::Fill),
            )
            .push(button("Revert").on_press(Message::RevertButtonPress))
            .push(button("Save").on_press(Message::SaveButtonPress))
            .push(button("Save All").on_press(Message::SaveAllButtonPress))
            .into()
    }
}
