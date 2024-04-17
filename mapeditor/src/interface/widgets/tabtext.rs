use graphics::*;
use cosmic_text::{Attrs, Metrics};

use crate::{
    DrawSetting,
    interface::label::*,
    collection::*,
};

pub struct TabText {
    pub text: usize,
    pub button: usize,
    pub is_selected: bool,
    pub is_hover: bool,
    pub visible: bool,
}

impl TabText {
    pub fn new(systems: &mut DrawSetting, pos: Vec2) -> Self {
        let mut image = Image::new(Some(systems.resource.tab_option.allocation), &mut systems.renderer, 1);
        // Setup the interface position, height, width, color and texture coordinate
        image.pos = Vec3::new(pos.x, pos.y, ORDER_TAB_BUTTON);
        image.hw = Vec2::new(194.0, 20.0);
        image.uv = Vec4::new(0.0, 0.0, 194.0, 20.0);
        let button = systems.gfx.add_image(image, 0);

        let txt = create_basic_label(systems,
            Vec3::new(pos.x + 24.0, pos.y - 1.0, ORDER_TAB_LABEL),
            Vec2::new(165.0, 20.0),
            Color::rgba(180, 180, 180, 255));
        let text = systems.gfx.add_text(txt, 1);

        systems.gfx.set_visible(text, false);
        systems.gfx.set_visible(button, false);

        Self {
            text,
            button,
            is_selected: false,
            is_hover: false,
            visible: false,
        }
    }

    pub fn init(&mut self, systems: &mut DrawSetting, msg: &str, width: f32) {
        systems.gfx.set_text(&mut systems.renderer, self.text, msg);
        
        let (mut uv, mut size) = (systems.gfx.get_uv(self.button),
                                systems.gfx.get_size(self.button));
        // Change width
        size.x = width;
        uv.z = width;
        systems.gfx.set_size(self.button, size);
        systems.gfx.set_uv(self.button, uv);

        systems.gfx.set_visible(self.text, true);
        systems.gfx.set_visible(self.button, true);
        self.visible = true;
    }

    pub fn update(&mut self, systems: &mut DrawSetting, msg: &str, is_select: bool) {
        if !self.visible {
            return;
        }
        
        systems.gfx.set_text(&mut systems.renderer, self.text, msg);

        if self.is_selected != is_select {
            self.is_selected = is_select;

            let mut uv = systems.gfx.get_uv(self.button);

            if is_select {
                uv.y = 40.0;
            } else {
                uv.y = 0.0;
            }
            systems.gfx.set_uv(self.button, uv);
        }
    }

    pub fn close(&mut self, systems: &mut DrawSetting) {
        if !self.visible {
            return;
        }

        systems.gfx.set_text(&mut systems.renderer, self.text, "");
        let mut uv = systems.gfx.get_uv(self.button);
        uv.y = 0.0;
        systems.gfx.set_uv(self.button, uv);

        self.is_hover = false;
        self.is_selected = false;

        systems.gfx.set_visible(self.text, false);
        systems.gfx.set_visible(self.button, false);
        self.visible = false;
    }

    pub fn set_select(&mut self, systems: &mut DrawSetting, is_select: bool) {
        if !self.visible {
            return;
        }
        if self.is_selected != is_select {
            self.is_selected = is_select;

            let mut uv = systems.gfx.get_uv(self.button);
            if is_select {
                uv.y = 40.0;
            } else {
                uv.y = 0.0;
            }
            systems.gfx.set_uv(self.button, uv);
        }
    }

    pub fn set_hover(&mut self, systems: &mut DrawSetting, is_hover: bool) {
        if !self.visible {
            return;
        }
        if self.is_hover != is_hover {
            self.is_hover = is_hover;

            if !self.is_selected {
                let mut uv = systems.gfx.get_uv(self.button);
                if is_hover {
                    uv.y = 20.0;
                } else {
                    uv.y = 0.0;
                }
                systems.gfx.set_uv(self.button, uv);
            }
        }
    }
}