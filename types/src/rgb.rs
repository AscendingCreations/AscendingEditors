use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Deserialize, Serialize)]
pub struct Rgba {
    pub r: i16,
    pub g: i16,
    pub b: i16,
    pub a: i16,
}
