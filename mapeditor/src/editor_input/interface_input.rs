use crate::{collection::*, editor_input::*, gfx_collection::*};

#[allow(clippy::too_many_arguments)]
pub fn interface_input(
    systems: &mut DrawSetting,
    inputtype: &MouseInputType,
    screen_pos: Vec2,
    gameinput: &mut GameInput,
    gui: &mut Interface,
    tileset: &mut Tileset,
    mapview: &mut MapView,
    database: &mut EditorData,
    config_data: &mut ConfigData,
) {
    match inputtype {
        MouseInputType::LeftDown => {
            if gui.tileset_list.scrollbar.in_scrollbar(systems, screen_pos) {
                gui.tileset_list
                    .scrollbar
                    .hold_scrollbar(systems, screen_pos.y);
            } else if gui.scrollbar.in_scrollbar(systems, screen_pos) {
                gui.scrollbar.hold_scrollbar(systems, screen_pos.y);
            } else if gui.current_tab == TAB_PROPERTIES
                && gui.selected_dropbox >= 0
                && gui.editor_selectionbox[gui.selected_dropbox as usize]
                    .scrollbar
                    .in_scrollbar(systems, screen_pos)
            {
                gui.editor_selectionbox[gui.selected_dropbox as usize]
                    .scrollbar
                    .hold_scrollbar(systems, screen_pos.y);
            }

            if !is_scrollbar_in_hold(gui) {
                // Tools
                let click_button = gui.click_tool_button(systems, screen_pos);
                if let Some(button_index) = click_button {
                    gui_button_select(
                        button_index,
                        systems,
                        gameinput,
                        gui,
                        tileset,
                        mapview,
                        database,
                        config_data,
                    );
                }

                // Tab Options
                let click_tab_option = gui.click_tab_option(systems, screen_pos);
                if let Some(tab_index) = click_tab_option {
                    gui.select_tab_option(systems, tab_index);
                    // Open
                    match gui.current_tab {
                        TAB_ATTRIBUTE => {
                            open_attribute_settings(systems, gui, gui.current_tab_data + 1, vec![])
                        }
                        TAB_ZONE => {
                            mapview.update_map_zone(systems, gui.current_tab_data as usize);
                            gui.open_zone_settings(systems, mapview);
                        }
                        _ => {}
                    }
                }

                // Textbox / Buttons
                match gui.current_tab {
                    TAB_ATTRIBUTE | TAB_ZONE => gui.select_textbox(systems, screen_pos),
                    TAB_PROPERTIES => {
                        // Buttons
                        let click_button = gui.click_buttons(systems, screen_pos);
                        if let Some(button_index) = click_button {
                            match button_index {
                                0 => database.save_all_maps(mapview, config_data),
                                1 => {
                                    database.reset_all_map();
                                    database.load_map_data(systems, mapview);
                                    database.load_link_maps(mapview);
                                    update_map_name(systems, gui, database);
                                }
                                2 => {
                                    gui.preference.open(systems);
                                    open_preference_tab(&mut gui.preference, systems, config_data);
                                }
                                _ => {}
                            }
                        }

                        // Selection box
                        let click_button = gui.click_selectionbox(systems, screen_pos);
                        if let Some(selection_index) = click_button {
                            if matches!(selection_index, 0 | 1) {
                                if !gui.editor_selectionbox[selection_index].is_list_visible {
                                    gui.editor_selectionbox[selection_index].show_list(systems);
                                    gui.selected_dropbox = selection_index as i32;
                                } else {
                                    gui.editor_selectionbox[selection_index].hide_list(systems);
                                    gui.selected_dropbox = -1;
                                }
                            } // Weather & Music
                        }

                        // Dropdown List
                        if gui.selected_dropbox >= 0 {
                            let click_button = gui.editor_selectionbox
                                [gui.selected_dropbox as usize]
                                .click_list(systems, screen_pos);
                            if let Some(selection_index) = click_button {
                                gui.editor_selectionbox[gui.selected_dropbox as usize]
                                    .switch_list(systems, selection_index);

                                match gui.selected_dropbox {
                                    0 => {
                                        mapview.fixed_weather = gui.editor_selectionbox
                                            [gui.selected_dropbox as usize]
                                            .selected_index
                                            as u8;
                                        database.set_map_change(mapview);
                                        update_map_name(systems, gui, database);
                                    }
                                    1 => {
                                        let index = gui.editor_selectionbox
                                            [gui.selected_dropbox as usize]
                                            .selected_index;
                                        if index == 0 {
                                            mapview.music = None;
                                        } else {
                                            let list_name = gui.editor_selectionbox
                                                [gui.selected_dropbox as usize]
                                                .list[index]
                                                .clone();
                                            mapview.music = Some(list_name);
                                            database.set_map_change(mapview);
                                            update_map_name(systems, gui, database);
                                        }
                                    }
                                    _ => {}
                                }

                                gui.editor_selectionbox[gui.selected_dropbox as usize]
                                    .hide_list(systems);
                            }
                        }

                        click_dir_block(systems, gui, screen_pos);
                    }
                    _ => {}
                }
            }
        }
        MouseInputType::LeftDownMove => {
            if gui.scrollbar.in_hold {
                gui.scrollbar.move_scrollbar(systems, screen_pos.y, false);
                gui.update_scroll(systems, gui.scrollbar.cur_value);
                gui.scrollbar.set_hover(systems, screen_pos);
            } else if gui.current_tab == TAB_PROPERTIES
                && gui.selected_dropbox >= 0
                && gui.editor_selectionbox[gui.selected_dropbox as usize]
                    .scrollbar
                    .in_hold
            {
                gui.editor_selectionbox[gui.selected_dropbox as usize]
                    .scrollbar
                    .move_scrollbar(systems, screen_pos.y, false);
                let scrollbar_value = gui.editor_selectionbox[gui.selected_dropbox as usize]
                    .scrollbar
                    .cur_value;
                gui.editor_selectionbox[gui.selected_dropbox as usize]
                    .update_list(systems, scrollbar_value);
                gui.editor_selectionbox[gui.selected_dropbox as usize]
                    .scrollbar
                    .set_hover(systems, screen_pos);
            }
        }
        MouseInputType::Move => {
            gui.hover_tool_button(systems, screen_pos);
            gui.hover_buttons(systems, screen_pos);
            gui.hover_selectionbox(systems, screen_pos);
            gui.hover_tab_option(systems, screen_pos);
            gui.scrollbar.set_hover(systems, screen_pos);
            if gui.current_tab == TAB_PROPERTIES && gui.selected_dropbox >= 0 {
                gui.editor_selectionbox[gui.selected_dropbox as usize]
                    .hover_list(systems, screen_pos);
                gui.editor_selectionbox[gui.selected_dropbox as usize]
                    .scrollbar
                    .set_hover(systems, screen_pos);
            }
        }
        MouseInputType::Release => {
            gui.reset_tool_button_click(systems);
            gui.release_click(systems);
            gui.release_selectionbox_click(systems);
            gui.scrollbar.release_scrollbar(systems);
            if gui.current_tab == TAB_PROPERTIES && gui.selected_dropbox >= 0 {
                gui.editor_selectionbox[gui.selected_dropbox as usize]
                    .scrollbar
                    .release_scrollbar(systems);
            }
        }
    }
}

