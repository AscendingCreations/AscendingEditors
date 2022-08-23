use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MyDuration(pub chrono::Duration);

#[allow(dead_code)]
impl MyDuration {
    pub fn milliseconds(mills: i64) -> MyDuration {
        MyDuration(chrono::Duration::milliseconds(mills))
    }

    pub fn as_std(&self) -> std::time::Duration {
        if let Ok(dur) = self.0.to_std() {
            dur
        } else {
            std::time::Duration::from_millis(0)
        }
    }
}

impl From<chrono::Duration> for MyDuration {
    fn from(duration: chrono::Duration) -> MyDuration {
        MyDuration(duration)
    }
}

impl AsRef<chrono::Duration> for MyDuration {
    fn as_ref(&self) -> &chrono::Duration {
        &self.0
    }
}

impl std::ops::Deref for MyDuration {
    type Target = chrono::Duration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for MyDuration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let dur = self.num_milliseconds();

        dur.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for MyDuration {
    fn deserialize<D>(deserializer: D) -> Result<MyDuration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let dur = i64::deserialize(deserializer)?;
        let duration = chrono::Duration::milliseconds(dur);
        Ok(MyDuration(duration))
    }
}
