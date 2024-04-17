use graphics::*;
use guillotiere::euclid::num::Floor;

use crate::DrawSetting;

enum TextureState {
    Normal,
    Hover,
    Click,
}

pub struct Scrollbar {
    pub images: Vec<usize>,
    pub in_hold: bool,
    pub cur_value: usize,
    in_hover: bool,
    max_value: usize,
    scrollbar_size: usize,
    hold_pos: f32,
    start_pos: usize,
    end_pos: usize,
    length: usize,
    max_scroll_size: usize,
    min_bar_size: usize,
    default_pos: Vec3,
    pub visible: bool,
}

impl Scrollbar {
    pub fn new(
        systems: &mut DrawSetting,
        pos: Vec3,
        max_value: usize,
        max_scroll_size: usize,
        min_bar_size: usize,
        render_layer: usize,
    ) -> Self {
        let mut images = Vec::with_capacity(3);

        let mut scrollbar_size = (max_scroll_size / (max_value + 1)).floor();
        if scrollbar_size < min_bar_size {
            scrollbar_size = min_bar_size;
        }

        // Top Corner of Scrollbar
        let mut image = Image::new(
            Some(systems.resource.scrollbar.allocation),
            &mut systems.renderer,
            1,
        );
        image.pos = Vec3::new(pos.x, pos.y, pos.z);
        image.hw = Vec2::new(10.0, 4.0);
        image.uv = Vec4::new(0.0, 0.0, 10.0, 4.0);
        images.push(systems.gfx.add_image(image, render_layer));

        // Center of Scrollbar
        let mut image = Image::new(
            Some(systems.resource.scrollbar.allocation),
            &mut systems.renderer,
            1,
        );
        image.pos = Vec3::new(pos.x, pos.y - scrollbar_size as f32, pos.z);
        image.hw = Vec2::new(10.0, scrollbar_size as f32);
        image.uv = Vec4::new(0.0, 5.0, 10.0, 6.0);
        images.push(systems.gfx.add_image(image, render_layer));

        // Bottom Corner of Scrollbar
        let mut image = Image::new(
            Some(systems.resource.scrollbar.allocation),
            &mut systems.renderer,
            1,
        );
        image.pos =
            Vec3::new(pos.x, pos.y - scrollbar_size as f32 - 4.0, pos.z);
        image.hw = Vec2::new(10.0, 4.0);
        image.uv = Vec4::new(0.0, 12.0, 10.0, 4.0);
        images.push(systems.gfx.add_image(image, render_layer));

        images.iter().for_each(|image| {
            systems.gfx.set_visible(*image, false);
        });

        let start_pos = pos.y as usize;
        let end_pos = pos.y as usize - (max_scroll_size - scrollbar_size);
        let length = start_pos - end_pos;

        Self {
            images,
            in_hover: false,
            cur_value: 0,
            in_hold: false,
            max_value,
            scrollbar_size,
            hold_pos: 0.0,
            start_pos,
            end_pos,
            length,
            max_scroll_size,
            min_bar_size,
            default_pos: pos,
            visible: false,
        }
    }

    pub fn update_scroll_max_value(
        &mut self,
        systems: &mut DrawSetting,
        max_value: usize,
    ) {
        if self.max_value == max_value {
            reset_scrollbar(systems, self);
            return;
        }

        let mut scrollbar_size =
            (self.max_scroll_size / (max_value + 1)).floor();
        if scrollbar_size < self.min_bar_size {
            scrollbar_size = self.min_bar_size;
        }

        // Top Corner of Scrollbar
        systems.gfx.set_pos(
            self.images[0],
            Vec3::new(
                self.default_pos.x,
                self.default_pos.y,
                self.default_pos.z,
            ),
        );

        // Center of Scrollbar
        systems.gfx.set_pos(
            self.images[1],
            Vec3::new(
                self.default_pos.x,
                self.default_pos.y - scrollbar_size as f32,
                self.default_pos.z,
            ),
        );
        systems
            .gfx
            .set_size(self.images[1], Vec2::new(10.0, scrollbar_size as f32));

        // Bottom Corner of Scrollbar
        systems.gfx.set_pos(
            self.images[2],
            Vec3::new(
                self.default_pos.x,
                self.default_pos.y - scrollbar_size as f32 - 4.0,
                self.default_pos.z,
            ),
        );

        // Reset data
        self.end_pos = self.default_pos.y as usize
            - (self.max_scroll_size - scrollbar_size);
        self.length = self.start_pos - self.end_pos;
        self.scrollbar_size = scrollbar_size;
        self.cur_value = 0;
        self.in_hover = false;
        self.in_hold = false;
        self.hold_pos = 0.0;
        self.max_value = max_value;
    }

    pub fn in_scrollbar(
        &mut self,
        systems: &mut DrawSetting,
        mouse_pos: Vec2,
    ) -> bool {
        let (pos0, pos2, size) = (
            systems.gfx.get_pos(self.images[0]),
            systems.gfx.get_pos(self.images[2]),
            systems.gfx.get_size(self.images[0]),
        );
        mouse_pos.x >= pos0.x
            && mouse_pos.x <= pos0.x + size.x
            && mouse_pos.y >= pos2.y
            && mouse_pos.y <= pos0.y + 4.0
    }