pub fn interface_key_input(
    event: &KeyEvent,
    gui: &mut Interface,
    mapview: &mut MapView,
    database: &mut EditorData,
    systems: &mut DrawSetting,
) -> bool {
    let mut result = false;
    match gui.current_tab {
        TAB_ATTRIBUTE => {
            let attribute = MapAttribute::convert_to_plain_enum(gui.current_tab_data + 1);
            match attribute {
                MapAttribute::Warp(_) => {
                    if gui.selected_textbox >= 0 {
                        if gui.selected_textbox < 2 {
                            gui.editor_textbox[gui.selected_textbox as usize]
                                .enter_numeric(systems, event, 5, true);
                        } else {
                            gui.editor_textbox[gui.selected_textbox as usize]
                                .enter_numeric(systems, event, 5, false);
                        }
                        result = true;
                    }
                }
                MapAttribute::Sign(_) => {
                    if gui.selected_textbox >= 0 {
                        gui.editor_textbox[gui.selected_textbox as usize]
                            .enter_text(systems, event, 100);
                        result = true;
                    }
                }
                MapAttribute::ItemSpawn(_) => {
                    if gui.selected_textbox >= 0 {
                        gui.editor_textbox[gui.selected_textbox as usize]
                            .enter_numeric(systems, event, 6, false);
                        result = true;
                    }
                }
                MapAttribute::Shop(_) => {
                    if gui.selected_textbox >= 0 {
                        gui.editor_textbox[gui.selected_textbox as usize]
                            .enter_numeric(systems, event, 6, false);
                        result = true;
                    }
                }
                _ => {}
            }
        }
        TAB_ZONE => {
            if gui.selected_textbox >= 0 {
                gui.editor_textbox[gui.selected_textbox as usize]
                    .enter_numeric(systems, event, 5, false);
                match gui.selected_textbox {
                    0 => {
                        let value = gui.editor_textbox[gui.selected_textbox as usize]
                            .data
                            .parse::<i64>()
                            .unwrap_or_default();
                        mapview.map_zone_setting[gui.current_tab_data as usize].max_npc =
                            value as u64
                    } // Max NPC
                    _ => {
                        if !gui.editor_textbox[gui.selected_textbox as usize]
                            .data
                            .is_empty()
                        {
                            let value = gui.editor_textbox[gui.selected_textbox as usize]
                                .data
                                .parse::<i64>()
                                .unwrap_or_default();
                            mapview.map_zone_setting[gui.current_tab_data as usize].npc_id
                                [(gui.selected_textbox - 1) as usize] = Some(value as u64);
                        } else {
                            mapview.map_zone_setting[gui.current_tab_data as usize].npc_id
                                [(gui.selected_textbox - 1) as usize] = None;
                        }
                    } // Npc ID
                }
                database.set_map_change(mapview);
                update_map_name(systems, gui, database);
                result = true;
            }
        }
        _ => {}
    }
    result
}

