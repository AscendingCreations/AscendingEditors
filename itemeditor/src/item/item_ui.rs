use crate::{item::*, load_config, ConfigData};
use ascending_types::*;
use ascending_ui::*;

use iced::{
    widget::{column, row, Column, Container, Scrollable},
    Color, Element, Length,
};

#[allow(dead_code)]
#[derive(Educe)]
#[educe(Default)]
pub struct ItemUI {
    #[educe(Default(expression = "Vec::with_capacity(MAX_ITEMS)"))]
    pub data: Vec<(ItemData, bool)>,
    menu: ItemUiMenu,
    generic: ItemUiGeneric, //Generic Item Data.
    data_ui: ItemUiData,    //Item Generic Data Types.
    currentid: usize,
    config: ConfigData,
}

impl UiRenderer for ItemUI {
    type Message = Message;
    fn update(&mut self, msg: Message) -> Option<Box<dyn UiRenderer<Message = Message>>> {
        match msg {
            Message::SaveAllButtonPress => {
                self.save_all();
                return None;
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
                return None;
            }
            Message::RevertButtonPress => {
                let item = ItemData::load_file(self.currentid).unwrap();
                self.data[self.currentid].0 = item.0;
                self.data[self.currentid].1 = false;
                self.set_object_to_layout(self.currentid);
                return None;
            }
            Message::ListSelect(data) => {
                self.currentid = data.id;
                self.menu.list_selected = Some(data);
                self.set_object_to_layout(self.currentid);
                return None;
            }
            Message::DataInput((i, data)) => {
                self.data_ui.input[i].value = data.get_data();
                self.data[self.currentid].0.data[i] = self.data_ui.input[i].value;
            }
            Message::TypeSelect(item_type) => {
                self.generic.type_selected = Some(item_type);
                self.data[self.currentid].0.itemtype = item_type;
            }
            Message::NameInput(value) => {
                if value.len() < 64 {
                    self.generic.txt_value = value;
                    self.data[self.currentid].0.name = self.generic.txt_value.clone();
                    self.menu.list[self.currentid].name = self.generic.txt_value.clone();
                    self.menu.list_selected = Some(self.menu.list[self.currentid].clone());
                } else {
                    return None;
                }
            }
            Message::SoundInput(value) => {
                self.generic.sound_name = value;
                if !self.generic.sound_name.is_empty() {
                    self.data[self.currentid].0.sound_index = Some(self.generic.sound_name.clone());
                } else {
                    self.data[self.currentid].0.sound_index = None;
                }
            }
            Message::GenericI32Input((id, data)) => {
                let value = data.get_data();
                if id == 0 {
                    self.generic.animation_input.value = data.get_data();
                    if value >= 0 {
                        self.data[self.currentid].0.animation = Some(value as u32);
                    } else {
                        self.data[self.currentid].0.animation = None;
                    }
                }
            }
            Message::GenericInput((id, data)) => {
                let value = data.get_data();

                match id {
                    1 => {
                        self.generic.level_input.value = value;
                        self.data[self.currentid].0.levelreq = value;
                    }
                    2 => {
                        self.generic.sound_input.value = value;
                        self.data[self.currentid].0.soundid = value;
                    }
                    3 => {
                        self.generic.stack_limit_input.value = value;
                        self.data[self.currentid].0.stacklimit = value;
                    }
                    4 => {
                        self.generic.sprite_input.value = value;
                        self.data[self.currentid].0.sprite = value;
                    }
                    5 => {
                        self.generic.type2.value = value;
                        self.data[self.currentid].0.itemtype2 = value as u8;
                    }
                    _ => return None,
                }
            }
            Message::BasePriceInput((_, data)) => {
                self.generic.base_price_input.value = data.get_data();
                self.data[self.currentid].0.baseprice = self.generic.base_price_input.value;
            }
            Message::Repairable(value) => {
                self.generic.repairable = value;
                self.data[self.currentid].0.repairable = value;
            }
            Message::Stackable(value) => {
                self.generic.stackable = value;
                self.data[self.currentid].0.stackable = value;
            }
            Message::Breakable(value) => {
                self.generic.breakable = value;
                self.data[self.currentid].0.breakable = value;
            }
            Message::ChooseColor => {
                self.generic.show_color = true;
                return None;
            }
            Message::SubmitColor(color) => {
                self.generic.color = color;
                self.data[self.currentid].0.rgba.r = (color.r * 255.0) as i16;
                self.data[self.currentid].0.rgba.g = (color.g * 255.0) as i16;
                self.data[self.currentid].0.rgba.b = (color.b * 255.0) as i16;
                self.data[self.currentid].0.rgba.a = (color.a * 255.0) as i16;
                self.generic.show_color = false;
            }
            Message::CancelColor => {
                self.generic.show_color = false;
                return None;
            }
            _ => {
                return None;
            }
        }

        self.data[self.currentid].1 = true;
        None
    }

