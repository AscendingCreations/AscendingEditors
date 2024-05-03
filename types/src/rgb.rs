use serde::{Deserialize, Serialize};
use speedy::{Readable, Writable};

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, Default, Deserialize, Serialize, Readable, Writable,
)]
pub struct Rgba {
    pub r: i16,
    pub g: i16,
    pub b: i16,
    pub a: i16,
}
