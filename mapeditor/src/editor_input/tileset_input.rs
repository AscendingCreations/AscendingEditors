use crate::editor_input::*;

pub fn tileset_input(
    systems: &mut DrawSetting,
    inputtype: &MouseInputType,
    screen_pos: Vec2,
    gameinput: &mut GameInput,
    gui: &mut Interface,
    tileset: &mut Tileset,
    mapview: &mut MapView,
) {
    match inputtype {
        MouseInputType::LeftDown => {
            if !is_scrollbar_in_hold(gui) {
                // Check if mouse position is pointing to our tileset
                if in_tileset(screen_pos, tileset)
                    && gui.current_tab == TAB_LAYER
                {
                    // Calculate the tile position on the tileset based on mouse position
                    let tile_map_pos = get_tileset_pos(screen_pos, tileset)
                        .min(Vec2::new(
                            (MAX_TILE_X - 1) as f32,
                            (MAX_TILE_Y - 1) as f32,
                        ));
                    gameinput.tileset_start = tile_map_pos;
                    gameinput.tileset_end = tile_map_pos;
                    gameinput.selected_size = tileset.set_selection(
                        systems,
                        gameinput.tileset_start,
                        gameinput.tileset_end,
                    );
                    mapview.change_selection_preview_size(
                        systems,
                        gameinput.selected_size,
                    );
                    gameinput.presstype = PressType::Tileset;
                }

                // Tileset List
                if gui.tileset_list.select_list(systems, screen_pos) {
                    // This will process the switching of tileset
                    let tileset_index = gui.tileset_list.selected_tileset;
                    systems.gfx.set_text(
                        &mut systems.renderer,
                        gui.labels[LABEL_TILESET],
                        &systems.resource.tilesheet[tileset_index].name,
                    );
                    tileset.change_tileset(&systems.resource, tileset_index);
                    gui.tileset_list.hide(systems);
                }
            }
        }
        MouseInputType::LeftDownMove => {
            if !is_scrollbar_in_hold(gui) {
                // Check if mouse position is pointing to our tileset
                if in_tileset(screen_pos, tileset)
                    && gameinput.presstype == PressType::Tileset
                {
                    // Calculate the tile position on the tileset based on mouse position
                    let tile_map_pos = get_tileset_pos(screen_pos, tileset)
                        .min(Vec2::new(
                            (MAX_TILE_X - 1) as f32,
                            (MAX_TILE_Y - 1) as f32,
                        ));
                    if gameinput.tileset_end != tile_map_pos {
                        gameinput.tileset_end = tile_map_pos;
                        gameinput.selected_size = tileset.set_selection(
                            systems,
                            gameinput.tileset_start,
                            gameinput.tileset_end,
                        );
                        mapview.change_selection_preview_size(
                            systems,
                            gameinput.selected_size,
                        );
                    }
                }
            } else if gui.tileset_list.scrollbar.in_hold {
                // Update our tileset list based on the scrollbar value
                gui.tileset_list.scrollbar.move_scrollbar(
                    systems,
                    screen_pos.y,
                    false,
                );
                if gui
                    .tileset_list
                    .update_scroll(gui.tileset_list.scrollbar.cur_value)
                {
                    gui.tileset_list.update_list(systems);
                }
                gui.tileset_list.scrollbar.set_hover(systems, screen_pos);
            }
        }
        MouseInputType::Move => {
            gui.tileset_list.hover_selection(systems, screen_pos);
            gui.tileset_list.scrollbar.set_hover(systems, screen_pos);
        }
        MouseInputType::Release => {
            gui.tileset_list.scrollbar.release_scrollbar(systems);
        }
    }
}
