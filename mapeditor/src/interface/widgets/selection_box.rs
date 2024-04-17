use cosmic_text::{Attrs, Metrics};
use graphics::*;

const MAX_VISIBLE_LIST: usize = 5;

use crate::{
    interface::{label::*, scrollbar::*},
    DrawSetting,
};

pub struct ListText {
    pub rect: usize,
    pub text: usize,
    is_hover: bool,
    is_select: bool,
}

impl ListText {
    pub fn set_hover(&mut self, systems: &mut DrawSetting, is_hover: bool) {
        if self.is_hover == is_hover || self.is_select {
            return;
        }
        self.is_hover = is_hover;
        if self.is_hover {
            systems
                .gfx
                .set_color(self.rect, Color::rgba(55, 55, 55, 255));
        } else {
            systems
                .gfx
                .set_color(self.rect, Color::rgba(35, 35, 35, 255));
        }
    }

    pub fn set_select(&mut self, systems: &mut DrawSetting, is_select: bool) {
        if self.is_select == is_select {
            return;
        }
        self.is_select = is_select;
        if self.is_select {
            systems
                .gfx
                .set_color(self.rect, Color::rgba(20, 20, 20, 255));
        } else if self.is_hover {
            systems
                .gfx
                .set_color(self.rect, Color::rgba(55, 55, 55, 255));
        } else {
            systems
                .gfx
                .set_color(self.rect, Color::rgba(35, 35, 35, 255));
        }
    }
}

pub struct SelectionBox {
    pub button: usize,
    pub rect: Vec<usize>,
    pub text: usize,
    pub list_text: Vec<ListText>,
    pub list: Vec<String>,
    pub scrollbar: Scrollbar,
    pub is_list_visible: bool,
    pub selected_index: usize,
    is_hover: bool,
    is_click: bool,
    list_exceed: bool,
    start_index: usize,
}

impl SelectionBox {
    pub fn new(
        systems: &mut DrawSetting,
        pos: Vec2,
        z_order: [f32; 6],
        width: f32,
        list: Vec<String>,
        render_layer: usize,
    ) -> Self {
        let mut rect0 = Rect::new(&mut systems.renderer, 0);
        // Dropdown Box
        rect0
            .set_position(Vec3::new(pos.x, pos.y, z_order[0]))
            .set_size(Vec2::new(width - 21.0, 24.0))
            .set_border_width(1.0)
            .set_border_color(Color::rgba(20, 20, 20, 255))
            .set_color(Color::rgba(35, 35, 35, 255))
            .set_use_camera(true);

        // Dropdown Box Image
        let mut button_image = Image::new(
            Some(systems.resource.selection_drop_button.allocation),
            &mut systems.renderer,
            0,
        );
        button_image.pos = Vec3::new(pos.x + (width - 22.0), pos.y, z_order[0]);
        button_image.hw = Vec2::new(22.0, 24.0);
        button_image.uv = Vec4::new(0.0, 0.0, 22.0, 24.0);
        let button = systems.gfx.add_image(button_image, 0);

        // List
        let visible_list = list.len().min(MAX_VISIBLE_LIST);
        let list_size = 4.0 + (20.0 * visible_list as f32);
        let mut list_text = Vec::new();
        let list_exceed = list.len() > MAX_VISIBLE_LIST;

        let left_over = if list.len() > MAX_VISIBLE_LIST {
            list.len() - MAX_VISIBLE_LIST
        } else {
            0
        };
        let scrollbar = Scrollbar::new(
            systems,
            Vec3::new(pos.x + width - 13.0, pos.y - 6.0, z_order[5]),
            left_over,
            90,
            20,
            render_layer,
        );

        let mut rect1 = Rect::new(&mut systems.renderer, 0);
        rect1
            .set_position(Vec3::new(
                pos.x,
                pos.y - (list_size - 1.0),
                z_order[2],
            ))
            .set_size(Vec2::new(width, list_size))
            .set_border_width(1.0)
            .set_border_color(Color::rgba(20, 20, 20, 255))
            .set_color(Color::rgba(35, 35, 35, 255))
            .set_use_camera(true);

        let rect = vec![
            systems.gfx.add_rect(rect0, 0),
            systems.gfx.add_rect(rect1, 0),
        ];
        systems.gfx.set_visible(rect[1], false);

        for (index, list) in list.iter().enumerate().take(visible_list) {
            let lpos =
                Vec2::new(pos.x + 4.0, pos.y - 22.0 - (20.0 * index as f32));

            let mut lrect = Rect::new(&mut systems.renderer, 0);
            lrect
                .set_position(Vec3::new(lpos.x - 2.0, lpos.y + 1.0, z_order[3]))
                .set_color(Color::rgba(35, 35, 35, 255))
                .set_use_camera(true);
            if list_exceed {
                lrect.set_size(Vec2::new(width - 17.0, 20.0));
            } else {
                lrect.set_size(Vec2::new(width - 4.0, 20.0));
            }

            let mut ltext = create_basic_label(
                systems,
                Vec3::new(lpos.x, lpos.y, z_order[4]),
                Vec2::new(width - 20.0, 20.0),
                Color::rgba(180, 180, 180, 255),
            );
            ltext.set_text(
                &mut systems.renderer,
                list,
                Attrs::new(),
                Shaping::Advanced,
            );

            let (rect, text) = (
                systems.gfx.add_rect(lrect, 0),
                systems.gfx.add_text(ltext, 1),
            );
            systems.gfx.set_visible(rect, false);
            systems.gfx.set_visible(text, false);

            list_text.push(ListText {
                rect,
                text,
                is_hover: false,
                is_select: false,
            });
        }

        // Selected Data Text
        let mut txt = create_basic_label(
            systems,
            Vec3::new(pos.x + 4.0, pos.y + 1.0, z_order[1]),
            Vec2::new(width - 26.0, 20.0),
            Color::rgba(180, 180, 180, 255),
        );
        txt.set_text(
            &mut systems.renderer,
            &list[0],
            Attrs::new(),
            Shaping::Advanced,
        );
        let text = systems.gfx.add_text(txt, 1);

        Self {
            button,
            rect,
            text,
            list_text,
            list,
            scrollbar,
            is_list_visible: false,
            is_hover: false,
            is_click: false,
            selected_index: 0,
            list_exceed,
            start_index: 0,
        }
    }

