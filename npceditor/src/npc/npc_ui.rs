use crate::{load_config, npc::*, ConfigData};
use ascending_types::*;
use ascending_ui::*;
use iced::{
    widget::{column, scrollable, Container},
    Element, Length,
};

#[allow(dead_code)]
#[derive(Educe)]
#[educe(Default)]
pub struct NpcUI {
    #[educe(Default(expression = Vec::with_capacity(MAX_NPCS)))]
    pub data: Vec<(NpcData, bool)>,
    menu: NpcUIMenu,
    generic: NpcUIGenerics,
    settings: NpcUISettings,
    pub enemies: NpcEnemies,
    pub currentid: usize,
    pub currentitemdropslot: usize,
    pub currentenemyslot: usize,
    config: ConfigData,
}

impl UiRenderer for NpcUI {
    type Message = Message;

    fn update(&mut self, msg: Message) {
        match msg {
            Message::SaveAllButtonPress => {
                self.save_all();
                return;
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
                return;
            }
            Message::RevertButtonPress => {
                let item = NpcData::load_file(self.currentid).unwrap();
                self.data[self.currentid].0 = item.0;
                self.data[self.currentid].1 = false;
                self.set_object_to_layout(self.currentid);
                return;
            }
            Message::ListSelect(data) => {
                self.currentid = data.id;
                self.menu.list_selected = Some(data);
                self.set_object_to_layout(self.currentid);
                return;
            }
            Message::BehaviourTypeSelect(data) => {
                self.generic.behaviour_selected = Some(data);
                self.data[self.currentid].0.behaviour = data;
            }
            Message::GenericBoolInput((id, data)) => match id {
                0 => {
                    self.settings.target_auto_switch = data.get_data();
                    self.data[self.currentid].0.target_auto_switch = data.get_data();
                }
                1 => {
                    self.settings.target_attacked_switch = data.get_data();
                    self.data[self.currentid].0.target_attacked_switch = data.get_data();
                }
                2 => {
                    self.settings.target_range_dropout = data.get_data();
                    self.data[self.currentid].0.target_range_dropout = data.get_data();
                }
                3 => {
                    self.settings.can_target = data.get_data();
                    self.data[self.currentid].0.can_target = data.get_data();
                }
                4 => {
                    self.settings.can_move = data.get_data();
                    self.data[self.currentid].0.can_move = data.get_data();
                }
                5 => {
                    self.settings.can_attack_player = data.get_data();
                    self.data[self.currentid].0.can_attack_player = data.get_data();
                }
                6 => {
                    self.settings.has_selfonly = data.get_data();
                    self.data[self.currentid].0.has_selfonly = data.get_data();
                }
                7 => {
                    self.settings.has_friendonly = data.get_data();
                    self.data[self.currentid].0.has_friendonly = data.get_data();
                }
                8 => {
                    self.settings.has_groundonly = data.get_data();
                    self.data[self.currentid].0.has_groundonly = data.get_data();
                }
                9 => {
                    self.settings.has_allys = data.get_data();
                    self.data[self.currentid].0.has_allys = data.get_data();
                }
                10 => {
                    self.settings.can_attack = data.get_data();
                    self.data[self.currentid].0.can_attack = data.get_data();
                }
                11 => {
                    self.settings.runsaway = data.get_data();
                    self.data[self.currentid].0.runsaway = data.get_data();
                }
                12 => {
                    self.settings.canpassthru = data.get_data();
                    self.data[self.currentid].0.canpassthru = data.get_data();
                }
                13 => {
                    self.settings.isanimated = data.get_data();
                    self.data[self.currentid].0.isanimated = data.get_data();
                }
                14 => {
                    self.settings.has_enemies = data.get_data();
                    self.data[self.currentid].0.has_enemies = data.get_data();
                }
                _ => return,
            },
            Message::GenericU8Input((id, data)) => match id {
                0 => {
                    self.generic.sizex_input.value = data.get_data();
                    self.data[self.currentid].0.size.x = data.get_data();
                }
                1 => {
                    self.generic.sizey_input.value = data.get_data();
                    self.data[self.currentid].0.size.y = data.get_data();
                }
                2 => {
                    self.generic.sizeh_input.value = data.get_data();
                    self.data[self.currentid].0.size.height = data.get_data();
                }
                3 => {
                    self.generic.sizew_input.value = data.get_data();
                    self.data[self.currentid].0.size.width = data.get_data();
                }
                _ => return,
            },
            Message::GenericI32Input((id, data)) => match id {
                0 => {
                    self.generic.sprite_input.value = data.get_data();
                    self.data[self.currentid].0.sprite = data.get_data();
                }
                1 => {
                    self.generic.level_input.value = data.get_data();
                    self.data[self.currentid].0.level = data.get_data();
                }
                2 => {
                    self.generic.sight_input.value = data.get_data();
                    self.data[self.currentid].0.sight = data.get_data();
                }
                3 => {
                    self.generic.follow_sight_input.value = data.get_data();
                    self.data[self.currentid].0.follow_sight = data.get_data();
                }
                4 => {
                    self.generic.range_input.value = data.get_data();
                    self.data[self.currentid].0.range = data.get_data();
                }
                _ => return,
            },
            Message::GenericU32Input((id, data)) => match id {
                0 => {
                    self.settings.run_damage.value = data.get_data();
                    self.data[self.currentid].0.run_damage = data.get_data();
                }
                1 => {
                    self.generic.maxhp_input.value = data.get_data();
                    self.data[self.currentid].0.maxhp = data.get_data();
                }
                2 => {
                    self.generic.maxsp_input.value = data.get_data();
                    self.data[self.currentid].0.maxsp = data.get_data();
                }
                3 => {
                    self.generic.maxmp_input.value = data.get_data();
                    self.data[self.currentid].0.maxmp = data.get_data();
                }
                4 => {
                    self.generic.pdamage_input.value = data.get_data();
                    self.data[self.currentid].0.pdamage = data.get_data();
                }
                5 => {
                    self.generic.pdef_input.value = data.get_data();
                    self.data[self.currentid].0.pdefense = data.get_data();
                }
                6 => {
                    self.generic.walkdistance_input.value = data.get_data();
                    self.data[self.currentid].0.walkdistance = data.get_data();
                }
                7 => {
                    self.generic.mindamage_input.value = data.get_data();
                    self.data[self.currentid].0.mindamage = data.get_data();
                }
                8 => {
                    self.generic.maxdamage_input.value = data.get_data();
                    self.data[self.currentid].0.maxdamage = data.get_data();
                }
                // Drop ID
                9 => {
                    self.generic.item_drops[0].item_id.value = data.get_data();
                    self.data[self.currentid].0.drops[self.currentitemdropslot].items[0].item =
                        data.get_data();
                }
                10 => {
                    self.generic.item_drops[1].item_id.value = data.get_data();
                    self.data[self.currentid].0.drops[self.currentitemdropslot].items[1].item =
                        data.get_data();
                }
                11 => {
                    self.generic.item_drops[2].item_id.value = data.get_data();
                    self.data[self.currentid].0.drops[self.currentitemdropslot].items[2].item =
                        data.get_data();
                }
                12 => {
                    self.generic.item_drops[3].item_id.value = data.get_data();
                    self.data[self.currentid].0.drops[self.currentitemdropslot].items[3].item =
                        data.get_data();
                }
                13 => {
                    self.generic.item_drops[4].item_id.value = data.get_data();
                    self.data[self.currentid].0.drops[self.currentitemdropslot].items[4].item =
                        data.get_data();
                }
                // Drop Amount
                14 => {
                    self.generic.item_drops[0].amount.value = data.get_data();
                    self.data[self.currentid].0.drops[self.currentitemdropslot].items[0].amount =
                        data.get_data();
                }
                15 => {
                    self.generic.item_drops[1].amount.value = data.get_data();
                    self.data[self.currentid].0.drops[self.currentitemdropslot].items[1].amount =
                        data.get_data();
                }
                16 => {
                    self.generic.item_drops[2].amount.value = data.get_data();
                    self.data[self.currentid].0.drops[self.currentitemdropslot].items[2].amount =
                        data.get_data();
                }
                17 => {
                    self.generic.item_drops[3].amount.value = data.get_data();
                    self.data[self.currentid].0.drops[self.currentitemdropslot].items[3].amount =
                        data.get_data();
                }
                18 => {
                    self.generic.item_drops[4].amount.value = data.get_data();
                    self.data[self.currentid].0.drops[self.currentitemdropslot].items[4].amount =
                        data.get_data();
                }
                // Drop Data
                19 => {
                    self.generic.slotshares_input.value = data.get_data();
                    self.data[self.currentid].0.drops[self.currentitemdropslot].shares =
                        data.get_data();
                }
                20 => {
                    self.generic.freeshares_input.value = data.get_data();
                    self.data[self.currentid].0.free_shares = data.get_data();
                }
                // Enemy
                21 => {
                    self.enemies.npc_index_input.value = data.get_data();
                }
                _ => return,
            },
            Message::GenericI64Input((id, data)) => match id {
                0 => {
                    self.settings.target_switch_chance.value = data.get_data();
                    self.data[self.currentid].0.target_auto_switch_chance = data.get_data();
                }
                1 => {
                    self.generic.movement_wait_input.value = data.get_data();
                    self.data[self.currentid].0.movement_wait = data.get_data();
                }
                2 => {
                    self.generic.attack_wait_input.value = data.get_data();
                    self.data[self.currentid].0.attack_wait = data.get_data();
                }
                3 => {
                    self.generic.intervaled_wait_input.value = data.get_data();
                    self.data[self.currentid].0.intervaled_wait = data.get_data();
                }
                4 => {
                    self.generic.respawn_wait_input.value = data.get_data();
                    self.data[self.currentid].0.respawn_wait = data.get_data();
                }
                5 => {
                    self.generic.spawn_wait_input.value = data.get_data();
                    self.data[self.currentid].0.spawn_wait = data.get_data();
                }
                6 => {
                    self.generic.exp_input.value = data.get_data();
                    self.data[self.currentid].0.exp = data.get_data();
                }
                _ => return,
            },
            Message::ChooseTime1 => {
                self.settings.show_time[0] = true;
                return;
            }
            Message::SubmitTime1(time) => {
                use chrono::Timelike;
                let naive: chrono::NaiveTime = time.into();
                self.data[self.currentid].0.spawntime.0.hour = naive.hour();
                self.data[self.currentid].0.spawntime.0.min = naive.minute();
                self.data[self.currentid].0.spawntime.0.sec = naive.second();
                self.settings.spawntime_data.0 = self.data[self.currentid].0.spawntime.0;
            }
            Message::ChooseTime2 => {
                self.settings.show_time[1] = true;
                return;
            }
            Message::SubmitTime2(time) => {
                use chrono::Timelike;
                let naive: chrono::NaiveTime = time.into();
                self.data[self.currentid].0.spawntime.1.hour = naive.hour();
                self.data[self.currentid].0.spawntime.1.min = naive.minute();
                self.data[self.currentid].0.spawntime.1.sec = naive.second();
                self.settings.spawntime_data.1 = self.data[self.currentid].0.spawntime.1;
            }
            Message::CancelTime => {
                self.settings.show_time[0] = false;
                self.settings.show_time[1] = false;
            }
            Message::NameInput(value) => {
                if value.len() < 64 {
                    self.generic.txt_value = value;
                    self.data[self.currentid]
                        .0
                        .name
                        .clone_from(&self.generic.txt_value);
                    self.menu.list[self.currentid]
                        .name
                        .clone_from(&self.generic.txt_value);
                    self.menu.list_selected = Some(self.menu.list[self.currentid].clone());
                } else {
                    return;
                }
            }
            Message::ItemDropSlotSelect(data) => {
                self.generic.itemdrop_selected = Some(data);
                self.currentitemdropslot = data;

                self.generic.slotshares_input.value =
                    self.data[self.currentid].0.drops[self.currentitemdropslot].shares;
                self.generic.item_drops[0].item_id.value =
                    self.data[self.currentid].0.drops[self.currentitemdropslot].items[0].item;
                self.generic.item_drops[0].amount.value =
                    self.data[self.currentid].0.drops[self.currentitemdropslot].items[0].amount;
                self.generic.item_drops[1].item_id.value =
                    self.data[self.currentid].0.drops[self.currentitemdropslot].items[1].item;
                self.generic.item_drops[1].amount.value =
                    self.data[self.currentid].0.drops[self.currentitemdropslot].items[1].amount;
                self.generic.item_drops[2].item_id.value =
                    self.data[self.currentid].0.drops[self.currentitemdropslot].items[2].item;
                self.generic.item_drops[2].amount.value =
                    self.data[self.currentid].0.drops[self.currentitemdropslot].items[2].amount;
                self.generic.item_drops[3].item_id.value =
                    self.data[self.currentid].0.drops[self.currentitemdropslot].items[3].item;
                self.generic.item_drops[3].amount.value =
                    self.data[self.currentid].0.drops[self.currentitemdropslot].items[3].amount;
                self.generic.item_drops[4].item_id.value =
                    self.data[self.currentid].0.drops[self.currentitemdropslot].items[4].item;
                self.generic.item_drops[4].amount.value =
                    self.data[self.currentid].0.drops[self.currentitemdropslot].items[4].amount;
            }
            Message::EnemyListSelect(data) => {
                self.currentenemyslot = data.id;
                self.enemies.enemydrop_select = Some(data);
                select_list(self);
            }
            Message::AddEnemy => add_enemy_data(self, self.enemies.npc_index_input.value),
            Message::UpdateEnemy => update_enemy_data(self, self.enemies.npc_index_input.value),
            Message::RemoveEnemy => remove_enemy_data(self),
        }

        self.data[self.currentid].1 = true;
    }

