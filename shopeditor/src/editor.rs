use ascending_logger::info;
use database::{ItemData, MAX_SHOP_ITEM, ShopData};
use gpui_kit::{
    App, AppContext, Context, Entity, ParentElement, SharedString, Subscription, Window,
    base::IndexPath,
    component::{
        input::{InputEvent, InputState},
        label::Label,
        list::{ListDelegate, ListEvent, ListItem, ListState},
        select::{SelectEvent, SelectState},
    },
};
use tokio::sync::mpsc::Receiver;

use crate::commands::{CommandHandler, MessageCommands};

mod renderer;

pub struct ListData {
    items: Vec<String>,
    selected_index: Option<IndexPath>,
}

impl ListDelegate for ListData {
    type Item = ListItem;

    fn items_count(&self, _section: usize, _cx: &App) -> usize {
        self.items.len()
    }

    fn render_item(
        &mut self,
        ix: IndexPath,
        _window: &mut Window,
        _cx: &mut Context<ListState<Self>>,
    ) -> Option<Self::Item> {
        self.items.get(ix.row).map(|item| {
            ListItem::new(ix)
                .child(Label::new(item.clone()))
                .selected(Some(ix) == self.selected_index)
        })
    }

    fn set_selected_index(
        &mut self,
        ix: Option<IndexPath>,
        _window: &mut Window,
        cx: &mut Context<ListState<Self>>,
    ) {
        self.selected_index = ix;
        cx.notify();
    }
}

pub struct Editor {
    pub index: usize,
    pub data: Vec<(ShopData, bool)>,
    pub itemdata: Vec<(ItemData, bool)>,
    pub command_handler: CommandHandler,

    data_list: Entity<SelectState<Vec<SharedString>>>,
    name_input: Entity<InputState>,

    shop_list: Entity<ListState<ListData>>,
    list_index: usize,
    max_item: Entity<InputState>,

    shop_index: Entity<InputState>,
    shop_amount: Entity<InputState>,
    shop_price: Entity<InputState>,

    _subscriptions: Vec<Subscription>,
}

