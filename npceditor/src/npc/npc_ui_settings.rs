use crate::npc::*;
use araiseal_styles::{CheckBoxStyle, TEXT_WHITE};
use araiseal_ui::*;
use iced::pure::{
    widget::{Button, Checkbox, Column, Row, Rule, Text},
    Element,
};
use iced::{
    alignment::{Alignment, Horizontal, Vertical},
    Length,
};
use iced_aw::pure::time_picker::{Time, TimePicker};

#[derive(Educe)]
#[educe(Default)]
pub struct NpcUISettings {
    pub target_auto_switch: bool,
    pub target_attacked_switch: bool,
    pub target_range_dropout: bool,
    pub can_target: bool,
    pub can_move: bool,
    pub can_attack_player: bool,
    pub has_selfonly: bool,
    pub has_friendonly: bool,
    pub has_groundonly: bool,
    pub has_enemyonly: bool,
    pub has_enemies: bool,
    pub has_allys: bool,
    pub can_attack: bool,
    pub runsaway: bool,
    pub canpassthru: bool,
    pub isanimated: bool,
    #[educe(Default(expression = "[Time::now_hms(false); 2]"))]
    pub times: [Time; 2],
    pub show_time: [bool; 2],
    #[educe(Default(expression = "NumInput::new(0)"))]
    pub target_switch_chance: NumInput<i64, Message>,
    #[educe(Default(expression = "NumInput::new(0)"))]
    pub run_damage: NumInput<u32, Message>,
}

