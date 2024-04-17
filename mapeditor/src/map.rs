pub mod attributes;
mod recording;
use bit_op::{bit_u8::*, BitOp};

use cosmic_text::{Attrs, Metrics, Weight};
use graphics::*;
use indexmap::IndexMap;

pub use attributes::*;
use recording::*;

use crate::{
    collection::*, create_basic_label, map_data::*, ConfigData, DrawSetting,
};

pub struct MapAttributes {
    pub pos: Vec2,
    pub image: usize,
    pub text: usize,
    pub attribute: MapAttribute,
}

impl MapAttributes {
    pub fn set_attribute(
        &mut self,
        systems: &mut DrawSetting,
        attribute: MapAttribute,
    ) {
        self.attribute = attribute.clone();
        systems
            .gfx
            .set_color(self.image, MapAttribute::get_color(&self.attribute));
        systems.gfx.set_text(
            &mut systems.renderer,
            self.text,
            MapAttribute::as_map_str(&self.attribute),
        );
        let size = systems.gfx.get_measure(self.text);
        let mut pos = systems.gfx.get_pos(self.text);
        pos.x = self.pos.x + (TEXTURE_SIZE as f32 * 0.5) - (size.x * 0.5);
        systems.gfx.set_pos(self.text, pos);
    }
}

#[derive(Default)]
pub struct MapZone {
    pub pos: Vec<Vec2>,
}

#[derive(Default)]
pub struct MapZoneSetting {
    pub max_npc: u64,
    pub npc_id: [Option<u64>; 5],
}

#[derive(Default, Clone, Copy)]
pub struct DirBlockTile {
    pub bg: usize,
    pub dir: [usize; 4],
    pub dir_data: u8,
}

impl DirBlockTile {
    pub fn set_data(
        &mut self,
        systems: &mut DrawSetting,
        dir_visible: [bool; 4], // Up, Left, Down, Right
    ) {
        // B0 = Down, B1 = Up, B2 = Left, B3 = Right
        let mut dir_data = 0;
        if dir_visible[0] {
            dir_data.set(B1);
        }
        if dir_visible[1] {
            dir_data.set(B2);
        }
        if dir_visible[2] {
            dir_data.set(B0);
        }
        if dir_visible[3] {
            dir_data.set(B3);
        }
        self.dir_data = dir_data;

        for (index, visible) in dir_visible.iter().enumerate() {
            systems.gfx.set_visible(self.dir[index], *visible);
        }
    }

    pub fn set_data_bit(&mut self, systems: &mut DrawSetting, dir: u8) {
        self.dir_data = dir;

        // B0 = Down, B1 = Up, B2 = Left, B3 = Right
        let mut dir_visible = [false; 4]; // Up, Left, Down, Right
        if self.dir_data.get(B0) == 0b00000001 {
            dir_visible[2] = true;
        }
        if self.dir_data.get(B1) == 0b00000010 {
            dir_visible[0] = true;
        }
        if self.dir_data.get(B2) == 0b00000100 {
            dir_visible[1] = true;
        }
        if self.dir_data.get(B3) == 0b00001000 {
            dir_visible[3] = true;
        }

        for (index, visible) in dir_visible.iter().enumerate() {
            systems.gfx.set_visible(self.dir[index], *visible);
        }
    }

    pub fn update(&mut self, systems: &mut DrawSetting) {
        self.set_data_bit(systems, self.dir_data);
    }
}

pub struct MapView {
    pub maps: Vec<Map>,
    pub link_map_selection: Vec<usize>,
    pub selection_preview: usize,
    preview_pos: Vec2,
    preview_size: Vec2,

    pub map_attributes: Vec<MapAttributes>,
    pub map_zone: Vec<usize>,
    pub map_dir_block: Vec<DirBlockTile>,
    pub map_zone_loc: [MapZone; 5],
    pub map_zone_setting: [MapZoneSetting; 5],
    pub fixed_weather: u8,
    pub music: Option<String>,

    // Recording
    pub record: Records,
}

