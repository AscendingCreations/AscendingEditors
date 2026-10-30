use database::item_parser::data_labels;
use gpui_kit::{
    Context, ImageSource, IntoElement, ParentElement, Render, Resource, Styled, Window,
    base::{h_flex, v_flex},
    component::{
        ActiveTheme, StyledExt, WindowExt,
        button::{Button, ButtonVariants},
        checkbox::Checkbox,
        input::Input,
        select::Select,
    },
    img, px,
};

use std::path::Path;

use crate::{commands::MessageCommands, editor::Editor};

impl Editor {
    pub fn render_data_selection(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        h_flex()
            .w_full()
            .gap(px(10.))
            .child("List:")
            .child(Select::new(&self.data_list))
            .child(
                Button::new("Save")
                    .primary()
                    .label("Save")
                    .on_click(cx.listener(move |editor, _ev, wn, cx| {
                        let _ = editor
                            .command_handler
                            .tx
                            .blocking_send(MessageCommands::Save(editor.index));

                        editor.update_list(wn, cx);
                        wn.push_notification("Saved!", cx);
                    })),
            )
            .child(
                Button::new("Save All")
                    .primary()
                    .label("Save All")
                    .on_click(cx.listener(move |editor, _ev, wn, cx| {
                        let items = editor
                            .data
                            .iter()
                            .enumerate()
                            .filter(|e| e.1.1)
                            .map(|e| e.0)
                            .collect();
                        let _ = editor
                            .command_handler
                            .tx
                            .blocking_send(MessageCommands::SaveAll(items));

                        editor.update_list(wn, cx);
                        wn.push_notification("Saved!", cx);
                    })),
            )
            .child(
                Button::new("Revert")
                    .primary()
                    .label("Revert")
                    .on_click(cx.listener(move |editor, _ev, wn, cx| {
                        let _ = editor
                            .command_handler
                            .tx
                            .blocking_send(MessageCommands::Revert(editor.index));

                        wn.push_notification("Reverted!", cx);
                    })),
            )
    }
}

impl Render for Editor {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let sprite = self.data[self.index].0.sprite;

