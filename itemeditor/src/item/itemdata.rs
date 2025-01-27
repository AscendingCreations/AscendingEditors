use ascending_types::*;
use serde::{Deserialize, Serialize};
use speedy::{Readable, Writable};
use std::io::Read;
use std::{fs::OpenOptions, io::Write};

#[derive(Clone, Debug, Deserialize, Serialize, Educe, Readable, Writable)]
#[educe(Default)]
pub struct ItemData {
    pub name: String,
    pub levelreq: u16,
    pub soundid: u16,
    pub sprite: u16,
    pub animation: Option<u32>,
    pub data: [i16; 20],
    pub itemtype: ItemTypes,
    pub itemtype2: u8,
    pub breakable: bool,
    pub stackable: bool,
    #[educe(Default = 1)]
    pub stacklimit: u16,
    pub baseprice: u64,
    pub repairable: bool,
    pub rgba: Rgba,
    pub sound_index: Option<String>,
}

impl ItemData {
    pub fn create_files() -> Result<(), String> {
        for i in 0..MAX_ITEMS {
            let name = format!("./data/items/json/{}.json", i);

            match OpenOptions::new().write(true).create_new(true).open(&name) {
                Ok(file) => {
                    let data = ItemData::default();

                    if let Err(e) = serde_json::to_writer_pretty(&file, &data) {
                        return Err(format!("Serdes File Error {:?}", e));
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(e) => return Err(format!("Failed to open {}, Err {:?}", name, e)),
            }

            let name = format!("./data/items/{}.bin", i);

            match OpenOptions::new().write(true).create_new(true).open(&name) {
                Ok(mut file) => {
                    let data = ItemData::default();

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
        let name = format!("./data/items/json/{}.json", id);

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
        let name = format!("./data/items/{}.bin", id);

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

    pub fn load_files(save_json: bool) -> Result<Vec<(ItemData, bool)>, String> {
        let mut items = Vec::<(ItemData, bool)>::new();

        for i in 0..MAX_ITEMS {
            let mut result = Self::load_file(i)?;

            if result.1 {
                if save_json {
                    result.0.save_file(i)?;
                }
                result.0.save_bin_file(i)?;
                result.1 = false;
            }

            items.push(result);
        }
        Ok(items)
    }

    pub fn load_file(id: usize) -> Result<(ItemData, bool), String> {
        let name = format!("./data/items/{}.bin", id);

        match OpenOptions::new().read(true).open(&name) {
            Ok(mut file) => {
                let mut bytes = Vec::new();
                match file.read_to_end(&mut bytes) {
                    Ok(_) => Ok((ItemData::read_from_buffer(&bytes).unwrap(), false)),
                    Err(_) => Ok((ItemData::default(), true)),
                }
            }
            Err(e) => Err(format!("Failed to open {}, Err {:?}", name, e)),
        }
    }
}

pub fn data_labels(id: usize, item_type: ItemTypes) -> &'static str {
    match item_type {
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
            0 => "Melee Damage",
            1 => "Magic Damage",
            2 => "Durability",
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
    }
}
