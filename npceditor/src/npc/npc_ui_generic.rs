use std::path::Path;

use crate::npc::*;
use ascending_types::*;
use ascending_ui::*;
use iced::{
    alignment::Alignment,
    widget::{column, container, row, text, text_input, Container, Image, PickList, Rule},
    Element, Length,
};

#[derive(Educe)]
#[educe(Default)]
pub struct NpcUIGenerics {
    pub txt_value: String,
    #[educe(Default(expression = NumInput::new(0)))]
    pub sprite_input: NumInput<i32, Message>,
    #[educe(Default(expression = NumInput::new(1)))]
    pub level_input: NumInput<i32, Message>,
    #[educe(Default(expression = NumInput::new(1)))]
    pub respawn_wait_input: NumInput<i64, Message>,
    #[educe(Default(expression = NumInput::new(1)))]
    pub movement_wait_input: NumInput<i64, Message>,
    #[educe(Default(expression = NumInput::new(1)))]
    pub attack_wait_input: NumInput<i64, Message>,
    #[educe(Default(expression = NumInput::new(1)))]
    pub intervaled_wait_input: NumInput<i64, Message>,
    #[educe(Default(expression = NumInput::new(1)))]
    pub spawn_wait_input: NumInput<i64, Message>,
    #[educe(Default(expression = NumInput::new(1)))]
    pub maxhp_input: NumInput<u32, Message>,
    #[educe(Default(expression = NumInput::new(0)))]
    pub maxsp_input: NumInput<u32, Message>,
    #[educe(Default(expression = NumInput::new(0)))]
    pub maxmp_input: NumInput<u32, Message>,
    #[educe(Default(expression = NumInput::new(0)))]
    pub sight_input: NumInput<i32, Message>,
    #[educe(Default(expression = NumInput::new(0)))]
    pub follow_sight_input: NumInput<i32, Message>,
    #[educe(Default(expression = NumInput::new(0)))]
    pub walkdistance_input: NumInput<u32, Message>,
    #[educe(Default(expression = NumInput::new(1)))]
    pub pdamage_input: NumInput<u32, Message>,
    #[educe(Default(expression = NumInput::new(1)))]
    #[allow(dead_code)]
    pub mdamage_input: NumInput<u32, Message>,
    #[educe(Default(expression = NumInput::new(1)))]
    pub pdef_input: NumInput<u32, Message>,
    #[educe(Default(expression = NumInput::new(1)))]
    #[allow(dead_code)]
    pub mdef_input: NumInput<u32, Message>,
    #[educe(Default(expression = NumInput::new(1)))]
    pub sizex_input: NumInput<u8, Message>,
    #[educe(Default(expression = NumInput::new(1)))]
    pub sizey_input: NumInput<u8, Message>,
    #[educe(Default(expression = NumInput::new(1)))]
    pub sizeh_input: NumInput<u8, Message>,
    #[educe(Default(expression = NumInput::new(1)))]
    pub sizew_input: NumInput<u8, Message>,
    #[educe(Default(expression = NumInput::new(1)))]
    pub maxdamage_input: NumInput<u32, Message>,
    #[educe(Default(expression = NumInput::new(1)))]
    pub mindamage_input: NumInput<u32, Message>,
    #[educe(Default(expression = NumInput::new(0)))]
    pub range_input: NumInput<i32, Message>,
    #[educe(Default(expression = NumInput::new(0)))]
    pub exp_input: NumInput<i64, Message>,
    pub behaviours: Vec<AIBehavior>,
    pub behaviour_selected: Option<AIBehavior>,
    pub item_drops: [Items; 5],
    pub itemdrop_selected: Option<usize>,
    #[educe(Default(expression = NumInput::new(0)))]
    pub slotshares_input: NumInput<u32, Message>,
    #[educe(Default(expression = NumInput::new(0)))]
    pub freeshares_input: NumInput<u32, Message>,
}

#[derive(Educe)]
#[educe(Default)]
pub struct Items {
    #[educe(Default(expression = NumInput::new(0)))]
    pub item_id: NumInput<u32, Message>,
    #[educe(Default(expression = NumInput::new(0)))]
    pub amount: NumInput<u32, Message>,
}

