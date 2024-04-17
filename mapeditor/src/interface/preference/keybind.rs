use cosmic_text::{Attrs, Metrics};
use graphics::*;

use winit::{event::*, keyboard::*};

use crate::{
    collection::*,
    interface::{button::*, label::*},
    DrawSetting,
};

pub enum EditorKey {
    KeyLoad,
    KeySave,
    KeyUndo,
    KeyRedo,
    KeyDraw,
    KeyErase,
    KeyFill,
    KeyEyetool,
    Count,
}

impl EditorKey {
    pub fn as_str<'a>(key: usize) -> &'a str {
        match key {
            0 => "Load",
            1 => "Save",
            2 => "Undo",
            3 => "Redo",
            4 => "Draw",
            5 => "Erase",
            6 => "Fill",
            7 => "Eyetool",
            _ => "Error",
        }
    }
}

pub struct KeybindWindow {
    pub window: usize,
    pub text: usize,
    pub is_open: bool,
    reset_button: bool,
    pub buttons: [Button; 2],
    pub hold_key_modifier: [bool; 3],
    pub key_index: usize,

    pub key_code: Option<Key>,
    pub key_modifier: [bool; 3],
}

impl KeybindWindow {
    pub fn new(systems: &mut DrawSetting) -> Self {
        // This will consist all rect that will shape the preference window design
        let window_size = Vec2::new(300.0, 100.0);
        let window_pos = Vec2::new(
            ((systems.size.width / ZOOM_LEVEL) * 0.5) - (window_size.x * 0.5),
            ((systems.size.height / ZOOM_LEVEL) * 0.5) - (window_size.y * 0.5),
        )
        .floor();
        let mut wndw = Rect::new(&mut systems.renderer, 0);
        wndw.set_size(window_size)
            .set_position(Vec3::new(
                window_pos.x,
                window_pos.y,
                ORDER_KEYBIND_WINDOW,
            ))
            .set_radius(3.0)
            .set_border_color(Color::rgba(10, 10, 10, 255))
            .set_border_width(2.0)
            .set_color(Color::rgba(50, 50, 50, 255))
            .set_use_camera(true);
        let window = systems.gfx.add_rect(wndw, 4);

        // Text
        let text_pos = Vec3::new(
            window_pos.x,
            window_pos.y + window_size.y - 45.0,
            ORDER_KEYBIND_TEXT,
        );
        let mut txt = create_basic_label(
            systems,
            text_pos,
            Vec2::new(window_size.x, 20.0),
            Color::rgba(180, 180, 180, 255),
        );
        txt.set_text(
            &mut systems.renderer,
            "Please enter a Key",
            Attrs::new(),
            Shaping::Advanced,
        );
        center_text(&mut txt);
        let text = systems.gfx.add_text(txt, 5);

        // Buttons
        let button_x = window_pos.x + ((window_size.x * 0.5).floor() - 82.0);
        let mut buttons = [
            Button::new(
                systems,
                systems.resource.preference_button.allocation,
                "Cancel",
                Vec2::new(button_x + 85.0, window_pos.y + 15.0),
                Vec2::new(80.0, 22.0),
                [ORDER_KEYBIND_BUTTON, ORDER_KEYBIND_BUTTON_TEXT],
                2.0,
                [2, 3],
            ),
            Button::new(
                systems,
                systems.resource.preference_button.allocation,
                "Save",
                Vec2::new(button_x, window_pos.y + 15.0),
                Vec2::new(80.0, 22.0),
                [ORDER_KEYBIND_BUTTON, ORDER_KEYBIND_BUTTON_TEXT],
                2.0,
                [2, 3],
            ),
        ];

        buttons.iter_mut().for_each(|button| {
            systems.gfx.set_visible(button.image, false);
            systems.gfx.set_visible(button.text, false);
        });
        systems.gfx.set_visible(window, false);
        systems.gfx.set_visible(text, false);

        Self {
            window,
            text,
            reset_button: false,
            buttons,
            is_open: false,
            key_code: None,
            hold_key_modifier: [false; 3],
            key_modifier: [false; 3],
            key_index: 0,
        }
    }

    pub fn open_key(&mut self, systems: &mut DrawSetting, key_index: usize) {
        self.is_open = true;
        systems.gfx.set_visible(self.window, true);
        systems.gfx.set_visible(self.text, true);
        self.buttons.iter_mut().for_each(|button| {
            systems.gfx.set_visible(button.image, true);
            systems.gfx.set_visible(button.text, true);
        });
        self.key_code = None;
        self.key_modifier = [false; 3];
        self.key_index = key_index;
        systems.gfx.set_text(
            &mut systems.renderer,
            self.text,
            "Please enter a Key",
        );
        systems.gfx.center_text(self.text);
    }

