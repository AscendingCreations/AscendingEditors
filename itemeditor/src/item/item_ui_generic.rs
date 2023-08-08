use crate::item::*;
use araiseal_styles::{self, CheckBoxStyle, TEXT_WHITE};
use araiseal_types::*;
use araiseal_ui::*;
use iced::{
    alignment::{Alignment, Horizontal, Vertical},
    theme,
    widget::{Checkbox, Column, Container, Image, PickList, Row, Rule, Text, TextInput},
    Color, Element, Length,
};
use iced_aw::{style::color_picker::ColorPickerStyles, ColorPicker};
use std::string::ToString;

#[allow(dead_code)]
#[derive(Educe)]
#[educe(Default)]
pub struct ItemUiGeneric {
    #[educe(Default(expression = "Vec::with_capacity(23)"))]
    pub type_list: Vec<ItemTypes>,
    pub type_selected: Option<ItemTypes>,
    pub txt_value: String,
    #[educe(Default(expression = "NumInput::new(0)"))]
    pub level_input: NumInput<u16, Message>,
    #[educe(Default(expression = "NumInput::new(0)"))]
    pub sound_input: NumInput<u16, Message>,
    #[educe(Default(expression = "NumInput::new(0)"))]
    pub sprite_input: NumInput<u16, Message>,
    #[educe(Default(expression = "NumInput::new(0)"))]
    pub type2: NumInput<u16, Message>,
    pub breakable: bool,
    pub stackable: bool,
    pub repairable: bool,
    #[educe(Default(expression = "NumInput::new(0)"))]
    pub stack_limit_input: NumInput<u16, Message>,
    #[educe(Default(expression = "NumInput::new(0)"))]
    pub base_price_input: NumInput<u64, Message>,
    pub color: Color,
    pub show_color: bool,
}

impl ItemUiGeneric {
    pub fn layout(&self, _item_type: ItemTypes) -> Element<Message> {
        let column = Column::new()
            .spacing(6)
            .align_items(Alignment::Center)
            .push(
                Text::new("Generic")
                    .width(Length::Fill)
                    .vertical_alignment(Vertical::Bottom)
                    .horizontal_alignment(Horizontal::Center),
            )
            .push(Rule::horizontal(0));

        let row = Row::new()
            .spacing(12)
            .align_items(Alignment::Center)
            .push(
                Column::new().push(
                    TextInput::new("Name", &self.txt_value)
                        .on_submit(Message::NameInput)
                        .width(Length::Fixed(256.0))
                        .padding(3),
                ),
            )
            .push(
                Row::new().spacing(6).push(
                    Text::new("Item Type")
                        .horizontal_alignment(Horizontal::Center),
                ), /*.push(PickList::new(
                       &self.type_list[..],
                       self.type_selected,
                       Message::TypeSelect,
                   )),*/
            )
            .push(
                Row::new().spacing(6).push(
                    Text::new("Item Type 2")
                        .horizontal_alignment(Horizontal::Center),
                ), /* .push(self.type2.view(5, 0, 100, 1, Message::GenericInput)),*/
            );

        let sprite_value = self.sprite_input.value;

        let row1 = Row::new().spacing(6).push(
            Column::new()
                .spacing(5)
                .push(Text::new("Item Sprite".to_string()).style(theme::Text::Color(TEXT_WHITE)))
                .push(
                    Row::new()
                        .align_items(Alignment::Center)
                        .spacing(6)
                        .push(self.sprite_input.view(4, 0, 1000, 1, Message::GenericInput))
                        .push(
                            Container::new(
                                Image::new(format!("./resources/items/{}.png", sprite_value))
                                    .width(Length::Fixed(32.0))
                                    .height(Length::Fixed(32.0)),
                            )
                            .center_x()
                            .center_y()
                            .width(Length::Fixed(32.0))
                            .height(Length::Fixed(32.0)),
                        ),
                ),
        );

        let row2 = Row::new()
            .spacing(6)
            .push(
                Column::new()
                    .spacing(5)
                    .push(Text::new("Level Req".to_string()).style(theme::Text::Color(TEXT_WHITE)))
                    .push(self.level_input.view(1, 0, 200, 1, Message::GenericInput)),
            )
            .push(
                Column::new()
                    .spacing(5)
                    .push(Text::new("Sound ID".to_string()).style(theme::Text::Color(TEXT_WHITE)))
                    .push(self.sound_input.view(2, 0, 100, 1, Message::GenericInput)),
            )
            .push(
                Column::new()
                    .spacing(5)
                    .push(
                        Text::new("Stack Limit".to_string()).style(theme::Text::Color(TEXT_WHITE)),
                    )
                    .push(
                        self.stack_limit_input
                            .view(3, 1, 1000, 1, Message::GenericInput),
                    ),
            )
            .push(
                Column::new()
                    .spacing(5)
                    .push(
                        Text::new("Base Store Price".to_string())
                            .style(theme::Text::Color(TEXT_WHITE)),
                    )
                    .push(
                        self.base_price_input
                            .view(0, 0, 999999999, 1, Message::BasePriceInput),
                    ),
            );

        let row4 = Row::new()
            .spacing(6)
            .push(Checkbox::new(
                "Breakable".to_owned(),
                self.breakable,
                Message::Breakable,
            ))
            .push(Checkbox::new(
                "Repairable".to_owned(),
                self.repairable,
                Message::Repairable,
            ))
            .push(Checkbox::new(
                "Stackable".to_owned(),
                self.stackable,
                Message::Stackable,
            ));

        let colorpicker = ColorPicker::new(
            self.show_color,
            self.color,
            button("Set Color")
                .style(theme::Button::Custom(Box::new(
                    araiseal_styles::Button::Secondary,
                )))
                .on_press(Message::ChooseColor),
            Message::CancelColor,
            Message::SubmitColor,
        )
        .style(ColorPickerStyles::custom(
            araiseal_styles::CustomColorPicker,
        ));

        let style = Box::new(araiseal_styles::ColorContainer { color: self.color });

        let row5 = Row::new()
            .spacing(10)
            .align_items(Alignment::Center)
            .push(colorpicker)
            .push(
                Text::new(format!(
                    "R: {:.0} G: {:.0} B: {:.0} A: {:.0}",
                    self.color.r * 255.0,
                    self.color.g * 255.0,
                    self.color.b * 255.0,
                    self.color.a * 255.0
                ))
                .style(theme::Text::Color(TEXT_WHITE)),
            )
            .push(
                Container::new(Row::new())
                    .height(Length::Fixed(32.0))
                    .width(Length::Fixed(32.0))
                    .style(theme::Container::Custom(style)),
            );

        column.push(row1).push(row2).push(row4).push(row5).into()
    }
}
