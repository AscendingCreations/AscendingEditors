use crate::npc::*;
use ascending_types::*;
use ascending_ui::*;
use iced::{
    widget::{row, PickList},
    Element, Length,
};

#[derive(Educe)]
#[educe(Default)]
pub struct NpcUIMenu {
    #[educe(Default(expression = "Vec::with_capacity(MAX_NPCS)"))]
    pub list: Vec<ListData>,
    pub list_selected: Option<ListData>,
}

impl NpcUIMenu {
    pub fn layout(&self) -> Element<Message> {
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