    pub fn hold_scrollbar(&mut self, systems: &mut DrawSetting, pos_y: f32) {
        if !self.visible {
            return;
        }
        if !self.in_hold {
            let pos = systems.gfx.get_pos(self.images[0]);
            self.in_hold = true;
            self.hold_pos = (pos.y + 4.0) - pos_y;
            set_texture_state(systems, &mut self.images, TextureState::Click);
        }
    }

    pub fn release_scrollbar(&mut self, systems: &mut DrawSetting) {
        if self.in_hold {
            self.in_hold = false;
            if self.in_hover {
                set_texture_state(
                    systems,
                    &mut self.images,
                    TextureState::Hover,
                );
            } else {
                set_texture_state(
                    systems,
                    &mut self.images,
                    TextureState::Normal,
                );
            }
        }
    }

    pub fn set_hover(&mut self, systems: &mut DrawSetting, mouse_pos: Vec2) {
        if !self.visible {
            return;
        }
        let (pos0, pos2, size) = (
            systems.gfx.get_pos(self.images[0]),
            systems.gfx.get_pos(self.images[2]),
            systems.gfx.get_size(self.images[0]),
        );
        self.in_hover = mouse_pos.x >= pos0.x
            && mouse_pos.x <= pos0.x + size.x
            && mouse_pos.y >= pos2.y
            && mouse_pos.y <= pos0.y + 4.0;

        if !self.in_hold {
            if self.in_hover {
                set_texture_state(
                    systems,
                    &mut self.images,
                    TextureState::Hover,
                );
            } else {
                set_texture_state(
                    systems,
                    &mut self.images,
                    TextureState::Normal,
                );
            }
        }
    }

    pub fn move_scrollbar(
        &mut self,
        systems: &mut DrawSetting,
        pos_y: f32,
        forced: bool,
    ) {
        if !forced && (!self.in_hold || !self.visible) {
            return;
        }

        let mut y = pos_y + self.hold_pos;
        y = y.clamp(self.end_pos as f32, self.start_pos as f32);

        let mut pos = systems.gfx.get_pos(self.images[0]);
        pos.y = y;
        systems.gfx.set_pos(self.images[0], pos);

        let mut pos = systems.gfx.get_pos(self.images[1]);
        pos.y = y - self.scrollbar_size as f32;
        systems.gfx.set_pos(self.images[1], pos);

        let mut pos = systems.gfx.get_pos(self.images[2]);
        pos.y = y - self.scrollbar_size as f32 - 4.0;
        systems.gfx.set_pos(self.images[2], pos);

        // Calculate the current value
        self.cur_value = (((self.start_pos as f32 - y) / self.length as f32)
            * self.max_value as f32)
            .floor() as usize;
    }

    pub fn show(&mut self, systems: &mut DrawSetting) {
        self.visible = true;
        self.images.iter().for_each(|image| {
            systems.gfx.set_visible(*image, true);
        });
    }

    pub fn hide(&mut self, systems: &mut DrawSetting) {
        self.visible = false;
        self.images.iter().for_each(|image| {
            systems.gfx.set_visible(*image, false);
        });
    }

    pub fn unload(&mut self, systems: &mut DrawSetting) {
        self.images.iter().for_each(|image| {
            systems.gfx.remove_gfx(*image);
        });
    }
}

pub fn reset_scrollbar(systems: &mut DrawSetting, scrollbar: &mut Scrollbar) {
    scrollbar.move_scrollbar(systems, scrollbar.start_pos as f32, true);
    scrollbar.set_hover(systems, Vec2::new(0.0, 0.0));
}

fn set_texture_state(
    systems: &mut DrawSetting,
    image: &mut [usize],
    state: TextureState,
) {
    match state {
        TextureState::Normal => {
            let mut uv = systems.gfx.get_uv(image[0]);
            uv.y = 0.0; // Top
            systems.gfx.set_uv(image[0], uv);
            let mut uv = systems.gfx.get_uv(image[1]);
            uv.y = 5.0; // Center
            systems.gfx.set_uv(image[1], uv);
            let mut uv = systems.gfx.get_uv(image[2]);
            uv.y = 12.0; // Bottom
            systems.gfx.set_uv(image[2], uv);
        }
        TextureState::Hover => {
            let mut uv = systems.gfx.get_uv(image[0]);
            uv.y = 16.0; // Top
            systems.gfx.set_uv(image[0], uv);
            let mut uv = systems.gfx.get_uv(image[1]);
            uv.y = 21.0; // Center
            systems.gfx.set_uv(image[1], uv);
            let mut uv = systems.gfx.get_uv(image[2]);
            uv.y = 28.0; // Bottom
            systems.gfx.set_uv(image[2], uv);
        }
        TextureState::Click => {
            let mut uv = systems.gfx.get_uv(image[0]);
            uv.y = 32.0; // Top
            systems.gfx.set_uv(image[0], uv);
            let mut uv = systems.gfx.get_uv(image[1]);
            uv.y = 37.0; // Center
            systems.gfx.set_uv(image[1], uv);
            let mut uv = systems.gfx.get_uv(image[2]);
            uv.y = 44.0; // Bottom
            systems.gfx.set_uv(image[2], uv);
        }
    }
}
