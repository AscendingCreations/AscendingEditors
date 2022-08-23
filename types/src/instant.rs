use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ops::Add;
use std::time::Instant;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MyInstant(pub std::time::Instant);

#[allow(dead_code)]
impl MyInstant {
    pub fn now() -> MyInstant {
        MyInstant(Instant::now())
    }
}

impl From<Instant> for MyInstant {
    fn from(instant: Instant) -> MyInstant {
        MyInstant(instant)
    }
}

impl AsRef<Instant> for MyInstant {
    fn as_ref(&self) -> &Instant {
        &self.0
    }
}

impl std::ops::Deref for MyInstant {
    type Target = Instant;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for MyInstant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut dur: i64 = 0;

        if let Ok(approx) =
            chrono::Duration::from_std(self.0.saturating_duration_since(Instant::now()))
        {
            if approx > chrono::Duration::milliseconds(1) {
                dur = approx.num_milliseconds();
            }
        }

        dur.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for MyInstant {
    fn deserialize<D>(deserializer: D) -> Result<MyInstant, D::Error>
    where
        D: Deserializer<'de>,
    {
        let dur: i64 = i64::deserialize(deserializer)?;
        let duration = chrono::Duration::milliseconds(dur);
        let mut instant_now = Instant::now();

        if let Ok(dur) = duration.to_std() {
            instant_now += dur;
        }

        Ok(MyInstant(instant_now))
    }
}

impl Add<chrono::Duration> for MyInstant {
    type Output = MyInstant;

    fn add(self, other: chrono::Duration) -> MyInstant {
        if let Ok(dur) = other.to_std() {
            MyInstant(self.0 + dur)
        } else {
            MyInstant(self.0)
        }
    }
}

impl Add<std::time::Duration> for MyInstant {
    type Output = MyInstant;

    fn add(self, other: std::time::Duration) -> MyInstant {
        MyInstant(self.0 + other)
    }
}
