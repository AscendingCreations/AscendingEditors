use cosmic_text::{Attrs, Metrics};
use winit::{event::*, keyboard::*};

use graphics::*;

use crate::DrawSetting;

pub struct Textbox {
    pub image: usize,
    pub text: usize,
    pub data: String,
    pub is_selected: bool,
}

impl Textbox {
    pub fn new(
        systems: &mut DrawSetting,
        textbox_pos: Vec3,
        textbox_size: Vec2,
        can_wrap: bool,
        render_layer: [usize; 2],
    ) -> Self {
        let mut img = Rect::new(&mut systems.renderer, 0);
        img.set_size(textbox_size)
            .set_position(textbox_pos)
            .set_border_color(Color::rgba(80, 80, 80, 255))
            .set_border_width(1.0)
            .set_color(Color::rgba(80, 80, 80, 255))
            .set_use_camera(true);
        let image = systems.gfx.add_rect(img, render_layer[0]);

        let mut txt = Text::new(
            &mut systems.renderer,
            Some(Metrics::new(16.0, 16.0).scale(systems.scale as f32)),
            Vec3::new(textbox_pos.x + 2.0, textbox_pos.y - 2.0, textbox_pos.z),
            textbox_size,
            1.0,
        );
        txt.set_buffer_size(
            &mut systems.renderer,
            textbox_size.x as i32,
            systems.size.height as i32,
        )
        .set_bounds(Some(Bounds::new(
            textbox_pos.x,
            textbox_pos.y,
            textbox_pos.x + textbox_size.x,
            textbox_pos.y + textbox_size.y,
        )))
        .set_default_color(Color::rgba(200, 200, 200, 255))
        .set_text(
            &mut systems.renderer,
            "",
            Attrs::new(),
            Shaping::Advanced,
        );
        txt.use_camera = true;
        txt.changed = true;
        if can_wrap {
            txt.set_wrap(&mut systems.renderer, cosmic_text::Wrap::Word);
        }
        let text = systems.gfx.add_text(txt, render_layer[1]);

        Self {
            image,
            text,
            data: String::new(),
            is_selected: false,
        }
    }

    pub fn input_text(&mut self, systems: &mut DrawSetting, text: String) {
        self.data.clear();
        self.data.push_str(&text);
        systems
            .gfx
            .set_text(&mut systems.renderer, self.text, &self.data);
    }

    pub fn enter_numeric(
        &mut self,
        systems: &mut DrawSetting,
        event: &KeyEvent,
        limit: usize,
        can_be_negative: bool,
    ) {
        if !event.state.is_pressed() || !self.is_selected {
            return;
        }

        if event.physical_key == PhysicalKey::Code(KeyCode::Backspace) {
            self.data.pop();
        } else if event.physical_key == PhysicalKey::Code(KeyCode::Delete) {
            self.data.clear();
        } else {
            if self.data.len() >= limit {
                return;
            }
            if let Some(char) = event.logical_key.to_text() {
                if is_numeric(char) {
                    if self.data.len() == 1 && &self.data == "0" {
                        self.data.pop();
                    }
                    self.data.push_str(char);
                } else if char.contains('-')
                    && can_be_negative
                    && self.data.is_empty()
                {
                    self.data.push_str(char);
                }
            }
        }
        systems
            .gfx
            .set_text(&mut systems.renderer, self.text, &self.data);
    }

    pub fn enter_text(
        &mut self,
        systems: &mut DrawSetting,
        event: &KeyEvent,
        limit: usize,
    ) {
        if !event.state.is_pressed() || !self.is_selected {
            return;
        }

        if event.physical_key == PhysicalKey::Code(KeyCode::Backspace) {
            self.data.pop();
        } else if event.physical_key == PhysicalKey::Code(KeyCode::Delete) {
            self.data.clear();
        } else {
            if self.data.len() >= limit {
                return;
            }
            if is_text(event) {
                if let Some(char) = event.logical_key.to_text() {
                    self.data.push_str(char);
                }
            }
        }
        systems
            .gfx
            .set_text(&mut systems.renderer, self.text, &self.data);
    }

    pub fn set_select(&mut self, systems: &mut DrawSetting, is_select: bool) {
        if self.is_selected == is_select {
            return;
        }
        self.is_selected = is_select;
        if self.is_selected {
            systems
                .gfx
                .set_border_color(self.image, Color::rgba(180, 180, 180, 255));
        } else {
            systems
                .gfx
                .set_border_color(self.image, Color::rgba(80, 80, 80, 255));
        }
    }

    pub fn set_visible(&mut self, systems: &mut DrawSetting, visible: bool) {
        systems.gfx.set_visible(self.image, visible);
    }
}

pub fn is_numeric(char: &str) -> bool {
    char.trim().parse::<i64>().is_ok()
}

pub fn is_text(event: &KeyEvent) -> bool {
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
                | KeyCode::Slash
                | KeyCode::Space,
        )
    )
}
