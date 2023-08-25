use crate::layout::*;
use crate::widgets::*;
use araiseal_types::*;
use arr_macro::arr;
use graphics::iced_wgpu::Renderer;
use graphics::iced_widget::{column, text, Column, Row, Rule};
use graphics::iced_winit::{
    core::{
        alignment::{Alignment, Horizontal, Vertical},
        Element, Length,
    },
    style::Theme,
};

#[derive(Educe)]
#[educe(Default)]
pub struct ItemUiData {
    #[educe(Default(expression = "arr![NumInput::new(0); 20]"))]
    pub input: [NumInput<i16, Message>; 20],
}

impl ItemUiData {
    pub fn layout(&self, item_type: ItemTypes) -> Element<Message, Renderer<Theme>> {
        let mut i: i32 = 0;
        let mut col: Column<Message, Renderer<Theme>> = column![
            text("Data Inputs")
                .width(Length::Fill)
                .vertical_alignment(Vertical::Bottom)
                .horizontal_alignment(Horizontal::Center),
            Rule::horizontal(0)
        ]
        .spacing(6)
        .align_items(Alignment::Center)
        .width(Length::Shrink);

        let mut row: Row<Message, Renderer<Theme>> =
            Row::new().spacing(12).align_items(Alignment::Start);

        for (id, control) in self.input.iter().enumerate() {
            if i == 6 {
                i = 0;
                col = col.push(row);
                row = Row::new().spacing(12).align_items(Alignment::Start);
            }

            row = row.push(
                Column::<Message, Renderer<Theme>>::with_children(vec![
                    data_labels(id, item_type).into(),
                    control.view(id, i16::MIN, i16::MAX, 1, Message::DataInput),
                ])
                .spacing(5),
            );
            i += 1;
        }

        col.push(row).into()
    }
}
