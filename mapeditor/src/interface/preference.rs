pub mod keybind;

use cosmic_text::{Attrs, Metrics};
use graphics::*;

use winit::{event::*, keyboard::*};

pub use keybind::*;

use crate::{
    collection::*,
    config::*,
    interface::{
        button::*, checkbox::*, color_selection::*, label::*, scrollbar::*,
    },
    DrawSetting,
};

use super::button;

pub const PREF_TAB_GENERAL: usize = 0;
pub const PREF_TAB_KEYBIND: usize = 1;

pub struct MenuButton {
    pub image: usize,
    pub text: usize,
    is_selected: bool,
}

impl MenuButton {
    pub fn new(systems: &mut DrawSetting, pos: Vec2, msg: &str) -> Self {
        let mut img = Rect::new(&mut systems.renderer, 0);
        img.set_position(Vec3::new(pos.x, pos.y, ORDER_PREFERENCE_MENU_BUTTON))
            .set_size(Vec2::new(118.0, 20.0))
            .set_color(Color::rgba(50, 50, 50, 0))
            .set_use_camera(true);
        let image = systems.gfx.add_rect(img, 2);
        systems.gfx.set_visible(image, false);

        let mut txt = create_label(
            systems,
            Vec3::new(pos.x + 2.0, pos.y, ORDER_PREFERENCE_MENU_BUTTON_TEXT),
            Vec2::new(118.0, 20.0),
            Bounds::new(pos.x, pos.y, pos.x + 120.0, pos.y + 20.0),
            Color::rgba(20, 20, 20, 255),
        );
        txt.set_text(
            &mut systems.renderer,
            msg,
            Attrs::new(),
            Shaping::Advanced,
        );
        let text = systems.gfx.add_text(txt, 3);
        systems.gfx.set_visible(text, false);

        Self {
            image,
            text,
            is_selected: false,
        }
    }

    pub fn set_select(&mut self, systems: &mut DrawSetting, is_selected: bool) {
        if self.is_selected == is_selected {
            return;
        }

        self.is_selected = is_selected;
        if self.is_selected {
            systems
                .gfx
                .set_color(self.image, Color::rgba(50, 50, 50, 255));
            systems
                .gfx
                .set_color(self.text, Color::rgba(180, 180, 180, 255));
        } else {
            systems
                .gfx
                .set_color(self.image, Color::rgba(50, 50, 50, 0));
            systems
                .gfx
                .set_color(self.text, Color::rgba(50, 50, 50, 255));
        }
    }
}

pub struct KeyList {
    pub text: usize,
    pub key_string: usize,
    pub key_button: usize,
    is_hover: bool,
}

impl KeyList {
    pub fn new(
        systems: &mut DrawSetting,
        pos: Vec2,
        msg: &str,
        keystr: &str,
    ) -> Self {
        let label_size = Vec2::new(100.0, 20.0);
        let mut txt = create_label(
            systems,
            Vec3::new(pos.x, pos.y, ORDER_PREFERENCE_KEYLIST_TEXT),
            label_size,
            Bounds::new(
                pos.x,
                pos.y,
                pos.x + label_size.x,
                pos.y + label_size.y,
            ),
            Color::rgba(180, 180, 180, 255),
        );
        txt.set_text(
            &mut systems.renderer,
            msg,
            Attrs::new(),
            Shaping::Advanced,
        );
        let text = systems.gfx.add_text(txt, 3);
        systems.gfx.set_visible(text, false);

        let key_pos =
            Vec3::new(pos.x + 100.0, pos.y, ORDER_PREFERENCE_KEYLIST_TEXT);
        let key_label_size = Vec2::new(200.0, 20.0);
        let mut keystring = create_label(
            systems,
            key_pos,
            key_label_size,
            Bounds::new(
                key_pos.x,
                key_pos.y,
                key_pos.x + key_label_size.x,
                key_pos.y + key_label_size.y,
            ),
            Color::rgba(180, 180, 180, 255),
        );
        keystring.set_text(
            &mut systems.renderer,
            keystr,
            Attrs::new(),
            Shaping::Advanced,
        );
        let key_string = systems.gfx.add_text(keystring, 3);
        systems.gfx.set_visible(key_string, false);

        let mut keybutton = Rect::new(&mut systems.renderer, 0);
        keybutton
            .set_size(key_label_size)
            .set_position(Vec3::new(
                key_pos.x - 3.0,
                key_pos.y,
                ORDER_PREFERENCE_KEYLIST_BUTTON,
            ))
            .set_color(Color::rgba(50, 50, 50, 255))
            .set_use_camera(true);
        let key_button = systems.gfx.add_rect(keybutton, 2);
        systems.gfx.set_visible(key_button, false);

        Self {
            text,
            key_string,
            key_button,
            is_hover: false,
        }
    }

