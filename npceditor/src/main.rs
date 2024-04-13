#![windows_subsystem = "windows"]

#[macro_use]
extern crate educe;
extern crate serde_big_array;

mod npc;

use araiseal_logger::*;
use araiseal_ui::*;
use iced::{
    executor, font,
    widget::{Column, Container},
    Application, Command, Element, Length, Settings, Theme,
};
use npc::*;
use std::fs;

pub fn main() -> Result<(), String> {
    let logger = Box::new(MyLogger::new("npc_editor_log.txt"));
    logger.set_boxed_logger().unwrap();

    info!("starting up");
    info!("Setting Panic Hook");

    std::panic::set_hook(Box::new(|panic_info| {
        let bt = backtrace::Backtrace::new();
        error!("PANIC: {}, BACKTRACE: {:?}", panic_info, bt);
    }));

    if let Err(e) = fs::create_dir_all("./data/npcs/bin/") {
        return Err(format!("Err: {:?}", e));
    }

    info!("Checked or Created Directorys");
    NpcData::create_files()?;

    info!("Checked or Created Files");

    if let Err(err) = Pages::run(Settings::default()) {
        error!("{}", err);
    }

    Ok(())
}

pub struct Pages {
    page: Box<dyn UiRenderer<Message = Message>>,
}

impl Application for Pages {
    type Message = Message;
    type Flags = ();
    type Executor = executor::Default;
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                page: Box::new(NpcUI::new()),
            },
            font::load(iced_aw::core::icons::BOOTSTRAP_FONT_BYTES).map(Message::FontLoaded),
        )
    }

    fn title(&self) -> String {
        self.page.title().to_string()
    }

    fn update(&mut self, event: Message) -> Command<Message> {
        self.page.update(event);
        Command::none()
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

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}
