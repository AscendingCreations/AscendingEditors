use ascending_logger::info;
use database::{ItemData, ItemTypes};
use gpui_kit::{
    AppContext, Context, Entity, SharedString, Subscription, Window,
    base::IndexPath,
    component::{
        input::{InputEvent, InputState},
        select::{SelectEvent, SelectState},
    },
};
use tokio::sync::mpsc::Receiver;

use crate::commands::{CommandHandler, MessageCommands};

mod renderer;

pub struct Editor {
    pub index: usize,
    pub data: Vec<(ItemData, bool)>,
    pub command_handler: CommandHandler,

    data_list: Entity<SelectState<Vec<SharedString>>>,
    name_input: Entity<InputState>,

    itemtype_list: Entity<SelectState<Vec<SharedString>>>,
    itemtype2_input: Entity<InputState>,
    sprite_input: Entity<InputState>,
    level_req_input: Entity<InputState>,
    sound_id_input: Entity<InputState>,
    stack_limit_input: Entity<InputState>,
    base_price_input: Entity<InputState>,
    animation_input: Entity<InputState>,
    sound_input: Entity<InputState>,
    data_input: Vec<Entity<InputState>>,

    breakable: bool,
    repairable: bool,
    stackable: bool,

    _subscriptions: Vec<Subscription>,
}