impl MapView {
    pub fn new(
        systems: &mut DrawSetting,
        config_data: &mut ConfigData,
    ) -> Self {
        let mut maps = Vec::with_capacity(9);
        let mut link_map_selection = Vec::with_capacity(8);

        // Create 9 maps for our view of the main map and the surrounding maps
        for count in 0..9 {
            let mut map = Map::new(&mut systems.renderer, TEXTURE_SIZE);

            // Set default position of each view
            // Note: Index '0' is the main view on the center
            // while the other view are for surrounding maps
            match count {
                1 => {
                    map.pos = Vec2::new(215.0, 719.0);
                } // Top Left
                2 => {
                    map.pos = Vec2::new(257.0, 719.0);
                } // Top
                3 => {
                    map.pos = Vec2::new(899.0, 719.0);
                } // Top Right
                4 => {
                    map.pos = Vec2::new(215.0, 77.0);
                } // Left
                5 => {
                    map.pos = Vec2::new(899.0, 77.0);
                } // Right
                6 => {
                    map.pos = Vec2::new(215.0, 35.0);
                } // Bottom Left
                7 => {
                    map.pos = Vec2::new(257.0, 35.0);
                } // Bottom
                8 => {
                    map.pos = Vec2::new(899.0, 35.0);
                } // Bottom Right
                _ => {
                    map.pos = Vec2::new(257.0, 77.0);
                } // Center / Main
            }

            map.can_render = true;
            maps.push(map);
        }

        for count in 0..8 {
            let mut image = Rect::new(&mut systems.renderer, 0);
            image
                .set_size(match count {
                    1 => Vec2::new(
                        TEXTURE_SIZE as f32 * 32.0,
                        TEXTURE_SIZE as f32 * 2.0,
                    ), // Top
                    2 => Vec2::new(
                        TEXTURE_SIZE as f32 * 2.0,
                        TEXTURE_SIZE as f32 * 2.0,
                    ), // Top Right
                    3 => Vec2::new(
                        TEXTURE_SIZE as f32 * 2.0,
                        TEXTURE_SIZE as f32 * 32.0,
                    ), // Left
                    4 => Vec2::new(
                        TEXTURE_SIZE as f32 * 2.0,
                        TEXTURE_SIZE as f32 * 32.0,
                    ), // Right
                    5 => Vec2::new(
                        TEXTURE_SIZE as f32 * 2.0,
                        TEXTURE_SIZE as f32 * 2.0,
                    ), // Bottom Left
                    6 => Vec2::new(
                        TEXTURE_SIZE as f32 * 32.0,
                        TEXTURE_SIZE as f32 * 2.0,
                    ), // Bottom
                    7 => Vec2::new(
                        TEXTURE_SIZE as f32 * 2.0,
                        TEXTURE_SIZE as f32 * 2.0,
                    ), // Bottom Right
                    _ => Vec2::new(
                        TEXTURE_SIZE as f32 * 2.0,
                        TEXTURE_SIZE as f32 * 2.0,
                    ), // Top Left
                })
                // We set the link selection image at the same position as the linked map
                // We add +1 on the count as the linked map started on index 1 instead of 0
                .set_position(Vec3::new(
                    maps[count + 1].pos.x,
                    maps[count + 1].pos.y,
                    ORDER_MAP_LINK_SELECT,
                ))
                .set_color(Color::rgba(0, 0, 0, 130))
                .set_use_camera(true);

            link_map_selection.push(systems.gfx.add_rect(image, 0));
        }

        // This will create the selection box on the map view
        let mut selectionpreview = Rect::new(&mut systems.renderer, 0);
        selectionpreview
            .set_size(Vec2::new(TEXTURE_SIZE as f32, TEXTURE_SIZE as f32))
            .set_position(Vec3::new(
                maps[0].pos.x,
                maps[0].pos.y,
                ORDER_MAP_SELECTION,
            ))
            .set_color(Color::rgba(
                config_data.map_selection_color[0],
                config_data.map_selection_color[1],
                config_data.map_selection_color[2],
                150,
            ))
            .set_use_camera(true);
        let selection_preview = systems.gfx.add_rect(selectionpreview, 0);

        // Map Attributes & Map Zones
        let mut map_attributes = Vec::with_capacity(1024);
        let mut map_zone = Vec::with_capacity(1024);
        let mut map_dir_block = Vec::with_capacity(1024);
        for i in 0..1024 {
            let pos = Vec2::new(
                maps[0].pos.x + ((i % 32) * TEXTURE_SIZE) as f32,
                maps[0].pos.y + ((i / 32) * TEXTURE_SIZE) as f32,
            );
            // BG
            let mut img = Rect::new(&mut systems.renderer, 0);
            img.set_size(Vec2::new(TEXTURE_SIZE as f32, TEXTURE_SIZE as f32))
                .set_position(Vec3::new(pos.x, pos.y, ORDER_MAP_ATTRIBUTE_BG))
                .set_color(Color::rgba(0, 0, 0, 0))
                .set_use_camera(true);

            // Text
            let label_size = Vec2::new(32.0, 32.0);
            let txt = create_basic_label(
                systems,
                Vec3::new(pos.x, pos.y - 13.0, ORDER_MAP_ATTRIBUTE_TEXT),
                label_size,
                Color::rgba(255, 255, 255, 255),
            );

            let (image, text) =
                (systems.gfx.add_rect(img, 0), systems.gfx.add_text(txt, 1));
            systems.gfx.set_visible(image, false);
            systems.gfx.set_visible(text, false);

            map_attributes.push(MapAttributes {
                pos,
                image,
                text,
                attribute: MapAttribute::Walkable,
            });

            // Zone BG
            let mut zone_box = Rect::new(&mut systems.renderer, 0);
            zone_box
                .set_size(Vec2::new(TEXTURE_SIZE as f32, TEXTURE_SIZE as f32))
                .set_position(Vec3::new(pos.x, pos.y, ORDER_MAP_ZONE))
                .set_color(Color::rgba(0, 0, 0, 0))
                .set_use_camera(true);
            map_zone.push(systems.gfx.add_rect(zone_box, 0));

            // Dir Block
            let mut block_bg = Image::new(
                Some(systems.resource.direction_block_tile.allocation),
                &mut systems.renderer,
                0,
            );
            block_bg.pos = Vec3::new(pos.x, pos.y, ORDER_MAP_DIRBLOCK);
            block_bg.hw = Vec2::new(TEXTURE_SIZE as f32, TEXTURE_SIZE as f32);
            block_bg.uv =
                Vec4::new(0.0, 0.0, TEXTURE_SIZE as f32, TEXTURE_SIZE as f32);
            let bg = systems.gfx.add_image(block_bg, 0);
            systems.gfx.set_visible(bg, false);

            let mut dir0 = Image::new(
                Some(systems.resource.direction_block_tile.allocation),
                &mut systems.renderer,
                0,
            );
            dir0.pos = Vec3::new(pos.x, pos.y, ORDER_MAP_DIRBLOCK);
            dir0.hw = Vec2::new(TEXTURE_SIZE as f32, TEXTURE_SIZE as f32);
            dir0.uv =
                Vec4::new(20.0, 0.0, TEXTURE_SIZE as f32, TEXTURE_SIZE as f32);

            let mut dir1 = Image::new(
                Some(systems.resource.direction_block_tile.allocation),
                &mut systems.renderer,
                0,
            );
            dir1.pos = Vec3::new(pos.x, pos.y, ORDER_MAP_DIRBLOCK);
            dir1.hw = Vec2::new(TEXTURE_SIZE as f32, TEXTURE_SIZE as f32);
            dir1.uv =
                Vec4::new(40.0, 0.0, TEXTURE_SIZE as f32, TEXTURE_SIZE as f32);

            let mut dir2 = Image::new(
                Some(systems.resource.direction_block_tile.allocation),
                &mut systems.renderer,
                0,
            );
            dir2.pos = Vec3::new(pos.x, pos.y, ORDER_MAP_DIRBLOCK);
            dir2.hw = Vec2::new(TEXTURE_SIZE as f32, TEXTURE_SIZE as f32);
            dir2.uv =
                Vec4::new(60.0, 0.0, TEXTURE_SIZE as f32, TEXTURE_SIZE as f32);

            let mut dir3 = Image::new(
                Some(systems.resource.direction_block_tile.allocation),
                &mut systems.renderer,
                0,
            );
            dir3.pos = Vec3::new(pos.x, pos.y, ORDER_MAP_DIRBLOCK);
            dir3.hw = Vec2::new(TEXTURE_SIZE as f32, TEXTURE_SIZE as f32);
            dir3.uv =
                Vec4::new(80.0, 0.0, TEXTURE_SIZE as f32, TEXTURE_SIZE as f32);

            let dir = [
                systems.gfx.add_image(dir0, 0),
                systems.gfx.add_image(dir1, 0),
                systems.gfx.add_image(dir2, 0),
                systems.gfx.add_image(dir3, 0),
            ];
            for data in dir.iter() {
                systems.gfx.set_visible(*data, false);
            }

            map_dir_block.push(DirBlockTile {
                bg,
                dir,
                dir_data: 0,
            })
        }

        Self {
            maps,
            link_map_selection,
            selection_preview,
            preview_pos: Vec2::new(0.0, 0.0),
            preview_size: Vec2::new(1.0, 1.0),
            map_attributes,
            map_zone,
            map_dir_block,
            map_zone_loc: Default::default(),
            map_zone_setting: Default::default(),
            record: Records::new(),
            fixed_weather: 0,
            music: None,
        }
    }

