use crate::npc::*;
use araiseal_styles::{self, TEXT_WHITE};
use araiseal_types::*;
use araiseal_ui::*;
use iced::pure::{
    widget::{Column, Container, PickList, Row, Rule, Text, TextInput},
    Element,
};
use iced::{
    alignment::{Alignment, Horizontal, Vertical},
    Length,
};

#[derive(Educe)]
#[educe(Default)]
pub struct NpcUIGenerics {
    pub txt_value: String,
    #[educe(Default(expression = "NumInput::new(0)"))]
    pub sprite_input: NumInput<i32, Message>,
    #[educe(Default(expression = "NumInput::new(1)"))]
    pub level_input: NumInput<i32, Message>,
    #[educe(Default(expression = "NumInput::new(1)"))]
    pub respawn_wait_input: NumInput<i64, Message>,
    #[educe(Default(expression = "NumInput::new(1)"))]
    pub movement_wait_input: NumInput<i64, Message>,
    #[educe(Default(expression = "NumInput::new(1)"))]
    pub attack_wait_input: NumInput<i64, Message>,
    #[educe(Default(expression = "NumInput::new(1)"))]
    pub intervaled_wait_input: NumInput<i64, Message>,
    #[educe(Default(expression = "NumInput::new(1)"))]
    pub spawn_wait_input: NumInput<i64, Message>,
    #[educe(Default(expression = "NumInput::new(1)"))]
    pub maxhp_input: NumInput<u32, Message>,
    #[educe(Default(expression = "NumInput::new(0)"))]
    pub maxsp_input: NumInput<u32, Message>,
    #[educe(Default(expression = "NumInput::new(0)"))]
    pub maxmp_input: NumInput<u32, Message>,
    #[educe(Default(expression = "NumInput::new(0)"))]
    pub sight_input: NumInput<i32, Message>,
    #[educe(Default(expression = "NumInput::new(0)"))]
    pub follow_sight_input: NumInput<i32, Message>,
    #[educe(Default(expression = "NumInput::new(0)"))]
    pub walkdistance_input: NumInput<u32, Message>,
    #[educe(Default(expression = "NumInput::new(1)"))]
    pub pdamage_input: NumInput<u32, Message>,
    #[educe(Default(expression = "NumInput::new(1)"))]
    pub mdamage_input: NumInput<u32, Message>,
    #[educe(Default(expression = "NumInput::new(1)"))]
    pub pdef_input: NumInput<u32, Message>,
    #[educe(Default(expression = "NumInput::new(1)"))]
    pub mdef_input: NumInput<u32, Message>,
    #[educe(Default(expression = "NumInput::new(1)"))]
    pub sizex_input: NumInput<u8, Message>,
    #[educe(Default(expression = "NumInput::new(1)"))]
    pub sizey_input: NumInput<u8, Message>,
    #[educe(Default(expression = "NumInput::new(1)"))]
    pub sizeh_input: NumInput<u8, Message>,
    #[educe(Default(expression = "NumInput::new(1)"))]
    pub sizew_input: NumInput<u8, Message>,
    #[educe(Default(expression = "NumInput::new(1)"))]
    pub maxdamage_input: NumInput<u32, Message>,
    #[educe(Default(expression = "NumInput::new(1)"))]
    pub mindamage_input: NumInput<u32, Message>,
    pub behaviours: Vec<AIBehavior>,
    pub behaviour_selected: Option<AIBehavior>,
    pub item_drops: [Items; 10],
    #[educe(Default(expression = "NumInput::new(1)"))]
    pub max_drops: NumInput<u32, Message>,
}

#[derive(Educe)]
#[educe(Default)]
pub struct Items {
    #[educe(Default(expression = "NumInput::new(0)"))]
    pub item_id: NumInput<u32, Message>,
    #[educe(Default(expression = "NumInput::new(0)"))]
    pub chance: NumInput<u32, Message>,
    #[educe(Default(expression = "NumInput::new(0)"))]
    pub amount: NumInput<u32, Message>,
}

