#![allow(non_upper_case_globals)]

mod date;
mod duration;
mod enums;
mod instant;
mod position;
mod rgb;
mod sharedstructs;

pub use date::MyDate;
pub use duration::MyDuration;
pub use enums::*;
pub use instant::MyInstant;
pub use position::*;
pub use rgb::*;
pub use sharedstructs::*;

pub const VITALS_MAX: usize = VitalTypes::Count as usize;

///Map Data Maxs
pub const MAX_MAPS: usize = 3000;
pub const MAP_MAX_X: usize = 32;
pub const MAP_MAX_Y: usize = 32;
pub const MAX_SKILL_INV: usize = 11;
///Array Data Maxs
pub const MAX_NPCS: usize = 1000;
pub const MAX_ITEMS: usize = 2000;
pub const MAX_SHOPS: usize = 100;

pub const MAX_SHOP_ITEM: usize = 20;
