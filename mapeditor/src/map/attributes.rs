use bytey::{ByteBufferRead, ByteBufferWrite};
use graphics::*;
use serde::{Deserialize, Serialize};
use speedy::{Endianness, Readable, Writable};

pub const MAX_ATTRIBUTE: usize = 8;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default, Readable, Writable)]
pub struct WarpData {
    pub map_x: i32,
    pub map_y: i32,
    pub map_group: u64,
    pub tile_x: u32,
    pub tile_y: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default, Readable, Writable)]
pub struct ItemSpawnData {
    pub index: u32,
    pub amount: u16,
    pub timer: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Readable, Writable)]
pub enum MapAttribute {
    Walkable,
    Blocked,
    NpcBlocked,
    Warp(WarpData),
    Sign(String),
    ItemSpawn(ItemSpawnData),
    Storage,
    Shop(u16),
    Count,
}

#[derive(Debug)]
pub enum InsertTypes {
    Int(i64),
    UInt(u64),
    Str(String),
    Bool(bool),
}

impl InsertTypes {
    pub fn get_int(&self) -> i64 {
        match self {
            InsertTypes::Int(data) => *data,
            _ => 0,
        }
    }

    pub fn get_uint(&self) -> u64 {
        match self {
            InsertTypes::UInt(data) => *data,
            _ => 0,
        }
    }

    pub fn get_string(&self) -> String {
        match self {
            InsertTypes::Str(data) => data.clone(),
            _ => String::new(),
        }
    }
}

impl MapAttribute {
    pub fn as_str<'a>(attribute: u32) -> &'a str {
        match attribute {
            0 => "Walkable",
            1 => "Blocked",
            2 => "NpcBlocked",
            3 => "Warp",
            4 => "Sign",
            5 => "Item",
            6 => "Storage",
            7 => "Shop",
            _ => "",
        }
    }

    pub fn as_map_str<'a>(attribute: &MapAttribute) -> &'a str {
        match attribute {
            MapAttribute::Blocked => "B",
            MapAttribute::NpcBlocked => "N",
            MapAttribute::Warp(_) => "W",
            MapAttribute::Sign(_) => "S",
            MapAttribute::ItemSpawn(_) => "I",
            MapAttribute::Storage => "S",
            MapAttribute::Shop(_) => "S",
            _ => "",
        }
    }

    pub fn get_color(attribute: &MapAttribute) -> Color {
        match attribute {
            MapAttribute::Blocked => Color::rgba(200, 10, 10, 100),
            MapAttribute::NpcBlocked => Color::rgba(200, 50, 10, 100),
            MapAttribute::Warp(_) => Color::rgba(10, 10, 200, 100),
            MapAttribute::Sign(_) => Color::rgba(10, 200, 10, 100),
            MapAttribute::ItemSpawn(_) => Color::rgba(180, 180, 180, 100),
            MapAttribute::Storage => Color::rgba(160, 170, 20, 255),
            MapAttribute::Shop(_) => Color::rgba(200, 50, 100, 255),
            _ => Color::rgba(0, 0, 0, 0),
        }
    }

    pub fn convert_to_enum(attribute: u32, data: &[InsertTypes]) -> Self {
        match attribute {
            1 => MapAttribute::Blocked,
            2 => MapAttribute::NpcBlocked,
            3 => MapAttribute::Warp(WarpData {
                map_x: data[0].get_int() as i32,
                map_y: data[1].get_int() as i32,
                map_group: data[2].get_uint(),
                tile_x: data[3].get_uint() as u32,
                tile_y: data[4].get_uint() as u32,
            }),
            4 => MapAttribute::Sign(data[0].get_string()),
            5 => MapAttribute::ItemSpawn(ItemSpawnData {
                index: data[0].get_uint() as u32,
                amount: data[1].get_uint() as u16,
                timer: data[1].get_uint(),
            }),
            6 => MapAttribute::Storage,
            7 => MapAttribute::Shop(data[0].get_uint() as u16),
            _ => MapAttribute::Walkable,
        }
    }

    pub fn convert_to_plain_enum(attribute: u32) -> Self {
        match attribute {
            1 => MapAttribute::Blocked,
            2 => MapAttribute::NpcBlocked,
            3 => MapAttribute::Warp(WarpData::default()),
            4 => MapAttribute::Sign(String::new()),
            5 => MapAttribute::ItemSpawn(ItemSpawnData::default()),
            6 => MapAttribute::Storage,
            7 => MapAttribute::Shop(0),
            _ => MapAttribute::Walkable,
        }
    }

    pub fn convert_to_num(attribute: &MapAttribute) -> u32 {
        match attribute {
            MapAttribute::Blocked => 1,
            MapAttribute::NpcBlocked => 2,
            MapAttribute::Warp(_) => 3,
            MapAttribute::Sign(_) => 4,
            MapAttribute::ItemSpawn(_) => 5,
            MapAttribute::Storage => 6,
            MapAttribute::Shop(_) => 7,
            _ => 0,
        }
    }
}
