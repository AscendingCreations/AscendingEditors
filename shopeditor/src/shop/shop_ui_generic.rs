use crate::shop::*;
use ascending_types::{MAX_ITEMS, MAX_SHOP_ITEM};
use ascending_ui::*;
use iced::{
    alignment::Alignment,
    widget::{column, row, text_input, PickList, Rule},
    Element, Length,
};

#[allow(dead_code)]
#[derive(Educe)]
#[educe(Default)]
pub struct ShopUiGeneric {
    pub txt_value: String,
    //#[educe(Default(expression = "NumInput::new(0)"))]
    //pub sprite_input: NumInput<u16, Message>,
    #[educe(Default(expression = NumInput::new(0)))]
    pub max_shop_item: NumInput<u16, Message>,
    #[educe(Default(expression = Vec::with_capacity(MAX_SHOP_ITEM)))]
    pub slot_list: Vec<u16>,
    pub slot_selected: Option<u16>,
    #[educe(Default(expression = NumInput::new(0)))]
    pub item_index: NumInput<u16, Message>,
    #[educe(Default(expression = NumInput::new(0)))]
    pub item_amount: NumInput<u16, Message>,
    #[educe(Default(expression = NumInput::new(0)))]
    pub item_price: NumInput<u64, Message>,
}

impl ShopUiGeneric {
    pub fn layout(&self) -> Element<Message> {
        let row0 = row![
            "Name:",
            text_input("Name", &self.txt_value)
                .on_input(Message::NameInput)
                .width(Length::Fixed(256.0))
                .padding(3),
            "Max Shop Item",
            self.max_shop_item.view(1, 0, 20, 1, Message::GenericInput),
        ]
        .spacing(12)
        .align_items(Alignment::Center);

        let row1 = row![
            "Slot:",
            PickList::new(&self.slot_list[..], self.slot_selected, Message::SlotSelect,),
            "Index:",
            self.item_index
                .view(2, 0, MAX_ITEMS as u16, 1, Message::GenericInput),
            "Amount:",
            self.item_amount
                .view(3, 0, u16::MAX, 1, Message::GenericInput),
            "Price:",
            self.item_price
                .view(1, 0, u64::MAX, 1, Message::GenericInput64),
        ]
        .spacing(5);

        column![row0, Rule::horizontal(0), row1]
            .spacing(6)
            .align_items(Alignment::Center)
            .into()
    }
}