    // This function create an effect when we are hovering on the linked map
    pub fn hover_linked_selection(
        &mut self,
        systems: &mut DrawSetting,
        pos: Vec2,
    ) -> Option<usize> {
        let mut result = None;
        for (index, selection) in self.link_map_selection.iter_mut().enumerate()
        {
            let (position, size, color) = (
                systems.gfx.get_pos(*selection),
                systems.gfx.get_size(*selection),
                systems.gfx.get_color(*selection),
            );
            let is_within_pos = pos.x >= position.x
                && pos.x <= position.x + size.x
                && pos.y >= position.y
                && pos.y <= position.y + size.y;

            if is_within_pos {
                if color != Color::rgba(0, 0, 0, 0) {
                    systems.gfx.set_color(*selection, Color::rgba(0, 0, 0, 0));
                }
                result = Some(index);
            } else if color != Color::rgba(0, 0, 0, 130) {
                systems.gfx.set_color(*selection, Color::rgba(0, 0, 0, 130));
            }
        }
        result
    }

    pub fn set_attribute(
        &mut self,
        systems: &mut DrawSetting,
        set_pos: Vec2,
        attributes: MapAttribute,
    ) {
        let tilepos = get_tile_pos(set_pos.x as i32, set_pos.y as i32);

        // Record change for undo purpose
        let last_attribute = self.map_attributes[tilepos].attribute.clone();
        let last_attribute_num = MapAttribute::convert_to_num(&last_attribute);
        let data = match last_attribute {
            MapAttribute::Warp(warpdata) => {
                vec![
                    InsertTypes::Int(warpdata.map_x as i64),
                    InsertTypes::Int(warpdata.map_y as i64),
                    InsertTypes::UInt(warpdata.map_group),
                    InsertTypes::UInt(warpdata.tile_x as u64),
                    InsertTypes::UInt(warpdata.tile_y as u64),
                ]
            }
            MapAttribute::Sign(text) => vec![InsertTypes::Str(text)],
            MapAttribute::ItemSpawn(itemdata) => {
                vec![
                    InsertTypes::UInt(itemdata.index as u64),
                    InsertTypes::UInt(itemdata.amount as u64),
                    InsertTypes::UInt(itemdata.timer),
                ]
            }
            MapAttribute::Shop(index) => vec![InsertTypes::UInt(index as u64)],
            _ => vec![],
        };
        self.record.push_undo(
            Vec3::new(set_pos.x, set_pos.y, 0.0),
            RecordType::Attribute,
            last_attribute_num as i64,
            data,
        );

        self.map_attributes[tilepos].set_attribute(systems, attributes);
    }

