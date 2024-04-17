use cosmic_text::{Attrs, Metrics};
use graphics::*;
use wgpu::core::device::resource;

use crate::{
    collection::*,
    interface::{label::*, scrollbar::*},
    resource::*,
    DrawSetting,
};

const MAX_VISIBLE_LIST: u32 = 18;

pub struct SelectButton {
    pub image: usize,
    pub in_hover: bool,
    pub is_selected: bool,
}

impl SelectButton {
    pub fn set_hover(&mut self, systems: &mut DrawSetting, in_hover: bool) {
        if self.in_hover != in_hover {
            self.in_hover = in_hover;
            if !self.is_selected {
                let (size, mut uv) = (
                    systems.gfx.get_size(self.image),
                    systems.gfx.get_uv(self.image),
                );

                if in_hover {
                    uv.y = size.y;
                } else {
                    uv.y = 0.0;
                }
                systems.gfx.set_uv(self.image, uv);
            }
        }
    }

    pub fn set_select(&mut self, systems: &mut DrawSetting, is_select: bool) {
        if self.is_selected != is_select {
            self.is_selected = is_select;

            let (size, mut uv) = (
                systems.gfx.get_size(self.image),
                systems.gfx.get_uv(self.image),
            );

            if is_select {
                uv.y = size.y * 2.0;
            } else {
                uv.y = 0.0;
            }
            systems.gfx.set_uv(self.image, uv);
        }
    }
}

pub struct TilesetList {
    pub visible: bool,
    pub bg: Vec<usize>,
    pub selection_buttons: Vec<SelectButton>,
    pub texts: Vec<usize>,
    start_view_index: usize,
    pub selected_tileset: usize,
    view_index: Option<usize>,
    pub scrollbar: Scrollbar,
}

impl TilesetList {
    pub fn new(systems: &mut DrawSetting) -> Self {
        let mut bg1 = Rect::new(&mut systems.renderer, 0);
        bg1.set_size(Vec2::new(200.0, 400.0))
            .set_position(Vec3::new(11.0, 369.0, ORDER_TILESETLIST))
            .set_color(Color::rgba(50, 50, 50, 255))
            .set_use_camera(true);
        let mut bg2 = Rect::new(&mut systems.renderer, 0);
        bg2.set_size(Vec2::new(8.0, 377.0))
            .set_position(Vec3::new(200.0, 381.0, ORDER_TILESETLIST_SCROLL_BG))
            .set_color(Color::rgba(30, 30, 30, 255))
            .set_use_camera(true);

        // Tileset List and Button
        // This limit the amount of item on the list if tileset count is lower than the visible count
        // Note: If the tileset count is more than the visible count, we will limit the items with the visible count
        let max_view = std::cmp::min(
            systems.resource.tilesheet.len() as u32,
            MAX_VISIBLE_LIST,
        ) as usize;
        let mut texts = Vec::with_capacity(max_view);
        let mut selection_buttons = Vec::with_capacity(max_view);
        for index in 0..max_view {
            // Create the selectable buttons
            let mut image = Image::new(
                Some(systems.resource.tileset_list_select.allocation),
                &mut systems.renderer,
                0,
            );
            image.pos = Vec3::new(
                bg1.position.x + 3.0,
                bg1.position.y + 369.0 - (21.0 * index as f32),
                ORDER_TILESETLIST_BUTTON,
            );
            image.hw = Vec2::new(183.0, 20.0);
            image.uv = Vec4::new(0.0, 0.0, 183.0, 20.0);
            let button = SelectButton {
                image: systems.gfx.add_image(image, 0),
                in_hover: false,
                is_selected: false,
            };
            systems.gfx.set_visible(button.image, false);

            selection_buttons.push(button);

            // Create the text
            let mut text = create_basic_label(
                systems,
                Vec3::new(
                    bg1.position.x + 7.0,
                    bg1.position.y + 369.0 - (21.0 * index as f32),
                    ORDER_TILESETLIST_LABEL,
                ),
                Vec2::new(100.0, 20.0),
                Color::rgba(180, 180, 180, 255),
            );
            text.set_text(
                &mut systems.renderer,
                &systems.resource.tilesheet[index].name,
                Attrs::new(),
                Shaping::Advanced,
            );

            let index = systems.gfx.add_text(text, 1);
            systems.gfx.set_visible(index, false);

            texts.push(index);
        }

        // Scrollbar
        let max_tileset = systems.resource.tilesheet.len() as u32;
        let scrollbar_value =
            max_tileset.max(MAX_VISIBLE_LIST) - MAX_VISIBLE_LIST;
        let scrollbar = Scrollbar::new(
            systems,
            Vec3::new(
                bg1.position.x + 188.0,
                bg1.position.y + 389.0,
                ORDER_TILESETLIST_SCROLLBAR,
            ),
            scrollbar_value as usize,
            377,
            20,
            0,
        );

        // We set the default selected tileset
        selection_buttons[0].set_select(systems, true);

        let bg =
            vec![systems.gfx.add_rect(bg1, 0), systems.gfx.add_rect(bg2, 0)];
        systems.gfx.set_visible(bg[0], false);
        systems.gfx.set_visible(bg[1], false);

        Self {
            visible: false,
            bg,
            selection_buttons,
            texts,
            start_view_index: 0, // We will use this to adjust the visible item on the list
            selected_tileset: 0,
            view_index: Some(0),
            scrollbar,
        }
    }

