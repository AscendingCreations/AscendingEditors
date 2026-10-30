use gpui_kit::SharedString;

use crate::AIBehavior;

impl AIBehavior {
    pub fn convert_to_string(index: usize) -> SharedString {
        SharedString::new(match index {
            1 => "Agressive",
            2 => "Reactive",
            3 => "HelpReactive",
            4 => "Healer",
            5 => "AgressiveHealer",
            6 => "ReactiveHealer",
            _ => "Friendly",
        })
    }

    pub fn get_list() -> Vec<SharedString> {
        let mut list = Vec::with_capacity((Self::ReactiveHealer as usize) + 1);
        for i in 0..=Self::ReactiveHealer as usize {
            list.push(Self::convert_to_string(i));
        }
        list
    }

    pub fn from_index(index: usize) -> Self {
        match index {
            1 => AIBehavior::Agressive,
            2 => AIBehavior::Reactive,
            3 => AIBehavior::HelpReactive,
            4 => AIBehavior::Healer,
            5 => AIBehavior::AgressiveHealer,
            6 => AIBehavior::ReactiveHealer,
            _ => AIBehavior::Friendly,
        }
    }
}
