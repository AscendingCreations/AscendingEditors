use cosmic_text::{Attrs, Metrics};
use graphics::*;
use indexmap::IndexMap;

use crate::collection::ZOOM_LEVEL;

use crate::{
    collection::*,
    interface::{label::*, scrollbar::*, textbox::*},
    DrawSetting,
};

#[derive(Clone, PartialEq, Eq)]
pub enum DialogType {
    None,
    ExitConfirm,
    MapSave,
    MapLoad,
}

#[derive(Clone, PartialEq, Eq)]
pub enum DialogButtonType {
    None,
    Confirm,
    Decline,
    Cancel,
}

#[derive(Debug)]
pub enum DialogData {
    DataNone,
    MapLocation((i32, i32, i64)),
    MapList(IndexMap<String, (i32, i32, i64)>),
}

pub struct DialogButton {
    pub image: usize,
    pub text: usize,
    pub button_type: DialogButtonType,
    in_hover: bool,
    in_click: bool,
}

impl DialogButton {
    pub fn new(
        systems: &mut DrawSetting,
        message: &str,
        pos: Vec2,
        text_size: Vec2,
        button_type: DialogButtonType,
    ) -> Self {
        let mut img = Image::new(
            Some(systems.resource.dialog_button.allocation),
            &mut systems.renderer,
            1,
        );
        img.pos = Vec3::new(pos.x, pos.y, ORDER_DIALOG_BUTTON);
        img.hw = Vec2::new(103.0, 36.0);
        img.uv = Vec4::new(0.0, 0.0, 103.0, 36.0);
        let image = systems.gfx.add_image(img, 2);

        let adjust_x = 51.0 - (text_size.x * 0.5).floor();
        let mut txt = create_label(
            systems,
            Vec3::new(pos.x + adjust_x, pos.y + 8.0, ORDER_DIALOG_BUTTON_TEXT),
            Vec2::new(text_size.x, text_size.y),
            Bounds::new(pos.x, pos.y + 8.0, pos.x + 103.0, pos.y + 28.0),
            Color::rgba(200, 200, 200, 255),
        );
        txt.set_text(
            &mut systems.renderer,
            message,
            Attrs::new(),
            Shaping::Advanced,
        );
        // Adjust text x position
        let message_size = txt.measure();
        txt.pos.x = pos.x + (51.0 - (message_size.x * 0.5)).floor();
        txt.changed = true;
        let text = systems.gfx.add_text(txt, 3);

        Self {
            image,
            text,
            button_type,
            in_hover: false,
            in_click: false,
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
                uv.y = 36.0;
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
            let (mut txtpos, mut uv, imgpos) = (
                systems.gfx.get_pos(self.text),
                systems.gfx.get_uv(self.image),
                systems.gfx.get_pos(self.image),
            );
            uv.y = 72.0;
            txtpos.y = imgpos.y + 6.0;
            systems.gfx.set_uv(self.image, uv);
            systems.gfx.set_pos(self.text, txtpos);
        } else {
            let (mut txtpos, mut uv, imgpos) = (
                systems.gfx.get_pos(self.text),
                systems.gfx.get_uv(self.image),
                systems.gfx.get_pos(self.image),
            );
            if !self.in_hover {
                uv.y = 0.0;
            } else {
                uv.y = 36.0;
            }
            txtpos.y = imgpos.y + 8.0;
            systems.gfx.set_uv(self.image, uv);
            systems.gfx.set_pos(self.text, txtpos);
        }
    }
}

pub struct Dialog {
    pub dialog_type: DialogType,
    pub bg: usize,
    pub window: usize,
    pub buttons: Vec<DialogButton>,
    pub message: usize,
    did_click: bool,
    // Content Data
    pub content_image: Vec<usize>,
    pub content_text: Vec<usize>,
    pub editor_textbox: Vec<Textbox>,
    pub editor_data: Vec<String>,
    //pub editor_text: Vec<Text>,
    pub editing_index: usize,
    pub scrollbar: Scrollbar,
    start_view_index: usize, // Use for scrollbar
}