    pub fn get_attribute(&mut self, pos: Vec2) -> MapAttribute {
        let tilepos = get_tile_pos(pos.x as i32, pos.y as i32);
        self.map_attributes[tilepos].attribute.clone()
    }

    pub fn set_attribute_fill(
        &mut self,
        systems: &mut DrawSetting,
        set_pos: Vec2,
        attribute: MapAttribute,
    ) {
        let tilepos = get_tile_pos(set_pos.x as i32, set_pos.y as i32);

        // We will only change the tiles that have a similar texture id, and this will be use to check
        let comparedata = self.map_attributes[tilepos].attribute.clone();
        if comparedata == attribute {
            return;
        }

        // This will hold the location that need to be paint
        let mut paint_to_map: Vec<Vec2> = Vec::with_capacity(0);

        // Place our starting location on to be paint collection
        paint_to_map.push(set_pos);

        // Loop through our collections of position that requires to be paint
        while let Some(pos) = paint_to_map.pop() {
            // Record change for undo purpose
            let last_attribute = self.map_attributes[tilepos].attribute.clone();
            let last_attribute_num =
                MapAttribute::convert_to_num(&last_attribute);
            let data = match last_attribute {
                MapAttribute::Warp(warpdata) => {
                    vec![
                        InsertTypes::Int(warpdata.map_x as i64),
                        InsertTypes::Int(warpdata.map_y as i64),
                        InsertTypes::UInt(warpdata.map_group),
                        InsertTypes::UInt(warpdata.tile_x as u64),
                        InsertTypes::UInt(warpdata.tile_y as u64),
                    ]
                }
                MapAttribute::Sign(text) => vec![InsertTypes::Str(text)],
                MapAttribute::ItemSpawn(itemdata) => {
                    vec![
                        InsertTypes::UInt(itemdata.index as u64),
                        InsertTypes::UInt(itemdata.amount as u64),
                        InsertTypes::UInt(itemdata.timer),
                    ]
                }
                MapAttribute::Shop(index) => {
                    vec![InsertTypes::UInt(index as u64)]
                }
                _ => vec![],
            };
            self.record.push_undo(
                Vec3::new(set_pos.x, set_pos.y, 0.0),
                RecordType::Attribute,
                last_attribute_num as i64,
                data,
            );

            // Paint the map
            let tile_pos = get_tile_pos(pos.x as i32, pos.y as i32);
            self.map_attributes[tile_pos]
                .set_attribute(systems, attribute.clone());

            // Check direction
            for dir in 0..4 {
                // Get the surrounding map position
                let mut adjust_pos = Vec2::new(0.0, 0.0);
                match dir {
                    1 => {
                        adjust_pos.y = 1.0;
                    } // Up
                    2 => {
                        adjust_pos.x = -1.0;
                    } // Left
                    3 => {
                        adjust_pos.x = 1.0;
                    } // Right
                    _ => {
                        adjust_pos.y = -1.0;
                    } // Down
                }
                let checkpos = pos + adjust_pos;

                if checkpos.x >= 0.0
                    && checkpos.x < 32.0
                    && checkpos.y >= 0.0
                    && checkpos.y < 32.0
                {
                    // Check the map texture id and we make sure that we only change
                    // if they have the same texture id as the starting tile
                    let check_tilepos =
                        get_tile_pos(checkpos.x as i32, checkpos.y as i32);
                    let check_data =
                        self.map_attributes[check_tilepos].attribute.clone();
                    if check_data == comparedata {
                        paint_to_map.push(checkpos);
                    }
                }
            }
        }
    }

