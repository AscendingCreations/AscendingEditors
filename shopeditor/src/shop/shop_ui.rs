use crate::{load_config, shop::*, ConfigData};
use ascending_types::*;
use ascending_ui::*;

use iced::{
    widget::{column, scrollable, Container},
    Element, Length,
};

#[allow(dead_code)]
#[derive(Educe)]
#[educe(Default)]
pub struct ShopUI {
    #[educe(Default(expression = Vec::with_capacity(MAX_SHOPS)))]
    pub data: Vec<(ShopData, bool)>,
    menu: ShopUiMenu,
    generic: ShopUiGeneric, //Generic Shop Data.
    currentid: usize,
    current_shopid: usize,
    config: ConfigData,
}

impl UiRenderer for ShopUI {
    type Message = Message;
    fn update(&mut self, msg: Message) {
        match msg {
            Message::SaveAllButtonPress => {
                self.save_all();
                return;
            }
            Message::SaveButtonPress => {
                if self.config.save_json {
                    self.data[self.currentid]
                        .0
                        .save_file(self.currentid)
                        .unwrap();
                }
                self.data[self.currentid]
                    .0
                    .save_bin_file(self.currentid)
                    .unwrap();
                return;
            }
            Message::RevertButtonPress => {
                let shop = ShopData::load_file(self.currentid).unwrap();
                self.data[self.currentid].0 = shop.0;
                self.data[self.currentid].1 = false;
                self.set_object_to_layout(self.currentid);
                return;
            }
            Message::ListSelect(data) => {
                self.currentid = data.id;
                self.menu.list_selected = Some(data);
                self.set_object_to_layout(self.currentid);
                return;
            }
            Message::NameInput(value) => {
                if value.len() < 64 {
                    self.generic.txt_value = value;
                    self.data[self.currentid]
                        .0
                        .name
                        .clone_from(&self.generic.txt_value);
                    self.menu.list[self.currentid]
                        .name
                        .clone_from(&self.generic.txt_value);
                    self.menu.list_selected = Some(self.menu.list[self.currentid].clone());
                } else {
                    return;
                }
            }
            Message::SlotSelect(slot) => {
                self.generic.slot_selected = Some(slot);
                self.current_shopid = slot as usize;
                self.load_shop_slot(self.currentid, self.current_shopid);
            }
            Message::GenericInput((id, data)) => {
                let value = data.get_data();

                match id {
                    1 => {
                        self.generic.max_shop_item.value = value;
                        self.data[self.currentid].0.max_item = value;
                    }
                    2 => {
                        self.generic.item_index.value = value;
                        self.data[self.currentid].0.item[self.current_shopid].index = value;
                    }
                    3 => {
                        self.generic.item_amount.value = value;
                        self.data[self.currentid].0.item[self.current_shopid].amount = value;
                    }
                    _ => return,
                }
            }
            Message::GenericInput64((id, data)) => {
                let value = data.get_data();

                match id {
                    1 => {
                        self.generic.item_price.value = value;
                        self.data[self.currentid].0.item[self.current_shopid].price = value;
                    }
                    _ => return,
                }
            }
        }

        self.data[self.currentid].1 = true;
    }

    fn view(&self) -> Element<Message> {
        self.layout()
    }
}

impl ShopUI {
    pub fn new() -> Self {
        let config: ConfigData = load_config();

        let mut ui = ShopUI {
            data: ShopData::load_files(config.save_json).unwrap(),
            config,
            ..Default::default()
        };

        for (i, v) in ui.data.iter().enumerate() {
            ui.menu.list.push(ListData::new(i, v.0.name.clone()));
        }
        ui.menu.list_selected = Some(ui.menu.list[0].clone());

        for i in 0..MAX_SHOP_ITEM {
            ui.generic.slot_list.push(i as u16);
        }

        ui.set_object_to_layout(0);
        ui
    }

    pub fn save_all(&mut self) {
        for (i, v) in self.data.iter().enumerate() {
            if !v.1 {
                continue;
            }

            if self.config.save_json {
                if let Err(e) = v.0.save_file(i) {
                    println!("Could not save Shop {}, err {}", i, e);
                }
            }
            if let Err(e) = v.0.save_bin_file(i) {
                println!("Could not save bin Shop {}, err {}", i, e);
            }
        }
    }

    fn set_object_to_layout(&mut self, index: usize) {
        self.generic.max_shop_item.value = self.data[index].0.max_item;

        self.generic.slot_selected = Some(0);
        self.current_shopid = 0;
        self.load_shop_slot(index, self.current_shopid);
    }

    fn load_shop_slot(&mut self, index: usize, shopslot: usize) {
        self.generic.item_index.value = self.data[index].0.item[shopslot].index;
        self.generic.item_amount.value = self.data[index].0.item[shopslot].amount;
        self.generic.item_price.value = self.data[index].0.item[shopslot].price;
    }

    fn layout(&self) -> Element<Message> {
        Container::new(
            column![
                self.menu.layout(),
                scrollable(
                    Container::new(self.generic.layout())
                        .padding(5)
                        .width(Length::Fill)
                        .center_x(Length::Fill)
                )
            ]
            .spacing(20),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(10)
        .into()
    }
}