impl Dialog {
    pub fn new(
        systems: &mut DrawSetting,
        dialog_type: DialogType,
        data: Option<IndexMap<String, bool>>,
    ) -> Self {
        // This image is for the transparent shadow that will render behind the dialog
        let mut img = Rect::new(&mut systems.renderer, 0);
        img.set_position(Vec3::new(0.0, 0.0, ORDER_DIALOG_SHADOW))
            .set_size(Vec2::new(systems.size.width, systems.size.height))
            .set_color(Color::rgba(0, 0, 0, 200))
            .set_use_camera(true);
        let bg = systems.gfx.add_rect(img, 2);

        // Window and button position/size calculations
        let window_size = Vec2::new(
            match dialog_type {
                DialogType::ExitConfirm => 384.0,
                DialogType::MapSave => 456.0,
                DialogType::MapLoad => 456.0,
                _ => 384.0,
            },
            match dialog_type {
                DialogType::ExitConfirm => 108.0,
                DialogType::MapSave => 201.0,
                DialogType::MapLoad => 144.0,
                _ => 108.0,
            },
        );
        let window_pos = Vec2::new(
            ((systems.size.width / ZOOM_LEVEL) * 0.5) - (window_size.x * 0.5),
            ((systems.size.height / ZOOM_LEVEL) * 0.5) - (window_size.y * 0.5),
        )
        .floor();
        let message_pos_y = match dialog_type {
            DialogType::ExitConfirm => window_pos.y + 62.0,
            DialogType::MapSave => window_pos.y + 155.0,
            DialogType::MapLoad => window_pos.y + 98.0,
            _ => 62.0,
        };
        let button_pos = Vec2::new(
            match dialog_type {
                DialogType::ExitConfirm => window_pos.x + 84.0,
                DialogType::MapLoad => window_pos.x + 120.0,
                DialogType::MapSave => window_pos.x + 64.0,
                _ => window_pos.x + 84.0,
            },
            window_pos.y + 18.0,
        );

        // Buttons
        let buttons = match dialog_type {
            DialogType::ExitConfirm => {
                vec![
                    DialogButton::new(
                        systems,
                        "Yes",
                        button_pos,
                        Vec2::new(103.0, 20.0),
                        DialogButtonType::Confirm,
                    ),
                    DialogButton::new(
                        systems,
                        "No",
                        button_pos + Vec2::new(113.0, 0.0),
                        Vec2::new(103.0, 20.0),
                        DialogButtonType::Cancel,
                    ),
                ]
            }
            DialogType::MapSave => {
                vec![
                    DialogButton::new(
                        systems,
                        "Save",
                        button_pos,
                        Vec2::new(103.0, 20.0),
                        DialogButtonType::Confirm,
                    ),
                    DialogButton::new(
                        systems,
                        "Don't Save",
                        button_pos + Vec2::new(113.0, 0.0),
                        Vec2::new(103.0, 20.0),
                        DialogButtonType::Decline,
                    ),
                    DialogButton::new(
                        systems,
                        "Cancel",
                        button_pos + Vec2::new(226.0, 0.0),
                        Vec2::new(103.0, 20.0),
                        DialogButtonType::Cancel,
                    ),
                ]
            }
            DialogType::MapLoad => {
                vec![
                    DialogButton::new(
                        systems,
                        "Load",
                        button_pos,
                        Vec2::new(103.0, 20.0),
                        DialogButtonType::Confirm,
                    ),
                    DialogButton::new(
                        systems,
                        "Cancel",
                        button_pos + Vec2::new(113.0, 0.0),
                        Vec2::new(103.0, 20.0),
                        DialogButtonType::Cancel,
                    ),
                ]
            }
            _ => {
                vec![]
            }
        };

        // This will be the dialog window
        let mut wndw = Rect::new(&mut systems.renderer, 0);
        wndw.set_size(window_size)
            .set_position(Vec3::new(
                window_pos.x,
                window_pos.y,
                ORDER_DIALOG_WINDOW,
            ))
            .set_radius(3.0)
            .set_border_color(Color::rgba(10, 10, 10, 255))
            .set_border_width(2.0)
            .set_color(Color::rgba(50, 50, 50, 255))
            .set_use_camera(true);
        let window = systems.gfx.add_rect(wndw, 2);

        let msg = match dialog_type {
            DialogType::ExitConfirm => {
                "Are you sure that you want to close the editor?"
            }
            DialogType::MapSave => {
                "Would you like to save the changes to the following map/s?"
            }
            DialogType::MapLoad => {
                "Please enter the map location that you would like to load"
            }
            _ => "Error",
        };

        // Message
        let mut msg_text = create_label(
            systems,
            Vec3::new(300.0, message_pos_y, ORDER_DIALOG_MSG),
            Vec2::new(window_size.x, 20.0),
            Bounds::new(
                window_pos.x,
                message_pos_y,
                window_pos.x + window_size.x,
                message_pos_y + 20.0,
            ),
            Color::rgba(200, 200, 200, 255),
        ); // FPS
        msg_text.set_text(
            &mut systems.renderer,
            msg,
            Attrs::new(),
            Shaping::Advanced,
        );
        // Adjust message x position based on message text
        let message_size = msg_text.measure();
        msg_text.pos.x = window_pos.x
            + ((window_size.x * 0.5) - (message_size.x * 0.5)).floor();
        msg_text.changed = true;
        let message = systems.gfx.add_text(msg_text, 3);

        // Stored Data
        let editor_data = match dialog_type {
            DialogType::MapSave => {
                let list_data = data.unwrap();
                let mut text_data = Vec::with_capacity(list_data.len());
                for (key, value) in list_data.iter() {
                    if *value {
                        text_data.push(key.clone());
                    }
                }
                text_data
            }
            DialogType::MapLoad => {
                //vec![String::new(); 3]
                Vec::with_capacity(0)
            }
            _ => Vec::with_capacity(0),
        };

        // Content
        let mut scrollbar_x = window_pos.x;
        let content_image = match dialog_type {
            DialogType::MapSave => {
                let label_box_size = Vec2::new(364.0, 85.0);
                let label_box_pos = Vec2::new(
                    window_pos.x
                        + ((window_size.x * 0.5) - (label_box_size.x * 0.5))
                            .floor(),
                    window_pos.y + 65.0,
                );
                scrollbar_x = label_box_pos.x;
                let mut label_box = Rect::new(&mut systems.renderer, 0);
                label_box
                    .set_size(label_box_size)
                    .set_position(Vec3::new(
                        label_box_pos.x,
                        label_box_pos.y,
                        ORDER_DIALOG_CONTENT_IMG1,
                    ))
                    .set_color(Color::rgba(60, 60, 60, 255))
                    .set_use_camera(true);

                let mut scrollbar_box = Rect::new(&mut systems.renderer, 0);
                scrollbar_box
                    .set_size(Vec2::new(8.0, label_box_size.y - 4.0))
                    .set_position(Vec3::new(
                        label_box.position.x + 354.0,
                        label_box.position.y + 2.0,
                        ORDER_DIALOG_CONTENT_IMG2,
                    ))
                    .set_color(Color::rgba(40, 40, 40, 255))
                    .set_use_camera(true);
                vec![
                    systems.gfx.add_rect(label_box, 2),
                    systems.gfx.add_rect(scrollbar_box, 2),
                ]
            }
            _ => Vec::with_capacity(0),
        };
        let content_text = match dialog_type {
            DialogType::MapSave => {
                let mut data = Vec::with_capacity(4);
                for index in 0..4 {
                    let label_size = Vec2::new(362.0, 20.0);
                    let content_pos = Vec2::new(
                        window_pos.x
                            + ((window_size.x * 0.5) - (label_size.x * 0.5))
                                .floor(),
                        window_pos.y + 129.0 - (21.0 * index as f32),
                    )
                    .floor();
                    let mut text = create_label(
                        systems,
                        Vec3::new(
                            content_pos.x,
                            content_pos.y,
                            ORDER_DIALOG_CONTENT_TEXT,
                        ),
                        label_size,
                        Bounds::new(
                            content_pos.x,
                            content_pos.y,
                            content_pos.x + label_size.x - 14.0,
                            content_pos.y + 20.0,
                        ),
                        Color::rgba(200, 200, 200, 255),
                    ); // X
                    if index < editor_data.len() {
                        text.set_text(
                            &mut systems.renderer,
                            &editor_data[index],
                            Attrs::new(),
                            Shaping::Advanced,
                        );
                    } else {
                        text.set_text(
                            &mut systems.renderer,
                            "",
                            Attrs::new(),
                            Shaping::Advanced,
                        );
                    }
                    data.push(systems.gfx.add_text(text, 3));
                }
                data
            }
            DialogType::MapLoad => {
                // Text Size = X[10] Y[10] Group[45]
                let textbox_total_size = 240.0; // [10][5][50][5][10][5][50][5][45][5][50]
                let content_pos = Vec2::new(
                    window_pos.x
                        + ((window_size.x * 0.5) - (textbox_total_size * 0.5)),
                    window_pos.y + 66.0,
                )
                .floor();
                let mut mapx = create_label(
                    systems,
                    Vec3::new(
                        content_pos.x,
                        content_pos.y,
                        ORDER_DIALOG_CONTENT_TEXT,
                    ),
                    Vec2::new(window_size.x, 20.0),
                    Bounds::new(
                        content_pos.x,
                        content_pos.y,
                        content_pos.x + 10.0,
                        content_pos.y + 20.0,
                    ),
                    Color::rgba(200, 200, 200, 255),
                ); // X
                mapx.set_text(
                    &mut systems.renderer,
                    "X",
                    Attrs::new(),
                    Shaping::Advanced,
                );
                let mut mapy = create_label(
                    systems,
                    Vec3::new(
                        content_pos.x + 70.0,
                        content_pos.y,
                        ORDER_DIALOG_CONTENT_TEXT,
                    ),
                    Vec2::new(window_size.x, 20.0),
                    Bounds::new(
                        content_pos.x + 70.0,
                        content_pos.y,
                        content_pos.x + 80.0,
                        content_pos.y + 20.0,
                    ),
                    Color::rgba(200, 200, 200, 255),
                ); // Y
                mapy.set_text(
                    &mut systems.renderer,
                    "Y",
                    Attrs::new(),
                    Shaping::Advanced,
                );
                let mut mapgroup = create_label(
                    systems,
                    Vec3::new(
                        content_pos.x + 140.0,
                        content_pos.y,
                        ORDER_DIALOG_CONTENT_TEXT,
                    ),
                    Vec2::new(window_size.x, 20.0),
                    Bounds::new(
                        content_pos.x + 140.0,
                        content_pos.y,
                        content_pos.x + 185.0,
                        content_pos.y + 20.0,
                    ),
                    Color::rgba(200, 200, 200, 255),
                ); // Group
                mapgroup.set_text(
                    &mut systems.renderer,
                    "Group",
                    Attrs::new(),
                    Shaping::Advanced,
                );
                vec![
                    systems.gfx.add_text(mapx, 3),
                    systems.gfx.add_text(mapy, 3),
                    systems.gfx.add_text(mapgroup, 3),
                ]
            }
            _ => Vec::with_capacity(0),
        };

        // Textbox
        let editor_textbox = match dialog_type {
            DialogType::MapLoad => {
                let textbox_total_size = 240.0; // [10][50][5][10][50][5][45][50]
                let content_pos = Vec2::new(
                    window_pos.x
                        + ((window_size.x * 0.5) - (textbox_total_size * 0.5)),
                    window_pos.y + 66.0,
                )
                .floor();
                vec![
                    Textbox::new(
                        systems,
                        Vec3::new(
                            content_pos.x + 15.0,
                            content_pos.y,
                            ORDER_DIALOG_CONTENT_IMG1,
                        ),
                        Vec2::new(50.0, 24.0),
                        false,
                        [2, 3],
                    ),
                    Textbox::new(
                        systems,
                        Vec3::new(
                            content_pos.x + 85.0,
                            content_pos.y,
                            ORDER_DIALOG_CONTENT_IMG1,
                        ),
                        Vec2::new(50.0, 24.0),
                        false,
                        [2, 3],
                    ),
                    Textbox::new(
                        systems,
                        Vec3::new(
                            content_pos.x + 190.0,
                            content_pos.y,
                            ORDER_DIALOG_CONTENT_IMG1,
                        ),
                        Vec2::new(50.0, 24.0),
                        false,
                        [2, 3],
                    ),
                ]
            }
            _ => {
                vec![]
            }
        };

        // Handle Scrollbar data
        let mut scrollbar_amount = 0;
        if dialog_type == DialogType::MapSave && editor_data.len() > 4 {
            scrollbar_amount = editor_data.len() - 4;
        }
        let mut scrollbar = Scrollbar::new(
            systems,
            Vec3::new(
                scrollbar_x + 353.0,
                window_pos.y + 145.0,
                ORDER_DIALOG_SCROLLBAR,
            ),
            scrollbar_amount,
            75,
            5,
            2,
        );
        if dialog_type == DialogType::MapSave {
            scrollbar.show(systems);
        }

        Self {
            dialog_type,
            bg,
            message,
            window,
            buttons,
            did_click: false,
            content_image,
            content_text,
            editor_textbox,
            editor_data,
            editing_index: 0,
            scrollbar,
            start_view_index: 0,
        }
    }

