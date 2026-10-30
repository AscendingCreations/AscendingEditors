use gpui_kit::{
    Context, Edges, IntoElement, ParentElement, Render, Styled, Window,
    base::{StyledExt, h_flex, v_flex},
    component::{
        ActiveTheme, WindowExt,
        button::{Button, ButtonVariants},
        checkbox::Checkbox,
        input::Input,
        select::Select,
        tab::{Tab, TabBar},
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
            .child(
                TabBar::new("tabs")
                    .selected_index(self.cur_tab)
                    .on_click(cx.listener(|view, index, _, cx| {
                        view.cur_tab = *index;
                        cx.notify();
                    }))
                    .child(Tab::new().label("General"))
                    .child(Tab::new().label("Settings"))
                    .child(Tab::new().label("Drops"))
                    .child(Tab::new().label("Enemies")),
            )
            .child("")
            .child(match self.cur_tab {
                1 => h_flex()
                    .gap(px(10.))
                    .child(
                        v_flex()
                            .gap(px(5.))
                            .child(
                                Checkbox::new("target-auto-switch")
                                    .label("Target Auto Switch")
                                    .checked(self.target_auto_switch)
                                    .on_click(cx.listener(|view, checked, _, cx| {
                                        view.target_auto_switch = *checked;
                                        view.data[view.index].0.target_auto_switch = *checked;
                                        view.data[view.index].1 = true;
                                        view.save_temp_file();

                                        cx.notify();
                                    })),
                            )
                            .child(
                                Checkbox::new("target-attack-switch")
                                    .label("Target Attacked Switch")
                                    .checked(self.target_attacked_switch)
                                    .on_click(cx.listener(|view, checked, _, cx| {
                                        view.target_attacked_switch = *checked;
                                        view.data[view.index].0.target_attacked_switch = *checked;
                                        view.data[view.index].1 = true;
                                        view.save_temp_file();

                                        cx.notify();
                                    })),
                            )
                            .child(
                                Checkbox::new("target-range-dropout")
                                    .label("Target Range Dropout")
                                    .checked(self.target_range_dropout)
                                    .on_click(cx.listener(|view, checked, _, cx| {
                                        view.target_range_dropout = *checked;
                                        view.data[view.index].0.target_range_dropout = *checked;
                                        view.data[view.index].1 = true;
                                        view.save_temp_file();

                                        cx.notify();
                                    })),
                            )
                            .child(
                                Checkbox::new("can-target")
                                    .label("Can Target")
                                    .checked(self.can_target)
                                    .on_click(cx.listener(|view, checked, _, cx| {
                                        view.can_target = *checked;
                                        view.data[view.index].0.can_target = *checked;
                                        view.data[view.index].1 = true;
                                        view.save_temp_file();

                                        cx.notify();
                                    })),
                            )
                            .child(
                                Checkbox::new("can-move")
                                    .label("Can Move")
                                    .checked(self.can_move)
                                    .on_click(cx.listener(|view, checked, _, cx| {
                                        view.can_move = *checked;
                                        view.data[view.index].0.can_move = *checked;
                                        view.data[view.index].1 = true;
                                        view.save_temp_file();

                                        cx.notify();
                                    })),
                            )
                            .child(
                                Checkbox::new("can-attack-player")
                                    .label("Can Attack Player")
                                    .checked(self.can_attack_player)
                                    .on_click(cx.listener(|view, checked, _, cx| {
                                        view.can_attack_player = *checked;
                                        view.data[view.index].0.can_attack_player = *checked;
                                        view.data[view.index].1 = true;
                                        view.save_temp_file();

                                        cx.notify();
                                    })),
                            )
                            .child(
                                Checkbox::new("has-self-only")
                                    .label("Has Self Only")
                                    .checked(self.has_selfonly)
                                    .on_click(cx.listener(|view, checked, _, cx| {
                                        view.has_selfonly = *checked;
                                        view.data[view.index].0.has_selfonly = *checked;
                                        view.data[view.index].1 = true;
                                        view.save_temp_file();

                                        cx.notify();
                                    })),
                            )
                            .child("")
                            .child("Start Time")
                            .child(
                                h_flex()
                                    .gap(px(5.))
                                    .child("Hr")
                                    .child(Input::new(&self.start_hour)),
                            )
                            .child(
                                h_flex()
                                    .gap(px(5.))
                                    .child("Mn")
                                    .child(Input::new(&self.start_minute)),
                            )
                            .child(
                                h_flex()
                                    .gap(px(5.))
                                    .child("Sc")
                                    .child(Input::new(&self.start_second)),
                            ),
                    )
                    .child(
                        v_flex()
                            .gap(px(5.))
                            .child(
                                Checkbox::new("has-friendly-only")
                                    .label("Has Friendly Only")
                                    .checked(self.has_friendonly)
                                    .on_click(cx.listener(|view, checked, _, cx| {
                                        view.has_friendonly = *checked;
                                        view.data[view.index].0.has_friendonly = *checked;
                                        view.data[view.index].1 = true;
                                        view.save_temp_file();

                                        cx.notify();
                                    })),
                            )
                            .child(
                                Checkbox::new("has-ground-only")
                                    .label("Has Ground Only")
                                    .checked(self.has_groundonly)
                                    .on_click(cx.listener(|view, checked, _, cx| {
                                        view.has_groundonly = *checked;
                                        view.data[view.index].0.has_groundonly = *checked;
                                        view.data[view.index].1 = true;
                                        view.save_temp_file();

                                        cx.notify();
                                    })),
                            )
                            .child(
                                Checkbox::new("has-ally")
                                    .label("Has Ally")
                                    .checked(self.has_allys)
                                    .on_click(cx.listener(|view, checked, _, cx| {
                                        view.has_allys = *checked;
                                        view.data[view.index].0.has_allys = *checked;
                                        view.data[view.index].1 = true;
                                        view.save_temp_file();

                                        cx.notify();
                                    })),
                            )
                            .child(
                                Checkbox::new("can-attack")
                                    .label("Can Attack")
                                    .checked(self.can_attack)
                                    .on_click(cx.listener(|view, checked, _, cx| {
                                        view.can_attack = *checked;
                                        view.data[view.index].0.can_attack = *checked;
                                        view.data[view.index].1 = true;
                                        view.save_temp_file();

                                        cx.notify();
                                    })),
                            )
                            .child(
                                Checkbox::new("runs-away")
                                    .label("Runs Away")
                                    .checked(self.runsaway)
                                    .on_click(cx.listener(|view, checked, _, cx| {
                                        view.runsaway = *checked;
                                        view.data[view.index].0.runsaway = *checked;
                                        view.data[view.index].1 = true;
                                        view.save_temp_file();

                                        cx.notify();
                                    })),
                            )
                            .child(
                                Checkbox::new("can-pass-through")
                                    .label("Can Pass Through")
                                    .checked(self.canpassthru)
                                    .on_click(cx.listener(|view, checked, _, cx| {
                                        view.canpassthru = *checked;
                                        view.data[view.index].0.canpassthru = *checked;
                                        view.data[view.index].1 = true;
                                        view.save_temp_file();

                                        cx.notify();
                                    })),
                            )
                            .child(
                                Checkbox::new("is-animated")
                                    .label("Is Animated")
                                    .checked(self.isanimated)
                                    .on_click(cx.listener(|view, checked, _, cx| {
                                        view.isanimated = *checked;
                                        view.data[view.index].0.isanimated = *checked;
                                        view.data[view.index].1 = true;
                                        view.save_temp_file();

                                        cx.notify();
                                    })),
                            )
                            .child("")
                            .child("End Time")
                            .child(
                                h_flex()
                                    .gap(px(5.))
                                    .child("Hr")
                                    .child(Input::new(&self.end_hour)),
                            )
                            .child(
                                h_flex()
                                    .gap(px(5.))
                                    .child("Mn")
                                    .child(Input::new(&self.end_minute)),
                            )
                            .child(
                                h_flex()
                                    .gap(px(5.))
                                    .child("Sc")
                                    .child(Input::new(&self.end_second)),
                            ),
                    ), // Settings
                2 => v_flex().gap(px(5.)).child(
                    h_flex()
                        .gap(px(10.))
                        .child(
                            v_flex()
                                .gap(px(5.))
                                .child("Drop Item Slot")
                                .child(Select::new(&self.drop_list))
                                .child("Free Shares")
                                .child(Input::new(&self.free_shares)),
                        )
                        .child(
                            v_flex()
                                .gap(px(5.))
                                .child(
                                    h_flex()
                                        .gap(px(10.))
                                        .child("Shares")
                                        .child(Input::new(&self.drop_share)),
                                )
                                .child(
                                    h_flex()
                                        .gap(px(10.))
                                        .child(
                                            v_flex()
                                                .gap(px(5.))
                                                .child("Item ID 1")
                                                .child(Input::new(&self.drop_item[0]))
                                                .child("Amount")
                                                .child(Input::new(&self.drop_amount[0])),
                                        )
                                        .child(
                                            v_flex()
                                                .gap(px(5.))
                                                .child("Item ID 2")
                                                .child(Input::new(&self.drop_item[1]))
                                                .child("Amount")
                                                .child(Input::new(&self.drop_amount[1])),
                                        )
                                        .child(
                                            v_flex()
                                                .gap(px(5.))
                                                .child("Item ID 3")
                                                .child(Input::new(&self.drop_item[2]))
                                                .child("Amount")
                                                .child(Input::new(&self.drop_amount[2])),
                                        )
                                        .child(
                                            v_flex()
                                                .gap(px(5.))
                                                .child("Item ID 4")
                                                .child(Input::new(&self.drop_item[3]))
                                                .child("Amount")
                                                .child(Input::new(&self.drop_amount[3])),
                                        )
                                        .child(
                                            v_flex()
                                                .gap(px(5.))
                                                .child("Item ID 5")
                                                .child(Input::new(&self.drop_item[4]))
                                                .child("Amount")
                                                .child(Input::new(&self.drop_amount[4])),
                                        ),
                                ),
                        ),
                ), // Drops
                3 => v_flex()
                    .gap(px(5.))
                    .child(
                        Checkbox::new("has-enemy")
                            .label("Has Enemy")
                            .checked(self.has_enemies)
                            .on_click(cx.listener(|view, checked, _, cx| {
                                view.has_enemies = *checked;
                                view.data[view.index].0.has_enemies = *checked;
                                view.data[view.index].1 = true;
                                view.save_temp_file();

                                cx.notify();
                            })),
                    )
                    .child(
                        h_flex()
                            .gap(px(10.))
                            .child("Enemy List")
                            .child(Select::new(&self.enemy_list)),
                    )
                    .child(
                        h_flex().gap(px(10.)).child(
                            h_flex()
                                .gap(px(10.))
                                .child("NPC Index")
                                .child(Input::new(&self.enemy_index).w(px(100.)))
                                .child(Button::new("update").primary().label("Update").on_click(
                                    cx.listener(|view, _event, wn, cx| {
                                        view.update_enemy(wn, cx);
                                        cx.notify();
                                    }),
                                ))
                                .child(Button::new("add").primary().label("Add").on_click(
                                    cx.listener(|view, _event, wn, cx| {
                                        view.add_enemy(wn, cx);
                                        cx.notify();
                                    }),
                                ))
                                .child(Button::new("remove").primary().label("Remove").on_click(
                                    cx.listener(|view, _event, wn, cx| {
                                        view.remove_enemy(wn, cx);
                                        cx.notify();
                                    }),
                                )),
                        ),
                    ), // Enemies
                _ => v_flex()
                    .gap(px(5.))
                    .child(
                        h_flex()
                            .gap(px(10.))
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
                                    .child("Sprite:")
                                    .child(Input::new(&self.sprite).w(px(100.))),
                            )
                            .child(
                                h_flex()
                                    .gap(px(10.))
                                    .child("Behaviour:")
                                    .child(Select::new(&self.behaviour).w(px(150.))),
                            ),
                    )
                    .child(
                        h_flex()
                            .gap(px(10.))
                            .child(
                                v_flex()
                                    .gap(px(5.))
                                    .child("Level:")
                                    .child(Input::new(&self.level).w(px(150.))),
                            )
                            .child(
                                v_flex()
                                    .gap(px(5.))
                                    .child("Phy. Dmg:")
                                    .child(Input::new(&self.pdamage).w(px(150.))),
                            )
                            .child(
                                v_flex()
                                    .gap(px(5.))
                                    .child("Move Wait: (ms)")
                                    .child(Input::new(&self.movement_wait).w(px(150.))),
                            )
                            .child(
                                v_flex()
                                    .gap(px(5.))
                                    .child("Min Dmg:")
                                    .child(Input::new(&self.mindamage).w(px(150.))),
                            )
                            .child(
                                v_flex()
                                    .gap(px(5.))
                                    .child("Range:")
                                    .child(Input::new(&self.range).w(px(150.))),
                            ),
                    )
                    .child(
                        h_flex()
                            .gap(px(10.))
                            .child(
                                v_flex()
                                    .gap(px(5.))
                                    .child("Max HP:")
                                    .child(Input::new(&self.maxhp).w(px(150.))),
                            )
                            .child(
                                v_flex()
                                    .gap(px(5.))
                                    .child("Phy. Def:")
                                    .child(Input::new(&self.pdefense).w(px(150.))),
                            )
                            .child(
                                v_flex()
                                    .gap(px(5.))
                                    .child("Atk Wait: (ms)")
                                    .child(Input::new(&self.attack_wait).w(px(150.))),
                            )
                            .child(
                                v_flex()
                                    .gap(px(5.))
                                    .child("Max Dmg:")
                                    .child(Input::new(&self.maxdamage).w(px(150.))),
                            )
                            .child(
                                v_flex()
                                    .gap(px(5.))
                                    .child("Exp:")
                                    .child(Input::new(&self.exp).w(px(150.))),
                            ),
                    )
                    .child(
                        h_flex()
                            .gap(px(10.))
                            .child(
                                v_flex()
                                    .gap(px(5.))
                                    .child("Max SP:")
                                    .child(Input::new(&self.maxsp).w(px(150.))),
                            )
                            .child(
                                v_flex()
                                    .gap(px(5.))
                                    .child("Sight Range:")
                                    .child(Input::new(&self.sight).w(px(150.))),
                            )
                            .child(
                                v_flex()
                                    .gap(px(5.))
                                    .child("Interval Wait: (ms)")
                                    .child(Input::new(&self.intervaled_wait).w(px(150.))),
                            )
                            .child(
                                v_flex()
                                    .gap(px(5.))
                                    .child("Respawn Wait: (ms)")
                                    .child(Input::new(&self.respawn_wait).w(px(150.))),
                            ),
                    )
                    .child(
                        h_flex()
                            .gap(px(10.))
                            .child(
                                v_flex()
                                    .gap(px(5.))
                                    .child("Max MP:")
                                    .child(Input::new(&self.maxmp).w(px(150.))),
                            )
                            .child(
                                v_flex()
                                    .gap(px(5.))
                                    .child("Follow Sight Range:")
                                    .child(Input::new(&self.follow_sight).w(px(150.))),
                            )
                            .child(
                                v_flex()
                                    .gap(px(5.))
                                    .child("Walk Dist. frm Spawn:")
                                    .child(Input::new(&self.walkdistance).w(px(150.))),
                            )
                            .child(
                                v_flex()
                                    .gap(px(5.))
                                    .child("Spawn Wait: (ms)")
                                    .child(Input::new(&self.spawn_wait).w(px(150.))),
                            ),
                    )
                    .child("")
                    .child("Block Size")
                    .child(
                        h_flex()
                            .gap(px(10.))
                            .child(
                                h_flex()
                                    .gap(px(10.))
                                    .child("X")
                                    .child(Input::new(&self.size_x).w(px(150.))),
                            )
                            .child(
                                h_flex()
                                    .gap(px(10.))
                                    .child("Y")
                                    .child(Input::new(&self.size_y).w(px(150.))),
                            )
                            .child(
                                h_flex()
                                    .gap(px(10.))
                                    .child("H")
                                    .child(Input::new(&self.size_h).w(px(150.))),
                            )
                            .child(
                                h_flex()
                                    .gap(px(10.))
                                    .child("W")
                                    .child(Input::new(&self.size_w).w(px(150.))),
                            ),
                    ), // General
            })
    }
}
