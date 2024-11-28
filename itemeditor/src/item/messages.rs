use ascending_types::*;
use ascending_ui::*;
use iced::Color;

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    SaveButtonPress,
    SaveAllButtonPress,
    RevertButtonPress,
    ListSelect(ListData),
    TypeSelect(ItemTypes),
    DataInput((usize, NumInputMessage<i16>)),
    GenericInput((usize, NumInputMessage<u16>)),
    GenericI32Input((usize, NumInputMessage<i32>)),
    BasePriceInput((usize, NumInputMessage<u64>)),
    GenericBoolInput((usize, CheckBoxMessage)),
    NameInput(String),
    SoundInput(String),
    ChooseColor,
    SubmitColor(Color),
    CancelColor,
}