impl NpcUIGenerics {
    pub fn layout(&self) -> Element<Message> {
        let image_path = format!("./resources/npc/p{}.png", self.sprite_input.value);
        column![
            row![
                Rule::horizontal(0),
                text("General Information:"),
                Rule::horizontal(0),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
            row![
                text("Name:"),
                text_input("Name", &self.txt_value)
                    .on_input(Message::NameInput)
                    .width(Length::Fixed(256.0))
                    .padding(3),
                text("Sprite:"),
                self.sprite_input
                    .view(0, 0, 1000, 1, Message::GenericI32Input, None),
                text("Behaviour:"),
                PickList::new(
                    &self.behaviours[..],
                    self.behaviour_selected,
                    Message::BehaviourTypeSelect,
                )
                .width(Length::Fill),
            ]
            .spacing(15),
            row![column![
                "NPC Sprite",
                row![if Path::new(&image_path).exists() {
                    container(
                        Image::new(&image_path)
                            .width(Length::Fixed(32.0))
                            .height(Length::Fixed(32.0))
                            .content_fit(iced::ContentFit::None),
                    )
                    .width(Length::Fixed(32.0))
                    .height(Length::Fixed(32.0))
                } else {
                    Container::new("")
                        .width(Length::Fixed(32.0))
                        .height(Length::Fixed(32.0))
                }]
                .align_y(Alignment::Center)
                .spacing(6),
            ]
            .spacing(5),]
            .spacing(6),
            text("SPRITE ICON HERE"),
            row![
                column![
                    text("Level:"),
                    self.level_input
                        .view(1, 1, 200, 1, Message::GenericI32Input, None),
                    text("Max HP:"),
                    self.maxhp_input
                        .view(1, 1, u32::MAX, 1, Message::GenericU32Input, None),
                    text("Max Sp:"),
                    self.maxsp_input
                        .view(2, 1, u32::MAX, 1, Message::GenericU32Input, None),
                    text("Max Mp:"),
                    self.maxmp_input
                        .view(3, 1, u32::MAX, 1, Message::GenericU32Input, None),
                ]
                .spacing(6)
                .align_x(Alignment::Center),
                column![
                    text("Pysical Damage:"),
                    self.pdamage_input
                        .view(4, 1, u32::MAX, 1, Message::GenericU32Input, None),
                    text("Pysical Defense:"),
                    self.pdef_input
                        .view(5, 1, u32::MAX, 1, Message::GenericU32Input, None),
                    text("Sight Range:"),
                    self.sight_input
                        .view(2, 1, i32::MAX, 1, Message::GenericI32Input, None),
                    text("Follow Sight Range:"),
                    self.follow_sight_input
                        .view(3, 0, i32::MAX, 1, Message::GenericI32Input, None),
                ]
                .spacing(6)
                .align_x(Alignment::Center),
                column![
                    text("Movement Wait (ms):"),
                    self.movement_wait_input.view(
                        1,
                        1,
                        i64::MAX,
                        1,
                        Message::GenericI64Input,
                        None
                    ),
                    text("Attack Wait (ms):"),
                    self.attack_wait_input
                        .view(2, 1, i64::MAX, 1, Message::GenericI64Input, None),
                    text("Intervaled Wait (ms):"),
                    self.intervaled_wait_input.view(
                        3,
                        1,
                        i64::MAX,
                        1,
                        Message::GenericI64Input,
                        None
                    ),
                    text("Walk Dist. from spawn:"),
                    self.walkdistance_input
                        .view(6, 0, u32::MAX, 1, Message::GenericU32Input, None),
                ]
                .spacing(6)
                .align_x(Alignment::Center),
                column![
                    text("Min Damage (Avoids Defense):"),
                    self.mindamage_input
                        .view(7, 1, u32::MAX, 1, Message::GenericU32Input, None),
                    text("Max Damage (regardless of power):"),
                    self.maxdamage_input
                        .view(8, 1, u32::MAX, 1, Message::GenericU32Input, None),
                    text("Respawn Wait (ms):"),
                    self.respawn_wait_input
                        .view(4, 1, i64::MAX, 1, Message::GenericI64Input, None),
                    text("Spawn Wait (ms):"),
                    self.spawn_wait_input
                        .view(5, 1, i64::MAX, 1, Message::GenericI64Input, None),
                ]
                .spacing(6)
                .align_x(Alignment::Center),
                column![
                    text("Range:"),
                    self.range_input
                        .view(4, 0, i32::MAX, 1, Message::GenericI32Input, None),
                    text("Exp:"),
                    self.exp_input
                        .view(6, 0, i64::MAX, 1, Message::GenericI64Input, None),
                ]
                .spacing(6)
                .align_x(Alignment::Center),
            ]
            .spacing(15),
            row![
                Rule::horizontal(0),
                text("Block Size:"),
                Rule::horizontal(0),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
            row![
                text("X:"),
                self.sizex_input
                    .view(0, 1, u8::MAX, 1, Message::GenericU8Input, None),
                text("Y:"),
                self.sizey_input
                    .view(1, 1, u8::MAX, 1, Message::GenericU8Input, None),
                text("H:"),
                self.sizeh_input
                    .view(2, 1, u8::MAX, 1, Message::GenericU8Input, None),
                text("W:"),
                self.sizew_input
                    .view(3, 1, u8::MAX, 1, Message::GenericU8Input, None),
            ]
            .spacing(15),
            row![
                Rule::horizontal(0),
                text("Item Drops:"),
                Rule::horizontal(0),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
            row![
                column![
                    text("Drop Item Slot:"),
                    PickList::new(
                        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
                        self.itemdrop_selected,
                        Message::ItemDropSlotSelect,
                    ),
                    text("Free Shares:"),
                    self.freeshares_input
                        .view(20, 1, u32::MAX, 1, Message::GenericU32Input, None),
                ]
                .spacing(10),
                column![
                    row![
                        text("Shares:"),
                        self.slotshares_input.view(
                            19,
                            1,
                            u32::MAX,
                            1,
                            Message::GenericU32Input,
                            None
                        ),
                    ]
                    .spacing(10),
                    row![
                        column![
                            text("Item ID 1:"),
                            self.item_drops[0].item_id.view(
                                9,
                                1,
                                u32::MAX,
                                1,
                                Message::GenericU32Input,
                                None
                            ),
                            text("Amount:"),
                            self.item_drops[0].amount.view(
                                14,
                                1,
                                u32::MAX,
                                1,
                                Message::GenericU32Input,
                                None
                            ),
                        ],
                        column![
                            text("Item ID 2:"),
                            self.item_drops[1].item_id.view(
                                10,
                                1,
                                u32::MAX,
                                1,
                                Message::GenericU32Input,
                                None
                            ),
                            text("Amount:"),
                            self.item_drops[1].amount.view(
                                15,
                                1,
                                u32::MAX,
                                1,
                                Message::GenericU32Input,
                                None
                            ),
                        ],
                        column![
                            text("Item ID 3:"),
                            self.item_drops[2].item_id.view(
                                11,
                                1,
                                u32::MAX,
                                1,
                                Message::GenericU32Input,
                                None
                            ),
                            text("Amount:"),
                            self.item_drops[2].amount.view(
                                16,
                                1,
                                u32::MAX,
                                1,
                                Message::GenericU32Input,
                                None
                            ),
                        ],
                        column![
                            text("Item ID 4:"),
                            self.item_drops[3].item_id.view(
                                12,
                                1,
                                u32::MAX,
                                1,
                                Message::GenericU32Input,
                                None
                            ),
                            text("Amount:"),
                            self.item_drops[3].amount.view(
                                17,
                                1,
                                u32::MAX,
                                1,
                                Message::GenericU32Input,
                                None
                            ),
                        ],
                        column![
                            text("Item ID 5:"),
                            self.item_drops[4].item_id.view(
                                13,
                                1,
                                u32::MAX,
                                1,
                                Message::GenericU32Input,
                                None
                            ),
                            text("Amount:"),
                            self.item_drops[4].amount.view(
                                18,
                                1,
                                u32::MAX,
                                1,
                                Message::GenericU32Input,
                                None
                            ),
                        ],
                    ]
                    .spacing(5),
                ]
                .spacing(5),
            ]
            .spacing(25)
        ]
        .spacing(10)
        .align_x(Alignment::Center)
        .into()
    }
}