// This function help us switch the map setting tab that the editor is using
pub fn set_tab(
    systems: &mut DrawSetting,
    gui: &mut Interface,
    tab_index: usize,
    mapview: &mut MapView,
    tileset: &mut Tileset,
    gameinput: &mut GameInput,
) {
    if gui.current_tab != tab_index {
        // Set tab data to default
        for index in 0..MAX_TAB_LABEL {
            gui.tab_labels[index].close(systems);
            gui.tab_labels[index].set_select(systems, false);
        }
        gui.scrollbar.hide(systems);
        gui.current_tab_data = 0;
        gui.current_selected_area = 0;

        // Switch selected tab
        gui.buttons[gui.current_tab].set_state(systems, ButtonState::Normal);
        gui.buttons[tab_index].set_state(systems, ButtonState::Selected);
        gui.current_tab = tab_index;

        systems.gfx.set_visible(gui.labels[LABEL_TILESET], false);
        systems
            .gfx
            .set_visible(gui.labels[LABEL_OPT_HEADER_TEXT], false);
        systems.gfx.set_visible(tileset.selection, false);

        mapview.map_zone.iter_mut().for_each(|zone| {
            systems.gfx.set_visible(*zone, false);
        });

        mapview.map_attributes.iter_mut().for_each(|attribute| {
            systems.gfx.set_visible(attribute.text, false);
            systems.gfx.set_visible(attribute.image, false);
        });

        mapview.map_dir_block.iter_mut().for_each(|dir_block| {
            systems.gfx.set_visible(dir_block.bg, false);
            for i in 0..4 {
                systems.gfx.set_visible(dir_block.dir[i], false);
            }
        });

        systems.gfx.set_visible(gui.scrollbar_bg, false);

        if !gui.editor_label.is_empty() {
            gui.editor_label.iter().for_each(|label| {
                systems.gfx.remove_gfx(*label);
            });
        }
        gui.editor_label.clear();

        if !gui.editor_rect.is_empty() {
            gui.editor_rect.iter().for_each(|rect| {
                systems.gfx.remove_gfx(*rect);
            });
        }
        gui.editor_rect.clear();
        gui.dir_select = [false; 4];

        gui.editor_selectionbox
            .iter_mut()
            .for_each(|selection_box| {
                selection_box.scrollbar.unload(systems);
                systems.gfx.remove_gfx(selection_box.button);
                systems.gfx.remove_gfx(selection_box.text);
                systems.gfx.remove_gfx(selection_box.rect[0]);
                systems.gfx.remove_gfx(selection_box.rect[1]);
                selection_box.list_text.iter_mut().for_each(|list| {
                    systems.gfx.remove_gfx(list.rect);
                    systems.gfx.remove_gfx(list.text);
                });
            });
        gui.editor_selectionbox = vec![];

        gui.editor_textbox.iter_mut().for_each(|textbox| {
            systems.gfx.remove_gfx(textbox.image);
            systems.gfx.remove_gfx(textbox.text);
        });
        gui.editor_textbox = vec![];

        gui.editor_button.iter_mut().for_each(|button| {
            systems.gfx.remove_gfx(button.image);
            systems.gfx.remove_gfx(button.text);
        });
        gui.editor_button = vec![];

        systems.gfx.set_visible(gui.tab_opt_bg[0], false);
        systems.gfx.set_visible(gui.tab_opt_bg[1], false);

        // Load tab data
        match gui.current_tab {
            TAB_LAYER => {
                gui.tab_labels
                    .iter_mut()
                    .zip(MapLayers::LAYERS)
                    .for_each(|(tab_labels, layer)| {
                        tab_labels.init(systems, layer.as_str(), 194.0)
                    });

                gui.tab_labels[0].set_select(systems, true);

                tileset.map.changed = true;
                systems.gfx.set_visible(tileset.selection, true);

                systems.gfx.set_visible(gui.labels[LABEL_TILESET], true);

                mapview.change_selection_preview_size(systems, gameinput.selected_size);
            }
            TAB_ATTRIBUTE => {
                gui.start_view = 0;
                for index in 0..MAX_TAB_LABEL {
                    let sel_index = gui.start_view + index;
                    if sel_index < MAX_ATTRIBUTE - 1 {
                        gui.tab_labels[index].init(
                            systems,
                            MapAttribute::as_str(sel_index as u32 + 1),
                            180.0,
                        );
                    }
                }
                gui.tab_labels[0].set_select(systems, true);

                mapview.map_attributes.iter_mut().for_each(|attribute| {
                    systems.gfx.set_visible(attribute.text, true);
                    systems.gfx.set_visible(attribute.image, true);
                });

                reset_scrollbar(systems, &mut gui.scrollbar);
                systems.gfx.set_visible(gui.scrollbar_bg, true);
                gui.scrollbar.show(systems);

                systems.gfx.set_visible(gui.tab_opt_bg[0], true);
                systems.gfx.set_visible(gui.tab_opt_bg[1], true);

                systems
                    .gfx
                    .set_visible(gui.labels[LABEL_OPT_HEADER_TEXT], true);
                systems.gfx.set_text(
                    &mut systems.renderer,
                    gui.labels[LABEL_OPT_HEADER_TEXT],
                    "Attribute Properties",
                );
                systems.gfx.center_text(gui.labels[LABEL_OPT_HEADER_TEXT]);

                mapview.change_selection_preview_size(systems, Vec2::new(1.0, 1.0));
            }
            TAB_ZONE => {
                for index in 0..MAX_TAB_LABEL {
                    if index < 5 {
                        gui.tab_labels[index].init(systems, &format!("Zone {}", index + 1), 194.0);
                    }
                }
                gui.tab_labels[0].set_select(systems, true);

                mapview.map_zone.iter_mut().for_each(|zone| {
                    systems.gfx.set_visible(*zone, true);
                });

                systems.gfx.set_visible(gui.tab_opt_bg[0], true);
                systems.gfx.set_visible(gui.tab_opt_bg[1], true);

                systems
                    .gfx
                    .set_visible(gui.labels[LABEL_OPT_HEADER_TEXT], true);
                systems.gfx.set_text(
                    &mut systems.renderer,
                    gui.labels[LABEL_OPT_HEADER_TEXT],
                    "Zone Settings",
                );
                systems.gfx.center_text(gui.labels[LABEL_OPT_HEADER_TEXT]);

                mapview.update_map_zone(systems, 0);

                mapview.change_selection_preview_size(systems, Vec2::new(1.0, 1.0));

                let text_start_pos = systems.gfx.get_pos(gui.tab_opt_bg[0]);
                for i in 0..7 {
                    let mut ajdust_pos = Vec2::new(text_start_pos.x, text_start_pos.y);
                    let msg = match i {
                        1 => {
                            ajdust_pos += Vec2::new(10.0, 338.0);
                            "NPC ID".to_string()
                        }
                        2..=6 => {
                            ajdust_pos += Vec2::new(10.0, 312.0 - ((i - 2) * 23) as f32);
                            format!("{}", i - 1)
                        }
                        _ => {
                            ajdust_pos += Vec2::new(10.0, 368.0);
                            "Max NPC".to_string()
                        }
                    };
                    let mut text = create_basic_label(
                        systems,
                        Vec3::new(ajdust_pos.x, ajdust_pos.y, ORDER_ATTRIBUTE_LABEL),
                        Vec2::new(100.0, 20.0),
                        Color::rgba(180, 180, 180, 255),
                    );
                    text.set_text(&mut systems.renderer, &msg, Attrs::new(), Shaping::Advanced);
                    gui.editor_label.push(systems.gfx.add_text(text, 1));

                    if i != 1 {
                        let add_pos = match i {
                            0 => 85.0,
                            _ => 35.0,
                        };
                        let text_box = Textbox::new(
                            systems,
                            Vec3::new(
                                ajdust_pos.x + add_pos,
                                ajdust_pos.y,
                                ORDER_ATTRIBUTE_TEXTBOX,
                            ),
                            Vec2::new(60.0, 22.0),
                            false,
                            [0, 1],
                        );
                        gui.editor_textbox.push(text_box);
                    }
                }
                gui.editor_textbox[0]
                    .input_text(systems, mapview.map_zone_setting[0].max_npc.to_string()); // Max Npc
                for i in 0..5 {
                    if mapview.map_zone_setting[0].npc_id[i].is_some() {
                        gui.editor_textbox[i + 1].input_text(
                            systems,
                            mapview.map_zone_setting[0].npc_id[i].unwrap().to_string(),
                        );
                    }
                }
            }
            TAB_PROPERTIES => {
                systems.gfx.set_visible(gui.tab_opt_bg[0], true);

                mapview.map_dir_block.iter_mut().for_each(|dir_block| {
                    systems.gfx.set_visible(dir_block.bg, true);
                    for i in 0..4 {
                        systems.gfx.set_visible(dir_block.dir[i], true);
                    }
                    dir_block.update(systems);
                });

                let pos = systems.gfx.get_pos(gui.tab_opt_bg[0]);
                gui.editor_button = vec![
                    Button::new(
                        systems,
                        systems.resource.option_button.allocation,
                        "Save All Map",
                        Vec2::new(pos.x + 14.0, pos.y + 372.0),
                        Vec2::new(172.0, 36.0),
                        [ORDER_OPTION_BUTTON, ORDER_OPTION_BUTTON_TEXT],
                        8.0,
                        [0, 1],
                    ),
                    Button::new(
                        systems,
                        systems.resource.option_button.allocation,
                        "Reset All Map",
                        Vec2::new(pos.x + 14.0, pos.y + 332.0),
                        Vec2::new(172.0, 36.0),
                        [ORDER_OPTION_BUTTON, ORDER_OPTION_BUTTON_TEXT],
                        8.0,
                        [0, 1],
                    ),
                    Button::new(
                        systems,
                        systems.resource.option_button.allocation,
                        "Preference",
                        Vec2::new(pos.x + 14.0, pos.y + 292.0),
                        Vec2::new(172.0, 36.0),
                        [ORDER_OPTION_BUTTON, ORDER_OPTION_BUTTON_TEXT],
                        8.0,
                        [0, 1],
                    ),
                ];

                let content_pos = Vec2::new(25.0, 295.0);
                let mut text = create_basic_label(
                    systems,
                    Vec3::new(content_pos.x, content_pos.y, ORDER_ATTRIBUTE_LABEL),
                    Vec2::new(100.0, 20.0),
                    Color::rgba(180, 180, 180, 255),
                );
                text.set_text(
                    &mut systems.renderer,
                    "Weather",
                    Attrs::new(),
                    Shaping::Advanced,
                );
                gui.editor_label.push(systems.gfx.add_text(text, 1));

                let mut selectionbox = SelectionBox::new(
                    systems,
                    Vec2::new(content_pos.x, content_pos.y - 24.0),
                    [
                        ORDER_PROPERTIES_BUTTON,
                        ORDER_PROPERTIES_BUTTON_TEXT,
                        ORDER_DROPDOWN_WINDOW,
                        ORDER_DROPDOWN_SELECTION,
                        ORDER_DROPDOWN_TEXT,
                        ORDER_DROPDOWN_SCROLLBAR,
                    ],
                    168.0,
                    vec!["None".to_string(), "Rain".to_string(), "Snow".to_string()],
                    0,
                );
                selectionbox.switch_list(systems, mapview.fixed_weather as usize);
                gui.editor_selectionbox.push(selectionbox);

                let mut audio_list = systems.audio_list.audio.clone();
                audio_list.insert(0, "None".to_string());

                let mut text = create_basic_label(
                    systems,
                    Vec3::new(content_pos.x, content_pos.y - 54.0, ORDER_ATTRIBUTE_LABEL),
                    Vec2::new(100.0, 20.0),
                    Color::rgba(180, 180, 180, 255),
                );
                text.set_text(
                    &mut systems.renderer,
                    "Music",
                    Attrs::new(),
                    Shaping::Advanced,
                );
                gui.editor_label.push(systems.gfx.add_text(text, 1));

                let mut selectionbox = SelectionBox::new(
                    systems,
                    Vec2::new(content_pos.x, content_pos.y - 78.0),
                    [
                        ORDER_PROPERTIES_BUTTON,
                        ORDER_PROPERTIES_BUTTON_TEXT,
                        ORDER_DROPDOWN_WINDOW,
                        ORDER_DROPDOWN_SELECTION,
                        ORDER_DROPDOWN_TEXT,
                        ORDER_DROPDOWN_SCROLLBAR,
                    ],
                    168.0,
                    audio_list.clone(),
                    0,
                );
                if let Some(data) = &mapview.music {
                    if let Some(index) = audio_list.iter().position(|name| *name == *data) {
                        selectionbox.switch_list(systems, index);
                    }
                }
                gui.editor_selectionbox.push(selectionbox);

                let mut text = create_basic_label(
                    systems,
                    Vec3::new(content_pos.x, content_pos.y - 150.0, ORDER_ATTRIBUTE_LABEL),
                    Vec2::new(150.0, 20.0),
                    Color::rgba(180, 180, 180, 255),
                );
                text.set_text(
                    &mut systems.renderer,
                    "Direction Block",
                    Attrs::new(),
                    Shaping::Advanced,
                );
                gui.editor_label.push(systems.gfx.add_text(text, 1));

                for i in 0..4 {
                    let mut bg_rect = Rect::new(&mut systems.renderer, 0);
                    bg_rect
                        .set_size(Vec2::new(32.0, 32.0))
                        .set_border_width(1.0)
                        .set_color(Color::rgba(100, 100, 100, 255))
                        .set_border_color(Color::rgba(40, 40, 40, 255));
                    match i {
                        0 => {
                            bg_rect.set_position(Vec3::new(
                                content_pos.x + 32.0,
                                content_pos.y - 192.0,
                                ORDER_ATTRIBUTE_RECT,
                            ));
                        }
                        1 => {
                            bg_rect.set_position(Vec3::new(
                                content_pos.x,
                                content_pos.y - 224.0,
                                ORDER_ATTRIBUTE_RECT,
                            ));
                        }
                        2 => {
                            bg_rect.set_position(Vec3::new(
                                content_pos.x + 32.0,
                                content_pos.y - 256.0,
                                ORDER_ATTRIBUTE_RECT,
                            ));
                        }
                        _ => {
                            bg_rect.set_position(Vec3::new(
                                content_pos.x + 64.0,
                                content_pos.y - 224.0,
                                ORDER_ATTRIBUTE_RECT,
                            ));
                        }
                    }
                    gui.editor_rect.push(systems.gfx.add_rect(bg_rect, 0));
                }
            }
            _ => {}
        }
    }
}