    pub fn set_hover(&mut self, systems: &mut DrawSetting, is_hover: bool) {
        if self.is_hover == is_hover {
            return;
        }

        self.is_hover = is_hover;
        if self.is_hover {
            systems
                .gfx
                .set_color(self.key_button, Color::rgba(180, 180, 180, 255));
            systems
                .gfx
                .set_color(self.key_string, Color::rgba(40, 40, 40, 255));
        } else {
            systems
                .gfx
                .set_color(self.key_button, Color::rgba(50, 50, 50, 255));
            systems
                .gfx
                .set_color(self.key_string, Color::rgba(180, 180, 180, 255));
        }
    }
}

pub enum SettingData {
    None,
    Checkbox(Checkbox),
    Label(Text),
    ColorSelection(ColorSelection),
}

pub struct Preference {
    pub is_open: bool,
    pub bg: usize,
    pub window: [usize; 4],
    pub buttons: [Button; 3],
    pub menu_button: Vec<MenuButton>,
    pub scrollbar: Scrollbar,
    reset_button: bool,
    pub selected_menu: usize,
    // General
    pub setting_data: Vec<SettingData>,
    // Color selection
    pub is_coloreditor_open: Option<usize>,
    pub editing_index: usize,
    // Keybind
    pub key_list: Vec<KeyList>,
    pub keywindow: KeybindWindow,
}

