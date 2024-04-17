use graphics::*;
use crate::{
    collection::*,
    resource::*,
    ConfigData,
    DrawSetting
};

pub const MAX_TILE_X: u32 = 10;
pub const MAX_TILE_Y: u32 = 20;

pub struct Tileset {
    pub map: Map,
    pub selected_tile: usize,
    pub selection: usize,
    pub select_start: Vec2,
    pub select_size: Vec2,
}

impl Tileset {
    pub fn new(
        systems: &mut DrawSetting,
        config_data: &mut ConfigData,
    ) -> Self {
        let mut map = Map::new(&mut systems.renderer, TEXTURE_SIZE);

        // Loop throughout all texture and place them on the map based on their texture location
        for tiledata in &systems.resource.tilesheet[0].tile.tiles
        {
            let (id, x, y) = (
                tiledata.tex_id,
                tiledata.x / TEXTURE_SIZE,
                (MAX_TILE_Y - (tiledata.y / TEXTURE_SIZE) - 1),
            );
            // We make sure that we only set those that are not empty tile
            if id > 0 {
                map.set_tile(
                    (x, y, 0),
                    TileData {
                        id,
                        color: Color::rgba(255, 255, 255, 255),
                    },
                );
            }
        }
        // Adjust tileset position on interface
        map.pos = Vec2::new(11.0, 369.0);
        map.can_render = true;

        // Setup tile selection image settings
        // We set the selected tile at the very first tile
        let mut tileselection = Rect::new(&mut systems.renderer, 0);
        tileselection.set_position(Vec3::new(map.pos.x,
                                                map.pos.y + ((MAX_TILE_Y - 1) * TEXTURE_SIZE) as f32,
                                                ORDER_TILESET_SELECTION))
                            .set_size(Vec2::new(TEXTURE_SIZE as f32, TEXTURE_SIZE as f32))
                            .set_color(Color::rgba(config_data.tile_selection_color[0], 
                                                    config_data.tile_selection_color[1], 
                                                    config_data.tile_selection_color[2], 150))
                            .set_use_camera(true);

        Tileset {
            map,
            selected_tile: 0,
            selection: systems.gfx.add_rect(tileselection, 0),
            select_start: Vec2::new(0.0, (MAX_TILE_Y - 1) as f32),
            select_size: Vec2::new(1.0, 1.0),
        }
    }

    pub fn set_selection(&mut self, systems: &mut DrawSetting, start: Vec2, end: Vec2) -> Vec2 {
        // Let's arrange the start pos and end pos to make sure start pos consist the smallest value
        let start_pos = Vec2::new(
            if start.x > end.x { end.x } else { start.x },
            if start.y > end.y { end.y } else { start.y },
        );
        let end_pos = Vec2::new(
            if start.x > end.x { start.x } else { end.x },
            if start.y > end.y { start.y } else { end.y },
        );

        // Set data that will be use when placing tile on map
        self.select_start = start_pos;
        self.select_size = (end_pos - start_pos) + 1.0;

        // Adjust selection position and size
        systems.gfx.set_pos(self.selection, 
            Vec3::new(
                self.map.pos.x + (start_pos.x * TEXTURE_SIZE as f32),
                self.map.pos.y + (start_pos.y * TEXTURE_SIZE as f32),
                4.0
            ));
        systems.gfx.set_size(self.selection, 
            self.select_size * TEXTURE_SIZE as f32);

        self.select_size
    }

    pub fn change_tileset(
        &mut self,
        resource: &TextureAllocation,
        tileset_index: usize,
    ) {
        if self.selected_tile == tileset_index {
            return;
        }
        self.selected_tile = tileset_index;

        // Clear Tileset
        (0..MAX_TILE_X).for_each(|x| {
            (0..MAX_TILE_Y).for_each(|y| {
                self.map.set_tile((x, y, 0), TileData::default());
            });
        });

        // Loop throughout all texture and place them on the map based on their texture location
        for tiledata in &resource.tilesheet[tileset_index].tile.tiles {
            let (id, x, y) = (
                tiledata.tex_id,
                tiledata.x / TEXTURE_SIZE,
                (MAX_TILE_Y - (tiledata.y / TEXTURE_SIZE) - 1),
            );
            // We make sure that we only set those that are not empty tile
            if id > 0 {
                self.map.set_tile(
                    (x, y, 0),
                    TileData {
                        id,
                        color: Color::rgba(255, 255, 255, 255),
                    },
                );
            }
        }
    }
}

// Tileset //
pub fn in_tileset(screen_pos: Vec2, tileset: &Tileset) -> bool {
    screen_pos.x >= tileset.map.pos.x
        && screen_pos.x
            <= tileset.map.pos.x + (MAX_TILE_X * TEXTURE_SIZE) as f32
        && screen_pos.y >= tileset.map.pos.y
        && screen_pos.y
            <= tileset.map.pos.y + (MAX_TILE_Y * TEXTURE_SIZE) as f32
}

pub fn get_tileset_pos(screen_pos: Vec2, tileset: &Tileset) -> Vec2 {
    let tile_pos = screen_pos - Vec2::new(tileset.map.pos.x, tileset.map.pos.y);
    Vec2::new(
        (tile_pos.x / TEXTURE_SIZE as f32).floor(),
        (tile_pos.y / TEXTURE_SIZE as f32).floor(),
    )
}