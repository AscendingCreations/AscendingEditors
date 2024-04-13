use araiseal_types::*;
use bytey::{ByteBuffer, ByteBufferRead, ByteBufferWrite};
use serde::{Deserialize, Serialize};
use std::io::BufReader;
use std::{fs::OpenOptions, io::Write};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, Educe, ByteBufferRead, ByteBufferWrite)]
#[educe(Default)]
pub struct ShopItem {
    pub index: u16,
    pub amount: u16,
    pub price: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize, Educe, ByteBufferRead, ByteBufferWrite)]
#[educe(Default)]
pub struct ShopData {
    pub name: String,
    pub max_item: u16,
    pub item: [ShopItem; MAX_SHOP_ITEM],
}

impl ShopData {
    pub fn create_files() -> Result<(), String> {
        for i in 0..MAX_SHOPS {
            let name = format!("./data/shops/{}.json", i);

            match OpenOptions::new().write(true).create_new(true).open(&name) {
                Ok(file) => {
                    let data = ShopData::default();

                    if let Err(e) = serde_json::to_writer_pretty(&file, &data) {
                        return Err(format!("Serdes File Error {:?}", e));
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(e) => return Err(format!("Failed to open {}, Err {:?}", name, e)),
            }

            let name = format!("./data/shops/bin/{}.bin", i);

            match OpenOptions::new().write(true).create_new(true).open(&name) {
                Ok(mut file) => {
                    let data = ShopData::default();

                    let mut buf = match ByteBuffer::new() {
                        Ok(data) => data,
                        Err(_) => return Ok(()),
                    };
                    buf.write(data).unwrap();

                    if let Err(e) = file.write(buf.as_slice()) {
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
        let name = format!("./data/shops/{}.json", id);

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
        let name = format!("./data/shops/bin/{}.bin", id);

        let mut buf = match ByteBuffer::new() {
            Ok(data) => data,
            Err(_) => return Ok(()),
        };

        buf.write(self).unwrap();

        match OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open(&name)
        {
            Ok(mut file) => {
                if let Err(e) = file.write(buf.as_slice()) {
                    Err(format!("File Error {:?}", e))
                } else {
                    Ok(())
                }
            }
            Err(e) => Err(format!("Failed to open {}, Err {:?}", name, e)),
        }
    }

    pub fn load_files() -> Result<Vec<(ShopData, bool)>, String> {
        let mut shops = Vec::<(ShopData, bool)>::new();

        for i in 0..MAX_SHOPS {
            let mut result = match Self::load_file(i) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

            if result.1 {
                result.0.save_file(i)?;
                result.0.save_bin_file(i)?;
                result.1 = false;
            }

            shops.push(result);
        }
        Ok(shops)
    }

    pub fn load_file(id: usize) -> Result<(ShopData, bool), String> {
        let name = format!("./data/shops/{}.json", id);

        match OpenOptions::new().read(true).open(&name) {
            Ok(file) => {
                let reader = BufReader::new(file);

                match serde_json::from_reader(reader) {
                    Ok(v) => Ok((v, false)),
                    Err(e) => {
                        println!("Error {:?}", e);
                        Ok((ShopData::default(), true))
                    }
                }
            }
            Err(e) => Err(format!("Failed to open {}, Err {:?}", name, e)),
        }
    }
}