use crate::npc::*;
use araiseal_styles::TEXT_WHITE;
use araiseal_ui::*;
use iced::pure::{
    widget::{Column, PickList, Row, Rule, Text},
    Element,
};
use iced::{
    alignment::{Alignment, Horizontal, Vertical},
    Length,
};

#[derive(Educe)]
#[educe(Default)]
pub struct NpcUIEnemies {
    pub data: Vec<ListData>,
    pub data_selected: Option<ListData>,
    pub npc_data: Vec<ListData>,
    pub npc_selected: Option<ListData>,
    pub currentid: usize,
}

impl NpcUIEnemies {
    pub fn layout(&self) -> Element<Message> {
        Column::new()
            .spacing(6)
            .align_items(Alignment::Center)
            .push(
                Text::new(format!(
                    "Npc Enemies ({}/ {})",
                    self.currentid,
                    if !self.data.is_empty() {
                        self.data.len() - 1
                    } else {
                        0
                    }
                ))
                .width(Length::Fill)
                .vertical_alignment(Vertical::Bottom)
                .horizontal_alignment(Horizontal::Center)
                .color(TEXT_WHITE),
            )
            .push(Rule::horizontal(0))
            .push(
                Row::new()
                    .push(
                        button("Add")
                            .on_press(Message::EnemyAddButtonPress)
                            .style(araiseal_styles::Button::Primary),
                    )
                    .push(
                        button("Remove")
                            .on_press(Message::EnemyDelButtonPress)
                            .style(araiseal_styles::Button::Primary),
                    ),
            )
            .push(Rule::horizontal(0))
            .push(
                PickList::new(
                    &self.data[..],
                    self.data_selected.clone(),
                    Message::EnemyDataListSelect,
                )
                .style(araiseal_styles::CustomPickList)
                .width(Length::Fill),
            )
            .push(
                Text::new("Npc's")
                    .width(Length::Fill)
                    .vertical_alignment(Vertical::Bottom)
                    .horizontal_alignment(Horizontal::Center)
                    .color(TEXT_WHITE),
            )
            .push(
                PickList::new(
                    &self.npc_data[..],
                    self.npc_selected.clone(),
                    Message::EnemyListSelect,
                )
                .style(araiseal_styles::CustomPickList)
                .width(Length::Fill),
            )
            .into()
    }
}
