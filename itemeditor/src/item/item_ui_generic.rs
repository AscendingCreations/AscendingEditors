use crate::item::*;
use ascending_types::*;
use ascending_ui::*;
use iced::{
    alignment::Alignment,
    widget::{
        button, checkbox, column, container, row, text, text_input, Container, Image, PickList,
        Row, Rule,
    },
    Color, Element, Length,
};
use iced_aw::ColorPicker;
use std::path::Path;

#[allow(dead_code)]
#[derive(Educe)]
#[educe(Default)]
pub struct ItemUiGeneric {
    #[educe(Default(expression = Vec::with_capacity(23)))]
    pub type_list: Vec<ItemTypes>,
    pub type_selected: Option<ItemTypes>,
    pub txt_value: String,
    #[educe(Default(expression = NumInput::new(0)))]
    pub level_input: NumInput<u16, Message>,
    #[educe(Default(expression = NumInput::new(0)))]
    pub sound_input: NumInput<u16, Message>,
    #[educe(Default(expression = NumInput::new(0)))]
    pub sprite_input: NumInput<u16, Message>,
    #[educe(Default(expression = NumInput::new(0)))]
    pub type2: NumInput<u16, Message>,
    pub breakable: bool,
    pub stackable: bool,
    pub repairable: bool,
    #[educe(Default(expression = NumInput::new(0)))]
    pub stack_limit_input: NumInput<u16, Message>,
    #[educe(Default(expression = NumInput::new(0)))]
    pub base_price_input: NumInput<u64, Message>,
    #[educe(Default(expression = NumInput::new(-1)))]
    pub animation_input: NumInput<i32, Message>,
    pub sound_name: String,
    pub color: Color,
    pub show_color: bool,
}

impl ItemUiGeneric {
    pub fn layout(&self, _item_type: ItemTypes) -> Element<Message> {
        let row0 = row![
            text_input("Name", &self.txt_value)
                .on_input(Message::NameInput)
                .width(Length::Fixed(256.0))
                .padding(3),
            row![text("Item Type")].spacing(6),
            PickList::new(&self.type_list[..], self.type_selected, Message::TypeSelect,),
            row![text("Item Type 2")].spacing(6),
            self.type2.view(5, 0, 100, 1, Message::GenericInput, None),
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        let sprite_value = self.sprite_input.value;

        let image_path = format!("./resources/items/i{}.png", sprite_value);

        let row1 = row![column![
            "Item Sprite",
            row![
                self.sprite_input
                    .view(4, 0, 1000, 1, Message::GenericInput, None),
                if Path::new(&image_path).exists() {
                    container(
                        Image::new(&image_path)
                            .width(Length::Fixed(32.0))
                            .height(Length::Fixed(32.0)),
                    )
                    .width(Length::Fixed(32.0))
                    .height(Length::Fixed(32.0))
                } else {
                    Container::new("")
                        .width(Length::Fixed(32.0))
                        .height(Length::Fixed(32.0))
                }
            ]
            .align_y(Alignment::Center)
            .spacing(6),
        ]
        .spacing(5),]
        .spacing(6);

        let row2 = row![
            column![
                "Level Req",
                self.level_input
                    .view(1, 0, 200, 1, Message::GenericInput, None)
            ]
            .spacing(5),
            column![
                "Sound ID",
                self.sound_input
                    .view(2, 0, 100, 1, Message::GenericInput, None)
            ]
            .spacing(5),
            column![
                "Stack Limit",
                self.stack_limit_input
                    .view(3, 1, 1000, 1, Message::GenericInput, None)
            ]
            .spacing(5),
            column![
                "Base Store Price",
                self.base_price_input
                    .view(0, 0, 999999999, 1, Message::BasePriceInput, None)
            ]
            .spacing(5),
            column![
                "Animation",
                self.animation_input
                    .view(0, -1, 99999, 1, Message::GenericI32Input, None)
            ]
            .spacing(5)
        ]
        .spacing(6);

        let row3 = row![
            text("Sound Name:"),
            text_input("Sound Name", &self.sound_name)
                .on_input(Message::SoundInput)
                .width(Length::Fixed(256.0))
                .padding(3),
        ]
        .spacing(6);

        let row4 = row![
            checkbox("Breakable", self.breakable)
                .on_toggle(move |i| { Message::GenericBoolInput((0, CheckBoxMessage::Change(i))) }),
            checkbox("Repairable", self.repairable)
                .on_toggle(move |i| { Message::GenericBoolInput((1, CheckBoxMessage::Change(i))) }),
            checkbox("Stackable", self.stackable)
                .on_toggle(move |i| { Message::GenericBoolInput((2, CheckBoxMessage::Change(i))) }),
        ]
        .spacing(6);

        let colorpicker = ColorPicker::new(
            self.show_color,
            self.color,
            button("Set Color").on_press(Message::ChooseColor),
            Message::CancelColor,
            Message::SubmitColor,
        );

        let row5 = row![
            colorpicker,
            text(format!(
                "R: {:.0} G: {:.0} B: {:.0} A: {:.0}",
                self.color.r * 255.0,
                self.color.g * 255.0,
                self.color.b * 255.0,
                self.color.a * 255.0
            )),
            Container::new(Row::new())
                .height(Length::Fixed(32.0))
                .width(Length::Fixed(32.0))
        ]
        .spacing(10)
        .align_y(Alignment::Center);

        column![
            text("Generic"),
            Rule::horizontal(0),
            row0,
            row1,
            row2,
            row3,
            row4,
            row5
        ]
        .spacing(6)
        .align_x(Alignment::Center)
        .into()
    }
}
