use crate::item::*;
use arr_macro::arr;
use ascending_types::*;
use ascending_ui::*;
use iced::{
    alignment::Alignment,
    widget::{column, text, Row, Rule},
    Element, Length,
};

#[derive(Educe)]
#[educe(Default)]
pub struct ItemUiData {
    #[educe(Default(expression = arr![NumInput::new(0); 20]))]
    pub input: [NumInput<i16, Message>; 20],
}

impl ItemUiData {
    pub fn layout(&self, item_type: ItemTypes) -> Element<Message> {
        let mut i: i32 = 0;
        let mut col = column![text("Data Inputs"), Rule::horizontal(0)]
            .spacing(6)
            .align_x(Alignment::Center)
            .width(Length::Shrink);

        let mut row = Row::new().spacing(12).align_y(Alignment::Start);

        for (id, control) in self.input.iter().enumerate() {
            if i == 6 {
                i = 0;
                col = col.push(row);
                row = Row::new().spacing(12).align_y(Alignment::Start);
            }

            row = row.push(
                column![
                    data_labels(id, item_type),
                    control.view(id, i16::MIN, i16::MAX, 1, Message::DataInput, None)
                ]
                .spacing(5),
            );
            i += 1;
        }

        col.push(row).into()
    }
}
