use ascending_types::*;
use serde::{Deserialize, Serialize};
use speedy::{Readable, Writable};
use std::io::Read;
use std::{fs::OpenOptions, io::Write};

#[derive(Educe, Clone, Debug, Serialize, Deserialize, Eq, Readable, Writable)]
#[educe(PartialEq, Default)]
pub struct DropItem {
    pub item: u32,
    pub amount: u32,
}

#[derive(Educe, Clone, Debug, Serialize, Deserialize, Eq, Readable, Writable)]
#[educe(PartialEq, Default)]
pub struct NpcDrop {
    pub items: [DropItem; 5],
    pub shares: u32,
}

#[derive(Educe, Clone, Debug, Serialize, Deserialize, Eq, Readable, Writable)]
#[educe(PartialEq, Default)]
pub struct NpcData {
    pub name: String,
    #[educe(Default = 1)]
    pub level: i32,
    pub sprite: i32,
    #[educe(Default = 1)]
    pub respawn_wait: i64,
    #[educe(Default = 1)]
    pub movement_wait: i64,
    #[educe(Default = 1)]
    pub attack_wait: i64,
    #[educe(Default = 1)]
    pub intervaled_wait: i64,
    #[educe(Default = 1)]
    pub spawn_wait: i64,
    #[educe(Default = 1)]
    pub maxhp: u32,
    pub maxsp: u32,
    pub maxmp: u32,
    pub sight: i32,
    pub follow_sight: i32,
    pub walkdistance: u32,
    #[educe(Default = 1)]
    pub pdamage: u32,
    #[educe(Default = 1)]
    pub pdefense: u32,
    pub canpassthru: bool,
    #[educe(Default(expression = TileBox {
        x: 1,
        y: 1,
        width: 1,
        height: 1,
    }))]
    pub size: TileBox,
    pub behaviour: AIBehavior,
    #[educe(Default = 1)]
    pub maxdamage: u32,
    #[educe(Default = 1)]
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
    pub range: i32, // New       //attack range. How far they need to be to hit their target.
    pub enemies: Vec<u64>, // New //list of enemies the npcs can attack of other npc's... WAR!
    //pub drops: [(u32, u32, u32); 10], //item dropped on death, chance, amount
    //pub drops_max: u16,
    pub drops: [NpcDrop; 10],
    pub free_shares: u32,

    pub exp: i64,
}

impl NpcData {
    pub fn create_files() -> Result<(), String> {
        for i in 0..MAX_NPCS {
            let name = format!("./data/npcs/json/{}.json", i);

            match OpenOptions::new().write(true).create_new(true).open(&name) {
                Ok(file) => {
                    let data = NpcData::default();

                    if let Err(e) = serde_json::to_writer_pretty(&file, &data) {
                        return Err(format!("Serdes File Error {:?}", e));
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(e) => return Err(format!("Failed to open {}, Err {:?}", name, e)),
            }

            let name = format!("./data/npcs/{}.bin", i);

            match OpenOptions::new().write(true).create_new(true).open(&name) {
                Ok(mut file) => {
                    let data = NpcData::default();

                    let bytes = data.write_to_vec().unwrap();

                    if let Err(e) = file.write(bytes.as_slice()) {
                        return Err(format!("File Error {:?}", e));
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(e) => return Err(format!("Failed to open {}, Err {:?}", name, e)),
            }
        }

        Ok(())
    }

    pub fn save_file(&self, id: usize) -> Result<(), String> {
        let name = format!("./data/npcs/json/{}.json", id);

        match OpenOptions::new().truncate(true).write(true).open(&name) {
            Ok(file) => {
                if let Err(e) = serde_json::to_writer_pretty(&file, self) {
                    Err(format!("Serdes File Error {:?}", e))
                } else {
                    Ok(())
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::AlreadyExists => Ok(()),
            Err(e) => Err(format!("Failed to open {}, Err {:?}", name, e)),
        }
    }

    pub fn save_bin_file(&self, id: usize) -> Result<(), String> {
        let name = format!("./data/npcs/{}.bin", id);

        let bytes = self.write_to_vec().unwrap();

        match OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open(&name)
        {
            Ok(mut file) => {
                if let Err(e) = file.write(bytes.as_slice()) {
                    Err(format!("File Error {:?}", e))
                } else {
                    Ok(())
                }
            }
            Err(e) => Err(format!("Failed to open {}, Err {:?}", name, e)),
        }
    }

    pub fn load_files(save_json: bool) -> Result<Vec<(NpcData, bool)>, String> {
        let mut data = Vec::<(NpcData, bool)>::new();

        for i in 0..MAX_NPCS {
            let mut result = match Self::load_file(i) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

            if result.1 {
                if save_json {
                    result.0.save_file(i)?;
                }
                result.0.save_bin_file(i)?;
                result.1 = false;
            }

            data.push(result);
        }
        Ok(data)
    }

    pub fn load_file(id: usize) -> Result<(NpcData, bool), String> {
        let name = format!("./data/npcs/{}.bin", id);

        match OpenOptions::new().read(true).open(&name) {
            Ok(mut file) => {
                let mut bytes = Vec::new();
                match file.read_to_end(&mut bytes) {
                    Ok(_) => Ok((NpcData::read_from_buffer(&bytes).unwrap(), false)),
                    Err(_) => Ok((NpcData::default(), true)),
                }
            }
            Err(e) => Err(format!("Failed to open {}, Err {:?}", name, e)),
        }
    }
}
