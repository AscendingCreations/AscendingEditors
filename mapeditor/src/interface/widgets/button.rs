use cosmic_text::{Attrs, Metrics};
use graphics::*;

use crate::{collection::*, interface::*, DrawSetting};

pub struct Button {
    pub image: usize,
    pub text: usize,
    in_hover: bool,
    in_click: bool,
    button_size: Vec2,
    adjust_text_y: f32,
}

impl Button {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        systems: &mut DrawSetting,
        texture: usize,
        message: &str,
        pos: Vec2,
        button_size: Vec2,
        z_order: [f32; 2],
        adjust_text_y: f32,
        render_layer: [usize; 2],
    ) -> Self {
        let mut img = Image::new(Some(texture), &mut systems.renderer, 1);
        img.pos = Vec3::new(pos.x, pos.y, z_order[0]);
        img.hw = button_size;
        img.uv = Vec4::new(0.0, 0.0, button_size.x, button_size.y);
        let image = systems.gfx.add_image(img, render_layer[0]);

        let adjust_x =
            (button_size.x * 0.5).floor() - (button_size.x * 0.5).floor();
        let mut txt = create_label(
            systems,
            Vec3::new(pos.x + adjust_x, pos.y + adjust_text_y, z_order[1]),
            Vec2::new(button_size.x, 20.0),
            Bounds::new(
                pos.x,
                pos.y + adjust_text_y,
                pos.x + button_size.x,
                pos.y + button_size.y,
            ),
            Color::rgba(180, 180, 180, 255),
        );
        txt.set_text(
            &mut systems.renderer,
            message,
            Attrs::new(),
            Shaping::Advanced,
        );
        // Adjust text x position
        let message_size = txt.measure();
        txt.pos.x = pos.x
            + ((button_size.x * 0.5).floor() - (message_size.x * 0.5)).floor();
        txt.changed = true;
        let text = systems.gfx.add_text(txt, render_layer[1]);

        Self {
            image,
            text,
            in_hover: false,
            in_click: false,
            button_size,
            adjust_text_y,
        }
    }

    pub fn set_hover(&mut self, systems: &mut DrawSetting, in_hover: bool) {
        if self.in_hover == in_hover {
            return;
        }
        self.in_hover = in_hover;
        if !self.in_click {
            let mut uv = systems.gfx.get_uv(self.image);
            if self.in_hover {
                uv.y = self.button_size.y;
            } else {
                uv.y = 0.0;
            }
            systems.gfx.set_uv(self.image, uv);
        }
    }

    pub fn set_click(&mut self, systems: &mut DrawSetting, in_click: bool) {
        if self.in_click == in_click {
            return;
        }
        self.in_click = in_click;
        if self.in_click {
            let (mut txtpos, imgpos, mut uv) = (
                systems.gfx.get_pos(self.text),
                systems.gfx.get_pos(self.image),
                systems.gfx.get_uv(self.image),
            );
            uv.y = self.button_size.y * 2.0;
            txtpos.y = imgpos.y + (self.adjust_text_y - 2.0);
            systems.gfx.set_uv(self.image, uv);
            systems.gfx.set_pos(self.text, txtpos);
        } else {
            let (mut txtpos, imgpos, mut uv) = (
                systems.gfx.get_pos(self.text),
                systems.gfx.get_pos(self.image),
                systems.gfx.get_uv(self.image),
            );
            if !self.in_hover {
                uv.y = 0.0;
            } else {
                uv.y = self.button_size.y;
            }
            txtpos.y = imgpos.y + self.adjust_text_y;
            systems.gfx.set_uv(self.image, uv);
            systems.gfx.set_pos(self.text, txtpos);
        }
    }
}
