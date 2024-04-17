use indexmap::IndexMap;
use std::fs;
use std::io;
use std::path::Path;

use crate::collection::TEXTURE_SIZE;
use graphics::*;

#[derive(Default, Debug, Clone)]
pub struct AudioCollection {
    pub audio: Vec<String>,
}

impl AudioCollection {
    pub fn new() -> Self {
        let entries = match fs::read_dir("./audio") {
            Ok(data) => data,
            Err(_) => return AudioCollection::default(),
        };

        let mut audio = Vec::new();

        entries.for_each(|entry| {
            if let Ok(entry_data) = entry {
                let file_name = entry_data.file_name();
                let file_name_str = file_name.to_string_lossy().into_owned();
                audio.push(file_name_str);
            }
        });

        println!("{:?}", audio);

        AudioCollection { audio }
    }
}

pub struct TextureData {
    pub name: String,
    pub allocation: usize,
}

pub struct TilesheetData {
    pub name: String,
    pub tile: TileSheet,
}

pub struct TextureAllocation {
    pub bg_layout: TextureData,
    pub tool_icon: TextureData,
    pub tab_icon: TextureData,
    pub tileset_button: TextureData,
    pub tileset_list_bg: TextureData,
    pub tileset_list_select: TextureData,
    pub scrollbar: TextureData,
    pub tab_option: TextureData,
    pub dialog_button: TextureData,
    pub option_button: TextureData,
    pub preference_button: TextureData,
    pub selection_drop_button: TextureData,
    pub tileset_bg: TextureData,
    pub mapview_bg: TextureData,
    pub direction_block_tile: TextureData,
    pub tilesheet: Vec<TilesheetData>,
    // This will be used for eyedropper tool
    pub tile_location: IndexMap<usize, (u32, u32, u32)>,
}

