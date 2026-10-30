use serde::{Deserialize, Serialize};
use speedy::{Readable, Writable};
use std::io::Read;
use std::{
    fs::{self, OpenOptions},
    io::Write,
};

pub const MAX_SHOP_ITEM: usize = 20;
pub const MAX_SHOPS: usize = 100;

#[derive(
    Clone, Copy, Debug, Deserialize, Serialize, Default, Readable, Writable,
)]
pub struct ShopItem {
    pub index: u16,
    pub amount: u16,
    pub price: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default, Readable, Writable)]
pub struct ShopData {
    pub name: String,
    pub max_item: u16,
    pub item: [ShopItem; MAX_SHOP_ITEM],
}

impl ShopData {
    pub fn create_files() -> Result<(), String> {
        for i in 0..MAX_SHOPS {
            let name = format!("./json/shops/{i}.json");

            match OpenOptions::new().write(true).create_new(true).open(&name) {
                Ok(file) => {
                    let data = ShopData::default();

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

            let name = format!("./data/shops/{i}.bin");

            match OpenOptions::new().write(true).create_new(true).open(&name) {
                Ok(mut file) => {
                    let data = ShopData::default();

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
        let name: String = format!("./temp/shops/{id}.json");

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
        let name = format!("./temp/shops/{id}.bin");

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

    pub fn load_files() -> Result<Vec<(ShopData, bool)>, String> {
        let mut shops = Vec::<(ShopData, bool)>::new();

        for i in 0..MAX_SHOPS {
            let (result, edited) = if let Some(result) = Self::load_temp_file(i)
            {
                (result, true)
            } else if let Some(result) = Self::load_main_file(i) {
                (result, false)
            } else {
                let shop: ShopData = Self::default();
                shop.save_json_file(i)?;
                shop.save_bin_file(i)?;
                ShopData::finalize_file(i).map_err(|e| e.to_string())?;
                (shop, true)
            };

            shops.push((result, edited));
        }

        Ok(shops)
    }

    pub fn finalize_file(id: usize) -> std::io::Result<()> {
        let namebin = format!("./data/shops/{id}.bin");
        let tempnamebin = format!("./temp/shops/{id}.bin");
        let namejson = format!("./json/shops/{id}.json");
        let tempnamejson = format!("./temp/shops/{id}.json");

        fs::rename(tempnamebin, namebin)?;
        fs::rename(tempnamejson, namejson)
    }

    pub fn load_temp_file(id: usize) -> Option<ShopData> {
        let name = format!("./temp/shops/{id}.bin");

        Self::load_file(&name)
    }

    pub fn load_main_file(id: usize) -> Option<ShopData> {
        let name = format!("./data/shops/{id}.bin");

        Self::load_file(&name)
    }

    pub fn load_file(path: &str) -> Option<ShopData> {
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
