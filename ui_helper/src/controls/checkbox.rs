#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckBoxMessage {
    Change(bool),
}

impl CheckBoxMessage {
    pub fn get_data(&self) -> bool {
        let CheckBoxMessage::Change(data) = self;
        *data
    }
}