impl Editor {
    pub fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
        command_handler: CommandHandler,
        mut rx_update: Receiver<Option<(usize, ItemData)>>,
    ) -> Self {
        info!("Loading data");
        let data = ItemData::load_files().unwrap();

        info!("Creating item receiver");

        cx.spawn(async move |this, cx| {
            while let Some(result) = rx_update.recv().await {
                match result {
                    Some((index, item)) => {
                        let window_handle = this
                            .update(cx, move |_this, context| context.active_window())
                            .unwrap();

                        if let Some(window_handle) = window_handle {
                            let _ = cx.update_window(window_handle, |_view, win, app| {
                                this.update(app, move |this, context| {
                                    this.data[index].0 = item;
                                    this.data[index].1 = false;
                                    this.update_list(win, context);
                                    this.update_data(win, context);
                                    context.notify();
                                })
                            });
                        }
                    }
                    None => return,
                }
            }
        })
        .detach();

        info!("Creating list");
        let mut list = Vec::with_capacity(data.len());
        for (index, (data, _)) in data.iter().enumerate() {
            list.push(SharedString::new(format!("{index}: {}", data.name)));
        }

        let data_list = cx.new(|cx| SelectState::new(list, None, window, cx));
        data_list.update(cx, |state, cx| {
            state.set_selected_index(Some(IndexPath::new(0)), window, cx);
        });

        info!("Setting up widget");

        let name_input = cx.new(|cx| {
            let mut state = InputState::new(window, cx).placeholder("Name");
            state.set_value(SharedString::new(data[0].0.name.clone()), window, cx);
            state
        });

        let itemtype_list = cx.new(|cx| SelectState::new(ItemTypes::get_list(), None, window, cx));
        itemtype_list.update(cx, |state, cx| {
            state.set_selected_index(
                Some(IndexPath::new(data[0].0.itemtype as usize)),
                window,
                cx,
            );
        });

        let itemtype2_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Item Type")
                .default_value(format!("{}", data[0].0.itemtype2))
        });
        let sprite_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Sprite")
                .default_value(format!("{}", data[0].0.sprite))
        });
        let level_req_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Level Req")
                .default_value(format!("{}", data[0].0.levelreq))
        });
        let sound_id_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Sound ID")
                .default_value(format!("{}", data[0].0.soundid))
        });
        let stack_limit_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Stack Limit")
                .default_value(format!("{}", data[0].0.stacklimit))
        });
        let base_price_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Base Store Price")
                .default_value(format!("{}", data[0].0.baseprice))
        });
        let animation_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Animation")
                .default_value(format!(
                    "{}",
                    if let Some(val) = data[0].0.animation {
                        val as i32
                    } else {
                        -1
                    }
                ))
        });
        let sound_input = cx.new(|cx| {
            let mut state = InputState::new(window, cx).placeholder("Sound");
            state.set_value(
                if let Some(val) = &data[0].0.sound_index {
                    SharedString::new(val.clone())
                } else {
                    SharedString::new("")
                },
                window,
                cx,
            );
            state
        });

        let mut data_input = Vec::with_capacity(20);
        for val in data[0].0.data.iter() {
            data_input.push(cx.new(|cx| {
                let mut state = InputState::new(window, cx).placeholder("Data");
                state.set_value(SharedString::new(format!("{val}")), window, cx);
                state
            }));
        }

        let breakable = data[0].0.breakable;
        let repairable = data[0].0.repairable;
        let stackable = data[0].0.stackable;

        info!("Setting subscription");

        let mut _subscriptions = vec![
            cx.subscribe_in(
                &data_list,
                window,
                |view, _state, event, wn, cx| match event {
                    SelectEvent::Confirm(_) => {
                        if let Some(index) = view.data_list.read(cx).selected_index(cx) {
                            view.index = index.row;
                            view.update_data(wn, cx);
                        }
                    }
                },
            ),
            cx.subscribe_in(&name_input, window, {
                |view, _state, ev, _wn, cx| {
                    if let InputEvent::Blur = ev {
                        let value = view.name_input.read(cx).value().as_str().to_string();
                        if value != view.data[view.index].0.name {
                            println!("Updating");
                            view.data[view.index].0.name.clone_from(&value);
                            view.data[view.index].1 = true;
                            view.save_temp_file();
                            cx.notify()
                        }
                    }
                }
            }),
            cx.subscribe_in(
                &itemtype_list,
                window,
                |view, _state, event, _wn, cx| match event {
                    SelectEvent::Confirm(_) => {
                        if let Some(index) = view.itemtype_list.read(cx).selected_index(cx) {
                            view.data[view.index].0.itemtype = ItemTypes::from_index(index.row);
                            view.data[view.index].1 = true;
                            view.save_temp_file();
                            cx.notify()
                        }
                    }
                },
            ),
            cx.subscribe_in(&itemtype2_input, window, |view, state, event, _, cx| {
                if let InputEvent::Blur = event {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<u8>()
                        && new_value != view.data[view.index].0.itemtype2
                    {
                        view.data[view.index].0.itemtype2 = new_value;
                        view.data[view.index].1 = true;
                        view.save_temp_file();
                        cx.notify()
                    }
                }
            }),
            cx.subscribe_in(&sprite_input, window, |view, state, event, _, cx| {
                if let InputEvent::Blur = event {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<u16>()
                        && new_value != view.data[view.index].0.sprite
                    {
                        view.data[view.index].0.sprite = new_value;
                        view.data[view.index].1 = true;
                        view.save_temp_file();
                        cx.notify()
                    }
                }
            }),
        ];

        for (index, input) in data_input.iter().enumerate() {
            _subscriptions.push(
                cx.subscribe_in(input, window, move |view, state, event, _, cx| {
                    if let InputEvent::Blur = event {
                        let text = state.read(cx).value();
                        if let Ok(new_value) = text.parse::<i16>()
                            && new_value != view.data[view.index].0.data[index]
                        {
                            view.data[view.index].0.data[index] = new_value;
                            view.data[view.index].1 = true;
                            view.save_temp_file();
                            cx.notify()
                        }
                    }
                }),
            )
        }

        Editor {
            index: 0,
            data,
            command_handler,

            data_list,
            name_input,

            itemtype_list,
            itemtype2_input,
            sprite_input,
            level_req_input,
            sound_id_input,
            stack_limit_input,
            base_price_input,
            animation_input,
            sound_input,
            data_input,

            breakable,
            repairable,
            stackable,

            _subscriptions,
        }
    }

    pub fn update_list(&mut self, wn: &mut Window, cx: &mut Context<Self>) {
        self.data_list.update(cx, |state, cx| {
            let mut list = Vec::with_capacity(self.data.len());
            for (index, (data, _)) in self.data.iter().enumerate() {
                list.push(SharedString::new(format!("{index}: {}", data.name)))
            }

            state.set_items(list, wn, cx);
        });
    }

    pub fn save_temp_file(&mut self) {
        let _ = self
            .command_handler
            .tx
            .blocking_send(MessageCommands::TempSave(
                self.index,
                Box::new(self.data[self.index].0.clone()),
            ));
    }

    pub fn update_data(&mut self, wn: &mut Window, cx: &mut Context<Self>) {
        if let Some((data, _edited)) = self.data.get(self.index) {
            self.name_input.update(cx, |view, cx| {
                view.set_value(SharedString::new(data.name.clone()), wn, cx);
            });

            self.itemtype_list.update(cx, |view, cx| {
                view.set_selected_index(
                    Some(IndexPath {
                        section: 0,
                        row: data.itemtype as usize,
                        column: 0,
                    }),
                    wn,
                    cx,
                );
            });

            self.itemtype2_input.update(cx, |view, cx| {
                view.set_value(SharedString::new(format!("{}", data.itemtype2)), wn, cx);
            });

            self.sprite_input.update(cx, |view, cx| {
                view.set_value(SharedString::new(format!("{}", data.sprite)), wn, cx);
            });

            self.level_req_input.update(cx, |view, cx| {
                view.set_value(SharedString::new(format!("{}", data.levelreq)), wn, cx);
            });

            self.sound_id_input.update(cx, |view, cx| {
                view.set_value(SharedString::new(format!("{}", data.soundid)), wn, cx);
            });

            self.stack_limit_input.update(cx, |view, cx| {
                view.set_value(SharedString::new(format!("{}", data.stacklimit)), wn, cx);
            });

            self.base_price_input.update(cx, |view, cx| {
                view.set_value(SharedString::new(format!("{}", data.baseprice)), wn, cx);
            });

            self.animation_input.update(cx, |view, cx| {
                view.set_value(
                    SharedString::new(format!(
                        "{}",
                        if let Some(val) = data.animation {
                            val as i32
                        } else {
                            -1
                        }
                    )),
                    wn,
                    cx,
                );
            });

            self.sound_input.update(cx, |view, cx| {
                view.set_value(
                    if let Some(val) = &data.sound_index {
                        SharedString::new(val.clone())
                    } else {
                        SharedString::new("")
                    },
                    wn,
                    cx,
                );
            });

            for (index, input) in self.data_input.iter().enumerate() {
                if let Some(val) = data.data.get(index) {
                    input.update(cx, |view, cx| {
                        view.set_value(SharedString::new(format!("{val}")), wn, cx);
                    })
                }
            }

            self.breakable = data.breakable;
            self.repairable = data.repairable;
            self.stackable = data.stackable
        }
    }
}
