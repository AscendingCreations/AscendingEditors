use log::{set_logger, set_max_level, LevelFilter, Log, Metadata, Record, SetLoggerError};
use std::{borrow::Cow, fs::File, io::Write};

pub struct MyLogger {
    pub print: bool,
    /// Output file path and name of the file for logging too.
    pub filename: Cow<'static, str>,
}

impl MyLogger {
    pub fn new(filename: impl Into<Cow<'static, str>>) -> Box<Self> {
        Box::new(Self {
            print: false,
            filename: filename.into(),
        })
    }

    pub fn set_print(mut self: Box<Self>, print: bool) -> Box<Self> {
        self.as_mut().print = print;
        self
    }

    pub fn set_boxed_logger(self: Box<Self>) -> Result<(), SetLoggerError> {
        set_logger(Box::leak(self))?;
        set_max_level(LevelFilter::Info);
        Ok(())
    }
}

impl Log for MyLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let msg = format!("{} - {}\n", record.level(), record.args());

        if self.print {
            println!("{}", &msg);
        }

        let mut file = match File::options()
            .append(true)
            .create(true)
            .open(&*self.filename)
        {
            Ok(v) => v,
            Err(_) => return,
        };

        let _ = file.write(msg.as_bytes());
    }
    fn flush(&self) {}
}
