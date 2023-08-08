use chrono::offset::Utc;
use chrono::Duration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MyDate(pub chrono::NaiveDate);

#[allow(dead_code)]
impl MyDate {
    pub fn now() -> MyDate {
        MyDate(Utc::now().date_naive())
    }

    pub fn add_days(&mut self, days: i64) {
        let dur = Duration::days(days);

        self.0 = match self.0.checked_add_signed(dur) {
            Some(n) => n,
            None => self.0,
        };
    }
}

impl From<chrono::NaiveDate> for MyDate {
    fn from(date: chrono::NaiveDate) -> MyDate {
        MyDate(date)
    }
}

impl AsRef<chrono::NaiveDate> for MyDate {
    fn as_ref(&self) -> &chrono::NaiveDate {
        &self.0
    }
}

impl std::ops::Deref for MyDate {
    type Target = chrono::NaiveDate;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
