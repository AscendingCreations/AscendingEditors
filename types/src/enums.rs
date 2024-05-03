use serde::*;
use speedy::{Readable, Writable};
use strum_macros::Display;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Default)]
pub enum UserAccess {
    #[default]
    None,
    Monitor,
    Admin,
}

#[derive(
    Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Default, Display, Readable, Writable,
)]
pub enum AIBehavior {
    #[default]
    Friendly, //Never Attack or be attacked
    Agressive,       //Will attack on sight
    Reactive,        //Will attack when attacked
    HelpReactive,    //for npcs that when one gets attacked all in the area target the attacker.
    Healer,          //Will never Attack only heal other npcs
    AgressiveHealer, //Will attack on sight and heal
    ReactiveHealer,  //Will attack when attacked and heal
}

impl AIBehavior {
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

#[allow(dead_code)]
impl AIBehavior {
    pub fn is_agressive(&self) -> bool {
        matches!(self, AIBehavior::Agressive | AIBehavior::AgressiveHealer)
    }

    pub fn is_reactive(&self) -> bool {
        matches!(
            self,
            AIBehavior::Reactive | AIBehavior::HelpReactive | AIBehavior::ReactiveHealer
        )
    }

    pub fn is_healer(&self) -> bool {
        matches!(
            self,
            AIBehavior::Healer | AIBehavior::AgressiveHealer | AIBehavior::ReactiveHealer
        )
    }

    pub fn is_friendly(&self) -> bool {
        matches!(self, AIBehavior::Friendly)
    }
}

#[derive(
    Copy,
    Clone,
    Debug,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    Default,
    Display,
    Hash,
    Readable,
    Writable,
)]
pub enum ItemTypes {
    #[default]
    None,
    Weapon,
    Accessory,
    Cosmetic,
    Helmet,
    Armor,
    Trouser,
    Boots,
    Consume,
    Tool,
    Blueprint,
    Book,
    Questitem,
    Trap,
    Heavyobject,
    Key,
    Count,
}

impl ItemTypes {
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Default, Display)]
pub enum VitalTypes {
    Hp,
    Mp,
    Sp,
    #[default]
    #[strum(serialize = "None")]
    Count,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Display)]
pub enum MapLayers {
    Ground,
    Mask,
    Mask2,
    Anim1,
    Anim2,
    Anim3,
    Fringe,
    Fringe2,
    Count,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Default, Display)]
pub enum ToolType {
    #[default]
    None,
    Axe,
    Pick,
    Rod,
    Hoe,
    Scythe,
    Shovel,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, Display, Serialize, Deserialize)]
pub enum NpcMode {
    None,
    #[default]
    Normal,
    Pet,
    Summon,
    Boss,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MapPos {
    None,
    UpLeft(u64),
    Up(u64),
    UpRight(u64),
    Left(u64),
    Center(u64),
    Right(u64),
    DownLeft(u64),
    Down(u64),
    DownRight(u64),
}

#[allow(dead_code)]
impl MapPos {
    pub fn contains(self, id: u64) -> bool {
        matches!(self, MapPos::UpLeft(x)
            | MapPos::Up(x)
            | MapPos::UpRight(x)
            | MapPos::Left(x)
            | MapPos::Center(x)
            | MapPos::Right(x)
            | MapPos::DownLeft(x)
            | MapPos::Down(x)
            | MapPos::DownRight(x)
                if x == id )
    }

    pub fn get(self) -> Option<u64> {
        match self {
            MapPos::UpLeft(x)
            | MapPos::Up(x)
            | MapPos::UpRight(x)
            | MapPos::Left(x)
            | MapPos::Center(x)
            | MapPos::Right(x)
            | MapPos::DownLeft(x)
            | MapPos::Down(x)
            | MapPos::DownRight(x) => Some(x),
            MapPos::None => None,
        }
    }

    pub fn unwrap(self) -> u64 {
        match self {
            MapPos::UpLeft(x)
            | MapPos::Up(x)
            | MapPos::UpRight(x)
            | MapPos::Left(x)
            | MapPos::Center(x)
            | MapPos::Right(x)
            | MapPos::DownLeft(x)
            | MapPos::Down(x)
            | MapPos::DownRight(x) => x,
            MapPos::None => panic!("MapPos Can not be None for unwrap"),
        }
    }
}
