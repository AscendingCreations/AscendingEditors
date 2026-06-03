use crate::npc::*;
use ascending_types::*;
use ascending_ui::*;
use iced::{
    alignment::Alignment,
    widget::{button, checkbox, column, row, rule, text},
    Element,
};
use iced_aw::{time_picker::Time, TimePicker};

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
    pub has_enemies: bool,
    pub can_attack: bool,
    pub runsaway: bool,
    pub canpassthru: bool,
    pub isanimated: bool,
    #[educe(Default(expression = [Time::now_hms(false); 2]))]
    pub times: [Time; 2],
    pub show_time: [bool; 2],
    pub spawntime_data: (GameTime, GameTime),
    #[educe(Default(expression = NumInput::new(0)))]
    pub target_switch_chance: NumInput<i64, Message>,
    #[educe(Default(expression = NumInput::new(0)))]
    pub run_damage: NumInput<u32, Message>,
}

impl NpcUISettings {
    pub fn layout(&'_ self, _npc: &NpcData) -> Element<'_, Message> {
        column![
            row![
                rule::horizontal(1),
                text("NPC Settings:"),
                rule::horizontal(1),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
            row![
                column![
                    checkbox(self.target_auto_switch)
                        .label("Target Auto switch")
                        .on_toggle(move |i| {
                            Message::GenericBoolInput((0, CheckBoxMessage::Change(i)))
                        }),
                    checkbox(self.target_attacked_switch)
                        .label("Target Attacked Switch")
                        .on_toggle(move |i| Message::GenericBoolInput((
                            1,
                            CheckBoxMessage::Change(i)
                        ))),
                    checkbox(self.target_range_dropout)
                        .label("Target Range Dropout")
                        .on_toggle(move |i| Message::GenericBoolInput((
                            2,
                            CheckBoxMessage::Change(i)
                        ))),
                    checkbox(self.can_target)
                        .label("Can Target")
                        .on_toggle(move |i| {
                            Message::GenericBoolInput((3, CheckBoxMessage::Change(i)))
                        }),
                    checkbox(self.can_move)
                        .label("Can Move")
                        .on_toggle(move |i| {
                            Message::GenericBoolInput((4, CheckBoxMessage::Change(i)))
                        }),
                    checkbox(self.can_attack_player)
                        .label("Can Attack Player")
                        .on_toggle(move |i| {
                            Message::GenericBoolInput((5, CheckBoxMessage::Change(i)))
                        }),
                    checkbox(self.has_selfonly)
                        .label("Has Self Only")
                        .on_toggle(move |i| {
                            Message::GenericBoolInput((6, CheckBoxMessage::Change(i)))
                        }),
                ]
                .spacing(5),
                column![
                    checkbox(self.has_friendonly)
                        .label("Has Friendly Only")
                        .on_toggle(move |i| {
                            Message::GenericBoolInput((7, CheckBoxMessage::Change(i)))
                        }),
                    checkbox(self.has_groundonly)
                        .label("Has Ground Only")
                        .on_toggle(move |i| {
                            Message::GenericBoolInput((8, CheckBoxMessage::Change(i)))
                        }),
                    checkbox(self.has_allys)
                        .label("Has Ally")
                        .on_toggle(move |i| {
                            Message::GenericBoolInput((9, CheckBoxMessage::Change(i)))
                        }),
                    checkbox(self.can_attack)
                        .label("Can Attack")
                        .on_toggle(move |i| {
                            Message::GenericBoolInput((10, CheckBoxMessage::Change(i)))
                        }),
                    checkbox(self.runsaway)
                        .label("Runs Away")
                        .on_toggle(move |i| {
                            Message::GenericBoolInput((11, CheckBoxMessage::Change(i)))
                        }),
                    checkbox(self.canpassthru)
                        .label("Can Pass Through")
                        .on_toggle(move |i| {
                            Message::GenericBoolInput((12, CheckBoxMessage::Change(i)))
                        }),
                    checkbox(self.isanimated)
                        .label("Is Animated")
                        .on_toggle(move |i| {
                            Message::GenericBoolInput((13, CheckBoxMessage::Change(i)))
                        }),
                    checkbox(self.has_enemies)
                        .label("Has Enemies")
                        .on_toggle(move |i| {
                            Message::GenericBoolInput((14, CheckBoxMessage::Change(i)))
                        }),
                ]
                .spacing(5),
                column![
                    row![
                        self.target_switch_chance.view(
                            0,
                            1,
                            100_000_000,
                            1,
                            Message::GenericI64Input,
                            None
                        ),
                        text("Target Switch Chance out of 100,000,000: "),
                    ]
                    .spacing(6),
                    row![
                        self.run_damage
                            .view(0, 1, u32::MAX, 1, Message::GenericU32Input, None),
                        text("Run Min HP Needed: "),
                    ]
                    .spacing(6),
                    row![
                        TimePicker::new(
                            self.show_time[0],
                            self.times[0],
                            button("Set Start Time").on_press(Message::ChooseTime1),
                            Message::CancelTime,
                            Message::SubmitTime1,
                        )
                        .use_24h(),
                        text(format!("Start Time: {:?}", self.spawntime_data.0)),
                    ]
                    .spacing(6)
                    .align_y(Alignment::Center),
                    row![
                        TimePicker::new(
                            self.show_time[1],
                            self.times[1],
                            button("Set End Time").on_press(Message::ChooseTime2),
                            Message::CancelTime,
                            Message::SubmitTime2,
                        )
                        .use_24h(),
                        text(format!("End Time: {:?}", self.spawntime_data.1)),
                    ]
                    .spacing(6)
                    .align_y(Alignment::Center),
                ]
                .spacing(6),
            ]
            .spacing(6),
        ]
        .spacing(12)
        .align_x(Alignment::Start)
        .into()
    }
}
