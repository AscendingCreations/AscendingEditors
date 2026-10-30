use serde::{Deserialize, Serialize};
use speedy::{Readable, Writable};
use std::io::Read;
use std::{
    fs::{self, OpenOptions},
    io::Write,
};
pub mod item_parser;

pub const MAX_ITEMS: usize = 2000;

#[derive(Clone, Debug, Deserialize, Serialize, Default, Readable, Writable)]
pub struct Rgba {
    pub r: i16,
    pub g: i16,
    pub b: i16,
    pub a: i16,
}

#[derive(
    Clone, Copy, Debug, Deserialize, Serialize, Default, Readable, Writable,
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

#[derive(Clone, Debug, Deserialize, Serialize, Default, Readable, Writable)]
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
    pub stacklimit: u16,
    pub baseprice: u64,
    pub repairable: bool,
    pub rgba: Rgba,
    pub sound_index: Option<String>,
}

impl ItemData {
    pub fn create_files() -> Result<(), String> {
        for i in 0..MAX_ITEMS {
            let name = format!("./json/items/{i}.json");

            match OpenOptions::new().write(true).create_new(true).open(&name) {
                Ok(file) => {
                    let data = ItemData::default();

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

            let name = format!("./data/items/{i}.bin");

            match OpenOptions::new().write(true).create_new(true).open(&name) {
                Ok(mut file) => {
                    let data = ItemData::default();

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
        let name: String = format!("./temp/items/{id}.json");

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
        let name = format!("./temp/items/{id}.bin");

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

    pub fn load_files() -> Result<Vec<(ItemData, bool)>, String> {
        let mut items = Vec::<(ItemData, bool)>::new();

        for i in 0..MAX_ITEMS {
            let (result, edited) = if let Some(result) = Self::load_temp_file(i)
            {
                (result, true)
            } else if let Some(result) = Self::load_main_file(i) {
                (result, false)
            } else {
                let item: ItemData = Self::default();
                item.save_json_file(i)?;
                item.save_bin_file(i)?;
                ItemData::finalize_file(i).map_err(|e| e.to_string())?;
                (item, true)
            };

            items.push((result, edited));
        }

        Ok(items)
    }

    pub fn finalize_file(id: usize) -> std::io::Result<()> {
        let namebin = format!("./data/items/{id}.bin");
        let tempnamebin = format!("./temp/items/{id}.bin");
        let namejson = format!("./json/items/{id}.json");
        let tempnamejson = format!("./temp/items/{id}.json");

        fs::rename(tempnamebin, namebin)?;
        fs::rename(tempnamejson, namejson)
    }

    pub fn load_temp_file(id: usize) -> Option<ItemData> {
        let name = format!("./temp/items/{id}.bin");

        Self::load_file(&name)
    }

    pub fn load_main_file(id: usize) -> Option<ItemData> {
        let name = format!("./data/items/{id}.bin");

        Self::load_file(&name)
    }

    pub fn load_file(path: &str) -> Option<ItemData> {
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