    pub fn update_list(&mut self, systems: &mut DrawSetting, start_pos: usize) {
        if self.start_index == start_pos || !self.list_exceed {
            return;
        }
        self.start_index = start_pos;
        for index in 0..MAX_VISIBLE_LIST {
            let list_index = index + self.start_index;
            if list_index < self.list.len() {
                if self.selected_index == list_index {
                    self.list_text[index].set_select(systems, true);
                } else {
                    self.list_text[index].set_select(systems, false);
                }
                systems.gfx.set_text(
                    &mut systems.renderer,
                    self.list_text[index].text,
                    &self.list[list_index],
                );
            }
        }
    }

    pub fn show_list(&mut self, systems: &mut DrawSetting) {
        self.is_list_visible = true;

        systems.gfx.set_visible(self.rect[1], true);

        self.start_index = 0;
        for index in 0..MAX_VISIBLE_LIST {
            let list_index = index + self.start_index;
            if list_index < self.list.len() {
                if self.selected_index == list_index {
                    self.list_text[index].set_select(systems, true);
                } else {
                    self.list_text[index].set_select(systems, false);
                }
                systems.gfx.set_text(
                    &mut systems.renderer,
                    self.list_text[index].text,
                    &self.list[list_index],
                );
            }
        }

        self.list_text.iter_mut().for_each(|list_text| {
            systems.gfx.set_visible(list_text.text, true);
            systems.gfx.set_visible(list_text.rect, true);
        });
        if self.list_exceed {
            self.scrollbar.show(systems);
            reset_scrollbar(systems, &mut self.scrollbar);
        }
    }

    pub fn hide_list(&mut self, systems: &mut DrawSetting) {
        self.is_list_visible = false;
        self.scrollbar.hide(systems);
        systems.gfx.set_visible(self.rect[1], false);

        self.list_text.iter_mut().for_each(|list_text| {
            systems.gfx.set_visible(list_text.text, false);
            systems.gfx.set_visible(list_text.rect, false);
        });
    }

    pub fn set_hover(&mut self, systems: &mut DrawSetting, is_hover: bool) {
        if self.is_hover == is_hover {
            return;
        }
        self.is_hover = is_hover;
        if self.is_hover {
            systems
                .gfx
                .set_color(self.rect[0], Color::rgba(55, 55, 55, 255));
            let mut uv = systems.gfx.get_uv(self.button);
            uv.y = 24.0;
            systems.gfx.set_uv(self.button, uv);
        } else {
            systems
                .gfx
                .set_color(self.rect[0], Color::rgba(35, 35, 35, 255));
            let mut uv = systems.gfx.get_uv(self.button);
            uv.y = 0.0;
            systems.gfx.set_uv(self.button, uv);
        }
    }

    pub fn set_click(&mut self, systems: &mut DrawSetting, is_click: bool) {
        if self.is_click == is_click {
            return;
        }
        self.is_click = is_click;
        if self.is_click {
            systems
                .gfx
                .set_color(self.rect[0], Color::rgba(20, 20, 20, 255));
            let mut uv = systems.gfx.get_uv(self.button);
            uv.y = 48.0;
            systems.gfx.set_uv(self.button, uv);
        } else if self.is_hover {
            systems
                .gfx
                .set_color(self.rect[0], Color::rgba(55, 55, 55, 255));
            let mut uv = systems.gfx.get_uv(self.button);
            uv.y = 24.0;
            systems.gfx.set_uv(self.button, uv);
        } else {
            systems
                .gfx
                .set_color(self.rect[0], Color::rgba(35, 35, 35, 255));
            let mut uv = systems.gfx.get_uv(self.button);
            uv.y = 0.0;
            systems.gfx.set_uv(self.button, uv);
        }
    }

    pub fn click_list(
        &mut self,
        systems: &mut DrawSetting,
        mouse_pos: Vec2,
    ) -> Option<usize> {
        let mut found_button = None;
        for (index, list_text) in self.list_text.iter().enumerate() {
            let (pos, size) = (
                systems.gfx.get_pos(list_text.rect),
                systems.gfx.get_size(list_text.rect),
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

    pub fn hover_list(&mut self, systems: &mut DrawSetting, mouse_pos: Vec2) {
        // We check if buttons are within the mouse position
        self.list_text.iter_mut().for_each(|list_text| {
            let (pos, size) = (
                systems.gfx.get_pos(list_text.rect),
                systems.gfx.get_size(list_text.rect),
            );
            if (mouse_pos.x) >= pos.x
                && (mouse_pos.x) <= pos.x + size.x
                && (mouse_pos.y) >= pos.y
                && (mouse_pos.y) <= pos.y + size.y
            {
                list_text.set_hover(systems, true);
            } else {
                list_text.set_hover(systems, false);
            }
        });
    }

    pub fn switch_list(&mut self, systems: &mut DrawSetting, index: usize) {
        let list_index = index + self.start_index;
        if list_index == self.selected_index {
            return;
        }
        self.selected_index = list_index;
        systems.gfx.set_text(
            &mut systems.renderer,
            self.text,
            &self.list[self.selected_index],
        );
    }
}
