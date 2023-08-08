use num_enum::TryFromPrimitive;
use serde_repr::*;
use strum_macros::Display;

#[derive(
    Copy, Clone, Debug, TryFromPrimitive, Eq, PartialEq, Serialize_repr, Deserialize_repr, Default,
)]
#[repr(u8)]
pub enum UserAccess {
    #[default]
    None,
    Monitor,
    Admin,
}

#[derive(
    Copy,
    Clone,
    Debug,
    TryFromPrimitive,
    Eq,
    PartialEq,
    Serialize_repr,
    Deserialize_repr,
    Default,
    Display,
)]
#[repr(u8)]
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
    TryFromPrimitive,
    Eq,
    PartialEq,
    Serialize_repr,
    Deserialize_repr,
    Default,
    Display,
    Hash,
)]
#[repr(u8)]
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

#[derive(
    Copy,
    Clone,
    Debug,
    TryFromPrimitive,
    Eq,
    PartialEq,
    Serialize_repr,
    Deserialize_repr,
    Default,
    Display,
)]
#[repr(u8)]
pub enum VitalTypes {
    Hp,
    Mp,
    Sp,
    #[default]
    #[strum(serialize = "None")]
    Count,
}

#[derive(
    Copy, Clone, Serialize_repr, Deserialize_repr, Display, TryFromPrimitive, Eq, PartialEq,
)]
#[repr(u8)]
pub enum MapAttributes {
    None,
    Blocked,
    DirBlocked,
    NpcBlocked,
    PlayerBlocked,
    Bank,
    Shop,
    Door,
    Craft,
    Slide,
    Warp,
    Item,
    Portal,
    CheckPoint,
    Sign,
    Resource,
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

#[derive(
    Copy,
    Clone,
    Debug,
    TryFromPrimitive,
    Eq,
    PartialEq,
    Serialize_repr,
    Deserialize_repr,
    Default,
    Display,
)]
#[repr(u8)]
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

#[derive(
    Copy,
    Clone,
    Debug,
    TryFromPrimitive,
    Eq,
    PartialEq,
    Default,
    Display,
    Serialize_repr,
    Deserialize_repr,
)]
#[repr(u8)]
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
