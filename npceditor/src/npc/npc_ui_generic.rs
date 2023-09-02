use crate::npc::*;
use araiseal_types::*;
use araiseal_ui::*;
use iced::{
    alignment::{Alignment, Horizontal},
    widget::{
        column, row, text, text_input, PickList, Rule
    },
    Element, Length,
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
    pub item_drops: Items,
    pub itemdrop_selected: Option<usize>,
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
        column![
            row![
                Rule::horizontal(0),
                text("General Information:").horizontal_alignment(Horizontal::Center),
                Rule::horizontal(0),
            ].spacing(10).align_items(Alignment::Center),
            row![
                text("Name:"),
                text_input("Name", &self.txt_value).on_input(Message::NameInput).width(Length::Fixed(256.0)).padding(3),
                text("Sprite:"),
                self.sprite_input.view(0, 0, 1000, 1, Message::GenericI32Input),
                text("Behaviour:"),
                PickList::new(
                    &self.behaviours[..],
                    self.behaviour_selected,
                    Message::BehaviourTypeSelect,
                )
                .width(Length::Fill),
            ].spacing(15),
            text("SPRITE ICON HERE"),
            row![
                column![
                    text("Level:"),
                    self.level_input.view(1, 1, 200, 1, Message::GenericI32Input),
                    text("Max HP:"),
                    self.maxhp_input.view(1, 1, u32::MAX, 1, Message::GenericU32Input),
                    text("Max Sp:"),
                    self.maxsp_input.view(2, 1, u32::MAX, 1, Message::GenericU32Input),
                    text("Max Mp:"),
                    self.maxmp_input.view(3, 1, u32::MAX, 1, Message::GenericU32Input),
                ].spacing(6).align_items(Alignment::Center),
                column![
                    text("Pysical Damage:"),
                    self.pdamage_input.view(4, 1, u32::MAX, 1, Message::GenericU32Input),
                    text("Pysical Defense:"),
                    self.pdef_input.view(5, 1, u32::MAX, 1, Message::GenericU32Input),
                    text("Sight Range:"),
                    self.sight_input.view(2, 1, i32::MAX, 1, Message::GenericI32Input),
                    text("Follow Sight Range:"),
                    self.follow_sight_input.view(3, 0, i32::MAX, 1, Message::GenericI32Input),
                ].spacing(6).align_items(Alignment::Center),
                column![
                    text("Movement Wait (ms):"),
                    self.movement_wait_input.view(1, 1, i64::MAX, 1, Message::GenericI64Input),
                    text("Attack Wait (ms):"),
                    self.attack_wait_input.view(2, 1, i64::MAX, 1, Message::GenericI64Input),
                    text("Intervaled Wait (ms):"),
                    self.intervaled_wait_input.view(3, 1, i64::MAX, 1, Message::GenericI64Input),
                    text("Walk Dist. from spawn:"),
                    self.walkdistance_input.view(6, 0, u32::MAX, 1, Message::GenericU32Input),
                ].spacing(6).align_items(Alignment::Center),
                column![
                    text("Min Damage (Avoids Defense):"),
                    self.mindamage_input.view(7, 1, u32::MAX, 1, Message::GenericU32Input),
                    text("Max Damage (regardless of power):"),
                    self.maxdamage_input.view(8, 1, u32::MAX, 1, Message::GenericU32Input),
                    text("Respawn Wait (ms):"),
                    self.respawn_wait_input.view(4, 1, i64::MAX, 1, Message::GenericI64Input),
                    text("Spawn Wait (ms):"),
                    self.spawn_wait_input.view(5, 1, i64::MAX, 1, Message::GenericI64Input),
                ].spacing(6),
            ].spacing(15),
            row![
                Rule::horizontal(0),
                text("Block Size:").horizontal_alignment(Horizontal::Center),
                Rule::horizontal(0),
            ].spacing(10).align_items(Alignment::Center),
            row![
                text("X:"),
                self.sizex_input.view(0, 1, u8::MAX, 1, Message::GenericU8Input),
                text("Y:"),
                self.sizey_input.view(1, 1, u8::MAX, 1, Message::GenericU8Input),
                text("H:"),
                self.sizeh_input.view(2, 1, u8::MAX, 1, Message::GenericU8Input),
                text("W:"),
                self.sizew_input.view(3, 1, u8::MAX, 1, Message::GenericU8Input),
            ].spacing(15),
            row![
                Rule::horizontal(0),
                text("Item Drops:").horizontal_alignment(Horizontal::Center),
                Rule::horizontal(0),
            ].spacing(10).align_items(Alignment::Center),
            row![
                text("Drop Item Slot:"),
                PickList::new(
                    vec![0,1,2,3,4,5,6,7,8,9],
                    self.itemdrop_selected,
                    Message::ItemDropSlotSelect,
                ),
                text("Item ID:"),
                self.item_drops.item_id.view(9, 1, u32::MAX, 1, Message::GenericU32Input),
                text("Item Amount:"),
                self.item_drops.chance.view(10, 1, u32::MAX, 1, Message::GenericU32Input),
                text("Item Chance:"),
                self.item_drops.amount.view(11, 1, u32::MAX, 1, Message::GenericU32Input),
            ].spacing(6)
        ]
        .spacing(10)
        .align_items(Alignment::Center)
        .into()
    }
}