impl NpcUISettings {
    pub fn layout(&self, npc: &NpcData) -> Element<Message> {
        Column::new()
            .spacing(6)
            .align_items(Alignment::Center)
            .push(
                Text::new("Npc Settings")
                    .width(Length::Fill)
                    .vertical_alignment(Vertical::Bottom)
                    .horizontal_alignment(Horizontal::Center)
                    .color(TEXT_WHITE),
            )
            .push(Rule::horizontal(0))
            .push(
                Row::new()
                    .spacing(12)
                    .push(
                        Element::new(
                            Checkbox::new(
                                self.target_auto_switch,
                                String::from("Targeting Auto switch"),
                                CheckBoxMessage::Change,
                            )
                            .style(CheckBoxStyle),
                        )
                        .map(move |i| Message::GenericBoolInput((14, i))),
                    )
                    .push(
                        Element::new(
                            Checkbox::new(
                                self.target_attacked_switch,
                                String::from("Targets New Attacker"),
                                CheckBoxMessage::Change,
                            )
                            .style(CheckBoxStyle),
                        )
                        .map(move |i| Message::GenericBoolInput((15, i))),
                    ),
            )
            .push(
                Row::new()
                    .spacing(12)
                    .push(
                        Element::new(
                            Checkbox::new(
                                self.target_range_dropout,
                                String::from("Drop target when out of Range"),
                                CheckBoxMessage::Change,
                            )
                            .style(CheckBoxStyle),
                        )
                        .map(move |i| Message::GenericBoolInput((16, i))),
                    )
                    .push(
                        Element::new(
                            Checkbox::new(
                                self.can_target,
                                String::from("Enable Targeting"),
                                CheckBoxMessage::Change,
                            )
                            .style(CheckBoxStyle),
                        )
                        .map(move |i| Message::GenericBoolInput((17, i))),
                    ),
            )
            .push(
                Row::new()
                    .spacing(12)
                    .push(
                        Element::new(
                            Checkbox::new(
                                self.can_move,
                                String::from("Enable Npc Movement"),
                                CheckBoxMessage::Change,
                            )
                            .style(CheckBoxStyle),
                        )
                        .map(move |i| Message::GenericBoolInput((18, i))),
                    )
                    .push(
                        Element::new(
                            Checkbox::new(
                                self.can_attack_player,
                                String::from("Allow Attacking Players"),
                                CheckBoxMessage::Change,
                            )
                            .style(CheckBoxStyle),
                        )
                        .map(move |i| Message::GenericBoolInput((19, i))),
                    ),
            )
            .push(
                Row::new()
                    .spacing(12)
                    .push(
                        Element::new(
                            Checkbox::new(
                                self.has_selfonly,
                                String::from("Enable Self cast Skills"),
                                CheckBoxMessage::Change,
                            )
                            .style(CheckBoxStyle),
                        )
                        .map(move |i| Message::GenericBoolInput((20, i))),
                    )
                    .push(
                        Element::new(
                            Checkbox::new(
                                self.has_friendonly,
                                String::from("Enable Ally cast Skills"),
                                CheckBoxMessage::Change,
                            )
                            .style(CheckBoxStyle),
                        )
                        .map(move |i| Message::GenericBoolInput((21, i))),
                    ),
            )
            .push(
                Row::new()
                    .spacing(12)
                    .push(
                        Element::new(
                            Checkbox::new(
                                self.has_groundonly,
                                String::from("Enable Ground cast Skills"),
                                CheckBoxMessage::Change,
                            )
                            .style(CheckBoxStyle),
                        )
                        .map(move |i| Message::GenericBoolInput((22, i))),
                    )
                    .push(
                        Element::new(
                            Checkbox::new(
                                self.has_enemyonly,
                                String::from("Enable Enemy cast Skills"),
                                CheckBoxMessage::Change,
                            )
                            .style(CheckBoxStyle),
                        )
                        .map(move |i| Message::GenericBoolInput((23, i))),
                    ),
            )
            .push(
                Row::new()
                    .spacing(12)
                    .push(
                        Element::new(
                            Checkbox::new(
                                self.has_enemies,
                                String::from("Checks for Enemy Npc's during Targeting"),
                                CheckBoxMessage::Change,
                            )
                            .style(CheckBoxStyle),
                        )
                        .map(move |i| Message::GenericBoolInput((24, i))),
                    )
                    .push(
                        Element::new(
                            Checkbox::new(
                                self.has_allys,
                                String::from("Checks for Ally Npc's during Targeting"),
                                CheckBoxMessage::Change,
                            )
                            .style(CheckBoxStyle),
                        )
                        .map(move |i| Message::GenericBoolInput((25, i))),
                    ),
            )
            .push(
                Row::new()
                    .spacing(12)
                    .push(
                        Element::new(
                            Checkbox::new(
                                self.can_attack,
                                String::from("Enable Skill Casting"),
                                CheckBoxMessage::Change,
                            )
                            .style(CheckBoxStyle),
                        )
                        .map(move |i| Message::GenericBoolInput((26, i))),
                    )
                    .push(
                        Element::new(
                            Checkbox::new(
                                self.runsaway,
                                String::from("Will Run when HP is low."),
                                CheckBoxMessage::Change,
                            )
                            .style(CheckBoxStyle),
                        )
                        .map(move |i| Message::GenericBoolInput((27, i))),
                    ),
            )
            .push(
                Row::new()
                    .spacing(12)
                    .push(
                        Element::new(
                            Checkbox::new(
                                self.canpassthru,
                                String::from("Can walk thru."),
                                CheckBoxMessage::Change,
                            )
                            .style(CheckBoxStyle),
                        )
                        .map(move |i| Message::GenericBoolInput((28, i))),
                    )
                    .push(
                        Element::new(
                            Checkbox::new(
                                self.isanimated,
                                String::from("Always Animated movement."),
                                CheckBoxMessage::Change,
                            )
                            .style(CheckBoxStyle),
                        )
                        .map(move |i| Message::GenericBoolInput((29, i))),
                    ),
            )
            .push(
                Row::new()
                    .spacing(6)
                    .push(
                        TimePicker::new(
                            self.show_time[0],
                            self.times[0],
                            Button::new(Text::new("Set Start Time")).on_press(Message::ChooseTime1),
                            Message::CancelTime,
                            Message::SubmitTime1,
                        )
                        .use_24h(),
                    )
                    .push(Text::new(format!("Start Time: {:?}", npc.spawntime.0))),
            )
            .push(
                Row::new()
                    .spacing(6)
                    .push(
                        TimePicker::new(
                            self.show_time[1],
                            self.times[1],
                            Button::new(Text::new("Set End Time")).on_press(Message::ChooseTime2),
                            Message::CancelTime,
                            Message::SubmitTime2,
                        )
                        .use_24h(),
                    )
                    .push(Text::new(format!("End Time: {:?}", npc.spawntime.1))),
            )
            .push(
                Row::new()
                    .spacing(12)
                    .push(Text::new("Target Switch Chance out of 100,000,000: ").color(TEXT_WHITE))
                    .push(self.target_switch_chance.view(
                        0,
                        1,
                        100_000_000,
                        1,
                        Message::GenericI64Input,
                    ))
                    .push(Text::new("Run Min HP Needed: ").color(TEXT_WHITE))
                    .push(
                        self.run_damage
                            .view(2, 1, u32::MAX, 1, Message::GenericU32Input),
                    ),
            )
            .into()
    }
}
