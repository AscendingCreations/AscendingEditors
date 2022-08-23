use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Deserialize, Serialize, Hash)]

pub struct Position {
    pub x: i32,
    pub y: i32,
    pub map: u64,
}

#[allow(dead_code)]
impl Position {
    #[inline(always)]
    pub fn new(x: i32, y: i32, map: u64) -> Position {
        Position { x, y, map }
    }

    pub fn left_map(&self) -> bool {
        self.x < 0 || self.x >= MAP_MAX_X as i32 || self.y < 0 || self.y >= MAP_MAX_Y as i32
    }

    pub fn checkdistance(&self, target: Position) -> i32 {
        let x = self.x - target.x;
        let y = self.y - target.y;

        if x == 0 {
            return y.abs();
        }
        if y == 0 {
            return x.abs();
        }

        x.abs() + y.abs() - 1
    }

    pub fn map_offset(&self, mappos: MapPos) -> Position {
        match mappos {
            MapPos::UpLeft(_) => Position::new(
                self.x - MAP_MAX_X as i32,
                self.y - MAP_MAX_Y as i32,
                self.map,
            ),
            MapPos::Up(_) => Position::new(self.x, self.y - MAP_MAX_Y as i32, self.map),
            MapPos::UpRight(_) => Position::new(
                self.x + MAP_MAX_X as i32,
                self.y - MAP_MAX_Y as i32,
                self.map,
            ),
            MapPos::Left(_) => Position::new(self.x - MAP_MAX_X as i32, self.y, self.map),
            MapPos::None | MapPos::Center(_) => Position::new(self.x, self.y, self.map),
            MapPos::Right(_) => Position::new(self.x + MAP_MAX_X as i32, self.y, self.map),
            MapPos::DownLeft(_) => Position::new(
                self.x - MAP_MAX_X as i32,
                self.y + MAP_MAX_Y as i32,
                self.map,
            ),
            MapPos::Down(_) => Position::new(self.x, self.y + MAP_MAX_Y as i32, self.map),
            MapPos::DownRight(_) => Position::new(
                self.x + MAP_MAX_X as i32,
                self.y + MAP_MAX_Y as i32,
                self.map,
            ),
        }
    }
}
