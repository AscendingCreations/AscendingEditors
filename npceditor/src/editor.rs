use ascending_logger::info;
use database::{AIBehavior, NpcData};
use gpui_kit::{
    AppContext, Context, Entity, SharedString, Subscription, Window,
    base::IndexPath,
    component::{
        WindowExt,
        input::{InputEvent, InputState},
        select::{SelectEvent, SelectState},
    },
};
use tokio::sync::mpsc::Receiver;

use crate::commands::{CommandHandler, MessageCommands};

mod renderer;

pub struct Editor {
    pub index: usize,
    pub data: Vec<(NpcData, bool)>,
    pub command_handler: CommandHandler,

    cur_tab: usize,

    data_list: Entity<SelectState<Vec<SharedString>>>,
    name_input: Entity<InputState>,

    sprite: Entity<InputState>,
    behaviour: Entity<SelectState<Vec<SharedString>>>,
    level: Entity<InputState>,
    respawn_wait: Entity<InputState>,
    movement_wait: Entity<InputState>,
    attack_wait: Entity<InputState>,
    intervaled_wait: Entity<InputState>,
    spawn_wait: Entity<InputState>,
    maxhp: Entity<InputState>,
    maxsp: Entity<InputState>,
    maxmp: Entity<InputState>,
    sight: Entity<InputState>,
    follow_sight: Entity<InputState>,
    walkdistance: Entity<InputState>,
    pdamage: Entity<InputState>,
    pdefense: Entity<InputState>,
    maxdamage: Entity<InputState>,
    mindamage: Entity<InputState>,
    run_damage: Entity<InputState>,
    free_shares: Entity<InputState>,
    target_auto_switch_chance: Entity<InputState>,
    exp: Entity<InputState>,
    range: Entity<InputState>,

    canpassthru: bool,
    target_auto_switch: bool,
    target_attacked_switch: bool,
    target_range_dropout: bool,
    can_target: bool,
    can_move: bool,
    can_attack_player: bool,
    has_allys: bool,
    has_enemies: bool,
    can_attack: bool,
    has_selfonly: bool,
    has_friendonly: bool,
    has_groundonly: bool,
    runsaway: bool,
    isanimated: bool,

    size_x: Entity<InputState>,
    size_y: Entity<InputState>,
    size_w: Entity<InputState>,
    size_h: Entity<InputState>,

    e_index: usize,
    enemy_list: Entity<SelectState<Vec<SharedString>>>,
    enemy_index: Entity<InputState>,

    start_hour: Entity<InputState>,
    start_minute: Entity<InputState>,
    start_second: Entity<InputState>,

    end_hour: Entity<InputState>,
    end_minute: Entity<InputState>,
    end_second: Entity<InputState>,

    d_index: usize,
    drop_list: Entity<SelectState<Vec<SharedString>>>,
    drop_share: Entity<InputState>,
    drop_item: Vec<Entity<InputState>>,
    drop_amount: Vec<Entity<InputState>>,

    _subscriptions: Vec<Subscription>,
}

