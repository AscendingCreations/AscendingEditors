use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct TileBox {
    pub x: u8,
    pub y: u8,
    pub width: u8,
    pub height: u8,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct GameTime {
    pub hour: u32,
    pub min: u32,
    pub sec: u32,
}