    fn view(&self) -> Element<Message> {
        self.layout()
    }
}

impl NpcUI {
    pub fn new() -> Self {
        let config: ConfigData = load_config();

        let mut ui = NpcUI {
            data: NpcData::load_files(config.save_json).unwrap(),
            config,
            ..Default::default()
        };

        for (i, v) in ui.data.iter().enumerate() {
            ui.menu.list.push(ListData::new(i, v.0.name.clone()));
        }

        for i in 0..=AIBehavior::ReactiveHealer as usize {
            ui.generic.behaviours.push(AIBehavior::from_index(i))
        }

        ui.menu.list_selected = Some(ui.menu.list[0].clone());
        ui.set_object_to_layout(0);

        {
            new_enemies_data(&mut ui, 0);
        }

        ui
    }

    pub fn save_all(&mut self) {
        for (i, v) in self.data.iter().enumerate() {
            if !v.1 {
                continue;
            }

            if self.config.save_json {
                if let Err(e) = v.0.save_file(i) {
                    println!("Could not save NPC {}, err {}", i, e);
                }
            }
            if let Err(e) = v.0.save_bin_file(i) {
                println!("Could not save bin NPC {}, err {}", i, e);
            }
        }
    }

    fn set_object_to_layout(&mut self, index: usize) {
        self.generic.txt_value.clone_from(&self.data[index].0.name);
        self.generic.sprite_input.value = self.data[index].0.sprite;
        self.generic.behaviour_selected = Some(self.data[index].0.behaviour);
        self.generic.sizex_input.value = self.data[index].0.size.x;
        self.generic.sizey_input.value = self.data[index].0.size.y;
        self.generic.sizeh_input.value = self.data[index].0.size.height;
        self.generic.sizew_input.value = self.data[index].0.size.width;
        self.generic.sprite_input.value = self.data[index].0.sprite;
        self.generic.level_input.value = self.data[index].0.level;
        self.generic.sight_input.value = self.data[index].0.sight;
        self.generic.follow_sight_input.value = self.data[index].0.follow_sight;
        self.generic.maxhp_input.value = self.data[index].0.maxhp;
        self.generic.maxsp_input.value = self.data[index].0.maxsp;
        self.generic.maxmp_input.value = self.data[index].0.maxmp;
        self.generic.walkdistance_input.value = self.data[index].0.walkdistance;
        self.generic.pdamage_input.value = self.data[index].0.pdamage;
        self.generic.pdef_input.value = self.data[index].0.pdefense;
        self.generic.maxdamage_input.value = self.data[index].0.maxdamage;
        self.generic.mindamage_input.value = self.data[index].0.mindamage;
        self.generic.respawn_wait_input.value = self.data[index].0.respawn_wait;
        self.generic.movement_wait_input.value = self.data[index].0.movement_wait;
        self.generic.attack_wait_input.value = self.data[index].0.attack_wait;
        self.generic.intervaled_wait_input.value = self.data[index].0.intervaled_wait;
        self.generic.spawn_wait_input.value = self.data[index].0.spawn_wait;
        self.generic.range_input.value = self.data[index].0.range;
        self.generic.exp_input.value = self.data[index].0.exp;

        self.settings.target_auto_switch = self.data[index].0.target_auto_switch;
        self.settings.target_attacked_switch = self.data[index].0.target_attacked_switch;
        self.settings.target_range_dropout = self.data[index].0.target_range_dropout;
        self.settings.can_target = self.data[index].0.can_target;
        self.settings.can_move = self.data[index].0.can_move;
        self.settings.can_attack_player = self.data[index].0.can_attack_player;
        self.settings.has_selfonly = self.data[index].0.has_selfonly;
        self.settings.has_friendonly = self.data[index].0.has_friendonly;
        self.settings.has_groundonly = self.data[index].0.has_groundonly;
        self.settings.has_allys = self.data[index].0.has_allys;
        self.settings.has_enemies = self.data[index].0.has_enemies;
        self.settings.can_attack = self.data[index].0.can_attack;
        self.settings.runsaway = self.data[index].0.runsaway;
        self.settings.canpassthru = self.data[index].0.canpassthru;
        self.settings.isanimated = self.data[index].0.isanimated;
        self.settings.target_switch_chance.value = self.data[index].0.target_auto_switch_chance;
        self.settings.run_damage.value = self.data[index].0.run_damage;
        self.settings.spawntime_data.0 = self.data[index].0.spawntime.0;
        self.settings.spawntime_data.1 = self.data[index].0.spawntime.1;

        self.generic.itemdrop_selected = Some(0);
        self.currentitemdropslot = 0;

        self.generic.freeshares_input.value = self.data[index].0.free_shares;
        self.generic.slotshares_input.value =
            self.data[index].0.drops[self.currentitemdropslot].shares;

        self.generic.item_drops[0].item_id.value =
            self.data[index].0.drops[self.currentitemdropslot].items[0].item;
        self.generic.item_drops[0].amount.value =
            self.data[index].0.drops[self.currentitemdropslot].items[0].amount;
        self.generic.item_drops[1].item_id.value =
            self.data[index].0.drops[self.currentitemdropslot].items[1].item;
        self.generic.item_drops[1].amount.value =
            self.data[index].0.drops[self.currentitemdropslot].items[1].amount;
        self.generic.item_drops[2].item_id.value =
            self.data[index].0.drops[self.currentitemdropslot].items[2].item;
        self.generic.item_drops[2].amount.value =
            self.data[index].0.drops[self.currentitemdropslot].items[2].amount;
        self.generic.item_drops[3].item_id.value =
            self.data[index].0.drops[self.currentitemdropslot].items[3].item;
        self.generic.item_drops[3].amount.value =
            self.data[index].0.drops[self.currentitemdropslot].items[3].amount;
        self.generic.item_drops[4].item_id.value =
            self.data[index].0.drops[self.currentitemdropslot].items[4].item;
        self.generic.item_drops[4].amount.value =
            self.data[index].0.drops[self.currentitemdropslot].items[4].amount;

        new_enemies_data(self, index);
    }

    fn layout(&self) -> Element<Message> {
        Container::new(
            column![
                self.menu.layout(),
                scrollable(column![
                    Container::new(self.generic.layout())
                        .padding(5)
                        .width(Length::Fill)
                        .center_x(Length::Fill),
                    Container::new(self.settings.layout(&self.data[self.currentid].0))
                        .padding(5)
                        .width(Length::Fill)
                        .center_x(Length::Fill),
                    Container::new(self.enemies.layout())
                        .padding(5)
                        .width(Length::Fill)
                        .center_x(Length::Fill),
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
