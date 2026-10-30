use gpui_kit::{
    Context, Edges, IntoElement, ParentElement, Render, Styled, Window,
    base::{StyledExt, h_flex, v_flex},
    component::{
        ActiveTheme, WindowExt,
        button::{Button, ButtonVariants},
        input::Input,
        list::List,
        select::Select,
    },
    px,
};

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
                        let datas = editor
                            .data
                            .iter()
                            .enumerate()
                            .filter(|e| e.1.1)
                            .map(|e| e.0)
                            .collect();
                        let _ = editor
                            .command_handler
                            .tx
                            .blocking_send(MessageCommands::SaveAll(datas));

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
        v_flex()
            .items_center()
            .paddings(Edges::all(px(10.)))
            .gap(px(5.))
            .size_full()
            .bg(cx.theme().secondary)
            .child(self.render_data_selection(window, cx))
            .child("")
            .child(
                h_flex()
                    .w_full()
                    .gap(px(10.))
                    .child(
                        h_flex()
                            .w_full()
                            .gap(px(10.))
                            .child("Name")
                            .child(Input::new(&self.name_input)),
                    )
                    .child(
                        h_flex()
                            .gap(px(10.))
                            .child("Max Item")
                            .child(Input::new(&self.max_item).w(px(100.))),
                    ),
            )
            .child(
                List::new(&self.shop_list)
                    .scrollbar_visible(true)
                    .max_h(px(400.))
                    .h(px(400.)),
            )
            .child(
                h_flex()
                    .gap(px(10.))
                    .child(
                        h_flex()
                            .gap(px(10.))
                            .child("Index:")
                            .child(Input::new(&self.shop_index).w(px(100.))),
                    )
                    .child(
                        h_flex()
                            .gap(px(10.))
                            .child("Amount:")
                            .child(Input::new(&self.shop_amount).w(px(100.))),
                    )
                    .child(
                        h_flex()
                            .gap(px(10.))
                            .child("Price:")
                            .child(Input::new(&self.shop_price).w(px(100.))),
                    ),
            )
    }
}