impl Editor {
    pub fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
        command_handler: CommandHandler,
        mut rx_update: Receiver<Option<(usize, ShopData)>>,
    ) -> Self {
        info!("Loading data");
        let data = ShopData::load_files().unwrap();
        let itemdata = ItemData::load_files().unwrap();

        info!("Creating shop receiver");

        cx.spawn(async move |this, cx| {
            while let Some(result) = rx_update.recv().await {
                match result {
                    Some((index, data)) => {
                        let window_handle = this
                            .update(cx, move |_this, context| context.active_window())
                            .unwrap();

                        if let Some(window_handle) = window_handle {
                            let _ = cx.update_window(window_handle, |_view, win, app| {
                                this.update(app, move |this, context| {
                                    this.data[index].0 = data;
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

        let shop_list = cx.new(|cx| {
            let mut list = ListData {
                items: Vec::with_capacity(MAX_SHOP_ITEM),
                selected_index: None,
            };

            for (index, item) in data[0].0.item.iter().enumerate() {
                list.items.push(format!(
                    "{index}: {}",
                    if item.amount > 0 {
                        if let Some(result) = itemdata.get(item.index as usize) {
                            format!(
                                "[{}] {} x {} [Price: {}]",
                                item.index, result.0.name, item.amount, item.price
                            )
                        } else {
                            "Error".to_string()
                        }
                    } else {
                        "None".to_string()
                    }
                ));
            }

            let mut list = ListState::new(list, window, cx);
            list.set_selected_index(Some(IndexPath::new(0)), window, cx);
            list
        });

        let max_item = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Max Item")
                .default_value(format!("{}", data[0].0.max_item))
        });
        let shop_index = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Item")
                .default_value(format!("{}", data[0].0.item[0].index))
        });
        let shop_amount = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Amount")
                .default_value(format!("{}", data[0].0.item[0].amount))
        });
        let shop_price = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Price")
                .default_value(format!("{}", data[0].0.item[0].price))
        });

        info!("Setting subscription");

        let _subscriptions = vec![
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
            cx.subscribe_in(&shop_list, window, {
                |view, _, ev, wn, cx| {
                    if let ListEvent::Confirm(index) = ev {
                        view.list_index = index.row;
                        view.switch_index(index.row, wn, cx);
                        cx.notify();
                    }
                }
            }),
            cx.subscribe_in(&max_item, window, |view, state, event, _, cx| {
                if let InputEvent::Blur = event {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<u16>()
                        && new_value != view.data[view.index].0.max_item
                    {
                        view.data[view.index].0.max_item = new_value;
                        view.data[view.index].1 = true;
                        view.save_temp_file();
                        cx.notify()
                    }
                }
            }),
            cx.subscribe_in(&shop_index, window, |view, state, event, wn, cx| {
                if let InputEvent::Blur = event {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<u16>()
                        && new_value != view.data[view.index].0.item[view.list_index].index
                    {
                        view.data[view.index].0.item[view.list_index].index = new_value;
                        view.data[view.index].1 = true;
                        view.save_temp_file();
                        view.update_shop_list(wn, cx);
                        cx.notify()
                    }
                }
            }),
            cx.subscribe_in(&shop_amount, window, |view, state, event, wn, cx| {
                if let InputEvent::Blur = event {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<u16>()
                        && new_value != view.data[view.index].0.item[view.list_index].amount
                    {
                        view.data[view.index].0.item[view.list_index].amount = new_value;
                        view.data[view.index].1 = true;
                        view.save_temp_file();
                        view.update_shop_list(wn, cx);
                        cx.notify()
                    }
                }
            }),
            cx.subscribe_in(&shop_price, window, |view, state, event, wn, cx| {
                if let InputEvent::Blur = event {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<u64>()
                        && new_value != view.data[view.index].0.item[view.list_index].price
                    {
                        view.data[view.index].0.item[view.list_index].price = new_value;
                        view.data[view.index].1 = true;
                        view.save_temp_file();
                        view.update_shop_list(wn, cx);
                        cx.notify()
                    }
                }
            }),
        ];

        Editor {
            index: 0,
            data,
            itemdata,
            command_handler,

            data_list,
            name_input,

            shop_list,
            list_index: 0,
            max_item,
            shop_index,
            shop_amount,
            shop_price,

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

            self.max_item.update(cx, |view, cx| {
                view.set_value(SharedString::new(format!("{}", data.max_item)), wn, cx);
            });

            self.list_index = 0;

            self.shop_list.update(cx, |view, cx| {
                view.set_selected_index(Some(IndexPath::new(0)), wn, cx);
            });

            self.shop_index.update(cx, |view, cx| {
                view.set_value(SharedString::new(format!("{}", data.item[0].index)), wn, cx);
            });

            self.shop_amount.update(cx, |view, cx| {
                view.set_value(
                    SharedString::new(format!("{}", data.item[0].amount)),
                    wn,
                    cx,
                );
            });

            self.shop_price.update(cx, |view, cx| {
                view.set_value(SharedString::new(format!("{}", data.item[0].price)), wn, cx);
            });
        }
    }

    pub fn switch_index(&mut self, index: usize, wn: &mut Window, cx: &mut Context<Self>) {
        self.shop_index.update(cx, |view, cx| {
            view.set_value(
                SharedString::new(format!("{}", self.data[self.index].0.item[index].index)),
                wn,
                cx,
            );
        });

        self.shop_amount.update(cx, |view, cx| {
            view.set_value(
                SharedString::new(format!("{}", self.data[self.index].0.item[index].amount)),
                wn,
                cx,
            );
        });

        self.shop_price.update(cx, |view, cx| {
            view.set_value(
                SharedString::new(format!("{}", self.data[self.index].0.item[index].price)),
                wn,
                cx,
            );
        });
    }

    pub fn update_shop_list(&mut self, _wn: &mut Window, cx: &mut Context<Self>) {
        self.shop_list.update(cx, |state, _cx| {
            let mut list = ListData {
                items: Vec::with_capacity(MAX_SHOP_ITEM),
                selected_index: None,
            };

            for (index, item) in self.data[self.index].0.item.iter().enumerate() {
                list.items.push(format!(
                    "{index}: {}",
                    if item.amount > 0 {
                        if let Some(result) = self.itemdata.get(item.index as usize) {
                            format!(
                                "[{}] {} x {} [Price: {}]",
                                item.index, result.0.name, item.amount, item.price
                            )
                        } else {
                            "Error".to_string()
                        }
                    } else {
                        "None".to_string()
                    }
                ));
            }

            *state.delegate_mut() = list;
        });
    }
}
