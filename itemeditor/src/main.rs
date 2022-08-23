#![windows_subsystem = "windows"]

#[macro_use]
extern crate educe;
extern crate serde_big_array;

mod item;

use araiseal_logger::*;
use araiseal_ui::*;
use iced::pure::{
    widget::{Column, Container},
    Element, Sandbox,
};
use iced::{Length, Settings};
use item::*;
use std::fs;

pub fn main() -> Result<(), String> {
    let logger = Box::new(MyLogger::new("item_editor_log.txt"));
    logger.set_boxed_logger().unwrap();

    info!("starting up");
    info!("Setting Panic Hook");

    std::panic::set_hook(Box::new(|panic_info| {
        let bt = backtrace::Backtrace::new();

        error!("PANIC: {}, BACKTRACE: {:?}", panic_info, bt);
    }));

    if let Err(e) = fs::create_dir_all("./data/items/") {
        return Err(format!("Err: {:?}", e));
    }

    info!("Checked or Created Directorys");
    ItemData::create_files()?;

    info!("Checked or Created Files");

    if let Err(err) = Pages::run(Settings::default()) {
        error!("{}", err);
    }

    Ok(())
}

pub struct Pages {
    page: Box<dyn UiRenderer<Message = item::Message>>,
}

impl Sandbox for Pages {
    type Message = item::Message;

    fn new() -> Pages {
        Pages {
            page: Box::new(item::ItemUI::new()),
        }
    }

    fn title(&self) -> String {
        self.page.title().to_string()
    }

    fn update(&mut self, event: Message) {
        self.page.update(event);
    }

    fn view(&self) -> Element<Message> {
        let page = self.page.view();

        let content: Element<_> = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(20)
            .padding(20)
            .push(page)
            .into();

        Container::new(content)
            .height(Length::Fill)
            .style(araiseal_styles::MainContainer)
            .into()
    }
}
