use crate::shop::*;
use ascending_types::*;
use ascending_ui::*;
use iced::{
    widget::{button, row, PickList},
    Element, Length,
};

#[derive(Educe)]
#[educe(Default)]
pub struct ShopUiMenu {
    #[educe(Default(expression = Vec::with_capacity(MAX_SHOPS)))]
    pub list: Vec<ListData>,
    pub list_selected: Option<ListData>,
}

impl ShopUiMenu {
    pub fn layout(&'_ self) -> Element<'_, Message> {
        row![
            PickList::new(
                &self.list[..],
                self.list_selected.clone(),
                Message::ListSelect,
            )
            .width(Length::Fill),
            button("Revert").on_press(Message::RevertButtonPress),
            button("Save").on_press(Message::SaveButtonPress),
            button("Save All").on_press(Message::SaveAllButtonPress)
        ]
        .width(Length::Fill)
        .spacing(5)
        .into()
    }
}