pub fn open_attribute_settings(
    systems: &mut DrawSetting,
    gui: &mut Interface,
    attribute: u32,
    data: Vec<InsertTypes>,
) {
    let attr = MapAttribute::convert_to_plain_enum(attribute);
    // We will make it default that no textbox is selected
    gui.selected_textbox = -1;
    gui.selected_dropbox = -1;

    if !gui.editor_label.is_empty() {
        gui.editor_label.iter().for_each(|label| {
            systems.gfx.remove_gfx(*label);
        });
    }
    gui.editor_label = vec![];
    if !gui.editor_textbox.is_empty() {
        gui.editor_textbox.iter_mut().for_each(|textbox| {
            systems.gfx.remove_gfx(textbox.image);
            systems.gfx.remove_gfx(textbox.text);
        });
    }
    gui.editor_textbox = vec![];

    match attr {
        MapAttribute::Warp(_) => {
            gui.editor_label = Vec::with_capacity(7);
            for i in 0..7 {
                let mut ajdust_pos = systems.gfx.get_pos(gui.tab_opt_bg[0]);
                let msg = match i {
                    1 => {
                        ajdust_pos += Vec3::new(45.0, 340.0, 0.0);
                        "X"
                    }
                    2 => {
                        ajdust_pos += Vec3::new(45.0, 316.0, 0.0);
                        "Y"
                    }
                    3 => {
                        ajdust_pos += Vec3::new(10.0, 292.0, 0.0);
                        "Group"
                    }
                    4 => {
                        ajdust_pos += Vec3::new(10.0, 260.0, 0.0);
                        "Tile Location"
                    }
                    5 => {
                        ajdust_pos += Vec3::new(45.0, 232.0, 0.0);
                        "X"
                    }
                    6 => {
                        ajdust_pos += Vec3::new(45.0, 208.0, 0.0);
                        "Y"
                    }
                    _ => {
                        ajdust_pos += Vec3::new(10.0, 368.0, 0.0);
                        "Map Location"
                    }
                };
                let mut text = create_basic_label(
                    systems,
                    Vec3::new(ajdust_pos.x, ajdust_pos.y, ORDER_ATTRIBUTE_LABEL),
                    Vec2::new(100.0, 20.0),
                    Color::rgba(180, 180, 180, 255),
                );
                text.set_text(&mut systems.renderer, msg, Attrs::new(), Shaping::Advanced);
                gui.editor_label.push(systems.gfx.add_text(text, 1));
            }

            gui.editor_textbox = Vec::with_capacity(5);
            for i in 0..5 {
                let pos = systems.gfx.get_pos(gui.tab_opt_bg[0]);
                let textbox_pos = match i {
                    1 => Vec3::new(pos.x + 65.0, pos.y + 316.0, ORDER_ATTRIBUTE_TEXTBOX),
                    2 => Vec3::new(pos.x + 65.0, pos.y + 292.0, ORDER_ATTRIBUTE_TEXTBOX),
                    3 => Vec3::new(pos.x + 65.0, pos.y + 232.0, ORDER_ATTRIBUTE_TEXTBOX),
                    4 => Vec3::new(pos.x + 65.0, pos.y + 208.0, ORDER_ATTRIBUTE_TEXTBOX),
                    _ => Vec3::new(pos.x + 65.0, pos.y + 340.0, ORDER_ATTRIBUTE_TEXTBOX),
                };
                gui.editor_textbox.push(Textbox::new(
                    systems,
                    textbox_pos,
                    Vec2::new(60.0, 22.0),
                    false,
                    [0, 1],
                ));
            }
            // If data exist, place the data on textbox
            if !data.is_empty() {
                gui.editor_textbox[0].input_text(systems, data[0].get_int().to_string());
                gui.editor_textbox[1].input_text(systems, data[1].get_int().to_string());
                gui.editor_textbox[2].input_text(systems, data[2].get_uint().to_string());
                gui.editor_textbox[3].input_text(systems, data[3].get_uint().to_string());
                gui.editor_textbox[4].input_text(systems, data[4].get_uint().to_string());
            } else {
                gui.editor_textbox[0].input_text(systems, "0".to_string());
                gui.editor_textbox[1].input_text(systems, "0".to_string());
                gui.editor_textbox[2].input_text(systems, "0".to_string());
                gui.editor_textbox[3].input_text(systems, "0".to_string());
                gui.editor_textbox[4].input_text(systems, "0".to_string());
            }
        }
        MapAttribute::Sign(_) => {
            let pos = systems.gfx.get_pos(gui.tab_opt_bg[0]);
            let text = create_basic_label(
                systems,
                Vec3::new(pos.x + 10.0, pos.y + 368.0, ORDER_ATTRIBUTE_LABEL),
                Vec2::new(100.0, 20.0),
                Color::rgba(180, 180, 180, 255),
            );
            gui.editor_label = vec![systems.gfx.add_text(text, 1)];
            systems
                .gfx
                .set_text(&mut systems.renderer, gui.editor_label[0], "Sign Text");
            gui.editor_textbox = vec![Textbox::new(
                systems,
                Vec3::new(pos.x + 10.0, pos.y + 115.0, ORDER_ATTRIBUTE_TEXTBOX),
                Vec2::new(180.0, 250.0),
                true,
                [0, 1],
            )];
            // If data exist, place the data on textbox
            if !data.is_empty() {
                gui.editor_textbox[0].input_text(systems, data[0].get_string());
            } else {
                gui.editor_textbox[0].input_text(systems, String::new());
            }
        }
        MapAttribute::ItemSpawn(_) => {
            gui.editor_label = Vec::with_capacity(3);
            for i in 0..3 {
                let mut ajdust_pos = systems.gfx.get_pos(gui.tab_opt_bg[0]);
                let msg = match i {
                    1 => {
                        ajdust_pos += Vec3::new(10.0, 340.0, 0.0);
                        "Value"
                    }
                    2 => {
                        ajdust_pos += Vec3::new(10.0, 312.0, 0.0);
                        "Timer"
                    }
                    _ => {
                        ajdust_pos += Vec3::new(10.0, 368.0, 0.0);
                        "Index"
                    }
                };
                let mut text = create_basic_label(
                    systems,
                    Vec3::new(ajdust_pos.x, ajdust_pos.y, ORDER_ATTRIBUTE_LABEL),
                    Vec2::new(100.0, 20.0),
                    Color::rgba(180, 180, 180, 255),
                );
                text.set_text(&mut systems.renderer, msg, Attrs::new(), Shaping::Advanced);
                gui.editor_label.push(systems.gfx.add_text(text, 1));
            }

            gui.editor_textbox = Vec::with_capacity(3);
            for i in 0..3 {
                let pos = systems.gfx.get_pos(gui.tab_opt_bg[0]);
                let textbox_pos = match i {
                    1 => Vec3::new(pos.x + 65.0, pos.y + 340.0, ORDER_ATTRIBUTE_TEXTBOX),
                    2 => Vec3::new(pos.x + 65.0, pos.y + 312.0, ORDER_ATTRIBUTE_TEXTBOX),
                    _ => Vec3::new(pos.x + 65.0, pos.y + 368.0, ORDER_ATTRIBUTE_TEXTBOX),
                };
                gui.editor_textbox.push(Textbox::new(
                    systems,
                    textbox_pos,
                    Vec2::new(60.0, 22.0),
                    false,
                    [0, 1],
                ));
            }
            // If data exist, place the data on textbox
            if !data.is_empty() {
                gui.editor_textbox[0].input_text(systems, data[0].get_uint().to_string());
                gui.editor_textbox[1].input_text(systems, data[1].get_uint().to_string());
                gui.editor_textbox[2].input_text(systems, data[2].get_uint().to_string());
            } else {
                gui.editor_textbox[0].input_text(systems, "0".to_string());
                gui.editor_textbox[1].input_text(systems, "0".to_string());
                gui.editor_textbox[2].input_text(systems, "0".to_string());
            }
        }
        MapAttribute::Shop(_) => {
            gui.editor_label = Vec::with_capacity(1);
            let mut ajdust_pos = systems.gfx.get_pos(gui.tab_opt_bg[0]);
            ajdust_pos += Vec3::new(10.0, 368.0, 0.0);
            let mut text = create_basic_label(
                systems,
                Vec3::new(ajdust_pos.x, ajdust_pos.y, ORDER_ATTRIBUTE_LABEL),
                Vec2::new(100.0, 20.0),
                Color::rgba(180, 180, 180, 255),
            );
            text.set_text(
                &mut systems.renderer,
                "Index",
                Attrs::new(),
                Shaping::Advanced,
            );
            gui.editor_label.push(systems.gfx.add_text(text, 1));

            gui.editor_textbox = Vec::with_capacity(1);
            let pos = systems.gfx.get_pos(gui.tab_opt_bg[0]);
            let textbox_pos = Vec3::new(pos.x + 65.0, pos.y + 368.0, ORDER_ATTRIBUTE_TEXTBOX);
            gui.editor_textbox.push(Textbox::new(
                systems,
                textbox_pos,
                Vec2::new(60.0, 22.0),
                false,
                [0, 1],
            ));

            // If data exist, place the data on textbox
            if !data.is_empty() {
                gui.editor_textbox[0].input_text(systems, data[0].get_uint().to_string());
            } else {
                gui.editor_textbox[0].input_text(systems, "0".to_string());
            }
        }
        _ => {}
    }
}

fn click_dir_block(systems: &mut DrawSetting, gui: &mut Interface, screen_pos: Vec2) {
    if gui.current_tab != TAB_PROPERTIES {
        return;
    }

    for (index, rect) in gui.editor_rect.iter().enumerate() {
        let pos = systems.gfx.get_pos(*rect);
        if screen_pos.x >= pos.x
            && screen_pos.x <= pos.x + 32.0
            && screen_pos.y >= pos.y
            && screen_pos.y <= pos.y + 32.0
        {
            gui.dir_select[index] = !gui.dir_select[index];
            if gui.dir_select[index] {
                systems.gfx.set_color(*rect, Color::rgba(220, 50, 50, 255));
            } else {
                systems
                    .gfx
                    .set_color(*rect, Color::rgba(100, 100, 100, 255));
            }
        }
    }
}
