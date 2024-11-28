#![windows_subsystem = "windows"]

#[macro_use]
extern crate educe;
extern crate serde_big_array;

mod config;
mod shop;

use config::*;

use ascending_logger::*;
use ascending_ui::*;
use iced::{
    widget::{Column, Container},
    Element, Length,
};
use iced_aw::iced_fonts;
use shop::*;
use std::fs;

pub fn main() -> Result<iced::Result, String> {
    let logger = Box::new(MyLogger::new("shop_editor_log.txt"));
    logger.set_boxed_logger().unwrap();

    info!("starting up");
    info!("Setting Panic Hook");

    std::panic::set_hook(Box::new(|panic_info| {
        let bt = backtrace::Backtrace::new();

        error!("PANIC: {}, BACKTRACE: {:?}", panic_info, bt);
    }));

    if let Err(e) = fs::create_dir_all("./data/shops/json/") {
        return Err(format!("Err: {:?}", e));
    }

    info!("Checked or Created Directorys");
    ShopData::create_files()?;

    info!("Checked or Created Files");

    Ok(iced::application("Shop Editor", Pages::update, Pages::view)
        .font(iced_fonts::REQUIRED_FONT_BYTES)
        .run())
}

pub struct Pages {
    page: Box<dyn UiRenderer<Message = shop::Message>>,
}

impl Default for Pages {
    fn default() -> Self {
        Self {
            page: Box::new(ShopUI::new()),
        }
    }
}

impl Pages {
    fn update(&mut self, message: Message) {
        self.page.update(message);
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

        Container::new(content).height(Length::Fill).into()
    }
}