impl Editor {
    pub fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
        command_handler: CommandHandler,
        mut rx_update: Receiver<Option<(usize, NpcData)>>,
    ) -> Self {
        info!("Loading data");
        let data = NpcData::load_files().unwrap();

        info!("Creating npc receiver");

        cx.spawn(async move |this, cx| {
            while let Some(result) = rx_update.recv().await {
                match result {
                    Some((index, data)) => {
                        let window_handle = this
                            .update(cx, move |_this, context| context.active_window())
                            .unwrap();

                        if let Some(window_handle) = window_handle {
                            let _ = cx.update_window(window_handle, |_view, win, app| {
                                this.update(app, move |this, context| {
                                    this.data[index].0 = data;
                                    this.data[index].1 = false;
                                    this.update_list(win, context);
                                    this.update_data(win, context);
                                    context.notify();
                                })
                            });
                        }
                    }
                    None => return,
                }
            }
        })
        .detach();

        info!("Creating list");
        let mut list = Vec::with_capacity(data.len());
        for (index, (data, _)) in data.iter().enumerate() {
            list.push(SharedString::new(format!("{index}: {}", data.name)));
        }

        let data_list = cx.new(|cx| SelectState::new(list, None, window, cx));
        data_list.update(cx, |state, cx| {
            state.set_selected_index(Some(IndexPath::new(0)), window, cx);
        });

        info!("Setting up widget");

        let name_input = cx.new(|cx| {
            let mut state = InputState::new(window, cx).placeholder("Name");
            state.set_value(SharedString::new(data[0].0.name.clone()), window, cx);
            state
        });

        let behaviour = cx.new(|cx| SelectState::new(AIBehavior::get_list(), None, window, cx));
        behaviour.update(cx, |state, cx| {
            state.set_selected_index(
                Some(IndexPath::new(data[0].0.behaviour as usize)),
                window,
                cx,
            );
        });

        let mut list = Vec::with_capacity(data[0].0.enemies.len());
        for val in 0..data[0].0.enemies.len() {
            list.push(SharedString::new(format!("{val}")));
        }
        let enemy_list = cx.new(|cx| SelectState::new(list, None, window, cx));
        enemy_list.update(cx, |state, cx| {
            state.set_selected_index(Some(IndexPath::new(0)), window, cx);
        });

        let enemy_index = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Enemy Index")
                .default_value(format!(
                    "{}",
                    if let Some(val) = data[0].0.enemies.first() {
                        *val
                    } else {
                        0
                    }
                ))
        });

        let sprite = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Sprite")
                .default_value(format!("{}", data[0].0.sprite))
        });
        let level = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Level")
                .default_value(format!("{}", data[0].0.level))
        });
        let respawn_wait = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Respawn Wait")
                .default_value(format!("{}", data[0].0.respawn_wait))
        });
        let movement_wait = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Movement Wait")
                .default_value(format!("{}", data[0].0.movement_wait))
        });
        let attack_wait = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Attack Wait")
                .default_value(format!("{}", data[0].0.attack_wait))
        });
        let intervaled_wait = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Interval Wait")
                .default_value(format!("{}", data[0].0.intervaled_wait))
        });
        let spawn_wait = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Spawn Wait")
                .default_value(format!("{}", data[0].0.spawn_wait))
        });
        let maxhp = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Max HP")
                .default_value(format!("{}", data[0].0.maxhp))
        });
        let maxsp = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Max SP")
                .default_value(format!("{}", data[0].0.maxsp))
        });
        let maxmp = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Max MP")
                .default_value(format!("{}", data[0].0.maxmp))
        });
        let sight = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Sight")
                .default_value(format!("{}", data[0].0.sight))
        });
        let follow_sight = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Follow Sight")
                .default_value(format!("{}", data[0].0.follow_sight))
        });
        let walkdistance = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Walk Distance")
                .default_value(format!("{}", data[0].0.walkdistance))
        });
        let pdamage = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Physical Damage")
                .default_value(format!("{}", data[0].0.pdamage))
        });
        let pdefense = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Physical Defense")
                .default_value(format!("{}", data[0].0.pdefense))
        });
        let maxdamage = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Max Damage")
                .default_value(format!("{}", data[0].0.maxdamage))
        });
        let mindamage = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Min Damage")
                .default_value(format!("{}", data[0].0.mindamage))
        });
        let run_damage = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Run Damage")
                .default_value(format!("{}", data[0].0.run_damage))
        });
        let free_shares = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Free Shares")
                .default_value(format!("{}", data[0].0.free_shares))
        });
        let target_auto_switch_chance = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Target Auto Switch Chance")
                .default_value(format!("{}", data[0].0.target_auto_switch_chance))
        });
        let exp = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Exp")
                .default_value(format!("{}", data[0].0.exp))
        });
        let range = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Range")
                .default_value(format!("{}", data[0].0.range))
        });
        let size_x = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Size X")
                .default_value(format!("{}", data[0].0.size.x))
        });
        let size_y = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Size X")
                .default_value(format!("{}", data[0].0.size.y))
        });
        let size_w = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Size Width")
                .default_value(format!("{}", data[0].0.size.width))
        });
        let size_h = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Size Height")
                .default_value(format!("{}", data[0].0.size.height))
        });

        let start_hour = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Start Spawn Hour")
                .default_value(format!("{}", data[0].0.spawntime.0.hour))
        });
        let start_minute = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Start Spawn Hour")
                .default_value(format!("{}", data[0].0.spawntime.0.min))
        });
        let start_second = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Start Spawn Hour")
                .default_value(format!("{}", data[0].0.spawntime.0.sec))
        });

        let end_hour = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Start Spawn Hour")
                .default_value(format!("{}", data[0].0.spawntime.1.hour))
        });
        let end_minute = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Start Spawn Hour")
                .default_value(format!("{}", data[0].0.spawntime.1.min))
        });
        let end_second = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Start Spawn Hour")
                .default_value(format!("{}", data[0].0.spawntime.1.sec))
        });

        let canpassthru = data[0].0.canpassthru;
        let target_auto_switch = data[0].0.target_auto_switch;
        let target_attacked_switch = data[0].0.target_attacked_switch;
        let target_range_dropout = data[0].0.target_range_dropout;
        let can_target = data[0].0.can_target;
        let can_move = data[0].0.can_move;
        let can_attack_player = data[0].0.can_attack_player;
        let has_allys = data[0].0.has_allys;
        let has_enemies = data[0].0.has_enemies;
        let can_attack = data[0].0.can_attack;
        let has_selfonly = data[0].0.has_selfonly;
        let has_friendonly = data[0].0.has_friendonly;
        let has_groundonly = data[0].0.has_groundonly;
        let runsaway = data[0].0.runsaway;
        let isanimated = data[0].0.isanimated;

        let mut list = Vec::with_capacity(10);
        for val in 0..10 {
            list.push(SharedString::new(format!("{val}")));
        }
        let drop_list = cx.new(|cx| SelectState::new(list, None, window, cx));
        drop_list.update(cx, |state, cx| {
            state.set_selected_index(Some(IndexPath::new(0)), window, cx);
        });

        let drop_share = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Drop Share")
                .default_value(format!("{}", data[0].0.drops[0].shares))
        });

        let mut drop_item = Vec::with_capacity(5);
        let mut drop_amount = Vec::with_capacity(5);

        for dropitem in data[0].0.drops[0].items.iter() {
            drop_item.push(cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("Drop Share")
                    .default_value(format!("{}", dropitem.item))
            }));

            drop_amount.push(cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("Drop Amount")
                    .default_value(format!("{}", dropitem.amount))
            }));
        }

        info!("Setting subscription");

        let mut _subscriptions = vec![
            cx.subscribe_in(
                &data_list,
                window,
                |view, _state, event, wn, cx| match event {
                    SelectEvent::Confirm(_) => {
                        if let Some(index) = view.data_list.read(cx).selected_index(cx) {
                            view.index = index.row;
                            view.update_data(wn, cx);
                        }
                    }
                },
            ),
            cx.subscribe_in(&name_input, window, {
                |view, _state, ev, _wn, cx| {
                    if let InputEvent::Blur = ev {
                        let value = view.name_input.read(cx).value().as_str().to_string();
                        if value != view.data[view.index].0.name {
                            println!("Updating");
                            view.data[view.index].0.name.clone_from(&value);
                            view.data[view.index].1 = true;
                            view.save_temp_file();
                            cx.notify()
                        }
                    }
                }
            }),
            cx.subscribe_in(
                &behaviour,
                window,
                |view, _state, event, _wn, cx| match event {
                    SelectEvent::Confirm(_) => {
                        if let Some(index) = view.behaviour.read(cx).selected_index(cx) {
                            view.data[view.index].0.behaviour = AIBehavior::from_index(index.row);
                            view.data[view.index].1 = true;
                            view.save_temp_file();
                            cx.notify()
                        }
                    }
                },
            ),
            cx.subscribe_in(&sprite, window, |view, state, event, _, cx| {
                if let InputEvent::Blur = event {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<i32>()
                        && new_value != view.data[view.index].0.sprite
                    {
                        view.data[view.index].0.sprite = new_value;
                        view.data[view.index].1 = true;
                        view.save_temp_file();
                        cx.notify()
                    }
                }
            }),
            cx.subscribe_in(&level, window, |view, state, event, _, cx| {
                if let InputEvent::Blur = event {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<i32>()
                        && new_value != view.data[view.index].0.level
                    {
                        view.data[view.index].0.level = new_value;
                        view.data[view.index].1 = true;
                        view.save_temp_file();
                        cx.notify()
                    }
                }
            }),
            cx.subscribe_in(&respawn_wait, window, |view, state, event, _, cx| {
                if let InputEvent::Blur = event {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<i64>()
                        && new_value != view.data[view.index].0.respawn_wait
                    {
                        view.data[view.index].0.respawn_wait = new_value;
                        view.data[view.index].1 = true;
                        view.save_temp_file();
                        cx.notify()
                    }
                }
            }),
            cx.subscribe_in(&movement_wait, window, |view, state, event, _, cx| {
                if let InputEvent::Blur = event {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<i64>()
                        && new_value != view.data[view.index].0.movement_wait
                    {
                        view.data[view.index].0.movement_wait = new_value;
                        view.data[view.index].1 = true;
                        view.save_temp_file();
                        cx.notify()
                    }
                }
            }),
            cx.subscribe_in(&attack_wait, window, |view, state, event, _, cx| {
                if let InputEvent::Blur = event {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<i64>()
                        && new_value != view.data[view.index].0.attack_wait
                    {
                        view.data[view.index].0.attack_wait = new_value;
                        view.data[view.index].1 = true;
                        view.save_temp_file();
                        cx.notify()
                    }
                }
            }),
            cx.subscribe_in(&intervaled_wait, window, |view, state, event, _, cx| {
                if let InputEvent::Blur = event {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<i64>()
                        && new_value != view.data[view.index].0.intervaled_wait
                    {
                        view.data[view.index].0.intervaled_wait = new_value;
                        view.data[view.index].1 = true;
                        view.save_temp_file();
                        cx.notify()
                    }
                }
            }),
            cx.subscribe_in(&spawn_wait, window, |view, state, event, _, cx| {
                if let InputEvent::Blur = event {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<i64>()
                        && new_value != view.data[view.index].0.spawn_wait
                    {
                        view.data[view.index].0.spawn_wait = new_value;
                        view.data[view.index].1 = true;
                        view.save_temp_file();
                        cx.notify()
                    }
                }
            }),
            cx.subscribe_in(&maxhp, window, |view, state, event, _, cx| {
                if let InputEvent::Blur = event {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<u32>()
                        && new_value != view.data[view.index].0.maxhp
                    {
                        view.data[view.index].0.maxhp = new_value;
                        view.data[view.index].1 = true;
                        view.save_temp_file();
                        cx.notify()
                    }
                }
            }),
            cx.subscribe_in(&maxsp, window, |view, state, event, _, cx| {
                if let InputEvent::Blur = event {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<u32>()
                        && new_value != view.data[view.index].0.maxsp
                    {
                        view.data[view.index].0.maxsp = new_value;
                        view.data[view.index].1 = true;
                        view.save_temp_file();
                        cx.notify()
                    }
                }
            }),
            cx.subscribe_in(&maxmp, window, |view, state, event, _, cx| {
                if let InputEvent::Blur = event {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<u32>()
                        && new_value != view.data[view.index].0.maxmp
                    {
                        view.data[view.index].0.maxmp = new_value;
                        view.data[view.index].1 = true;
                        view.save_temp_file();
                        cx.notify()
                    }
                }
            }),
            cx.subscribe_in(&sight, window, |view, state, event, _, cx| {
                if let InputEvent::Blur = event {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<i32>()
                        && new_value != view.data[view.index].0.sight
                    {
                        view.data[view.index].0.sight = new_value;
                        view.data[view.index].1 = true;
                        view.save_temp_file();
                        cx.notify()
                    }
                }
            }),
            cx.subscribe_in(&follow_sight, window, |view, state, event, _, cx| {
                if let InputEvent::Blur = event {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<i32>()
                        && new_value != view.data[view.index].0.follow_sight
                    {
                        view.data[view.index].0.follow_sight = new_value;
                        view.data[view.index].1 = true;
                        view.save_temp_file();
                        cx.notify()
                    }
                }
            }),
            cx.subscribe_in(&walkdistance, window, |view, state, event, _, cx| {
                if let InputEvent::Blur = event {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<u32>()
                        && new_value != view.data[view.index].0.walkdistance
                    {
                        view.data[view.index].0.walkdistance = new_value;
                        view.data[view.index].1 = true;
                        view.save_temp_file();
                        cx.notify()
                    }
                }
            }),
            cx.subscribe_in(&pdamage, window, |view, state, event, _, cx| {
                if let InputEvent::Blur = event {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<u32>()
                        && new_value != view.data[view.index].0.pdamage
                    {
                        view.data[view.index].0.pdamage = new_value;
                        view.data[view.index].1 = true;
                        view.save_temp_file();
                        cx.notify()
                    }
                }
            }),
            cx.subscribe_in(&pdefense, window, |view, state, event, _, cx| {
                if let InputEvent::Blur = event {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<u32>()
                        && new_value != view.data[view.index].0.pdefense
                    {
                        view.data[view.index].0.pdefense = new_value;
                        view.data[view.index].1 = true;
                        view.save_temp_file();
                        cx.notify()
                    }
                }
            }),
            cx.subscribe_in(&maxdamage, window, |view, state, event, _, cx| {
                if let InputEvent::Blur = event {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<u32>()
                        && new_value != view.data[view.index].0.maxdamage
                    {
                        view.data[view.index].0.maxdamage = new_value;
                        view.data[view.index].1 = true;
                        view.save_temp_file();
                        cx.notify()
                    }
                }
            }),
            cx.subscribe_in(&mindamage, window, |view, state, event, _, cx| {
                if let InputEvent::Blur = event {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<u32>()
                        && new_value != view.data[view.index].0.mindamage
                    {
                        view.data[view.index].0.mindamage = new_value;
                        view.data[view.index].1 = true;
                        view.save_temp_file();
                        cx.notify()
                    }
                }
            }),
            cx.subscribe_in(&run_damage, window, |view, state, event, _, cx| {
                if let InputEvent::Blur = event {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<u32>()
                        && new_value != view.data[view.index].0.run_damage
                    {
                        view.data[view.index].0.run_damage = new_value;
                        view.data[view.index].1 = true;
                        view.save_temp_file();
                        cx.notify()
                    }
                }
            }),
            cx.subscribe_in(&free_shares, window, |view, state, event, _, cx| {
                if let InputEvent::Blur = event {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<u32>()
                        && new_value != view.data[view.index].0.free_shares
                    {
                        view.data[view.index].0.free_shares = new_value;
                        view.data[view.index].1 = true;
                        view.save_temp_file();
                        cx.notify()
                    }
                }
            }),
            cx.subscribe_in(
                &target_auto_switch_chance,
                window,
                |view, state, event, _, cx| {
                    if let InputEvent::Blur = event {
                        let text = state.read(cx).value();
                        if let Ok(new_value) = text.parse::<i64>()
                            && new_value != view.data[view.index].0.target_auto_switch_chance
                        {
                            view.data[view.index].0.target_auto_switch_chance = new_value;
                            view.data[view.index].1 = true;
                            view.save_temp_file();
                            cx.notify()
                        }
                    }
                },
            ),
            cx.subscribe_in(&exp, window, |view, state, event, _, cx| {
                if let InputEvent::Blur = event {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<i64>()
                        && new_value != view.data[view.index].0.exp
                    {
                        view.data[view.index].0.exp = new_value;
                        view.data[view.index].1 = true;
                        view.save_temp_file();
                        cx.notify()
                    }
                }
            }),
            cx.subscribe_in(&range, window, |view, state, event, _, cx| {
                if let InputEvent::Blur = event {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<i32>()
                        && new_value != view.data[view.index].0.range
                    {
                        view.data[view.index].0.range = new_value;
                        view.data[view.index].1 = true;
                        view.save_temp_file();
                        cx.notify()
                    }
                }
            }),
            cx.subscribe_in(&size_x, window, |view, state, event, _, cx| {
                if let InputEvent::Blur = event {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<u8>()
                        && new_value != view.data[view.index].0.size.x
                    {
                        view.data[view.index].0.size.x = new_value;
                        view.data[view.index].1 = true;
                        view.save_temp_file();
                        cx.notify()
                    }
                }
            }),
            cx.subscribe_in(&size_y, window, |view, state, event, _, cx| {
                if let InputEvent::Blur = event {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<u8>()
                        && new_value != view.data[view.index].0.size.y
                    {
                        view.data[view.index].0.size.y = new_value;
                        view.data[view.index].1 = true;
                        view.save_temp_file();
                        cx.notify()
                    }
                }
            }),
            cx.subscribe_in(&size_w, window, |view, state, event, _, cx| {
                if let InputEvent::Blur = event {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<u8>()
                        && new_value != view.data[view.index].0.size.width
                    {
                        view.data[view.index].0.size.width = new_value;
                        view.data[view.index].1 = true;
                        view.save_temp_file();
                        cx.notify()
                    }
                }
            }),
            cx.subscribe_in(&size_h, window, |view, state, event, _, cx| {
                if let InputEvent::Blur = event {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<u8>()
                        && new_value != view.data[view.index].0.size.height
                    {
                        view.data[view.index].0.size.height = new_value;
                        view.data[view.index].1 = true;
                        view.save_temp_file();
                        cx.notify()
                    }
                }
            }),
            cx.subscribe_in(
                &enemy_list,
                window,
                |view, _state, event, wn, cx| match event {
                    SelectEvent::Confirm(_) => {
                        if let Some(index) = view.enemy_list.read(cx).selected_index(cx) {
                            view.e_index = index.row;

                            if let Some(val) = view.data[view.index].0.enemies.get(view.e_index) {
                                view.enemy_index.update(cx, |view, cx| {
                                    view.set_value(SharedString::new(format!("{}", val)), wn, cx);
                                });
                                cx.notify()
                            }
                        }
                    }
                },
            ),
            cx.subscribe_in(
                &drop_list,
                window,
                |view, _state, event, wn, cx| match event {
                    SelectEvent::Confirm(_) => {
                        if let Some(index) = view.drop_list.read(cx).selected_index(cx) {
                            view.d_index = index.row;
                            let drop = view.data[view.index].0.drops[index.row];

                            view.drop_share.update(cx, |view, cx| {
                                view.set_value(
                                    SharedString::new(format!("{}", drop.shares)),
                                    wn,
                                    cx,
                                );
                            });

                            for i in 0..5 {
                                view.drop_item[i].update(cx, |view, cx| {
                                    view.set_value(
                                        SharedString::new(format!("{}", drop.items[i].item)),
                                        wn,
                                        cx,
                                    );
                                });

                                view.drop_amount[i].update(cx, |view, cx| {
                                    view.set_value(
                                        SharedString::new(format!("{}", drop.items[i].amount)),
                                        wn,
                                        cx,
                                    );
                                });
                            }

                            cx.notify()
                        }
                    }
                },
            ),
            cx.subscribe_in(&start_hour, window, |view, state, event, _, cx| {
                if let InputEvent::Blur = event {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<u32>()
                        && new_value != view.data[view.index].0.spawntime.0.hour
                    {
                        view.data[view.index].0.spawntime.0.hour = new_value;
                        view.data[view.index].1 = true;
                        view.save_temp_file();
                        cx.notify()
                    }
                }
            }),
            cx.subscribe_in(&start_minute, window, |view, state, event, _, cx| {
                if let InputEvent::Blur = event {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<u32>()
                        && new_value != view.data[view.index].0.spawntime.0.min
                    {
                        view.data[view.index].0.spawntime.0.min = new_value;
                        view.data[view.index].1 = true;
                        view.save_temp_file();
                        cx.notify()
                    }
                }
            }),
            cx.subscribe_in(&start_second, window, |view, state, event, _, cx| {
                if let InputEvent::Blur = event {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<u32>()
                        && new_value != view.data[view.index].0.spawntime.0.sec
                    {
                        view.data[view.index].0.spawntime.0.sec = new_value;
                        view.data[view.index].1 = true;
                        view.save_temp_file();
                        cx.notify()
                    }
                }
            }),
            cx.subscribe_in(&end_hour, window, |view, state, event, _, cx| {
                if let InputEvent::Blur = event {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<u32>()
                        && new_value != view.data[view.index].0.spawntime.1.hour
                    {
                        view.data[view.index].0.spawntime.1.hour = new_value;
                        view.data[view.index].1 = true;
                        view.save_temp_file();
                        cx.notify()
                    }
                }
            }),
            cx.subscribe_in(&end_minute, window, |view, state, event, _, cx| {
                if let InputEvent::Blur = event {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<u32>()
                        && new_value != view.data[view.index].0.spawntime.1.min
                    {
                        view.data[view.index].0.spawntime.1.min = new_value;
                        view.data[view.index].1 = true;
                        view.save_temp_file();
                        cx.notify()
                    }
                }
            }),
            cx.subscribe_in(&end_second, window, |view, state, event, _, cx| {
                if let InputEvent::Blur = event {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<u32>()
                        && new_value != view.data[view.index].0.spawntime.1.sec
                    {
                        view.data[view.index].0.spawntime.1.sec = new_value;
                        view.data[view.index].1 = true;
                        view.save_temp_file();
                        cx.notify()
                    }
                }
            }),
            cx.subscribe_in(&drop_share, window, |view, state, event, _, cx| {
                if let InputEvent::Blur = event {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<u32>()
                        && new_value != view.data[view.index].0.drops[view.d_index].shares
                    {
                        view.data[view.index].0.drops[view.d_index].shares = new_value;
                        view.data[view.index].1 = true;
                        view.save_temp_file();
                        cx.notify()
                    }
                }
            }),
        ];

        for i in 0..5 {
            _subscriptions.push(cx.subscribe_in(
                &drop_item[i],
                window,
                move |view, state, event, _, cx| {
                    if let InputEvent::Blur = event {
                        let text = state.read(cx).value();
                        if let Ok(new_value) = text.parse::<u32>()
                            && new_value
                                != view.data[view.index].0.drops[view.d_index].items[i].item
                        {
                            view.data[view.index].0.drops[view.d_index].items[i].item = new_value;
                            view.data[view.index].1 = true;
                            view.save_temp_file();
                            cx.notify()
                        }
                    }
                },
            ));

            _subscriptions.push(cx.subscribe_in(
                &drop_amount[i],
                window,
                move |view, state, event, _, cx| {
                    if let InputEvent::Blur = event {
                        let text = state.read(cx).value();
                        if let Ok(new_value) = text.parse::<u32>()
                            && new_value
                                != view.data[view.index].0.drops[view.d_index].items[i].amount
                        {
                            view.data[view.index].0.drops[view.d_index].items[i].amount = new_value;
                            view.data[view.index].1 = true;
                            view.save_temp_file();
                            cx.notify()
                        }
                    }
                },
            ));
        }

        Editor {
            index: 0,
            data,
            command_handler,

            cur_tab: 0,

            data_list,
            name_input,

            sprite,
            behaviour,
            level,
            respawn_wait,
            movement_wait,
            attack_wait,
            intervaled_wait,
            spawn_wait,
            maxhp,
            maxsp,
            maxmp,
            sight,
            follow_sight,
            walkdistance,
            pdamage,
            pdefense,
            maxdamage,
            mindamage,
            run_damage,
            free_shares,
            target_auto_switch_chance,
            exp,
            range,

            size_x,
            size_y,
            size_w,
            size_h,

            start_hour,
            start_minute,
            start_second,

            end_hour,
            end_minute,
            end_second,

            canpassthru,
            target_auto_switch,
            target_attacked_switch,
            target_range_dropout,
            can_target,
            can_move,
            can_attack_player,
            has_allys,
            has_enemies,
            can_attack,
            has_selfonly,
            has_friendonly,
            has_groundonly,
            runsaway,
            isanimated,

            e_index: 0,
            enemy_list,
            enemy_index,

            d_index: 0,
            drop_list,
            drop_share,
            drop_item,
            drop_amount,

            _subscriptions,
        }
    }

    pub fn update_list(&mut self, wn: &mut Window, cx: &mut Context<Self>) {
        self.data_list.update(cx, |state, cx| {
            let mut list = Vec::with_capacity(self.data.len());
            for (index, (data, _)) in self.data.iter().enumerate() {
                list.push(SharedString::new(format!("{index}: {}", data.name)))
            }

            state.set_items(list, wn, cx);
        });
    }

    pub fn save_temp_file(&mut self) {
        let _ = self
            .command_handler
            .tx
            .blocking_send(MessageCommands::TempSave(
                self.index,
                Box::new(self.data[self.index].0.clone()),
            ));
    }

    pub fn update_data(&mut self, wn: &mut Window, cx: &mut Context<Self>) {
        if let Some((data, _edited)) = self.data.get(self.index) {
            self.name_input.update(cx, |view, cx| {
                view.set_value(SharedString::new(data.name.clone()), wn, cx);
            });

            self.sprite.update(cx, |view, cx| {
                view.set_value(SharedString::new(format!("{}", data.sprite)), wn, cx);
            });
            self.behaviour.update(cx, |view, cx| {
                view.set_selected_index(
                    Some(IndexPath {
                        section: 0,
                        row: data.behaviour as usize,
                        column: 0,
                    }),
                    wn,
                    cx,
                );
            });
            self.level.update(cx, |view, cx| {
                view.set_value(SharedString::new(format!("{}", data.level)), wn, cx);
            });
            self.respawn_wait.update(cx, |view, cx| {
                view.set_value(SharedString::new(format!("{}", data.respawn_wait)), wn, cx);
            });
            self.movement_wait.update(cx, |view, cx| {
                view.set_value(SharedString::new(format!("{}", data.movement_wait)), wn, cx);
            });
            self.attack_wait.update(cx, |view, cx| {
                view.set_value(SharedString::new(format!("{}", data.attack_wait)), wn, cx);
            });
            self.intervaled_wait.update(cx, |view, cx| {
                view.set_value(
                    SharedString::new(format!("{}", data.intervaled_wait)),
                    wn,
                    cx,
                );
            });
            self.spawn_wait.update(cx, |view, cx| {
                view.set_value(SharedString::new(format!("{}", data.spawn_wait)), wn, cx);
            });
            self.maxhp.update(cx, |view, cx| {
                view.set_value(SharedString::new(format!("{}", data.maxhp)), wn, cx);
            });
            self.maxsp.update(cx, |view, cx| {
                view.set_value(SharedString::new(format!("{}", data.maxsp)), wn, cx);
            });
            self.maxmp.update(cx, |view, cx| {
                view.set_value(SharedString::new(format!("{}", data.maxmp)), wn, cx);
            });
            self.sight.update(cx, |view, cx| {
                view.set_value(SharedString::new(format!("{}", data.sight)), wn, cx);
            });
            self.follow_sight.update(cx, |view, cx| {
                view.set_value(SharedString::new(format!("{}", data.follow_sight)), wn, cx);
            });

            self.walkdistance.update(cx, |view, cx| {
                view.set_value(SharedString::new(format!("{}", data.walkdistance)), wn, cx);
            });
            self.pdamage.update(cx, |view, cx| {
                view.set_value(SharedString::new(format!("{}", data.pdamage)), wn, cx);
            });
            self.pdefense.update(cx, |view, cx| {
                view.set_value(SharedString::new(format!("{}", data.pdefense)), wn, cx);
            });
            self.maxdamage.update(cx, |view, cx| {
                view.set_value(SharedString::new(format!("{}", data.maxdamage)), wn, cx);
            });
            self.mindamage.update(cx, |view, cx| {
                view.set_value(SharedString::new(format!("{}", data.mindamage)), wn, cx);
            });
            self.run_damage.update(cx, |view, cx| {
                view.set_value(SharedString::new(format!("{}", data.run_damage)), wn, cx);
            });
            self.free_shares.update(cx, |view, cx| {
                view.set_value(SharedString::new(format!("{}", data.free_shares)), wn, cx);
            });

            self.target_auto_switch_chance.update(cx, |view, cx| {
                view.set_value(
                    SharedString::new(format!("{}", data.target_auto_switch_chance)),
                    wn,
                    cx,
                );
            });
            self.exp.update(cx, |view, cx| {
                view.set_value(SharedString::new(format!("{}", data.exp)), wn, cx);
            });
            self.range.update(cx, |view, cx| {
                view.set_value(SharedString::new(format!("{}", data.range)), wn, cx);
            });

            self.size_x.update(cx, |view, cx| {
                view.set_value(SharedString::new(format!("{}", data.size.x)), wn, cx);
            });
            self.size_y.update(cx, |view, cx| {
                view.set_value(SharedString::new(format!("{}", data.size.y)), wn, cx);
            });
            self.size_w.update(cx, |view, cx| {
                view.set_value(SharedString::new(format!("{}", data.size.width)), wn, cx);
            });
            self.size_h.update(cx, |view, cx| {
                view.set_value(SharedString::new(format!("{}", data.size.height)), wn, cx);
            });

            self.canpassthru = data.canpassthru;
            self.target_auto_switch = data.target_auto_switch;
            self.target_attacked_switch = data.target_attacked_switch;
            self.target_range_dropout = data.target_range_dropout;
            self.can_target = data.can_target;
            self.can_move = data.can_move;
            self.can_attack_player = data.can_attack_player;
            self.has_allys = data.has_allys;
            self.has_enemies = data.has_enemies;
            self.can_attack = data.can_attack;
            self.has_selfonly = data.has_selfonly;
            self.has_friendonly = data.has_friendonly;
            self.has_groundonly = data.has_groundonly;
            self.runsaway = data.runsaway;
            self.isanimated = data.isanimated;

            self.enemy_list.update(cx, |view, cx| {
                view.set_selected_index(Some(IndexPath::new(0)), wn, cx);
            });
            self.e_index = 0;

            self.enemy_index.update(cx, |view, cx| {
                view.set_value(
                    SharedString::new(format!(
                        "{}",
                        if let Some(val) = data.enemies.first() {
                            *val
                        } else {
                            0
                        }
                    )),
                    wn,
                    cx,
                );
            });

            self.start_hour.update(cx, |view, cx| {
                view.set_value(
                    SharedString::new(format!("{}", data.spawntime.0.hour)),
                    wn,
                    cx,
                );
            });
            self.start_minute.update(cx, |view, cx| {
                view.set_value(
                    SharedString::new(format!("{}", data.spawntime.0.min)),
                    wn,
                    cx,
                );
            });
            self.start_second.update(cx, |view, cx| {
                view.set_value(
                    SharedString::new(format!("{}", data.spawntime.0.sec)),
                    wn,
                    cx,
                );
            });

            self.end_hour.update(cx, |view, cx| {
                view.set_value(
                    SharedString::new(format!("{}", data.spawntime.1.hour)),
                    wn,
                    cx,
                );
            });
            self.end_minute.update(cx, |view, cx| {
                view.set_value(
                    SharedString::new(format!("{}", data.spawntime.1.min)),
                    wn,
                    cx,
                );
            });
            self.end_second.update(cx, |view, cx| {
                view.set_value(
                    SharedString::new(format!("{}", data.spawntime.1.sec)),
                    wn,
                    cx,
                );
            });

            self.drop_list.update(cx, |view, cx| {
                view.set_selected_index(Some(IndexPath::new(0)), wn, cx);
            });
            self.d_index = 0;

            self.drop_share.update(cx, |view, cx| {
                view.set_value(
                    SharedString::new(format!("{}", data.drops[0].shares)),
                    wn,
                    cx,
                );
            });

            for i in 0..5 {
                self.drop_item[i].update(cx, |view, cx| {
                    view.set_value(
                        SharedString::new(format!("{}", data.drops[0].items[i].item)),
                        wn,
                        cx,
                    );
                });

                self.drop_amount[i].update(cx, |view, cx| {
                    view.set_value(
                        SharedString::new(format!("{}", data.drops[0].items[i].amount)),
                        wn,
                        cx,
                    );
                });
            }
        }
    }

    pub fn update_enemy(&mut self, wn: &mut Window, cx: &mut Context<Self>) {
        let index = self.e_index;
        let text = self.enemy_index.read(cx).value();

        if let Ok(new_value) = text.parse::<u64>()
            && let Some(val) = self.data[self.index].0.enemies.get_mut(index)
        {
            *val = new_value;

            wn.push_notification("Enemy Updated", cx);
        }
        self.save_temp_file();
        self.data[self.index].1 = true;
    }

    pub fn add_enemy(&mut self, wn: &mut Window, cx: &mut Context<Self>) {
        let text = self.enemy_index.read(cx).value();
        let cur_index = self.data[self.index].0.enemies.len();

        if let Ok(new_value) = text.parse::<u64>() {
            self.data[self.index].0.enemies.push(new_value);

            self.enemy_list.update(cx, |state, cx| {
                let mut list = Vec::with_capacity(self.data[self.index].0.enemies.len());
                for id in 0..self.data[self.index].0.enemies.len() {
                    list.push(SharedString::new(format!("{id}")));
                }
                state.set_items(list, wn, cx);
                state.set_selected_index(Some(IndexPath::new(cur_index)), wn, cx);
            });

            wn.push_notification("Enemy Added", cx);
        }
        self.save_temp_file();
        self.data[self.index].1 = true;
    }

    pub fn remove_enemy(&mut self, wn: &mut Window, cx: &mut Context<Self>) {
        let index = self.e_index;
        self.data[self.index].0.enemies.remove(index);

        self.enemy_list.update(cx, |state, cx| {
            let mut list = Vec::with_capacity(self.data[self.index].0.enemies.len());
            for id in 0..self.data[self.index].0.enemies.len() {
                list.push(SharedString::new(format!("{id}")));
            }
            state.set_items(list, wn, cx);
            state.set_selected_index(Some(IndexPath::new(0)), wn, cx);
        });

        self.enemy_index.update(cx, |view, cx| {
            view.set_value(
                SharedString::new(format!(
                    "{}",
                    if let Some(val) = self.data[self.index].0.enemies.first() {
                        *val
                    } else {
                        0
                    }
                )),
                wn,
                cx,
            );
        });
        self.save_temp_file();
        self.data[self.index].1 = true;

        wn.push_notification("Enemy Removed", cx);
    }
}
