use crate::item::*;
use araiseal_styles::TEXT_WHITE;
use araiseal_types::*;
use araiseal_ui::*;
use arr_macro::arr;
use iced::{
    alignment::{Alignment, Horizontal, Vertical},
    theme,
    widget::{Column, Row, Rule, Text},
    Element, Length,
};

#[derive(Educe)]
#[educe(Default)]
pub struct ItemUiData {
    #[educe(Default(expression = "arr![NumInput::new(0); 20]"))]
    pub input: [NumInput<i16, Message>; 20],
}

impl ItemUiData {
    pub fn layout(&self, item_type: ItemTypes) -> Element<Message> {
        let mut i = 0;
        let mut column = Column::new()
            .spacing(6)
            .align_items(Alignment::Center)
            .push(
                Text::new("Data Inputs")
                    .width(Length::Fill)
                    .vertical_alignment(Vertical::Bottom)
                    .horizontal_alignment(Horizontal::Center),
            )
            .push(Rule::horizontal(0));
        let mut row = Row::new().spacing(12).align_items(Alignment::Start);

        for (id, control) in self.input.iter().enumerate() {
            if i == 6 {
                i = 0;
                column = column.push(row);
                row = Row::new().spacing(12).align_items(Alignment::Start);
            }

            row = row.push(
                Column::new()
                    .spacing(5)
                    .push(
                        Text::new(data_labels(id, item_type)),
                    )
                    .push(control.view(id, i16::MIN, i16::MAX, 1, Message::DataInput)),
            );
            i += 1;
        }

        column.push(row).into()
    }
}