    pub fn select_list(
        &mut self,
        systems: &mut DrawSetting,
        mouse_pos: Vec2,
    ) -> bool {
        if !self.visible {
            return false;
        }
        if let Some(index) = self.selection_buttons.iter().position(|button| {
            let (pos, size) = (
                systems.gfx.get_pos(button.image),
                systems.gfx.get_size(button.image),
            );
            mouse_pos.x >= pos.x
                && mouse_pos.x <= pos.x + size.x
                && mouse_pos.y >= pos.y
                && mouse_pos.y <= pos.y + size.y
        }) {
            let tileset_index = self.start_view_index + index;
            if self.selected_tileset != tileset_index {
                if let Some(view_index) = self.view_index {
                    self.selection_buttons[view_index]
                        .set_select(systems, false);
                }
                self.selection_buttons[index].set_select(systems, true);
                self.selected_tileset = tileset_index;
                self.view_index = Some(index);
                return true;
            }
        }
        false
    }

    // We use this function to update the list when the start view index has been adjusted
    pub fn update_list(&mut self, systems: &mut DrawSetting) {
        if !self.visible {
            return;
        }
        self.view_index = None;
        let max_view = std::cmp::min(
            systems.resource.tilesheet.len() as u32,
            MAX_VISIBLE_LIST,
        ) as usize;
        for index in 0..max_view {
            let tileset_index = index + self.start_view_index;
            if self.selected_tileset == tileset_index {
                self.selection_buttons[index].set_select(systems, true);
                self.view_index = Some(index);
            } else {
                self.selection_buttons[index].set_select(systems, false);
            }
            systems.gfx.set_text(
                &mut systems.renderer,
                self.texts[index],
                &systems.resource.tilesheet[tileset_index].name,
            );
        }
    }

    pub fn update_scroll(&mut self, scroll_index: usize) -> bool {
        if !self.visible {
            return false;
        }
        if self.start_view_index != scroll_index {
            self.start_view_index = scroll_index;
            return true;
        }
        false
    }

    pub fn hover_selection(
        &mut self,
        systems: &mut DrawSetting,
        mouse_pos: Vec2,
    ) {
        if !self.visible {
            return;
        }
        // We check if buttons are within the mouse position
        for index in 0..self.selection_buttons.len() {
            let (pos, size) = (
                systems.gfx.get_pos(self.selection_buttons[index].image),
                systems.gfx.get_size(self.selection_buttons[index].image),
            );
            if (mouse_pos.x) >= pos.x
                && (mouse_pos.x) <= pos.x + size.x
                && (mouse_pos.y) >= pos.y
                && (mouse_pos.y) <= pos.y + size.y
            {
                self.selection_buttons[index].set_hover(systems, true);
            } else {
                self.selection_buttons[index].set_hover(systems, false);
            }
        }
    }

    pub fn show(&mut self, systems: &mut DrawSetting) {
        if self.visible {
            return;
        }
        self.visible = true;
        systems.gfx.set_visible(self.bg[0], true);
        systems.gfx.set_visible(self.bg[1], true);
        self.texts.iter().for_each(|text| {
            systems.gfx.set_visible(*text, true);
        });

        self.scrollbar.show(systems);
        self.selection_buttons.iter_mut().for_each(|button| {
            systems.gfx.set_visible(button.image, true);
        });
    }

    pub fn hide(&mut self, systems: &mut DrawSetting) {
        self.visible = false;
        self.scrollbar.hide(systems);
        systems.gfx.set_visible(self.bg[0], false);
        systems.gfx.set_visible(self.bg[1], false);
        self.texts.iter().for_each(|text| {
            systems.gfx.set_visible(*text, false);
        });
        self.selection_buttons.iter_mut().for_each(|button| {
            systems.gfx.set_visible(button.image, false);
        });
    }
}