    pub fn set_dir_block(
        &mut self,
        systems: &mut DrawSetting,
        set_pos: Vec2,
        dir_visible: [bool; 4],
    ) {
        let tilepos = get_tile_pos(set_pos.x as i32, set_pos.y as i32);
        self.map_dir_block[tilepos].set_data(systems, dir_visible);
    }

    pub fn set_tile_group(
        &mut self,
        set_pos: Vec2,
        layer: u32,
        tileset: &Map,
        start_pos: Vec2,
        selection_size: Vec2,
    ) {
        for x in 0..selection_size.x as u32 {
            for y in 0..selection_size.y as u32 {
                // We load the tile data from the tileset
                let tiledata = tileset.get_tile((
                    start_pos.x as u32 + x,
                    start_pos.y as u32 + y,
                    0,
                ));

                // Make sure we only add tile that are not empty
                if tiledata.id > 0 {
                    // Make sure we wont set map outside the map size limit
                    if (set_pos.x as u32 + x) < 32
                        && (set_pos.y as u32 + y) < 32
                    {
                        // Record change for undo purpose
                        let last_texture = self.maps[0]
                            .get_tile((
                                set_pos.x as u32 + x,
                                set_pos.y as u32 + y,
                                layer,
                            ))
                            .id;
                        self.record.push_undo(
                            Vec3::new(
                                set_pos.x + x as f32,
                                set_pos.y + y as f32,
                                layer as f32,
                            ),
                            RecordType::Layer,
                            last_texture as i64,
                            vec![],
                        );

                        self.maps[0].set_tile(
                            (set_pos.x as u32 + x, set_pos.y as u32 + y, layer),
                            tiledata,
                        );
                    }
                }
            }
        }
    }

    pub fn delete_tile_group(&mut self, set_pos: Vec2, layer: u32, size: Vec2) {
        for x in 0..size.x as u32 {
            for y in 0..size.y as u32 {
                // Make sure we wont set map outside the map size limit
                if (set_pos.x as u32 + x) < 32 && (set_pos.y as u32 + y) < 32 {
                    let texture_id = self.maps[0]
                        .get_tile((
                            set_pos.x as u32 + x,
                            set_pos.y as u32 + y,
                            layer,
                        ))
                        .id;
                    if texture_id > 0 {
                        // Record change for undo purpose
                        let last_texture = self.maps[0]
                            .get_tile((
                                set_pos.x as u32 + x,
                                set_pos.y as u32 + y,
                                layer,
                            ))
                            .id;
                        self.record.push_undo(
                            Vec3::new(
                                set_pos.x + x as f32,
                                set_pos.y + y as f32,
                                layer as f32,
                            ),
                            RecordType::Layer,
                            last_texture as i64,
                            vec![],
                        );

                        self.maps[0].set_tile(
                            (set_pos.x as u32 + x, set_pos.y as u32 + y, layer),
                            TileData::default(),
                        );
                    }
                }
            }
        }
    }

    pub fn get_tile_data(&mut self, set_pos: Vec2) -> TileData {
        self.maps[0].get_tile((set_pos.x as u32, set_pos.y as u32, 0))
    }