    pub fn close_key(&mut self, systems: &mut DrawSetting) {
        self.is_open = false;
        systems.gfx.set_visible(self.window, false);
        systems.gfx.set_visible(self.text, false);
        self.buttons.iter_mut().for_each(|button| {
            systems.gfx.set_visible(button.image, false);
            systems.gfx.set_visible(button.text, false);
        });
    }

    pub fn edit_key(&mut self, event: &KeyEvent, systems: &mut DrawSetting) {
        match event.physical_key {
            PhysicalKey::Code(KeyCode::ControlLeft)
            | PhysicalKey::Code(KeyCode::ControlRight) => {
                self.hold_key_modifier[0] = event.state.is_pressed()
            }
            PhysicalKey::Code(KeyCode::ShiftLeft)
            | PhysicalKey::Code(KeyCode::ShiftRight) => {
                self.hold_key_modifier[1] = event.state.is_pressed()
            }
            PhysicalKey::Code(KeyCode::Space) => {
                self.hold_key_modifier[2] = event.state.is_pressed()
            }
            _ => {
                if is_valid_key_code(event) {
                    self.key_code = Some(event.logical_key.clone());
                    self.key_modifier = self.hold_key_modifier;

                    if let Some(keycode) = self.key_code.clone() {
                        let button_text =
                            get_key_name(keycode, self.key_modifier);
                        systems.gfx.set_text(
                            &mut systems.renderer,
                            self.text,
                            &button_text,
                        );
                        systems.gfx.center_text(self.text);
                    }
                }
            }
        }
    }

    pub fn get_key(&mut self) {
        if let Some(key) = &mut self.key_code {
            println!(
                "Key Code: {:?} Modifier {:?}",
                key.to_text(),
                self.key_modifier
            );
        }
    }

    pub fn hover_buttons(
        &mut self,
        systems: &mut DrawSetting,
        mouse_pos: Vec2,
    ) {
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
    }

    // This function check which buttons are within the click position and return the button index
    pub fn click_buttons(
        &mut self,
        systems: &mut DrawSetting,
        mouse_pos: Vec2,
    ) -> Option<usize> {
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
}

pub fn get_key_name(key_code: Key, key_code_modifier: [bool; 3]) -> String {
    let mut button_text = String::new();
    let mut did_add = false;
    for (index, modifier) in key_code_modifier.iter().enumerate() {
        if *modifier {
            match index {
                1 => {
                    if did_add {
                        button_text.push_str("+ Shift ");
                    } else {
                        button_text.push_str("Shift ");
                    }
                } // Shift
                2 => {
                    if did_add {
                        button_text.push_str("+ Space ");
                    } else {
                        button_text.push_str("Space ");
                    }
                } // Space
                _ => {
                    if did_add {
                        button_text.push_str("+ Ctrl ");
                    } else {
                        button_text.push_str("Ctrl ");
                    }
                } // Ctrl
            }
            did_add = true;
        }
    }
    if did_add {
        button_text
            .push_str(&format!("+ {}", key_code.to_text().unwrap_or_default()));
    } else {
        button_text.push_str(key_code.to_text().unwrap_or_default());
    }
    button_text
}

pub fn is_valid_key_code(event: &KeyEvent) -> bool {
    matches!(
        event.physical_key,
        PhysicalKey::Code(
            KeyCode::KeyA
                | KeyCode::KeyB
                | KeyCode::KeyC
                | KeyCode::KeyD
                | KeyCode::KeyE
                | KeyCode::KeyF
                | KeyCode::KeyG
                | KeyCode::KeyH
                | KeyCode::KeyI
                | KeyCode::KeyJ
                | KeyCode::KeyK
                | KeyCode::KeyL
                | KeyCode::KeyM
                | KeyCode::KeyN
                | KeyCode::KeyO
                | KeyCode::KeyP
                | KeyCode::KeyQ
                | KeyCode::KeyR
                | KeyCode::KeyS
                | KeyCode::KeyT
                | KeyCode::KeyU
                | KeyCode::KeyV
                | KeyCode::KeyW
                | KeyCode::KeyX
                | KeyCode::KeyY
                | KeyCode::KeyZ
                | KeyCode::Digit1
                | KeyCode::Digit2
                | KeyCode::Digit3
                | KeyCode::Digit4
                | KeyCode::Digit5
                | KeyCode::Digit6
                | KeyCode::Digit7
                | KeyCode::Digit8
                | KeyCode::Digit9
                | KeyCode::Digit0
                | KeyCode::Comma
                | KeyCode::Period
                | KeyCode::BracketLeft
                | KeyCode::BracketRight
                | KeyCode::Backquote
                | KeyCode::Minus
                | KeyCode::Equal
                | KeyCode::Quote
                | KeyCode::Backslash
                | KeyCode::Semicolon
                | KeyCode::Slash,
        )
    )
}
