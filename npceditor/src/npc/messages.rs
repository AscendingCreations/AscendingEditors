use ascending_types::*;
use ascending_ui::*;
use iced_aw::time_picker::Time;

#[derive(Debug, Clone)]
pub enum Message {
    SaveButtonPress,
    SaveAllButtonPress,
    RevertButtonPress,
    ListSelect(ListData),
    GenericU8Input((usize, NumInputMessage<u8>)),
    GenericI32Input((usize, NumInputMessage<i32>)),
    GenericU32Input((usize, NumInputMessage<u32>)),
    GenericI64Input((usize, NumInputMessage<i64>)),
    GenericBoolInput((usize, CheckBoxMessage)),
    ChooseTime1,
    SubmitTime1(Time),
    ChooseTime2,
    SubmitTime2(Time),
    CancelTime,
    BehaviourTypeSelect(AIBehavior),
    NameInput(String),
    ItemDropSlotSelect(usize),
    EnemyListSelect(ListData),
    AddEnemy,
    RemoveEnemy,
    UpdateEnemy,
}
