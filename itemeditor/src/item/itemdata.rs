use araiseal_types::*;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::BufReader;

#[derive(Clone, Debug, Deserialize, Serialize, Educe)]
#[educe(Default)]
pub struct ItemData {
    pub name: String,
    pub levelreq: u16,
    pub soundid: u16,
    pub sprite: u16,
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
}

impl ItemData {
    pub fn create_files() -> Result<(), String> {
        for i in 0..MAX_ITEMS {
            let name = format!("./data/items/{}.json", i);

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
        }

        Ok(())
    }

    pub fn save_file(&self, id: usize) -> Result<(), String> {
        let name = format!("./data/items/{}.json", id);

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

    pub fn load_files() -> Result<Vec<(ItemData, bool)>, String> {
        let mut items = Vec::<(ItemData, bool)>::new();

        for i in 0..MAX_ITEMS {
            let mut result = match Self::load_file(i) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

            if result.1 {
                result.0.save_file(i)?;
                result.1 = false;
            }

            items.push(result);
        }
        Ok(items)
    }

    pub fn load_file(id: usize) -> Result<(ItemData, bool), String> {
        let name = format!("./data/items/{}.json", id);

        match OpenOptions::new().read(true).open(&name) {
            Ok(file) => {
                let reader = BufReader::new(file);

                match serde_json::from_reader(reader) {
                    Ok(v) => Ok((v, false)),
                    Err(e) => {
                        println!("Error {:?}", e);
                        Ok((ItemData::default(), true))
                    }
                }
            }
            Err(e) => Err(format!("Failed to open {}, Err {:?}", name, e)),
        }
    }
}

pub fn data_labels(id: usize, item_type: ItemTypes) -> String {
    match item_type {
        ItemTypes::Weapon => match id {
            0 => String::from("Melee Damage"),
            1 => String::from("Magic Damage"),
            2 => String::from("Durability"),
            _ => String::from("None"),
        },
        ItemTypes::Accessory => match id {
            0 => String::from("Melee Defense"),
            1 => String::from("Magic Defense"),
            2 => String::from("Durability"),
            _ => String::from("None"),
        },
        ItemTypes::Cosmetic => String::from("None"),
        ItemTypes::Helmet => match id {
            0 => String::from("Melee Defense"),
            1 => String::from("Magic Defense"),
            2 => String::from("Durability"),
            _ => String::from("None"),
        },
        ItemTypes::Armor => match id {
            0 => String::from("Melee Defense"),
            1 => String::from("Magic Defense"),
            2 => String::from("Durability"),
            _ => String::from("None"),
        },
        ItemTypes::Trouser => match id {
            0 => String::from("Melee Defense"),
            1 => String::from("Magic Defense"),
            2 => String::from("Durability"),
            _ => String::from("None"),
        },
        ItemTypes::Boots => match id {
            0 => String::from("Melee Defense"),
            1 => String::from("Magic Defense"),
            2 => String::from("Durability"),
            _ => String::from("None"),
        },
        ItemTypes::Consume => match id {
            0 => String::from("Melee Damage"),
            1 => String::from("Magic Damage"),
            2 => String::from("Durability"),
            _ => String::from("None"),
        },
        ItemTypes::Tool => match id {
            0 => String::from("Melee Damage"),
            1 => String::from("Magic Damage"),
            2 => String::from("Durability"),
            _ => String::from("None"),
        },
        ItemTypes::Blueprint => String::from("None"),
        ItemTypes::Book => match id {
            0 => String::from("Book ID"),
            _ => String::from("None"),
        },
        ItemTypes::Questitem => match id {
            0 => String::from("Special ID"),
            1 => String::from("Reusable 0/1"),
            _ => String::from("None"),
        },
        ItemTypes::Trap => match id {
            0 => String::from("Melee Damage"),
            1 => String::from("Magic Damage"),
            _ => String::from("None"),
        },
        ItemTypes::Heavyobject => String::from("None"),
        ItemTypes::Key => match id {
            0 => String::from("Key ID"),
            1 => String::from("Reusable 0/1"),
            _ => String::from("None"),
        },
        _ => String::from("None"),
    }
}