    pub fn set_tile_fill(
        &mut self,
        set_pos: Vec2,
        layer: u32,
        tileset: &Map,
        tileset_pos: Vec2,
    ) {
        // Get the tile data from the tileset
        let tiledata =
            tileset.get_tile((tileset_pos.x as u32, tileset_pos.y as u32, 0));
        if tiledata.id == 0 {
            return;
        }

        // We will only change the tiles that have a similar texture id, and this will be use to check
        let comparedata = self.maps[0]
            .get_tile((set_pos.x as u32, set_pos.y as u32, layer))
            .id;
        if comparedata == tiledata.id {
            return;
        }

        // This will hold the location that need to be paint
        let mut paint_to_map: Vec<Vec2> = Vec::with_capacity(0);

        // Place our starting location on to be paint collection
        paint_to_map.push(set_pos);

        // Loop through our collections of position that requires to be paint
        while let Some(pos) = paint_to_map.pop() {
            // Record change for undo purpose
            let last_texture = self.maps[0]
                .get_tile((pos.x as u32, pos.y as u32, layer))
                .id;
            self.record.push_undo(
                Vec3::new(pos.x, pos.y, layer as f32),
                RecordType::Layer,
                last_texture as i64,
                vec![],
            );

            // Paint the map
            self.maps[0]
                .set_tile((pos.x as u32, pos.y as u32, layer), tiledata);

            // Check direction
            for dir in 0..4 {
                // Get the surrounding map position
                let mut adjust_pos = Vec2::new(0.0, 0.0);
                match dir {
                    1 => {
                        adjust_pos.y = 1.0;
                    } // Up
                    2 => {
                        adjust_pos.x = -1.0;
                    } // Left
                    3 => {
                        adjust_pos.x = 1.0;
                    } // Right
                    _ => {
                        adjust_pos.y = -1.0;
                    } // Down
                }
                let checkpos = pos + adjust_pos;

                if checkpos.x >= 0.0
                    && checkpos.x < 32.0
                    && checkpos.y >= 0.0
                    && checkpos.y < 32.0
                {
                    // Check the map texture id and we make sure that we only change
                    // if they have the same texture id as the starting tile
                    let check_data = self.maps[0]
                        .get_tile((checkpos.x as u32, checkpos.y as u32, layer))
                        .id;
                    if check_data == comparedata {
                        paint_to_map.push(checkpos);
                    }
                }
            }
        }
    }

    pub fn update_map_zone(
        &mut self,
        systems: &mut DrawSetting,
        zone_index: usize,
    ) {
        // Clear all
        self.map_zone.iter_mut().for_each(|zone| {
            systems.gfx.set_color(*zone, Color::rgba(0, 0, 0, 0));
        });
        // Add the selected zone
        for data in self.map_zone_loc[zone_index].pos.iter() {
            let tilenum = get_tile_pos(data.x as i32, data.y as i32);
            systems
                .gfx
                .set_color(self.map_zone[tilenum], get_zone_color(zone_index));
        }
    }

    pub fn add_map_zone(
        &mut self,
        systems: &mut DrawSetting,
        zone_index: usize,
        pos: Vec2,
    ) {
        // Record change for undo purpose
        let does_exist = if self.map_zone_loc[zone_index]
            .pos
            .iter()
            .any(|&check_pos| check_pos == pos)
        {
            1
        } else {
            0
        };
        self.record.push_undo(
            Vec3::new(pos.x, pos.y, zone_index as f32),
            RecordType::Zone,
            does_exist,
            vec![],
        );

        let tilenum = get_tile_pos(pos.x as i32, pos.y as i32);
        systems
            .gfx
            .set_color(self.map_zone[tilenum], get_zone_color(zone_index));
        if !self.map_zone_loc[zone_index]
            .pos
            .iter()
            .any(|&check_pos| check_pos == pos)
        {
            self.map_zone_loc[zone_index].pos.push(pos);
        }
    }

    pub fn set_zone_fill(
        &mut self,
        systems: &mut DrawSetting,
        set_pos: Vec2,
        zone_index: usize,
    ) {
        // Fill only empty area
        if self.map_zone_loc[zone_index]
            .pos
            .iter()
            .any(|&check_pos| check_pos == set_pos)
        {
            return;
        }

        // This will hold the location that need to be paint
        let mut paint_to_map: Vec<Vec2> = Vec::with_capacity(0);

        // Place our starting location on to be paint collection
        paint_to_map.push(set_pos);

        // Loop through our collections of position that requires to be paint
        while let Some(pos) = paint_to_map.pop() {
            // Record change for undo purpose
            let does_exist = if self.map_zone_loc[zone_index]
                .pos
                .iter()
                .any(|&check_pos| check_pos == pos)
            {
                1
            } else {
                0
            };
            self.record.push_undo(
                Vec3::new(pos.x, pos.y, zone_index as f32),
                RecordType::Zone,
                does_exist,
                vec![],
            );

            // Paint the map
            let tilenum = get_tile_pos(pos.x as i32, pos.y as i32);
            systems
                .gfx
                .set_color(self.map_zone[tilenum], get_zone_color(zone_index));
            if !self.map_zone_loc[zone_index]
                .pos
                .iter()
                .any(|&check_pos| check_pos == pos)
            {
                self.map_zone_loc[zone_index].pos.push(pos);
            }

            // Check direction
            for dir in 0..4 {
                // Get the surrounding map position
                let mut adjust_pos = Vec2::new(0.0, 0.0);
                match dir {
                    1 => {
                        adjust_pos.y = 1.0;
                    } // Up
                    2 => {
                        adjust_pos.x = -1.0;
                    } // Left
                    3 => {
                        adjust_pos.x = 1.0;
                    } // Right
                    _ => {
                        adjust_pos.y = -1.0;
                    } // Down
                }
                let checkpos = pos + adjust_pos;

                if checkpos.x >= 0.0
                    && checkpos.x < 32.0
                    && checkpos.y >= 0.0
                    && checkpos.y < 32.0
                {
                    // Check if zone is empty
                    if !self.map_zone_loc[zone_index]
                        .pos
                        .iter()
                        .any(|&check_pos| check_pos == checkpos)
                    {
                        paint_to_map.push(checkpos);
                    }
                }
            }
        }
    }

