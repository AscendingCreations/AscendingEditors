use std::{cell::RefCell, fs};

use ascending_logger::*;
use database::ItemData;
use gpui_kit::{
    AppContext, Global, SharedString, TitlebarOptions, WindowBounds, WindowOptions, assets::Assets,
    base::Root, px, size,
};
use tokio::{
    runtime::Runtime,
    sync::mpsc::{self, Receiver},
};

mod commands;
mod editor;

use commands::*;
use editor::*;

pub struct MessageReceiverGlobal(pub Receiver<Option<(usize, ItemData)>>);
impl Global for MessageReceiverGlobal {}

fn main() {
    let logger = Box::new(MyLogger::new("item_editor_log.txt"));
    logger.set_boxed_logger().unwrap();

    info!("starting up");
    info!("Setting Panic Hook");

    std::panic::set_hook(Box::new(|panic_info| {
        let bt = backtrace::Backtrace::new();

        error!("PANIC: {panic_info}, BACKTRACE: {bt:?}");
    }));

    fs::create_dir_all("./data/items/").unwrap();
    fs::create_dir_all("./json/items/").unwrap();
    fs::create_dir_all("./temp/items/").unwrap();
    info!("Checked or Created Directorys");

    ItemData::create_files().unwrap();
    info!("Checked or Created Files");

    info!("Setting up Command handler");

    let rt: Runtime = tokio::runtime::Builder::new_multi_thread()
        .thread_name("threads")
        .enable_all()
        .build()
        .unwrap();

    let (tx, rx) = mpsc::channel(1000);
    let (tx_update, rx_update) = mpsc::channel(5);

    let handle = rt.handle();

    let message_handle = tokio::task::Builder::new()
        .name("FileManager")
        .spawn_on(file_command_actor(rx, tx_update), handle)
        .unwrap();

    let command_handler = CommandHandler {
        _rt: RefCell::new(rt),
        tx,
        _message_handle: RefCell::new(message_handle),
    };

    info!("Running application");
    let app = gpui_kit::application().with_assets(Assets);

    app.run(move |cx| {
        // This must be called before using any GPUI Component features.
        gpui_kit::init(cx);

        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::centered(size(px(800.), px(700.)), cx)),
            titlebar: Some(TitlebarOptions {
                title: Some(SharedString::new("Item Editor")),
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            let view = cx.new(|cx| Editor::new(window, cx, command_handler, rx_update));

            // This first level on the window, should be a Root.
            cx.new(|cx| Root::new(view, window, cx))
        })
        .expect("Failed to open window");
    });
}
