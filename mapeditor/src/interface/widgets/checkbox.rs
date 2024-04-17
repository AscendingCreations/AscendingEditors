use graphics::*;
use cosmic_text::{Attrs, Metrics};

use crate::{
    collection::*,
    interface::label::*,
    DrawSetting,
};

pub struct Checkbox {
    pub window: Vec<usize>,
    pub text: usize,
    is_hover: bool,
    pub is_select: bool,
}

impl Checkbox {
    pub fn new(systems: &mut DrawSetting, 
                pos: Vec2,
                msg: &str,
                checkbox_size: Vec2,
                z_pos: [f32; 3],
                default_value: bool,
                render_layer: [usize; 2],
    ) -> Self {
        let mut window0 = Rect::new(&mut systems.renderer, 0);
        let mut window1 = Rect::new(&mut systems.renderer, 0);
        window0.set_size(checkbox_size)
                .set_position(Vec3::new(pos.x, pos.y, z_pos[0]))
                .set_color(Color::rgba(180, 180, 180, 0))
                .set_use_camera(true); // Button
        window1.set_size(Vec2::new(16.0, 16.0))
                .set_position(Vec3::new(pos.x + 2.0, pos.y + ((checkbox_size.y * 0.5) - 8.0), z_pos[1]))
                .set_use_camera(true); // Checkbox
        if default_value {
            window1.set_color(Color::rgba(200, 200, 200, 255))
                    .set_border_width(2.0)
                    .set_border_color(Color::rgba(100, 100, 100, 255));
        } else {
            window1.set_color(Color::rgba(100, 100, 100, 255))
                    .set_border_width(0.0);
        }

        let window = vec![
            systems.gfx.add_rect(window0, render_layer[0]),
            systems.gfx.add_rect(window1, render_layer[0]),
        ];
        systems.gfx.set_visible(window[0], false);
        systems.gfx.set_visible(window[1], false);
        
        let mut txt = create_label(systems,
                Vec3::new(pos.x + 24.0, pos.y, z_pos[2]),
                Vec2::new(checkbox_size.x - 24.0, checkbox_size.y),
                Bounds::new(pos.x + 24.0, pos.y, pos.x + checkbox_size.x + 24.0, pos.y + checkbox_size.y),
                Color::rgba(180, 180, 180, 255));
        txt.set_text(&mut systems.renderer, msg, Attrs::new(), Shaping::Advanced,);
        let text = systems.gfx.add_text(txt, render_layer[1]);
        systems.gfx.set_visible(text, false);
        
        Self {
            window,
            text,
            is_hover: false,
            is_select: default_value,
        }
    }
    
    pub fn set_hover(&mut self, systems: &mut DrawSetting, is_hover: bool) {
        if self.is_hover == is_hover {
            return;
        }

        self.is_hover = is_hover;
        if self.is_hover {
            systems.gfx.set_color(self.window[0], Color::rgba(180, 180, 180, 255));
            systems.gfx.set_color(self.text, Color::rgba(40, 40, 40, 255));
        } else {
            systems.gfx.set_color(self.window[0], Color::rgba(180, 180, 180, 0));
            systems.gfx.set_color(self.text, Color::rgba(180, 180, 180, 255));
        }
    }

    pub fn set_select(&mut self, systems: &mut DrawSetting, is_select: bool) {
        if self.is_select == is_select {
            return;
        }

        self.is_select = is_select;
        if self.is_select {
            systems.gfx.set_color(self.window[1], Color::rgba(200, 200, 200, 255));
            systems.gfx.set_border_width(self.window[1], 2.0);
            systems.gfx.set_border_color(self.window[1], Color::rgba(100, 100, 100, 255));
        } else {
            systems.gfx.set_color(self.window[1], Color::rgba(100, 100, 100, 255));
            systems.gfx.set_border_width(self.window[1], 0.0);
        }
    }

    pub fn unload(&mut self, systems: &mut DrawSetting) {
        systems.gfx.remove_gfx(self.window[0]);
        systems.gfx.remove_gfx(self.window[1]);
        systems.gfx.remove_gfx(self.text);
    }
}