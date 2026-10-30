use std::cell::RefCell;

use database::ShopData;
use tokio::{runtime::Runtime, sync::mpsc, task::JoinHandle};

pub enum MessageCommands {
    TempSave(usize, Box<ShopData>),
    Save(usize),
    SaveAll(Vec<usize>),
    Revert(usize),
}

pub struct CommandHandler {
    pub _rt: RefCell<Runtime>,
    pub tx: tokio::sync::mpsc::Sender<MessageCommands>,
    pub _message_handle: RefCell<JoinHandle<()>>,
}

pub async fn file_command_actor(
    mut receiver_rx: mpsc::Receiver<MessageCommands>,
    tx_update: mpsc::Sender<Option<(usize, ShopData)>>,
) {
    while let Some(command) = receiver_rx.recv().await {
        match command {
            MessageCommands::TempSave(index, data) => {
                let _ = data.save_json_file(index);
                let _ = data.save_bin_file(index);
            }
            MessageCommands::Save(index) => {
                let _ = ShopData::finalize_file(index);
            }
            MessageCommands::SaveAll(index) => {
                for i in index {
                    let _ = ShopData::finalize_file(i);
                }
            }
            MessageCommands::Revert(index) => {
                let _ = tx_update
                    .send(
                        ShopData::load_main_file(index)
                            .map(|data| (index, data)),
                    )
                    .await;
            }
        }
    }
}
