use cosmic_text::{Attrs, Metrics};
use graphics::*;

use crate::{
    collection::*,
    interface::{button::*, label::*, textbox::*},
    textbox, DrawSetting,
};

pub struct ColorEditor {
    pub is_open: bool,
    pub window: usize,
    pub label: Vec<usize>,
    pub textbox: Vec<Textbox>,
    pub button: Button,
    pub data: [u8; 4],
}

impl ColorEditor {
    pub fn new(
        systems: &mut DrawSetting,
        pos: Vec2,
        z_order: [f32; 3],
        data: [u8; 4],
        can_edit_alpha: bool,
        render_layer: [usize; 2],
    ) -> Self {
        let window_size = if can_edit_alpha {
            Vec2::new(100.0, 153.0)
        } else {
            Vec2::new(100.0, 128.0)
        };
        let window_pos = Vec3::new(pos.x, pos.y - window_size.y, z_order[0]);
        let mut wndw = Rect::new(&mut systems.renderer, 0);
        wndw.set_size(window_size)
            .set_position(window_pos)
            .set_color(Color::rgba(70, 70, 70, 255))
            .set_radius(3.0)
            .set_border_width(2.0)
            .set_border_color(Color::rgba(20, 20, 20, 255))
            .set_use_camera(true);
        let window = systems.gfx.add_rect(wndw, render_layer[0]);
        systems.gfx.set_visible(window, false);

        let content_pos = Vec2::new(
            window_pos.x + 10.0,
            (window_pos.y + window_size.y) - 9.0,
        );
        let mut label = vec![];
        let mut textbox = vec![];

        let lbl = create_basic_label(
            systems,
            Vec3::new(
                content_pos.x,
                content_pos.y - 25.0,
                ORDER_COLOREDIT_TEXTBOX_TEXT,
            ),
            Vec2::new(20.0, 20.0),
            Color::rgba(180, 180, 180, 255),
        );
        label.push(systems.gfx.add_text(lbl, render_layer[1]));
        systems.gfx.set_text(&mut systems.renderer, label[0], "R");
        systems.gfx.set_visible(label[0], false);
        let lbl = create_basic_label(
            systems,
            Vec3::new(
                content_pos.x,
                content_pos.y - 50.0,
                ORDER_COLOREDIT_TEXTBOX_TEXT,
            ),
            Vec2::new(20.0, 20.0),
            Color::rgba(180, 180, 180, 255),
        );
        label.push(systems.gfx.add_text(lbl, render_layer[1]));
        systems.gfx.set_text(&mut systems.renderer, label[1], "G");
        systems.gfx.set_visible(label[1], false);
        let lbl = create_basic_label(
            systems,
            Vec3::new(
                content_pos.x,
                content_pos.y - 75.0,
                ORDER_COLOREDIT_TEXTBOX_TEXT,
            ),
            Vec2::new(20.0, 20.0),
            Color::rgba(180, 180, 180, 255),
        );
        label.push(systems.gfx.add_text(lbl, render_layer[1]));
        systems.gfx.set_text(&mut systems.renderer, label[2], "B");
        systems.gfx.set_visible(label[2], false);

        textbox.push(Textbox::new(
            systems,
            Vec3::new(
                content_pos.x + 20.0,
                content_pos.y - 25.0,
                ORDER_COLOREDIT_TEXTBOX,
            ),
            Vec2::new(60.0, 24.0),
            false,
            render_layer,
        ));
        textbox[0].input_text(systems, data[0].to_string());
        textbox[0].set_visible(systems, false);
        systems.gfx.set_visible(textbox[0].text, false);
        textbox.push(Textbox::new(
            systems,
            Vec3::new(
                content_pos.x + 20.0,
                content_pos.y - 50.0,
                ORDER_COLOREDIT_TEXTBOX,
            ),
            Vec2::new(60.0, 24.0),
            false,
            render_layer,
        ));
        textbox[1].input_text(systems, data[1].to_string());
        textbox[1].set_visible(systems, false);
        systems.gfx.set_visible(textbox[1].text, false);
        textbox.push(Textbox::new(
            systems,
            Vec3::new(
                content_pos.x + 20.0,
                content_pos.y - 75.0,
                ORDER_COLOREDIT_TEXTBOX,
            ),
            Vec2::new(60.0, 24.0),
            false,
            render_layer,
        ));
        textbox[2].input_text(systems, data[2].to_string());
        textbox[2].set_visible(systems, false);
        systems.gfx.set_visible(textbox[2].text, false);

        if can_edit_alpha {
            let lbl = create_basic_label(
                systems,
                Vec3::new(
                    content_pos.x,
                    content_pos.y - 100.0,
                    ORDER_COLOREDIT_TEXTBOX_TEXT,
                ),
                Vec2::new(20.0, 20.0),
                Color::rgba(180, 180, 180, 255),
            );
            label.push(systems.gfx.add_text(lbl, render_layer[1]));
            systems.gfx.set_text(&mut systems.renderer, label[3], "A");
            systems.gfx.set_visible(label[3], false);

            textbox.push(Textbox::new(
                systems,
                Vec3::new(
                    content_pos.x + 20.0,
                    content_pos.y - 100.0,
                    ORDER_COLOREDIT_TEXTBOX,
                ),
                Vec2::new(60.0, 24.0),
                false,
                render_layer,
            ));
            textbox[3].input_text(systems, data[3].to_string());
            textbox[3].set_visible(systems, false);
            systems.gfx.set_visible(textbox[3].text, false);
        }

        let button = Button::new(
            systems,
            systems.resource.preference_button.allocation,
            "Apply",
            Vec2::new(window_pos.x + 10.0, window_pos.y + 10.0),
            Vec2::new(80.0, 22.0),
            [ORDER_COLOREDIT_BUTTON, ORDER_COLOREDIT_BUTTON_LABEL],
            2.0,
            render_layer,
        );
        systems.gfx.set_visible(button.image, false);
        systems.gfx.set_visible(button.text, false);

        Self {
            is_open: false,
            window,
            data,
            label,
            textbox,
            button,
        }
    }

