use gpui_kit::SharedString;

use crate::ItemTypes;

impl ItemTypes {
    pub fn convert_to_string(index: usize) -> SharedString {
        SharedString::new(match index {
            1 => "Weapon",
            2 => "Accessory",
            3 => "Cosmetic",
            4 => "Helmet",
            5 => "Armor",
            6 => "Trouser",
            7 => "Boots",
            8 => "Consume",
            9 => "Tool",
            10 => "Blueprint",
            11 => "Book",
            12 => "Questitem",
            13 => "Trap",
            14 => "Heavyobject",
            15 => "Key",
            _ => "None",
        })
    }

    pub fn get_list() -> Vec<SharedString> {
        let mut list = Vec::with_capacity(Self::Count as usize);
        for i in 0..Self::Count as usize {
            list.push(Self::convert_to_string(i));
        }
        list
    }

    pub fn from_index(index: usize) -> Self {
        match index {
            1 => ItemTypes::Weapon,
            2 => ItemTypes::Accessory,
            3 => ItemTypes::Cosmetic,
            4 => ItemTypes::Helmet,
            5 => ItemTypes::Armor,
            6 => ItemTypes::Trouser,
            7 => ItemTypes::Boots,
            8 => ItemTypes::Consume,
            9 => ItemTypes::Tool,
            10 => ItemTypes::Blueprint,
            11 => ItemTypes::Book,
            12 => ItemTypes::Questitem,
            13 => ItemTypes::Trap,
            14 => ItemTypes::Heavyobject,
            15 => ItemTypes::Key,
            _ => ItemTypes::None,
        }
    }
}

pub fn data_labels(id: usize, item_type: ItemTypes) -> SharedString {
    SharedString::new(match item_type {
        ItemTypes::Weapon => match id {
            0 => "Melee Damage",
            1 => "Magic Damage",
            2 => "Durability",
            _ => "None",
        },
        ItemTypes::Accessory => match id {
            0 => "Melee Defense",
            1 => "Magic Defense",
            2 => "Durability",
            _ => "None",
        },
        ItemTypes::Cosmetic => "None",
        ItemTypes::Helmet => match id {
            0 => "Melee Defense",
            1 => "Magic Defense",
            2 => "Durability",
            _ => "None",
        },
        ItemTypes::Armor => match id {
            0 => "Melee Defense",
            1 => "Magic Defense",
            2 => "Durability",
            _ => "None",
        },
        ItemTypes::Trouser => match id {
            0 => "Melee Defense",
            1 => "Magic Defense",
            2 => "Durability",
            _ => "None",
        },
        ItemTypes::Boots => match id {
            0 => "Melee Defense",
            1 => "Magic Defense",
            2 => "Durability",
            _ => "None",
        },
        ItemTypes::Consume => match id {
            0 => "Heal HP",
            1 => "Heal MP",
            2 => "Heal SP",
            _ => "None",
        },
        ItemTypes::Tool => match id {
            0 => "Melee Damage",
            1 => "Magic Damage",
            2 => "Durability",
            _ => "None",
        },
        ItemTypes::Blueprint => "None",
        ItemTypes::Book => match id {
            0 => "Book ID",
            _ => "None",
        },
        ItemTypes::Questitem => match id {
            0 => "Special ID",
            1 => "Reusable 0/1",
            _ => "None",
        },
        ItemTypes::Trap => match id {
            0 => "Melee Damage",
            1 => "Magic Damage",
            _ => "None",
        },
        ItemTypes::Heavyobject => "None",
        ItemTypes::Key => match id {
            0 => "Key ID",
            1 => "Reusable 0/1",
            _ => "None",
        },
        _ => "None",
    })
}
