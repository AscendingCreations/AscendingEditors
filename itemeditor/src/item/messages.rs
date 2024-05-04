use araiseal_types::*;
use araiseal_ui::*;
use iced::{font, Color};

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
    Repairable(bool),
    Stackable(bool),
    Breakable(bool),
    NameInput(String),
    SoundInput(String),
    ChooseColor,
    SubmitColor(Color),
    CancelColor,
    FontLoaded(Result<(), font::Error>),
}