impl NpcUIGenerics {
    pub fn layout(&self) -> Element<Message> {
        let mut column = Column::new()
            .spacing(6)
            .align_items(Alignment::Center)
            .push(
                Text::new("Skill Scripts Appended")
                    .width(Length::Fill)
                    .vertical_alignment(Vertical::Bottom)
                    .horizontal_alignment(Horizontal::Center)
                    .color(TEXT_WHITE),
            )
            .push(Rule::horizontal(0))
            .push(
                TextInput::new("Name", &self.txt_value[..], Message::NameInput)
                    .width(Length::Units(256))
                    .style(araiseal_styles::CustomTextInput)
                    .padding(3),
            )
            .push(
                Row::new()
                    .spacing(6)
                    .push(Text::new("Sprite:").color(TEXT_WHITE))
                    .push(
                        self.sprite_input
                            .view(0, 0, 1000, 1, Message::GenericI32Input),
                    ),
            )
            .push(Text::new("Sprite placeholder").color(TEXT_WHITE))
            .push(
                Row::new()
                    .spacing(6)
                    .push(Text::new("Level:").color(TEXT_WHITE))
                    .push(
                        self.level_input
                            .view(1, 1, 200, 1, Message::GenericI32Input),
                    )
                    .push(Text::new("Respawn Wait in milsecs:").color(TEXT_WHITE))
                    .push(self.respawn_wait_input.view(
                        1,
                        1,
                        i64::MAX,
                        1,
                        Message::GenericI64Input,
                    )),
            )
            .push(
                Row::new()
                    .spacing(6)
                    .push(Text::new("Movement Wait in milsecs:").color(TEXT_WHITE))
                    .push(self.movement_wait_input.view(
                        2,
                        1,
                        i64::MAX,
                        1,
                        Message::GenericI64Input,
                    ))
                    .push(Text::new("Attack Wait in milsecs:").color(TEXT_WHITE))
                    .push(
                        self.attack_wait_input
                            .view(3, 1, i64::MAX, 1, Message::GenericI64Input),
                    ),
            )
            .push(
                Row::new()
                    .spacing(6)
                    .push(Text::new("Intervaled Wait in milsecs:").color(TEXT_WHITE))
                    .push(self.intervaled_wait_input.view(
                        4,
                        1,
                        i64::MAX,
                        1,
                        Message::GenericI64Input,
                    ))
                    .push(Text::new("Spawn Wait in milsecs:").color(TEXT_WHITE))
                    .push(
                        self.spawn_wait_input
                            .view(5, 1, i64::MAX, 1, Message::GenericI64Input),
                    ),
            )
            .push(
                Row::new()
                    .spacing(6)
                    .push(Text::new("Max HP:").color(TEXT_WHITE))
                    .push(
                        self.maxhp_input
                            .view(3, 1, u32::MAX, 1, Message::GenericU32Input),
                    )
                    .push(Text::new("Max Sp:").color(TEXT_WHITE))
                    .push(
                        self.maxsp_input
                            .view(4, 1, u32::MAX, 1, Message::GenericU32Input),
                    ),
            )
            .push(
                Row::new()
                    .spacing(6)
                    .push(Text::new("Max Mp:").color(TEXT_WHITE))
                    .push(
                        self.maxmp_input
                            .view(5, 1, u32::MAX, 1, Message::GenericU32Input),
                    )
                    .push(Text::new("Sight Range:").color(TEXT_WHITE))
                    .push(
                        self.sight_input
                            .view(2, 0, i32::MAX, 1, Message::GenericI32Input),
                    ),
            )
            .push(
                Row::new()
                    .spacing(6)
                    .push(Text::new("Follow Sight Range:").color(TEXT_WHITE))
                    .push(
                        self.follow_sight_input
                            .view(3, 0, i32::MAX, 1, Message::GenericI32Input),
                    )
                    .push(Text::new("Walk Distance from spawn:").color(TEXT_WHITE))
                    .push(self.walkdistance_input.view(
                        6,
                        0,
                        u32::MAX,
                        1,
                        Message::GenericU32Input,
                    )),
            )
            .push(
                Row::new()
                    .spacing(6)
                    .push(Text::new("Pysical Damage:").color(TEXT_WHITE))
                    .push(
                        self.pdamage_input
                            .view(7, 1, u32::MAX, 1, Message::GenericU32Input),
                    ),
            )
            .push(
                Row::new()
                    .spacing(6)
                    .push(Text::new("Pysical Defense:").color(TEXT_WHITE))
                    .push(
                        self.pdef_input
                            .view(9, 1, u32::MAX, 1, Message::GenericU32Input),
                    ),
            )
            .push(
                Text::new("Block Size")
                    .color(TEXT_WHITE)
                    .horizontal_alignment(Horizontal::Center),
            )
            .push(
                Row::new()
                    .spacing(6)
                    .push(Text::new("x:").color(TEXT_WHITE))
                    .push(
                        self.sizex_input
                            .view(0, 1, u8::MAX, 1, Message::GenericU8Input),
                    )
                    .push(Text::new("y:").color(TEXT_WHITE))
                    .push(
                        self.sizey_input
                            .view(1, 1, u8::MAX, 1, Message::GenericU8Input),
                    )
                    .push(Text::new("height:").color(TEXT_WHITE))
                    .push(
                        self.sizeh_input
                            .view(2, 1, u8::MAX, 1, Message::GenericU8Input),
                    )
                    .push(Text::new("width:").color(TEXT_WHITE))
                    .push(
                        self.sizew_input
                            .view(3, 1, u8::MAX, 1, Message::GenericU8Input),
                    ),
            )
            .push(
                Row::new()
                    .spacing(6)
                    .push(Text::new("Minimal Damage (Avoids Defense):").color(TEXT_WHITE))
                    .push(
                        self.mindamage_input
                            .view(12, 1, u32::MAX, 1, Message::GenericU32Input),
                    )
                    .push(Text::new("Max Damage (regardless of power):").color(TEXT_WHITE))
                    .push(
                        self.maxdamage_input
                            .view(11, 1, u32::MAX, 1, Message::GenericU32Input),
                    ),
            )
            .push(
                Row::new()
                    .spacing(6)
                    .push(Text::new("Behaviour:").color(TEXT_WHITE))
                    .push(
                        PickList::new(
                            &self.behaviours[..],
                            self.behaviour_selected,
                            Message::BehaviourTypeSelect,
                        )
                        .style(araiseal_styles::CustomPickList)
                        .width(Length::Fill),
                    )
                    .push(Text::new("Drops Max:").color(TEXT_WHITE))
                    .push(self.max_drops.view(13, 1, 10, 1, Message::GenericU32Input)),
            )
            .push(
                Text::new("Item Drops")
                    .width(Length::Fill)
                    .vertical_alignment(Vertical::Center)
                    .horizontal_alignment(Horizontal::Center)
                    .color(TEXT_WHITE),
            )
            .push(Rule::horizontal(0));

        let mut row = Row::new().spacing(6).align_items(Alignment::Start);
        let mut i = 0;
        let mut cur = 0;

        for (id, control) in self.item_drops.iter().enumerate() {
            if i == 5 {
                i = 0;
                column = column.push(row);
                row = Row::new().spacing(6).align_items(Alignment::Start);
            }
            row = row.push(
                //Container::new(
                Column::new()
                    .spacing(10)
                    .push(Text::new(&format!("Item ID {}", id + 1)).color(TEXT_WHITE))
                    .push(
                        control
                            .item_id
                            .view(cur, 0, u32::MAX, 1, Message::ItemInput),
                    )
                    .push(Text::new(&format!("Drop Chance {}", id + 1)).color(TEXT_WHITE))
                    .push(
                        control
                            .chance
                            .view(cur, 0, u32::MAX, 1, Message::ItemChanceInput),
                    )
                    .push(Text::new(&format!("Drop Amount {}", id + 1)).color(TEXT_WHITE))
                    .push(
                        control
                            .amount
                            .view(cur, 0, u32::MAX, 1, Message::ItemAmountInput),
                    ),
                // )
                //.style(araiseal_styles::MainContainer),
            );

            i += 1;
            cur += 1;
        }

        column.push(row).into()
    }
}