    pub fn delete_map_zone(
        &mut self,
        systems: &mut DrawSetting,
        zone_index: usize,
        pos: Vec2,
    ) {
        // Record change for undo purpose
        let does_exist = if self.map_zone_loc[zone_index]
            .pos
            .iter()
            .any(|&check_pos| check_pos == pos)
        {
            1
        } else {
            0
        };
        self.record.push_undo(
            Vec3::new(pos.x, pos.y, zone_index as f32),
            RecordType::Zone,
            does_exist,
            vec![],
        );

        let tilenum = get_tile_pos(pos.x as i32, pos.y as i32);
        systems
            .gfx
            .set_color(self.map_zone[tilenum], Color::rgba(0, 0, 0, 0));
        self.map_zone_loc[zone_index]
            .pos
            .retain(|&check_pos| check_pos != pos);
    }

    pub fn hover_selection_preview(
        &mut self,
        systems: &mut DrawSetting,
        set_pos: Vec2,
    ) {
        if self.preview_pos != set_pos && set_pos.x < 32.0 && set_pos.y < 32.0 {
            self.preview_pos = set_pos;
            systems.gfx.set_pos(
                self.selection_preview,
                Vec3::new(
                    self.maps[0].pos.x + set_pos.x * TEXTURE_SIZE as f32,
                    self.maps[0].pos.y + set_pos.y * TEXTURE_SIZE as f32,
                    ORDER_MAP_SELECTION,
                ),
            );
            self.adjust_selection_preview(systems);
        }
    }

    pub fn change_selection_preview_size(
        &mut self,
        systems: &mut DrawSetting,
        size: Vec2,
    ) {
        self.preview_size = size;
        self.adjust_selection_preview(systems);
    }

    pub fn clear_map(&mut self, index: usize) {
        (0..9).for_each(|layer| {
            (0..32).for_each(|x| {
                (0..32).for_each(|y| {
                    self.maps[index]
                        .set_tile((x, y, layer), TileData::default());
                });
            });
        });
    }

    // This function ensure that the selection preview does not show outside the map boundary
    fn adjust_selection_preview(&mut self, systems: &mut DrawSetting) {
        let max_size = Vec2::new(32.0, 32.0);

        let clamped_x =
            (self.preview_pos.x + self.preview_size.x).min(max_size.x);
        let clamped_y =
            (self.preview_pos.y + self.preview_size.y).min(max_size.y);

        let new_size = Vec2::new(
            clamped_x - self.preview_pos.x,
            clamped_y - self.preview_pos.y,
        );

        systems.gfx.set_size(
            self.selection_preview,
            Vec2::new(
                new_size.x * TEXTURE_SIZE as f32,
                new_size.y * TEXTURE_SIZE as f32,
            ),
        );
    }

