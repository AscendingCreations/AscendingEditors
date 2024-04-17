use graphics::*;

use crate::DrawSetting;

#[derive(PartialEq, Eq)]
pub enum ButtonState {
    Normal,
    Selected,
}

pub struct ToolButton {
    pub index: usize,
    pub image: usize,
    pub state: ButtonState,
    pub in_hover: bool,
    pub in_click: bool,
}

impl ToolButton {
    pub fn set_state(&mut self, systems: &mut DrawSetting, state: ButtonState) {
        if self.state != state {
            self.state = state;
            let mut uv = systems.gfx.get_uv(self.image);
            let size = systems.gfx.get_size(self.image);
            match self.state {
                ButtonState::Normal => { uv.y = 0.0; }
                ButtonState::Selected => { uv.y = size.y * 2.0; },
            }
            systems.gfx.set_uv(self.image, uv);
        }
    }

    pub fn set_hover(&mut self, systems: &mut DrawSetting, hover: bool) {
        if self.in_hover != hover {
            self.in_hover = hover;
            if self.state == ButtonState::Normal {
                let mut uv = systems.gfx.get_uv(self.image);
                let size = systems.gfx.get_size(self.image);

                if self.in_hover {
                    uv.y = size.y;
                } else {
                    uv.y = 0.0;
                }
                systems.gfx.set_uv(self.image, uv);
            }
        }
    }

    pub fn set_click(&mut self, systems: &mut DrawSetting, click: bool) {
        if self.in_click != click {
            self.in_click = click;
            if self.state == ButtonState::Normal {
                let mut uv = systems.gfx.get_uv(self.image);
                let size = systems.gfx.get_size(self.image);

                if self.in_click {
                    uv.y = size.y * 2.0;
                } else {
                    uv.y = size.y;
                }
                systems.gfx.set_uv(self.image, uv);
            }
        }
    }
}