    pub fn open(&mut self, systems: &mut DrawSetting) {
        if self.is_open {
            return;
        }
        self.is_open = true;
        systems.gfx.set_visible(self.window, true);
        self.label.iter_mut().for_each(|label| {
            systems.gfx.set_visible(*label, true);
        });
        systems.gfx.set_visible(self.button.image, true);
        systems.gfx.set_visible(self.button.text, true);
        self.textbox.iter_mut().for_each(|textbox| {
            textbox.set_visible(systems, true);
            systems.gfx.set_visible(textbox.text, true);
            textbox.set_select(systems, false);
        });
    }

    pub fn close(&mut self, systems: &mut DrawSetting) {
        if !self.is_open {
            return;
        }
        self.is_open = false;
        self.textbox.iter_mut().for_each(|textbox| {
            textbox.set_visible(systems, false);
            systems.gfx.set_visible(textbox.text, false);
        });
        self.label.iter_mut().for_each(|label| {
            systems.gfx.set_visible(*label, false);
        });
        systems.gfx.set_visible(self.button.image, false);
        systems.gfx.set_visible(self.button.text, false);
        systems.gfx.set_visible(self.window, false);
    }

    pub fn unload(&mut self, systems: &mut DrawSetting) {
        self.textbox.iter_mut().for_each(|textbox| {
            systems.gfx.remove_gfx(textbox.image);
            systems.gfx.remove_gfx(textbox.text);
        });
        self.label.iter_mut().for_each(|label| {
            systems.gfx.remove_gfx(*label);
        });
        systems.gfx.remove_gfx(self.button.image);
        systems.gfx.remove_gfx(self.button.text);
        systems.gfx.remove_gfx(self.window);
    }
}

pub struct ColorSelection {
    pub image: usize,
    pub text: usize,
    is_hover: bool,

    pub color_editor: ColorEditor,
}

impl ColorSelection {
    pub fn new(
        systems: &mut DrawSetting,
        pos: Vec3,
        size: Vec2,
        color: [u8; 4],
        msg: Option<&str>,
        can_edit_alpha: bool,
        render_layer: [usize; 2],
    ) -> Self {
        let mut img = Rect::new(&mut systems.renderer, 0);
        img.set_size(size)
            .set_position(Vec3::new(pos.x, pos.y, pos.z))
            .set_color(Color::rgba(color[0], color[1], color[2], color[3]))
            .set_radius(3.0)
            .set_border_width(2.0)
            .set_border_color(Color::rgba(20, 20, 20, 255))
            .set_use_camera(true);
        let image = systems.gfx.add_rect(img, render_layer[0]);
        systems.gfx.set_visible(image, false);

        let text_pos = Vec2::new(pos.x + size.x + 10.0, pos.y);
        let mut txt = create_basic_label(
            systems,
            Vec3::new(text_pos.x, text_pos.y, pos.z),
            Vec2::new(100.0, 20.0),
            Color::rgba(180, 180, 180, 255),
        );

        if let Some(msg) = msg {
            txt.set_text(
                &mut systems.renderer,
                msg,
                Attrs::new(),
                Shaping::Advanced,
            );
            txt.set_bounds(Some(Bounds::new(
                text_pos.x,
                text_pos.y,
                text_pos.x + txt.measure().x,
                text_pos.y + 20.0,
            )));
        };
        let text = systems.gfx.add_text(txt, render_layer[1]);
        systems.gfx.set_visible(text, false);

        let color_editor = ColorEditor::new(
            systems,
            Vec2::new(pos.x, pos.y),
            [
                ORDER_COLOREDIT_WINDOW,
                ORDER_COLOREDIT_TEXTBOX,
                ORDER_COLOREDIT_TEXTBOX_TEXT,
            ],
            color,
            can_edit_alpha,
            render_layer,
        );

        Self {
            image,
            text,
            is_hover: false,
            color_editor,
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
                .set_border_color(self.image, Color::rgba(200, 200, 200, 255));
        } else {
            systems
                .gfx
                .set_border_color(self.image, Color::rgba(20, 20, 20, 255));
        }
    }

    pub fn open_color_editor(&mut self, systems: &mut DrawSetting) {
        self.color_editor.open(systems);
    }

    pub fn close_color_editor(&mut self, systems: &mut DrawSetting) {
        self.color_editor.close(systems);
    }

    pub fn unload(&mut self, systems: &mut DrawSetting) {
        self.color_editor.unload(systems);
        systems.gfx.remove_gfx(self.image);
        systems.gfx.remove_gfx(self.text);
    }
}
