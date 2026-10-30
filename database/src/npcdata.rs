use serde::{Deserialize, Serialize};
use speedy::{Readable, Writable};
use std::io::Read;
use std::{
    fs::{self, OpenOptions},
    io::Write,
};

pub mod npc_parser;

pub const MAX_NPCS: usize = 1000;

#[derive(
    Copy,
    Clone,
    Debug,
    Eq,
    PartialEq,
    Serialize,
    Deserialize,
    Default,
    Readable,
    Writable,
)]
pub enum AIBehavior {
    #[default]
    Friendly, //Never Attack or be attacked
    Agressive,       //Will attack on sight
    Reactive,        //Will attack when attacked
    HelpReactive, //for npcs that when one gets attacked all in the area target the attacker.
    Healer,       //Will never Attack only heal other npcs
    AgressiveHealer, //Will attack on sight and heal
    ReactiveHealer, //Will attack when attacked and heal
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Deserialize,
    Serialize,
    Default,
    Readable,
    Writable,
)]
pub struct TileBox {
    pub x: u8,
    pub y: u8,
    pub width: u8,
    pub height: u8,
}

#[derive(
    Copy,
    Clone,
    Debug,
    Serialize,
    Deserialize,
    Default,
    PartialEq,
    Eq,
    Readable,
    Writable,
)]
pub struct GameTime {
    pub hour: u32,
    pub min: u32,
    pub sec: u32,
}

#[derive(
    PartialEq,
    Default,
    Clone,
    Copy,
    Debug,
    Serialize,
    Deserialize,
    Eq,
    Readable,
    Writable,
)]
pub struct DropItem {
    pub item: u32,
    pub amount: u32,
}

#[derive(
    PartialEq,
    Default,
    Clone,
    Copy,
    Debug,
    Serialize,
    Deserialize,
    Eq,
    Readable,
    Writable,
)]
pub struct NpcDrop {
    pub items: [DropItem; 5],
    pub shares: u32,
}

#[derive(
    PartialEq,
    Default,
    Clone,
    Debug,
    Serialize,
    Deserialize,
    Eq,
    Readable,
    Writable,
)]
pub struct NpcData {
    pub name: String,
    pub level: i32,
    pub sprite: i32,
    pub respawn_wait: i64,
    pub movement_wait: i64,
    pub attack_wait: i64,
    pub intervaled_wait: i64,
    pub spawn_wait: i64,
    pub maxhp: u32,
    pub maxsp: u32,
    pub maxmp: u32,
    pub sight: i32,
    pub follow_sight: i32,
    pub walkdistance: u32,
    pub pdamage: u32,
    pub pdefense: u32,
    pub canpassthru: bool,
    pub size: TileBox,
    pub behaviour: AIBehavior,
    pub maxdamage: u32,
    pub mindamage: u32,
    pub target_auto_switch: bool,
    pub target_attacked_switch: bool,
    pub target_auto_switch_chance: i64,
    pub target_range_dropout: bool,
    pub can_target: bool,
    pub can_move: bool,
    pub can_attack_player: bool,
    pub has_allys: bool,
    pub has_enemies: bool, // New
    pub can_attack: bool,
    pub has_selfonly: bool,
    pub has_friendonly: bool,
    pub has_groundonly: bool,
    pub runsaway: bool,
    pub isanimated: bool,
    pub run_damage: u32,
    pub spawntime: (GameTime, GameTime), //skill type to cast it with and  percentage needed to cast and Max Percentage.
    pub range: i32, //attack range. How far they need to be to hit their target.
    pub enemies: Vec<u64>, //list of enemies the npcs can attack of other npc's... WAR!
    pub drops: [NpcDrop; 10],
    pub free_shares: u32,
    pub exp: i64,
}

