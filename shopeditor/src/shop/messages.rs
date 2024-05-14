use ascending_ui::*;
use iced::font;

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    SaveButtonPress,
    SaveAllButtonPress,
    RevertButtonPress,
    ListSelect(ListData),
    GenericInput((usize, NumInputMessage<u16>)),
    GenericInput64((usize, NumInputMessage<u64>)),
    NameInput(String),
    SlotSelect(u16),
    FontLoaded(Result<(), font::Error>),
}