        v_flex()
            .items_center()
            .paddings(px(10.))
            .gap(px(5.))
            .size_full()
            .bg(cx.theme().secondary)
            .child(self.render_data_selection(window, cx))
            .child("")
            .child(
                h_flex()
                    .gap(px(10.))
                    .w_full()
                    .child(
                        h_flex()
                            .w_full()
                            .gap(px(10.))
                            .child("Name:")
                            .child(Input::new(&self.name_input)),
                    )
                    .child(
                        h_flex()
                            .gap(px(10.))
                            .child("Item Type:")
                            .child(Select::new(&self.itemtype_list).w(px(150.))),
                    )
                    .child(
                        h_flex()
                            .gap(px(10.))
                            .child("Item Type 2:")
                            .child(Input::new(&self.itemtype2_input).w(px(80.))),
                    ),
            )
            .child(
                h_flex()
                    .gap(px(10.))
                    .child(
                        v_flex()
                            .gap(px(5.))
                            .child("Sprite:")
                            .child(Input::new(&self.sprite_input).w(px(80.))),
                    )
                    .child(
                        img(ImageSource::Resource(Resource::Path(
                            Path::new(&format!("img/items/i{sprite}.png")).into(),
                        )))
                        .w(px(40.))
                        .h(px(40.)),
                    ),
            )
            .child(
                h_flex()
                    .gap(px(10.))
                    .child(
                        v_flex()
                            .gap(px(5.))
                            .child("Level Req:")
                            .child(Input::new(&self.level_req_input).w(px(100.))),
                    )
                    .child(
                        v_flex()
                            .gap(px(5.))
                            .child("Sound ID:")
                            .child(Input::new(&self.sound_id_input).w(px(100.))),
                    )
                    .child(
                        v_flex()
                            .gap(px(5.))
                            .child("Stack Limit:")
                            .child(Input::new(&self.stack_limit_input).w(px(100.))),
                    )
                    .child(
                        v_flex()
                            .gap(px(5.))
                            .child("Base Price:")
                            .child(Input::new(&self.base_price_input).w(px(100.))),
                    )
                    .child(
                        v_flex()
                            .gap(px(5.))
                            .child("Animation:")
                            .child(Input::new(&self.animation_input).w(px(100.))),
                    ),
            )
            .child(
                h_flex()
                    .gap(px(10.))
                    .child("Sound Name:")
                    .child(Input::new(&self.sound_input).w(px(350.))),
            )
            .child(
                h_flex()
                    .gap(px(10.))
                    .child(
                        Checkbox::new("breakable")
                            .label("Breakable")
                            .checked(self.breakable)
                            .on_click(cx.listener(|view, checked, _, cx| {
                                view.breakable = *checked;
                                view.data[view.index].0.breakable = *checked;
                                view.data[view.index].1 = true;
                                view.save_temp_file();

                                cx.notify();
                            })),
                    )
                    .child(
                        Checkbox::new("repariable")
                            .label("Repairable")
                            .checked(self.repairable)
                            .on_click(cx.listener(|view, checked, _, cx| {
                                view.repairable = *checked;
                                view.data[view.index].0.repairable = *checked;
                                view.data[view.index].1 = true;
                                view.save_temp_file();

                                cx.notify();
                            })),
                    )
                    .child(
                        Checkbox::new("stackable")
                            .label("Stackable")
                            .checked(self.stackable)
                            .on_click(cx.listener(|view, checked, _, cx| {
                                view.stackable = *checked;
                                view.data[view.index].0.stackable = *checked;
                                view.data[view.index].1 = true;
                                view.save_temp_file();

                                cx.notify();
                            })),
                    ),
            )
            .child("")
            .child("- Data Inputs -")
            .child(
                h_flex()
                    .gap(px(10.))
                    .child(
                        v_flex()
                            .gap(px(5.))
                            .child(data_labels(0, self.data[self.index].0.itemtype))
                            .child(Input::new(&self.data_input[0]).w(px(120.))),
                    )
                    .child(
                        v_flex()
                            .gap(px(5.))
                            .child(data_labels(1, self.data[self.index].0.itemtype))
                            .child(Input::new(&self.data_input[1]).w(px(120.))),
                    )
                    .child(
                        v_flex()
                            .gap(px(5.))
                            .child(data_labels(2, self.data[self.index].0.itemtype))
                            .child(Input::new(&self.data_input[2]).w(px(120.))),
                    )
                    .child(
                        v_flex()
                            .gap(px(5.))
                            .child(data_labels(3, self.data[self.index].0.itemtype))
                            .child(Input::new(&self.data_input[3]).w(px(120.))),
                    )
                    .child(
                        v_flex()
                            .gap(px(5.))
                            .child(data_labels(4, self.data[self.index].0.itemtype))
                            .child(Input::new(&self.data_input[4]).w(px(120.))),
                    )
                    .child(
                        v_flex()
                            .gap(px(5.))
                            .child(data_labels(5, self.data[self.index].0.itemtype))
                            .child(Input::new(&self.data_input[5]).w(px(120.))),
                    ),
            )
            .child(
                h_flex()
                    .gap(px(10.))
                    .child(
                        v_flex()
                            .gap(px(5.))
                            .child(data_labels(6, self.data[self.index].0.itemtype))
                            .child(Input::new(&self.data_input[6]).w(px(120.))),
                    )
                    .child(
                        v_flex()
                            .gap(px(5.))
                            .child(data_labels(7, self.data[self.index].0.itemtype))
                            .child(Input::new(&self.data_input[7]).w(px(120.))),
                    )
                    .child(
                        v_flex()
                            .gap(px(5.))
                            .child(data_labels(8, self.data[self.index].0.itemtype))
                            .child(Input::new(&self.data_input[8]).w(px(120.))),
                    )
                    .child(
                        v_flex()
                            .gap(px(5.))
                            .child(data_labels(9, self.data[self.index].0.itemtype))
                            .child(Input::new(&self.data_input[9]).w(px(120.))),
                    )
                    .child(
                        v_flex()
                            .gap(px(5.))
                            .child(data_labels(10, self.data[self.index].0.itemtype))
                            .child(Input::new(&self.data_input[10]).w(px(120.))),
                    )
                    .child(
                        v_flex()
                            .gap(px(5.))
                            .child(data_labels(11, self.data[self.index].0.itemtype))
                            .child(Input::new(&self.data_input[11]).w(px(120.))),
                    ),
            )
            .child(
                h_flex()
                    .gap(px(10.))
                    .child(
                        v_flex()
                            .gap(px(5.))
                            .child(data_labels(12, self.data[self.index].0.itemtype))
                            .child(Input::new(&self.data_input[12]).w(px(120.))),
                    )
                    .child(
                        v_flex()
                            .gap(px(5.))
                            .child(data_labels(13, self.data[self.index].0.itemtype))
                            .child(Input::new(&self.data_input[13]).w(px(120.))),
                    )
                    .child(
                        v_flex()
                            .gap(px(5.))
                            .child(data_labels(14, self.data[self.index].0.itemtype))
                            .child(Input::new(&self.data_input[14]).w(px(120.))),
                    )
                    .child(
                        v_flex()
                            .gap(px(5.))
                            .child(data_labels(15, self.data[self.index].0.itemtype))
                            .child(Input::new(&self.data_input[15]).w(px(120.))),
                    )
                    .child(
                        v_flex()
                            .gap(px(5.))
                            .child(data_labels(16, self.data[self.index].0.itemtype))
                            .child(Input::new(&self.data_input[16]).w(px(120.))),
                    )
                    .child(
                        v_flex()
                            .gap(px(5.))
                            .child(data_labels(17, self.data[self.index].0.itemtype))
                            .child(Input::new(&self.data_input[17]).w(px(120.))),
                    ),
            )
            .child(
                h_flex()
                    .gap(px(10.))
                    .child(
                        v_flex()
                            .gap(px(5.))
                            .child(data_labels(18, self.data[self.index].0.itemtype))
                            .child(Input::new(&self.data_input[18]).w(px(120.))),
                    )
                    .child(
                        v_flex()
                            .gap(px(5.))
                            .child(data_labels(19, self.data[self.index].0.itemtype))
                            .child(Input::new(&self.data_input[19]).w(px(120.))),
                    ),
            )
    }
}