    pub fn apply_change(&mut self, systems: &mut DrawSetting, is_undo: bool) {
        let record_list = if is_undo {
            &self.record.undo
        } else {
            &self.record.redo
        };
        if record_list.is_empty() {
            return;
        }

        let get_change = if is_undo {
            self.record.get_last_undo()
        } else {
            self.record.get_last_redo()
        };

        if let Some(data) = get_change {
            if is_undo {
                self.record.set_redo_record();
            } else {
                self.record.set_undo_record();
            };

            for (_key, changedata) in data.changes.iter() {
                let pos = Vec3::new(
                    changedata.pos.x,
                    changedata.pos.y,
                    changedata.pos.z,
                );

                match changedata.record_type {
                    RecordType::Layer => {
                        let last_texture = self.maps[0]
                            .get_tile((
                                pos.x as u32,
                                pos.y as u32,
                                pos.z as u32,
                            ))
                            .id;
                        if is_undo {
                            self.record.push_redo(
                                Vec3::new(pos.x, pos.y, pos.z),
                                RecordType::Layer,
                                last_texture as i64,
                                vec![],
                            );
                        } else {
                            self.record.push_undo(
                                Vec3::new(pos.x, pos.y, pos.z),
                                RecordType::Layer,
                                last_texture as i64,
                                vec![],
                            );
                        }

                        let texture_id = changedata.id as u32;
                        self.maps[0].set_tile(
                            (pos.x as u32, pos.y as u32, pos.z as u32),
                            TileData {
                                id: texture_id as usize,
                                color: Color::rgba(255, 255, 255, 255),
                            },
                        );
                    }
                    RecordType::Attribute => {
                        let tilenum = get_tile_pos(pos.x as i32, pos.y as i32);
                        let last_attribute =
                            self.map_attributes[tilenum].attribute.clone();
                        let last_attribute_num =
                            MapAttribute::convert_to_num(&last_attribute);
                        let data = match last_attribute {
                            MapAttribute::Warp(warpdata) => {
                                vec![
                                    InsertTypes::Int(warpdata.map_x as i64),
                                    InsertTypes::Int(warpdata.map_y as i64),
                                    InsertTypes::UInt(warpdata.map_group),
                                    InsertTypes::UInt(warpdata.tile_x as u64),
                                    InsertTypes::UInt(warpdata.tile_y as u64),
                                ]
                            }
                            MapAttribute::Sign(text) => {
                                vec![InsertTypes::Str(text)]
                            }
                            MapAttribute::ItemSpawn(itemdata) => {
                                vec![
                                    InsertTypes::UInt(itemdata.index as u64),
                                    InsertTypes::UInt(itemdata.amount as u64),
                                    InsertTypes::UInt(itemdata.timer),
                                ]
                            }
                            MapAttribute::Shop(index) => {
                                vec![InsertTypes::UInt(index as u64)]
                            }
                            _ => vec![],
                        };
                        if is_undo {
                            self.record.push_redo(
                                Vec3::new(pos.x, pos.y, pos.z),
                                RecordType::Attribute,
                                last_attribute_num as i64,
                                data,
                            );
                        } else {
                            self.record.push_undo(
                                Vec3::new(pos.x, pos.y, pos.z),
                                RecordType::Attribute,
                                last_attribute_num as i64,
                                data,
                            );
                        }

                        let attribute_enum = MapAttribute::convert_to_enum(
                            changedata.id as u32,
                            &changedata.data,
                        );
                        let tilenum = get_tile_pos(pos.x as i32, pos.y as i32);
                        self.map_attributes[tilenum]
                            .set_attribute(systems, attribute_enum);
                    }
                    RecordType::Zone => {
                        // We will use the pos.z for the selected zone index
                        let zone_index = pos.z as usize;

                        let does_exist = if self.map_zone_loc[zone_index]
                            .pos
                            .iter()
                            .any(|&check_pos| {
                                check_pos == Vec2::new(pos.x, pos.y)
                            }) {
                            1
                        } else {
                            0
                        };
                        if is_undo {
                            self.record.push_redo(
                                Vec3::new(pos.x, pos.y, pos.z),
                                RecordType::Zone,
                                does_exist,
                                vec![],
                            );
                        } else {
                            self.record.push_undo(
                                Vec3::new(pos.x, pos.y, pos.z),
                                RecordType::Zone,
                                does_exist,
                                vec![],
                            );
                        }

                        let tilenum = get_tile_pos(pos.x as i32, pos.y as i32);
                        if changedata.id > 0 {
                            // Exist, add zone
                            systems.gfx.set_color(
                                self.map_zone[tilenum],
                                get_zone_color(zone_index),
                            );
                            if !self.map_zone_loc[zone_index].pos.iter().any(
                                |&check_pos| {
                                    check_pos == Vec2::new(pos.x, pos.y)
                                },
                            ) {
                                self.map_zone_loc[zone_index]
                                    .pos
                                    .push(Vec2::new(pos.x, pos.y));
                            }
                        } else {
                            // Does not exist, remove zone
                            systems.gfx.set_color(
                                self.map_zone[tilenum],
                                Color::rgba(0, 0, 0, 0),
                            );
                            self.map_zone_loc[zone_index].pos.retain(
                                |&check_pos| {
                                    check_pos != Vec2::new(pos.x, pos.y)
                                },
                            );
                        }
                    }
                }
            }
            self.record.stop_record();
        }
    }
}

pub fn get_zone_color(zone_index: usize) -> Color {
    match zone_index {
        1 => Color::rgba(200, 40, 40, 140),
        2 => Color::rgba(40, 200, 40, 140),
        3 => Color::rgba(150, 40, 150, 140),
        4 => Color::rgba(40, 150, 150, 140),
        _ => Color::rgba(40, 40, 200, 140),
    }
}

pub fn in_map(screen_pos: Vec2, mapview: &MapView) -> bool {
    screen_pos.x >= mapview.maps[0].pos.x
        && screen_pos.x <= mapview.maps[0].pos.x + (32 * TEXTURE_SIZE) as f32
        && screen_pos.y >= mapview.maps[0].pos.y
        && screen_pos.y <= mapview.maps[0].pos.y + (32 * TEXTURE_SIZE) as f32
}

pub fn get_map_pos(screen_pos: Vec2, mapview: &MapView) -> Vec2 {
    let tile_pos =
        screen_pos - Vec2::new(mapview.maps[0].pos.x, mapview.maps[0].pos.y);
    Vec2::new(
        (tile_pos.x / TEXTURE_SIZE as f32).floor(),
        (tile_pos.y / TEXTURE_SIZE as f32).floor(),
    )
}
