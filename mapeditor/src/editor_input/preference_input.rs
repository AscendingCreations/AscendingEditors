use crate::editor_input::*;

const GENERAL_HIDEFPS: usize = 0;
const GENERAL_HIDETILEBG: usize = 1;
const GENERAL_HIDEMAPBG: usize = 2;
const GENERAL_MAPCOLOR: usize = 3;
const GENERAL_TILECOLOR: usize = 4;

pub fn preference_input(
    systems: &mut DrawSetting,
    inputtype: &MouseInputType,
    screen_pos: Vec2,
    gui: &mut Interface,
    tileset: &mut Tileset,
    mapview: &mut MapView,
    config_data: &mut ConfigData,
) {
    // If preference is open, cancel all other inputs
    if !gui.preference.is_open {
        return;
    }

    match inputtype {
        MouseInputType::LeftDown => {
            if gui.preference.keywindow.is_open {
                let click_button =
                    gui.preference.keywindow.click_buttons(systems, screen_pos);
                if let Some(index) = click_button {
                    match index {
                        0 => gui.preference.keywindow.close_key(systems), // Cancel
                        _ => {
                            if let Some(keycode) =
                                &gui.preference.keywindow.key_code
                            {
                                let index = gui.preference.keywindow.key_index;
                                config_data.key_code[index] = keycode.clone();
                                config_data.key_code_modifier[index] =
                                    gui.preference.keywindow.key_modifier;
                                gui.preference.update_key_list(
                                    systems,
                                    index,
                                    config_data,
                                );
                            }
                            gui.preference.keywindow.close_key(systems)
                        } // Save
                    }
                }
                return;
            }

            if gui.preference.scrollbar.in_scrollbar(systems, screen_pos) {
                gui.preference
                    .scrollbar
                    .hold_scrollbar(systems, screen_pos.y);
            }

            if !gui.preference.scrollbar.in_hold {
                let click_button =
                    gui.preference.click_buttons(systems, screen_pos);
                if let Some(index) = click_button {
                    match index {
                        0 => {
                            config_data.set_data(load_config());
                            gui.preference.close(systems);
                        } // Cancel
                        1 => {
                            gui.preference
                                .reset_preference(systems, config_data);
                        } // Reset
                        _ => {
                            config_data.save_config().unwrap();
                            // Apply settings
                            systems.gfx.set_color(
                                mapview.selection_preview,
                                Color::rgba(
                                    config_data.map_selection_color[0],
                                    config_data.map_selection_color[1],
                                    config_data.map_selection_color[2],
                                    150,
                                ),
                            );
                            systems.gfx.set_color(
                                tileset.selection,
                                Color::rgba(
                                    config_data.tile_selection_color[0],
                                    config_data.tile_selection_color[1],
                                    config_data.tile_selection_color[2],
                                    150,
                                ),
                            );
                            gui.preference.close(systems);
                        } // Save
                    }
                }

                if gui.preference.select_menu_button(systems, screen_pos) {
                    open_preference_tab(
                        &mut gui.preference,
                        systems,
                        config_data,
                    );
                }

                match gui.preference.selected_menu {
                    PREF_TAB_GENERAL => {
                        if let Some(index) = gui.preference.is_coloreditor_open
                        {
                            if gui
                                .preference
                                .in_color_selection(systems, screen_pos)
                            {
                                gui.preference.select_text(systems, screen_pos);
                                if gui.preference.click_color_selection_button(
                                    systems, screen_pos,
                                ) {
                                    if let SettingData::ColorSelection(
                                        colorselection,
                                    ) =
                                        &mut gui.preference.setting_data[index]
                                    {
                                        let data =
                                            colorselection.color_editor.data;
                                        systems.gfx.set_color(
                                            colorselection.image,
                                            Color::rgba(
                                                data[0], data[1], data[2],
                                                data[3],
                                            ),
                                        );
                                        match index {
                                            GENERAL_MAPCOLOR => {
                                                config_data
                                                    .map_selection_color = data
                                            }
                                            GENERAL_TILECOLOR => {
                                                config_data
                                                    .tile_selection_color = data
                                            }
                                            _ => {}
                                        }
                                    }
                                    gui.preference
                                        .hide_color_selection(systems);
                                }
                                return;
                            }
                        }

                        if let Some(config_index) =
                            gui.preference.select_config(systems, screen_pos)
                        {
                            match &mut gui.preference.setting_data[config_index]
                            {
                                SettingData::Checkbox(checkbox) => {
                                    if checkbox.is_select {
                                        checkbox.set_select(systems, false);
                                    } else {
                                        checkbox.set_select(systems, true);
                                    }
                                    match config_index {
                                        GENERAL_HIDEFPS => {
                                            config_data.hide_fps =
                                                checkbox.is_select;
                                            systems.gfx.set_visible(
                                                gui.labels[LABEL_FPS],
                                                !checkbox.is_select,
                                            );
                                        }
                                        GENERAL_HIDETILEBG => {
                                            config_data.hide_tileset_bg =
                                                checkbox.is_select;
                                            systems.gfx.set_visible(
                                                gui.bg_layout[2],
                                                !checkbox.is_select,
                                            );
                                        }
                                        GENERAL_HIDEMAPBG => {
                                            config_data.hide_mapview_bg =
                                                checkbox.is_select;
                                            systems.gfx.set_visible(
                                                gui.bg_layout[1],
                                                !checkbox.is_select,
                                            );
                                        }
                                        _ => {}
                                    }
                                    gui.preference
                                        .hide_color_selection(systems);
                                }
                                SettingData::ColorSelection(colorselection) => {
                                    if gui
                                        .preference
                                        .is_coloreditor_open
                                        .is_none()
                                    {
                                        colorselection
                                            .open_color_editor(systems);
                                        gui.preference.is_coloreditor_open =
                                            Some(config_index);
                                    } else {
                                        gui.preference
                                            .hide_color_selection(systems);
                                    }
                                }
                                _ => {}
                            }
                        } else {
                            gui.preference.hide_color_selection(systems);
                        }
                    }
                    PREF_TAB_KEYBIND => {
                        if let Some(key_index) =
                            gui.preference.select_keylist(systems, screen_pos)
                        {
                            gui.preference
                                .keywindow
                                .open_key(systems, key_index);
                        }
                    }
                    _ => {}
                }
            }
        }
        MouseInputType::LeftDownMove => {
            gui.preference.scrollbar.move_scrollbar(
                systems,
                screen_pos.y,
                false,
            );
            if gui
                .preference
                .update_scroll(gui.preference.scrollbar.cur_value)
            {
                gui.preference.update_list();
            }
            gui.preference.scrollbar.set_hover(systems, screen_pos);
        }
        MouseInputType::Move => {
            gui.preference.hover_buttons(systems, screen_pos);
            gui.preference.scrollbar.set_hover(systems, screen_pos);
            if gui.preference.keywindow.is_open {
                gui.preference.keywindow.hover_buttons(systems, screen_pos);
            }
        }
        MouseInputType::Release => {
            gui.preference.release_click(systems);
            gui.preference.keywindow.release_click(systems);
            gui.preference.scrollbar.release_scrollbar(systems);
        }
    }
}

pub fn preference_key_input(
    systems: &mut DrawSetting,
    event: &KeyEvent,
    gui: &mut Interface,
) {
    if !event.state.is_pressed() {
        return;
    }

    match gui.preference.selected_menu {
        PREF_TAB_KEYBIND => {
            if gui.preference.keywindow.is_open {
                gui.preference.keywindow.edit_key(event, systems);
            }
        }
        PREF_TAB_GENERAL => {
            if let Some(index) = gui.preference.is_coloreditor_open {
                if let SettingData::ColorSelection(colorselection) =
                    &mut gui.preference.setting_data[index]
                {
                    if colorselection.color_editor.is_open {
                        colorselection.color_editor.textbox
                            [gui.preference.editing_index]
                            .enter_numeric(systems, event, 3, false);

                        let value = colorselection.color_editor.textbox
                            [gui.preference.editing_index]
                            .data
                            .parse::<i64>()
                            .unwrap_or_default();
                        colorselection.color_editor.data
                            [gui.preference.editing_index] =
                            (value as u8).min(255);
                    }
                }
            }
        }
        _ => {}
    }
}