    fn view(&self) -> Element<Message> {
        self.layout()
    }

    fn title(&self) -> &str {
        "Item Editor"
    }
}

impl ItemUI {
    pub fn new() -> Self {
        let config: ConfigData = load_config();

        let mut ui = ItemUI {
            data: ItemData::load_files(config.save_json).unwrap(),
            config,
            ..Default::default()
        };

        for (i, v) in ui.data.iter().enumerate() {
            ui.menu.list.push(ListData::new(i, v.0.name.clone()));
        }

        ui.menu.list_selected = Some(ui.menu.list[0].clone());

        for i in 0..ItemTypes::Count as usize {
            ui.generic.type_list.push(ItemTypes::from_index(i));
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
                    println!("Could not save Item {}, err {}", i, e);
                }
            }
            if let Err(e) = v.0.save_bin_file(i) {
                println!("Could not save bin Item {}, err {}", i, e);
            }
        }
    }

    fn set_object_to_layout(&mut self, index: usize) {
        for (id, control) in self.data_ui.input.iter_mut().enumerate() {
            control.value = self.data[index].0.data[id];
        }

        self.generic.type_selected = Some(self.data[index].0.itemtype);
        self.generic.txt_value = self.data[index].0.name.clone();
        self.generic.level_input.value = self.data[index].0.levelreq;
        self.generic.sound_input.value = self.data[index].0.soundid;
        self.generic.stack_limit_input.value = self.data[index].0.stacklimit;
        self.generic.sprite_input.value = self.data[index].0.sprite;
        self.generic.type2.value = self.data[index].0.itemtype2 as u16;
        self.generic.base_price_input.value = self.data[index].0.baseprice;
        self.generic.repairable = self.data[index].0.repairable;
        self.generic.stackable = self.data[index].0.stackable;
        self.generic.breakable = self.data[index].0.breakable;

        if let Some(name) = self.data[index].0.sound_index.clone() {
            self.generic.sound_name = name;
        } else {
            self.generic.sound_name = String::new();
        }

        if let Some(data) = self.data[index].0.animation {
            self.generic.animation_input.value = data as i32;
        } else {
            self.generic.animation_input.value = -1;
        }

        self.generic.color = Color::new(
            f32::from(self.data[index].0.rgba.r) / 255.0,
            f32::from(self.data[index].0.rgba.g) / 255.0,
            f32::from(self.data[index].0.rgba.b) / 255.0,
            f32::from(self.data[index].0.rgba.a) / 255.0,
        );
    }

    pub fn layout(&self) -> Element<Message> {
        let item_type = self.generic.type_selected.unwrap_or(ItemTypes::None);

        Container::new(
            column![
                self.menu.layout(),
                Scrollable::new(row![
                    column![
                        Container::new(
                            self.generic
                                .layout(self.generic.type_selected.unwrap_or(ItemTypes::None),)
                        )
                        .padding(5)
                        .width(Length::Fill)
                        .center_x()
                        .center_y(),
                        Container::new(self.data_ui.layout(item_type))
                            .padding(5)
                            .width(Length::Fill)
                            .center_x()
                            .center_y(),
                    ]
                    .spacing(5)
                    .width(Length::FillPortion(30)),
                    Column::new().width(Length::FillPortion(1))
                ])
            ]
            .spacing(20),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(10)
        .into()
    }
}