impl TextureAllocation {
    pub fn new(atlases: &mut [AtlasSet], renderer: &GpuRenderer) -> Result<Self, GraphicsError> {
        // This is how we load a image into a atlas/Texture. It returns the location of the image
        // within the texture. its x, y, w, h.  Texture loads the file. group_uploads sends it to the Texture
        // renderer is used to upload it to the GPU when done.
        let bg_layout = TextureData {
            name: "layout.png".to_string(),
            allocation: Texture::from_file("images/gui/layout.png")?
                .upload(&mut atlases[0], renderer)
                .ok_or_else(|| OtherError::new("failed to upload image"))?,
        };

        let tool_icon = TextureData {
            name: "tool_buttons.png".to_string(),
            allocation: Texture::from_file("images/gui/tool_buttons.png")?
                .upload(&mut atlases[0], renderer)
                .ok_or_else(|| OtherError::new("failed to upload image"))?,
        };

        let tab_option = TextureData {
            name: "tab_option_button.png".to_string(),
            allocation: Texture::from_file("images/gui/tab_option_button.png")?
                .upload(&mut atlases[0], renderer)
                .ok_or_else(|| OtherError::new("failed to upload image"))?,
        };

        let tab_icon = TextureData {
            name: "map_setting_buttons.png".to_string(),
            allocation: Texture::from_file("images/gui/map_setting_buttons.png")?
                .upload(&mut atlases[0], renderer)
                .ok_or_else(|| OtherError::new("failed to upload image"))?,
        };

        let tileset_button = TextureData {
            name: "tileset_selection_button.png".to_string(),
            allocation: Texture::from_file("images/gui/tileset_selection_button.png")?
                .upload(&mut atlases[0], renderer)
                .ok_or_else(|| OtherError::new("failed to upload image"))?,
        };

        let tileset_list_bg = TextureData {
            name: "tileset_list_bg.png".to_string(),
            allocation: Texture::from_file("images/gui/tileset_list_bg.png")?
                .upload(&mut atlases[0], renderer)
                .ok_or_else(|| OtherError::new("failed to upload image"))?,
        };

        let tileset_list_select = TextureData {
            name: "tileset_list_select.png".to_string(),
            allocation: Texture::from_file("images/gui/tileset_list_select.png")?
                .upload(&mut atlases[0], renderer)
                .ok_or_else(|| OtherError::new("failed to upload image"))?,
        };

        let scrollbar = TextureData {
            name: "scrollbar.png".to_string(),
            allocation: Texture::from_file("images/gui/scrollbar.png")?
                .upload(&mut atlases[0], renderer)
                .ok_or_else(|| OtherError::new("failed to upload image"))?,
        };

        let dialog_button = TextureData {
            name: "dialog_button.png".to_string(),
            allocation: Texture::from_file("images/gui/dialog_button.png")?
                .upload(&mut atlases[0], renderer)
                .ok_or_else(|| OtherError::new("failed to upload image"))?,
        };

        let option_button = TextureData {
            name: "option_button.png".to_string(),
            allocation: Texture::from_file("images/gui/option_button.png")?
                .upload(&mut atlases[0], renderer)
                .ok_or_else(|| OtherError::new("failed to upload image"))?,
        };

        let preference_button = TextureData {
            name: "preference_button.png".to_string(),
            allocation: Texture::from_file("images/gui/preference_button.png")?
                .upload(&mut atlases[0], renderer)
                .ok_or_else(|| OtherError::new("failed to upload image"))?,
        };

        let selection_drop_button = TextureData {
            name: "selection_drop_button.png".to_string(),
            allocation: Texture::from_file("images/gui/selection_drop_button.png")?
                .upload(&mut atlases[0], renderer)
                .ok_or_else(|| OtherError::new("failed to upload image"))?,
        };

        let tileset_bg = TextureData {
            name: "tileset_bg.png".to_string(),
            allocation: Texture::from_file("images/gui/tileset_bg.png")?
                .upload(&mut atlases[0], renderer)
                .ok_or_else(|| OtherError::new("failed to upload image"))?,
        };

        let mapview_bg = TextureData {
            name: "mapview_bg.png".to_string(),
            allocation: Texture::from_file("images/gui/mapview_bg.png")?
                .upload(&mut atlases[0], renderer)
                .ok_or_else(|| OtherError::new("failed to upload image"))?,
        };

        let direction_block_tile = TextureData {
            name: "direction_block_tile.png".to_string(),
            allocation: Texture::from_file("images/gui/direction_block_tile.png")?
                .upload(&mut atlases[0], renderer)
                .ok_or_else(|| OtherError::new("failed to upload image"))?,
        };

        let mut tile_location = IndexMap::new();
        let mut tilesheet = Vec::new();
        let mut count = 0;
        let mut path_found = true;
        while path_found {
            let path = format!("./images/tiles/tile_{}.png", count);
            if Path::new(&path).exists() {
                let res = TilesheetData {
                    name: format!("tile_{}.png", count),
                    tile: Texture::from_file(format!("images/tiles/tile_{}.png", count))?
                        .new_tilesheet(&mut atlases[1], renderer, TEXTURE_SIZE)
                        .ok_or_else(|| OtherError::new("failed to upload tiles"))?,
                };

                // Store the tile location
                for tile in &res.tile.tiles {
                    if tile.tex_id > 0 {
                        tile_location.insert(tile.tex_id, (tile.x, tile.y, count));
                    }
                }

                tilesheet.push(res);

                count += 1;
            } else {
                path_found = false;
            }
        }

        // Complete! We can now pass the result
        Ok(Self {
            bg_layout,
            tool_icon,
            tab_icon,
            tileset_button,
            tileset_list_bg,
            tileset_list_select,
            scrollbar,
            tab_option,
            dialog_button,
            option_button,
            preference_button,
            selection_drop_button,
            tileset_bg,
            mapview_bg,
            direction_block_tile,
            tilesheet,
            tile_location,
        })
    }
}
