use crate::npc::*;
use araiseal_ui::*;
use araiseal_types::*;
use iced::{
    alignment::{Alignment, Horizontal},
    widget::{column, row, text, checkbox, Rule},
    Element,
};
use iced_aw::{
    TimePicker,
    time_picker::Time
};

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
    pub has_allys: bool,
    pub can_attack: bool,
    pub runsaway: bool,
    pub canpassthru: bool,
    pub isanimated: bool,
    #[educe(Default(expression = "[Time::now_hms(false); 2]"))]
    pub times: [Time; 2],
    pub show_time: [bool; 2],
    pub spawntime_data: (GameTime, GameTime),
    #[educe(Default(expression = "NumInput::new(0)"))]
    pub target_switch_chance: NumInput<i64, Message>,
    #[educe(Default(expression = "NumInput::new(0)"))]
    pub run_damage: NumInput<u32, Message>,
}

impl NpcUISettings {
    pub fn layout(&self, _npc: &NpcData) -> Element<Message> {
        column![
            row![
                Rule::horizontal(0),
                text("NPC Settings:").horizontal_alignment(Horizontal::Center),
                Rule::horizontal(0),
            ].spacing(10).align_items(Alignment::Center),
            row![
                column![
                    Element::new(
                        checkbox("Target Auto switch", self.target_auto_switch)
                    ).map(move |i| Message::GenericBoolInput((0, i))),
                    Element::new(
                        checkbox("Target Attacked Switch", self.target_attacked_switch)
                    ).map(move |i| Message::GenericBoolInput((1, i))),
                    Element::new(
                        checkbox("Target Range Dropout", self.target_range_dropout)
                    ).map(move |i| Message::GenericBoolInput((2, i))),
                    Element::new(
                        checkbox("Can Target", self.can_target)
                    ).map(move |i| Message::GenericBoolInput((3, i))),
                    Element::new(
                        checkbox("Can Move", self.can_move)
                    ).map(move |i| Message::GenericBoolInput((4, i))),
                    Element::new(
                        checkbox("Can Attack Player", self.can_attack_player)
                    ).map(move |i| Message::GenericBoolInput((5, i))),
                    Element::new(
                        checkbox("Has Self Only", self.has_selfonly)
                    ).map(move |i| Message::GenericBoolInput((6, i))),
                ].spacing(5),
                column![
                    Element::new(
                        checkbox("Has Friendly Only", self.has_friendonly)
                    ).map(move |i| Message::GenericBoolInput((7, i))),
                    Element::new(
                        checkbox("Has Ground Only", self.has_groundonly)
                    ).map(move |i| Message::GenericBoolInput((8, i))),
                    Element::new(
                        checkbox("Has Ally", self.has_allys)
                    ).map(move |i| Message::GenericBoolInput((9, i))),
                    Element::new(
                        checkbox("Can Attack", self.can_attack)
                    ).map(move |i| Message::GenericBoolInput((10, i))),
                    Element::new(
                        checkbox("Runs Away", self.runsaway)
                    ).map(move |i| Message::GenericBoolInput((11, i))),
                    Element::new(
                        checkbox("Can Pass Through", self.canpassthru)
                    ).map(move |i| Message::GenericBoolInput((12, i))),
                    Element::new(
                        checkbox("Is Animated", self.isanimated)
                    ).map(move |i| Message::GenericBoolInput((13, i))),
                ].spacing(5),
                column![
                    row![
                        self.target_switch_chance.view(0, 1, 100_000_000, 1, Message::GenericI64Input),
                        text("Target Switch Chance out of 100,000,000: "),
                    ].spacing(6),
                    row![
                        self.run_damage.view(0, 1, u32::MAX, 1, Message::GenericU32Input),
                        text("Run Min HP Needed: "),
                    ].spacing(6),
                    row![
                        TimePicker::new(
                            self.show_time[0],
                            self.times[0],
                            button("Set Start Time").on_press(Message::ChooseTime1),
                            Message::CancelTime,
                            Message::SubmitTime1,
                        ).use_24h(),
                        text(format!("Start Time: {:?}", self.spawntime_data.0)),
                    ].spacing(6).align_items(Alignment::Center),
                    row![
                        TimePicker::new(
                            self.show_time[1],
                            self.times[1],
                            button("Set End Time").on_press(Message::ChooseTime1),
                            Message::CancelTime,
                            Message::SubmitTime1,
                        ).use_24h(),
                        text(format!("End Time: {:?}", self.spawntime_data.1)),
                    ].spacing(6).align_items(Alignment::Center),
                ].spacing(6),
            ].spacing(6),
            
        ]
        .spacing(12)
        .align_items(Alignment::Start)
        .into()
    }
}