impl Preference {
    pub fn new(systems: &mut DrawSetting) -> Self {
        // This image is for the transparent shadow that will render behind the preference
        let mut img = Rect::new(&mut systems.renderer, 0);
        img.set_position(Vec3::new(0.0, 0.0, ORDER_PREFERENCE_SHADOW))
            .set_size(Vec2::new(systems.size.width, systems.size.height))
            .set_color(Color::rgba(0, 0, 0, 200))
            .set_use_camera(true);
        let bg = systems.gfx.add_rect(img, 2);
        systems.gfx.set_visible(bg, false);

        // This will contain all rect that will shape the preference window design
        let window_size = Vec2::new(500.0, 350.0);
        let window_pos = Vec2::new(
            ((systems.size.width / ZOOM_LEVEL) * 0.5) - (window_size.x * 0.5),
            ((systems.size.height / ZOOM_LEVEL) * 0.5) - (window_size.y * 0.5),
        )
        .floor();
        let mut window0 = Rect::new(&mut systems.renderer, 0);
        window0
            .set_size(window_size)
            .set_position(Vec3::new(
                window_pos.x,
                window_pos.y,
                ORDER_PREFERENCE_WINDOW,
            ))
            .set_radius(3.0)
            .set_border_color(Color::rgba(10, 10, 10, 255))
            .set_border_width(2.0)
            .set_color(Color::rgba(50, 50, 50, 255))
            .set_use_camera(true);
        let mut window1 = Rect::new(&mut systems.renderer, 0);
        window1
            .set_size(Vec2::new(120.0, window_size.y - 65.0))
            .set_position(Vec3::new(
                window_pos.x + 20.0,
                window_pos.y + 45.0,
                ORDER_PREFERENCE_MENU,
            ))
            .set_color(Color::rgba(100, 100, 100, 255))
            .set_use_camera(true);
        let mut window2 = Rect::new(&mut systems.renderer, 0);
        window2
            .set_size(Vec2::new(window_size.x - 170.0, window_size.y - 65.0))
            .set_position(Vec3::new(
                window_pos.x + 150.0,
                window_pos.y + 45.0,
                ORDER_PREFERENCE_MENU,
            ))
            .set_color(Color::rgba(70, 70, 70, 255))
            .set_use_camera(true);
        let pos = Vec2::new(
            window2.position.x + window2.size.x - 10.0,
            window2.position.y + 2.0,
        );
        let mut window3 = Rect::new(&mut systems.renderer, 0);
        window3
            .set_size(Vec2::new(8.0, window_size.y - 69.0))
            .set_position(Vec3::new(pos.x, pos.y, ORDER_PREFERENCE_MENU))
            .set_color(Color::rgba(50, 50, 50, 255))
            .set_use_camera(true);

        // Buttons
        let button_x = window_pos.x + window_size.x - 20.0;
        let mut buttons = [
            Button::new(
                systems,
                systems.resource.preference_button.allocation,
                "Cancel",
                Vec2::new(button_x - 80.0, window_pos.y + 15.0),
                Vec2::new(80.0, 22.0),
                [ORDER_PREFERENCE_BUTTON, ORDER_PREFERENCE_BUTTON_TEXT],
                2.0,
                [2, 3],
            ),
            Button::new(
                systems,
                systems.resource.preference_button.allocation,
                "Reset",
                Vec2::new(button_x - 165.0, window_pos.y + 15.0),
                Vec2::new(80.0, 22.0),
                [ORDER_PREFERENCE_BUTTON, ORDER_PREFERENCE_BUTTON_TEXT],
                2.0,
                [2, 3],
            ),
            Button::new(
                systems,
                systems.resource.preference_button.allocation,
                "Save",
                Vec2::new(button_x - 250.0, window_pos.y + 15.0),
                Vec2::new(80.0, 22.0),
                [ORDER_PREFERENCE_BUTTON, ORDER_PREFERENCE_BUTTON_TEXT],
                2.0,
                [2, 3],
            ),
        ];
        buttons.iter_mut().for_each(|button| {
            systems.gfx.set_visible(button.image, false);
            systems.gfx.set_visible(button.text, false);
        });

        // Menu Buttons
        let button_y = window1.position.y + (window_size.y - 65.0);
        let mut menu_button = vec![
            MenuButton::new(
                systems,
                Vec2::new(window_pos.x + 21.0, button_y - 21.0),
                "General",
            ),
            MenuButton::new(
                systems,
                Vec2::new(window_pos.x + 21.0, button_y - 42.0),
                "Keybinds",
            ),
        ];
        menu_button[0].set_select(systems, true);

        // Scrollbar
        let scrollbar = Scrollbar::new(
            systems,
            Vec3::new(
                window2.position.x + window2.size.x - 11.0,
                window2.position.y + window2.size.y - 5.0,
                ORDER_PREFERENCE_SCROLLBAR,
            ),
            0,
            window2.size.y as usize - 10,
            20,
            2,
        );

        // Keybind Window
        let keywindow = KeybindWindow::new(systems);

        let mut window = [
            systems.gfx.add_rect(window0, 2), // Window
            systems.gfx.add_rect(window1, 2), // Menu BG
            systems.gfx.add_rect(window2, 2), // Content
            systems.gfx.add_rect(window3, 2),
        ]; // Scrollbar BG
        window.iter_mut().for_each(|window| {
            systems.gfx.set_visible(*window, false);
        });

        Self {
            is_open: false,
            bg,
            window,
            buttons,
            reset_button: false,
            menu_button,
            scrollbar,
            selected_menu: 0,
            setting_data: Vec::new(),
            key_list: Vec::new(),
            keywindow,
            is_coloreditor_open: None,
            editing_index: 0,
        }
    }

    pub fn open(&mut self, systems: &mut DrawSetting) {
        self.is_open = true;
        systems.gfx.set_visible(self.bg, true);
        self.window.iter_mut().for_each(|window| {
            systems.gfx.set_visible(*window, true);
        });
        self.buttons.iter_mut().for_each(|button| {
            systems.gfx.set_visible(button.image, true);
            systems.gfx.set_visible(button.text, true);
        });
        self.menu_button.iter_mut().for_each(|button| {
            systems.gfx.set_visible(button.image, true);
            systems.gfx.set_visible(button.text, true);
        });
        self.menu_button[self.selected_menu].set_select(systems, false);
        self.menu_button[0].set_select(systems, true);
        self.selected_menu = 0;
        self.scrollbar.show(systems);
    }

