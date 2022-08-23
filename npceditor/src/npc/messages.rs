use araiseal_types::*;
use araiseal_ui::*;
use iced_aw::pure::time_picker::Time;

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
    ItemInput((usize, NumInputMessage<u32>)),
    ItemChanceInput((usize, NumInputMessage<u32>)),
    ItemAmountInput((usize, NumInputMessage<u32>)),
    ChooseTime1,
    SubmitTime1(Time),
    ChooseTime2,
    SubmitTime2(Time),
    CancelTime,
    EnemyAddButtonPress,
    EnemyDelButtonPress,
    EnemyDataListSelect(ListData),
    EnemyListSelect(ListData),
    BehaviourTypeSelect(AIBehavior),
    NameInput(String),
}