impl NpcData {
    pub fn create_files() -> Result<(), String> {
        for i in 0..MAX_NPCS {
            let name = format!("./json/npcs/{i}.json");

            match OpenOptions::new().write(true).create_new(true).open(&name) {
                Ok(file) => {
                    let data = NpcData::default();

                    if let Err(e) = serde_json::to_writer_pretty(&file, &data) {
                        return Err(format!("Serdes File Error {e:?}"));
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                    continue;
                }
                Err(e) => {
                    return Err(format!("Failed to open {name}, Err {e:?}"));
                }
            }

            let name = format!("./data/npcs/{i}.bin");

            match OpenOptions::new().write(true).create_new(true).open(&name) {
                Ok(mut file) => {
                    let data = NpcData::default();

                    let bytes = data.write_to_vec().unwrap();

                    if let Err(e) = file.write(bytes.as_slice()) {
                        return Err(format!("File Error {e:?}"));
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                    continue;
                }
                Err(e) => {
                    return Err(format!("Failed to open {name}, Err {e:?}"));
                }
            }
        }

        Ok(())
    }

    pub fn save_json_file(&self, id: usize) -> Result<(), String> {
        let name: String = format!("./temp/npcs/{id}.json");

        match OpenOptions::new().truncate(true).write(true).open(&name) {
            Ok(file) => {
                if let Err(e) = serde_json::to_writer_pretty(&file, self) {
                    Err(format!("Serdes File Error {e:?}"))
                } else {
                    Ok(())
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                Ok(())
            }
            Err(e) => Err(format!("Failed to open {name}, Err {e:?}")),
        }
    }

    pub fn save_bin_file(&self, id: usize) -> Result<(), String> {
        let name = format!("./temp/npcs/{id}.bin");

        let bytes = self.write_to_vec().unwrap();

        match OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open(&name)
        {
            Ok(mut file) => {
                if let Err(e) = file.write(bytes.as_slice()) {
                    Err(format!("File Error {e:?}"))
                } else {
                    Ok(())
                }
            }
            Err(e) => Err(format!("Failed to open {name}, Err {e:?}")),
        }
    }

    pub fn load_files() -> Result<Vec<(NpcData, bool)>, String> {
        let mut npcs = Vec::<(NpcData, bool)>::new();

        for i in 0..MAX_NPCS {
            let (result, edited) = if let Some(result) = Self::load_temp_file(i)
            {
                (result, true)
            } else if let Some(result) = Self::load_main_file(i) {
                (result, false)
            } else {
                let npc: NpcData = Self::default();
                npc.save_json_file(i)?;
                npc.save_bin_file(i)?;
                NpcData::finalize_file(i).map_err(|e| e.to_string())?;
                (npc, true)
            };

            npcs.push((result, edited));
        }

        Ok(npcs)
    }

    pub fn finalize_file(id: usize) -> std::io::Result<()> {
        let namebin = format!("./data/npcs/{id}.bin");
        let tempnamebin = format!("./temp/npcs/{id}.bin");
        let namejson = format!("./json/npcs/{id}.json");
        let tempnamejson = format!("./temp/npcs/{id}.json");

        fs::rename(tempnamebin, namebin)?;
        fs::rename(tempnamejson, namejson)
    }

    pub fn load_temp_file(id: usize) -> Option<NpcData> {
        let name = format!("./temp/npcs/{id}.bin");

        Self::load_file(&name)
    }

    pub fn load_main_file(id: usize) -> Option<NpcData> {
        let name = format!("./data/npcs/{id}.bin");

        Self::load_file(&name)
    }

    pub fn load_file(path: &str) -> Option<NpcData> {
        let mut file = OpenOptions::new().read(true).open(path).ok()?;

        let len = if let Ok(meta) = file.metadata() {
            meta.len() as usize
        } else {
            0
        };

        let mut bytes = Vec::with_capacity(len);

        let _ = file.read_to_end(&mut bytes).ok()?;

        Self::read_from_buffer(&bytes).ok()
    }
}