    pub fn close(&mut self, systems: &mut DrawSetting) {
        self.is_open = false;
        self.scrollbar.hide(systems);
        if self.keywindow.is_open {
            self.keywindow.close_key(systems);
        }
        systems.gfx.set_visible(self.bg, false);
        self.window.iter_mut().for_each(|window| {
            systems.gfx.set_visible(*window, false);
        });
        self.buttons.iter_mut().for_each(|button| {
            systems.gfx.set_visible(button.image, false);
            systems.gfx.set_visible(button.text, false);
        });
        self.menu_button.iter_mut().for_each(|button| {
            systems.gfx.set_visible(button.image, false);
            systems.gfx.set_visible(button.text, false);
        });
        self.setting_data
            .iter_mut()
            .for_each(|setting| match setting {
                SettingData::Checkbox(checkbox) => {
                    systems.gfx.set_visible(checkbox.window[0], false);
                    systems.gfx.set_visible(checkbox.window[1], false);
                    systems.gfx.set_visible(checkbox.text, false);
                }
                SettingData::ColorSelection(colorselection) => {
                    systems.gfx.set_visible(colorselection.image, false);
                    systems.gfx.set_visible(colorselection.text, false);
                }
                SettingData::Label(_label) => {}
                _ => {}
            });
        self.key_list.iter_mut().for_each(|keylist| {
            systems.gfx.remove_gfx(keylist.key_button);
            systems.gfx.remove_gfx(keylist.key_string);
            systems.gfx.remove_gfx(keylist.text);
        });
        self.key_list = vec![];
    }

