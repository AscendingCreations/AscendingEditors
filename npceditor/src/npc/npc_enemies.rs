use crate::npc::*;
use ascending_ui::*;
use iced::{
    alignment::Horizontal,
    widget::{column, row, text, PickList, Rule},
    Element, Length,
};

#[derive(Educe)]
#[educe(Default)]
pub struct NpcEnemies {
    pub enemy_list: Vec<ListData>,
    pub enemydrop_select: Option<ListData>,
    #[educe(Default(expression = NumInput::new(0)))]
    pub npc_index_input: NumInput<u32, Message>,
}

impl NpcEnemies {
    pub fn layout(&self) -> Element<Message> {
        column![
            row![
                Rule::horizontal(0),
                text("NPC Enemies:").horizontal_alignment(Horizontal::Center),
                Rule::horizontal(0),
            ],
            row![
                text("Enemy List:"),
                PickList::new(
                    &self.enemy_list[..],
                    self.enemydrop_select.clone(),
                    Message::EnemyListSelect,
                ),
                text("Npc Index:"),
                self.npc_index_input
                    .view(21, 0, u32::MAX, 1, Message::GenericU32Input),
                button("Update").on_press(Message::UpdateEnemy),
                button("Add").on_press(Message::AddEnemy),
                button("Remove").on_press(Message::RemoveEnemy),
            ]
            .spacing(15),
        ]
        .width(Length::Fill)
        .spacing(12)
        .into()
    }
}

pub fn add_enemy_data(npc_ui: &mut NpcUI, npc_index: u32) {
    if npc_ui.data[npc_ui.currentid].0.enemies.is_empty() {
        npc_ui.enemies.enemy_list.clear();
    }

    let list_count = npc_ui.data[npc_ui.currentid].0.enemies.len();

    npc_ui.enemies.enemy_list.push(ListData::new(
        list_count,
        npc_ui.data[npc_index as usize].0.name.clone(),
    ));
    npc_ui.enemies.enemydrop_select = Some(npc_ui.enemies.enemy_list[list_count].clone());
    npc_ui.data[npc_ui.currentid]
        .0
        .enemies
        .push(npc_index as u64);
}

pub fn remove_enemy_data(npc_ui: &mut NpcUI) {
    if npc_ui.data[npc_ui.currentid].0.enemies.is_empty() {
        return;
    }

    npc_ui.data[npc_ui.currentid]
        .0
        .enemies
        .remove(npc_ui.currentenemyslot);

    new_enemies_data(npc_ui, npc_ui.currentid);
}

pub fn update_enemy_data(npc_ui: &mut NpcUI, npc_index: u32) {
    let got_data = !npc_ui.data[npc_ui.currentid].0.enemies.is_empty();

    if !got_data {
        npc_ui.enemies.enemy_list.clear();
        npc_ui.enemies.enemy_list.push(ListData::new(
            0,
            npc_ui.data[npc_index as usize].0.name.clone(),
        ));
        npc_ui.data[npc_ui.currentid]
            .0
            .enemies
            .push(npc_index as u64);
        npc_ui.currentenemyslot = 0;
    } else {
        npc_ui.enemies.enemy_list[npc_ui.currentenemyslot]
            .name
            .clone_from(&npc_ui.data[npc_index as usize].0.name);
        npc_ui.data[npc_ui.currentid].0.enemies[npc_ui.currentenemyslot] = npc_index as u64;
    }

    npc_ui.enemies.enemydrop_select =
        Some(npc_ui.enemies.enemy_list[npc_ui.currentenemyslot].clone());
}

pub fn new_enemies_data(npc_ui: &mut NpcUI, index: usize) {
    let got_data = !npc_ui.data[index].0.enemies.is_empty();

    npc_ui.enemies.enemy_list.clear();

    if !got_data {
        npc_ui
            .enemies
            .enemy_list
            .push(ListData::new(0, String::new()));
    } else {
        for (i, &npcindex) in npc_ui.data[index].0.enemies.iter().enumerate() {
            npc_ui.enemies.enemy_list.push(ListData::new(
                i,
                npc_ui.data[npcindex as usize].0.name.clone(),
            ));
        }
        npc_ui.enemies.npc_index_input.value = npc_ui.data[index].0.enemies[0] as u32;
    }
    npc_ui.enemies.enemydrop_select = Some(npc_ui.enemies.enemy_list[0].clone());
}

pub fn select_list(npc_ui: &mut NpcUI) {
    npc_ui.enemies.npc_index_input.value =
        npc_ui.data[npc_ui.currentid].0.enemies[npc_ui.currentenemyslot] as u32;
}