    pub fn unload(&mut self, systems: &mut DrawSetting) {
        self.scrollbar.unload(systems);
        systems.gfx.remove_gfx(self.bg);
        systems.gfx.remove_gfx(self.message);
        systems.gfx.remove_gfx(self.window);
        self.editor_textbox.iter().for_each(|textbox| {
            systems.gfx.remove_gfx(textbox.image);
            systems.gfx.remove_gfx(textbox.text);
        });
        self.buttons.iter().for_each(|button| {
            systems.gfx.remove_gfx(button.image);
            systems.gfx.remove_gfx(button.text);
        });
        self.content_image.iter().for_each(|image| {
            systems.gfx.remove_gfx(*image);
        });
        self.content_text.iter().for_each(|text| {
            systems.gfx.remove_gfx(*text);
        });
    }

    pub fn hover_buttons(
        &mut self,
        systems: &mut DrawSetting,
        mouse_pos: Vec2,
    ) {
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

    pub fn release_click(&mut self, systems: &mut DrawSetting) {
        if !self.did_click {
            return;
        }

        self.buttons.iter_mut().for_each(|button| {
            button.set_click(systems, false);
        });
    }

    pub fn click_buttons(
        &mut self,
        systems: &mut DrawSetting,
        mouse_pos: Vec2,
    ) -> DialogButtonType {
        let mut button_type = DialogButtonType::None;
        if let Some(buttons) = self.buttons.iter_mut().find(|button| {
            let (pos, size) = (
                systems.gfx.get_pos(button.image),
                systems.gfx.get_size(button.image),
            );
            (mouse_pos.x) >= pos.x
                && (mouse_pos.x) <= pos.x + size.x
                && (mouse_pos.y) >= pos.y
                && (mouse_pos.y) <= pos.y + size.y
        }) {
            buttons.set_click(systems, true);
            button_type = buttons.button_type.clone();
        }
        if button_type != DialogButtonType::None {
            self.did_click = true;
        }
        button_type
    }

    pub fn select_text(&mut self, systems: &mut DrawSetting, mouse_pos: Vec2) {
        if self.dialog_type != DialogType::MapLoad {
            return;
        }

        let last_selected = self.editing_index;
        let mut selected_index = -1;
        for (index, textbox) in self.editor_textbox.iter_mut().enumerate() {
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
            self.editor_textbox[last_selected].set_select(systems, false);
        }
        self.editing_index = selected_index as usize;
    }

    pub fn update_list(&mut self, systems: &mut DrawSetting) {
        for index in 0..4 {
            let text_index = index + self.start_view_index;
            if text_index < self.editor_data.len() {
                systems.gfx.set_text(
                    &mut systems.renderer,
                    self.content_text[index],
                    &self.editor_data[text_index],
                );
            }
        }
    }

    pub fn update_scroll(&mut self, scroll_index: usize) -> bool {
        if self.start_view_index != scroll_index {
            self.start_view_index = scroll_index;
            return true;
        }
        false
    }
}