    pub fn hover_buttons(
        &mut self,
        systems: &mut DrawSetting,
        mouse_pos: Vec2,
    ) {
        if self.keywindow.is_open {
            return;
        }
        // We check if buttons are within the mouse position
        self.buttons.iter_mut().for_each(|button| {
            let (pos, size) = (
                systems.gfx.get_pos(button.image),
                systems.gfx.get_size(button.image),
            );
            if (mouse_pos.x) >= pos.x
                && (mouse_pos.x) <= pos.x + size.x
                && (mouse_pos.y) >= pos.y
                && (mouse_pos.y) <= pos.y + size.y
            {
                button.set_hover(systems, true);
            } else {
                button.set_hover(systems, false);
            }
        });

        match self.selected_menu {
            PREF_TAB_KEYBIND => {
                self.key_list.iter_mut().for_each(|keylist| {
                    let (pos, size) = (
                        systems.gfx.get_pos(keylist.key_button),
                        systems.gfx.get_size(keylist.key_button),
                    );
                    if (mouse_pos.x) >= pos.x
                        && (mouse_pos.x) <= pos.x + size.x
                        && (mouse_pos.y) >= pos.y
                        && (mouse_pos.y) <= pos.y + size.y
                    {
                        keylist.set_hover(systems, true);
                    } else {
                        keylist.set_hover(systems, false);
                    }
                });
            }
            PREF_TAB_GENERAL => {
                self.setting_data.iter_mut().for_each(
                    |setting| match setting {
                        SettingData::Checkbox(checkbox) => {
                            let (pos, size) = (
                                systems.gfx.get_pos(checkbox.window[0]),
                                systems.gfx.get_size(checkbox.window[0]),
                            );
                            if (mouse_pos.x) >= pos.x
                                && (mouse_pos.x) <= pos.x + size.x
                                && (mouse_pos.y) >= pos.y
                                && (mouse_pos.y) <= pos.y + size.y
                            {
                                checkbox.set_hover(systems, true);
                            } else {
                                checkbox.set_hover(systems, false);
                            }
                        }
                        SettingData::ColorSelection(colorselection) => {
                            let (pos, size) = (
                                systems.gfx.get_pos(colorselection.image),
                                systems.gfx.get_size(colorselection.image),
                            );
                            if (mouse_pos.x) >= pos.x
                                && (mouse_pos.x) <= pos.x + size.x
                                && (mouse_pos.y) >= pos.y
                                && (mouse_pos.y) <= pos.y + size.y
                            {
                                colorselection.set_hover(systems, true);
                            } else {
                                colorselection.set_hover(systems, false);
                            }
                        }
                        _ => {}
                    },
                );

                if let Some(index) = self.is_coloreditor_open {
                    if let SettingData::ColorSelection(colorselection) =
                        &mut self.setting_data[index]
                    {
                        if colorselection.color_editor.is_open {
                            let (pos, size) = (
                                systems.gfx.get_pos(
                                    colorselection.color_editor.button.image,
                                ),
                                systems.gfx.get_size(
                                    colorselection.color_editor.button.image,
                                ),
                            );
                            if (mouse_pos.x) >= pos.x
                                && (mouse_pos.x) <= pos.x + size.x
                                && (mouse_pos.y) >= pos.y
                                && (mouse_pos.y) <= pos.y + size.y
                            {
                                colorselection
                                    .color_editor
                                    .button
                                    .set_hover(systems, true);
                            } else {
                                colorselection
                                    .color_editor
                                    .button
                                    .set_hover(systems, false);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    // This function should be called when the mouse button is not being pressed
    // This check if a button has been clicked, if yes, it will reset their click status
    pub fn release_click(&mut self, systems: &mut DrawSetting) {
        if !self.reset_button {
            return;
        }

        self.buttons.iter_mut().for_each(|button| {
            button.set_click(systems, false);
        });
        // Color Selection Button
        if self.selected_menu == PREF_TAB_GENERAL {
            if let Some(index) = self.is_coloreditor_open {
                if let SettingData::ColorSelection(colorselection) =
                    &mut self.setting_data[index]
                {
                    if colorselection.color_editor.is_open {
                        colorselection
                            .color_editor
                            .button
                            .set_click(systems, false);
                    }
                }
            }
        }
    }

    // This function check which buttons are within the click position and return the button index
    pub fn click_buttons(
        &mut self,
        systems: &mut DrawSetting,
        mouse_pos: Vec2,
    ) -> Option<usize> {
        if self.keywindow.is_open {
            return None;
        }
        let mut found_button = None;
        for (index, button) in self.buttons.iter().enumerate() {
            let (pos, size) = (
                systems.gfx.get_pos(button.image),
                systems.gfx.get_size(button.image),
            );
            if (mouse_pos.x) >= pos.x
                && (mouse_pos.x) <= pos.x + size.x
                && (mouse_pos.y) >= pos.y
                && (mouse_pos.y) <= pos.y + size.y
            {
                found_button = Some(index);
            }
        }
        if let Some(index) = found_button {
            self.buttons[index].set_click(systems, true);
            self.reset_button = true; // This remind us that a button has been clicked and needed to be reset
        }
        found_button
    }
    pub fn click_color_selection_button(
        &mut self,
        systems: &mut DrawSetting,
        mouse_pos: Vec2,
    ) -> bool {
        if self.selected_menu != PREF_TAB_GENERAL {
            return false;
        }
        if let Some(index) = self.is_coloreditor_open {
            if let SettingData::ColorSelection(colorselection) =
                &mut self.setting_data[index]
            {
                if colorselection.color_editor.is_open {
                    colorselection
                        .color_editor
                        .button
                        .set_click(systems, false);
                    let (pos, size) = (
                        systems
                            .gfx
                            .get_pos(colorselection.color_editor.button.image),
                        systems
                            .gfx
                            .get_size(colorselection.color_editor.button.image),
                    );
                    if (mouse_pos.x) >= pos.x
                        && (mouse_pos.x) <= pos.x + size.x
                        && (mouse_pos.y) >= pos.y
                        && (mouse_pos.y) <= pos.y + size.y
                    {
                        colorselection
                            .color_editor
                            .button
                            .set_click(systems, true);
                        self.reset_button = true;
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn select_menu_button(
        &mut self,
        systems: &mut DrawSetting,
        mouse_pos: Vec2,
    ) -> bool {
        if self.keywindow.is_open {
            return false;
        }
        let mut found_button = None;
        for (index, button) in self.menu_button.iter().enumerate() {
            let (pos, size) = (
                systems.gfx.get_pos(button.image),
                systems.gfx.get_size(button.image),
            );
            if (mouse_pos.x) >= pos.x
                && (mouse_pos.x) <= pos.x + size.x
                && (mouse_pos.y) >= pos.y
                && (mouse_pos.y) <= pos.y + size.y
            {
                found_button = Some(index);
            }
        }
        if let Some(index) = found_button {
            if self.selected_menu == index {
                return false;
            }
            self.menu_button[self.selected_menu].set_select(systems, false);
            self.menu_button[index].set_select(systems, true);
            self.selected_menu = index;
            return true;
        }
        false
    }

    pub fn select_keylist(
        &mut self,
        systems: &mut DrawSetting,
        mouse_pos: Vec2,
    ) -> Option<usize> {
        if self.keywindow.is_open {
            return None;
        }
        let mut found_button = None;
        for (index, keylist) in self.key_list.iter().enumerate() {
            let (pos, size) = (
                systems.gfx.get_pos(keylist.key_button),
                systems.gfx.get_pos(keylist.key_button),
            );
            if (mouse_pos.x) >= pos.x
                && (mouse_pos.x) <= pos.x + size.x
                && (mouse_pos.y) >= pos.y
                && (mouse_pos.y) <= pos.y + size.y
            {
                found_button = Some(index);
            }
        }
        found_button
    }

    pub fn select_config(
        &mut self,
        systems: &mut DrawSetting,
        mouse_pos: Vec2,
    ) -> Option<usize> {
        let mut found_button = None;
        for (index, config) in self.setting_data.iter().enumerate() {
            match config {
                SettingData::Checkbox(checkbox) => {
                    let (pos, size) = (
                        systems.gfx.get_pos(checkbox.window[0]),
                        systems.gfx.get_size(checkbox.window[0]),
                    );
                    if (mouse_pos.x) >= pos.x
                        && (mouse_pos.x) <= pos.x + size.x
                        && (mouse_pos.y) >= pos.y
                        && (mouse_pos.y) <= pos.y + size.y
                    {
                        found_button = Some(index);
                        break;
                    }
                }
                SettingData::ColorSelection(colorselection) => {
                    let (pos, size) = (
                        systems.gfx.get_pos(colorselection.image),
                        systems.gfx.get_size(colorselection.image),
                    );
                    if (mouse_pos.x) >= pos.x
                        && (mouse_pos.x) <= pos.x + size.x
                        && (mouse_pos.y) >= pos.y
                        && (mouse_pos.y) <= pos.y + size.y
                    {
                        found_button = Some(index);
                        break;
                    }
                }
                _ => {}
            }
        }
        found_button
    }

    pub fn update_scroll(&mut self, _cur_value: usize) -> bool {
        // Scrollbar is not being used on any tabs, but it will be kept for future expansion
        false
    }

    pub fn update_list(&mut self) {
        // Scrollbar is not being used on any tabs, but it will be kept for future expansion
    }

    pub fn update_key_list(
        &mut self,
        systems: &mut DrawSetting,
        key_index: usize,
        config_data: &mut ConfigData,
    ) {
        let (wpos, wsize) = (
            systems.gfx.get_pos(self.window[2]),
            systems.gfx.get_size(self.window[2]),
        );
        let pos = Vec2::new(wpos.x + 10.0, (wpos.y + wsize.y) - 30.0);
        let key_text = get_key_name(
            config_data.key_code[key_index].clone(),
            config_data.key_code_modifier[key_index],
        );
        self.key_list[key_index] = KeyList::new(
            systems,
            Vec2::new(pos.x, pos.y - (key_index as f32 * 21.0)),
            EditorKey::as_str(key_index),
            &key_text,
        );
    }

    pub fn reset_preference(
        &mut self,
        systems: &mut DrawSetting,
        config_data: &mut ConfigData,
    ) {
        // Reset data
        config_data.reset_config();
        open_preference_tab(self, systems, config_data);
    }

    pub fn select_text(&mut self, systems: &mut DrawSetting, mouse_pos: Vec2) {
        if self.selected_menu != PREF_TAB_GENERAL
            || self.is_coloreditor_open.is_none()
        {
            return;
        }
        if let Some(config_index) = self.is_coloreditor_open {
            if let SettingData::ColorSelection(colorselection) =
                &mut self.setting_data[config_index]
            {
                if !colorselection.color_editor.is_open {
                    return;
                }
                let last_selected = self.editing_index;
                let mut selected_index = -1;
                for (index, textbox) in
                    colorselection.color_editor.textbox.iter_mut().enumerate()
                {
                    let (pos, size) = (
                        systems.gfx.get_pos(textbox.image),
                        systems.gfx.get_size(textbox.image),
                    );

                    if (mouse_pos.x) >= pos.x
                        && (mouse_pos.x) <= pos.x + size.x
                        && (mouse_pos.y) >= pos.y
                        && (mouse_pos.y) <= pos.y + size.y
                    {
                        textbox.set_select(systems, true);
                        selected_index = index as i32;
                    } else {
                        textbox.set_select(systems, false);
                    }
                }
                if selected_index < 0 {
                    selected_index = last_selected as i32;
                    colorselection.color_editor.textbox[last_selected]
                        .set_select(systems, false);
                }
                self.editing_index = selected_index as usize;
            }
        }
    }

    pub fn hide_color_selection(&mut self, systems: &mut DrawSetting) {
        // Hide color selection if it is visible
        if let Some(index) = self.is_coloreditor_open {
            if let SettingData::ColorSelection(colorselection) =
                &mut self.setting_data[index]
            {
                colorselection.close_color_editor(systems);
            }
            self.is_coloreditor_open = None;
        }
    }

    pub fn in_color_selection(
        &mut self,
        systems: &mut DrawSetting,
        mouse_pos: Vec2,
    ) -> bool {
        if self.is_coloreditor_open.is_none()
            || self.selected_menu != PREF_TAB_GENERAL
        {
            return false;
        }
        if let Some(index) = self.is_coloreditor_open {
            if let SettingData::ColorSelection(colorselection) =
                &mut self.setting_data[index]
            {
                let (pos, size) = (
                    systems.gfx.get_pos(colorselection.color_editor.window),
                    systems.gfx.get_size(colorselection.color_editor.window),
                );
                if (mouse_pos.x) >= pos.x
                    && (mouse_pos.x) <= pos.x + size.x
                    && (mouse_pos.y) >= pos.y
                    && (mouse_pos.y) <= pos.y + size.y
                {
                    return true;
                }
            }
        }
        false
    }
}

pub fn open_preference_tab(
    preference: &mut Preference,
    systems: &mut DrawSetting,
    config_data: &mut ConfigData,
) {
    let (wpos, wsize) = (
        systems.gfx.get_pos(preference.window[2]),
        systems.gfx.get_size(preference.window[2]),
    );
    // Clear Data
    preference
        .setting_data
        .iter_mut()
        .for_each(|data| match data {
            SettingData::Checkbox(checkbox) => checkbox.unload(systems),
            SettingData::ColorSelection(colorselection) => {
                colorselection.unload(systems)
            }
            SettingData::Label(_label) => {}
            _ => {}
        });
    preference.setting_data = vec![];

    preference.key_list.iter_mut().for_each(|keylist| {
        systems.gfx.remove_gfx(keylist.key_button);
        systems.gfx.remove_gfx(keylist.key_string);
        systems.gfx.remove_gfx(keylist.text);
    });
    preference.key_list = vec![];

    match preference.selected_menu {
        PREF_TAB_KEYBIND => {
            preference.is_coloreditor_open = None;
            preference.scrollbar.update_scroll_max_value(systems, 0);
            preference.key_list = Vec::with_capacity(EditorKey::Count as usize);
            for key in 0..EditorKey::Count as usize {
                let pos = Vec2::new(wpos.x + 10.0, (wpos.y + wsize.y) - 30.0);
                let key_text = get_key_name(
                    config_data.key_code[key].clone(),
                    config_data.key_code_modifier[key],
                );
                let keylist = KeyList::new(
                    systems,
                    Vec2::new(pos.x, pos.y - (key as f32 * 21.0)),
                    EditorKey::as_str(key),
                    &key_text,
                );
                preference.key_list.push(keylist);
            }

            preference.key_list.iter_mut().for_each(|keylist| {
                systems.gfx.set_visible(keylist.key_button, true);
                systems.gfx.set_visible(keylist.key_string, true);
                systems.gfx.set_visible(keylist.text, true);
            });
        } // Keybind
        PREF_TAB_GENERAL => {
            preference.is_coloreditor_open = None;
            preference.scrollbar.update_scroll_max_value(systems, 0);
            preference.key_list = vec![];

            let pos: Vec2 = Vec2::new(wpos.x + 10.0, (wpos.y + wsize.y) - 30.0);
            preference.setting_data = vec![
                SettingData::Checkbox(Checkbox::new(
                    systems,
                    Vec2::new(pos.x, pos.y),
                    "Hide FPS?",
                    Vec2::new(wsize.x - 30.0, 20.0),
                    [
                        ORDER_PREFERENCE_SETTING_IMG1,
                        ORDER_PREFERENCE_SETTING_IMG2,
                        ORDER_PREFERENCE_SETTING_TEXT,
                    ],
                    config_data.hide_fps,
                    [2, 3],
                )),
                SettingData::Checkbox(Checkbox::new(
                    systems,
                    Vec2::new(pos.x, pos.y - 24.0),
                    "Hide Tileset Background?",
                    Vec2::new(wsize.x - 30.0, 20.0),
                    [
                        ORDER_PREFERENCE_SETTING_IMG1,
                        ORDER_PREFERENCE_SETTING_IMG2,
                        ORDER_PREFERENCE_SETTING_TEXT,
                    ],
                    config_data.hide_tileset_bg,
                    [2, 3],
                )),
                SettingData::Checkbox(Checkbox::new(
                    systems,
                    Vec2::new(pos.x, pos.y - 48.0),
                    "Hide Map View Background?",
                    Vec2::new(wsize.x - 30.0, 20.0),
                    [
                        ORDER_PREFERENCE_SETTING_IMG1,
                        ORDER_PREFERENCE_SETTING_IMG2,
                        ORDER_PREFERENCE_SETTING_TEXT,
                    ],
                    config_data.hide_mapview_bg,
                    [2, 3],
                )),
                SettingData::ColorSelection(ColorSelection::new(
                    systems,
                    Vec3::new(
                        pos.x,
                        pos.y - 72.0,
                        ORDER_PREFERENCE_SETTING_IMG1,
                    ),
                    Vec2::new(70.0, 20.0),
                    config_data.map_selection_color,
                    Some("Map Selection Color"),
                    false,
                    [2, 3],
                )),
                SettingData::ColorSelection(ColorSelection::new(
                    systems,
                    Vec3::new(
                        pos.x,
                        pos.y - 96.0,
                        ORDER_PREFERENCE_SETTING_IMG1,
                    ),
                    Vec2::new(70.0, 20.0),
                    config_data.tile_selection_color,
                    Some("Tile Selection Color"),
                    false,
                    [2, 3],
                )),
            ];

            if preference.is_open {
                preference.setting_data.iter_mut().for_each(|setting| {
                    match setting {
                        SettingData::Checkbox(checkbox) => {
                            systems.gfx.set_visible(checkbox.window[0], true);
                            systems.gfx.set_visible(checkbox.window[1], true);
                            systems.gfx.set_visible(checkbox.text, true);
                        }
                        SettingData::ColorSelection(colorselection) => {
                            systems.gfx.set_visible(colorselection.image, true);
                            systems.gfx.set_visible(colorselection.text, true);
                        }
                        SettingData::Label(_label) => {}
                        _ => {}
                    }
                });
            }
        } // General: Default
        _ => {}
    }
}
