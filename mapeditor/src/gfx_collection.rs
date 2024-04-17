use cosmic_text::Attrs;
use graphics::*;
use naga::proc::index;
use slab::Slab;

pub enum GfxType {
    Image(Image),
    Rect(Rect),
    Text(Text),
}

pub struct Gfx {
    pub gfx: GfxType,
    pub layer: usize,
    pub visible: bool,
}

pub struct GfxCollection {
    pub collection: Slab<Gfx>,
}

impl GfxCollection {
    pub fn new() -> Self {
        Self {
            collection: Slab::new(),
        }
    }

    pub fn add_image(&mut self, image: Image, layer: usize) -> usize {
        let gfx = Gfx {
            gfx: GfxType::Image(image),
            layer,
            visible: true,
        };
        self.collection.insert(gfx)
    }

    pub fn add_rect(&mut self, rect: Rect, layer: usize) -> usize {
        let gfx = Gfx {
            gfx: GfxType::Rect(rect),
            layer,
            visible: true,
        };
        self.collection.insert(gfx)
    }

    pub fn add_text(&mut self, text: Text, layer: usize) -> usize {
        let gfx = Gfx {
            gfx: GfxType::Text(text),
            layer,
            visible: true,
        };
        self.collection.insert(gfx)
    }

    pub fn remove_gfx(&mut self, index: usize) {
        self.collection.remove(index);
    }

    pub fn set_visible(&mut self, index: usize, visible: bool) {
        self.collection[index].visible = visible;
        if self.collection[index].visible {
            match &mut self.collection[index].gfx {
                GfxType::Image(image) => image.changed = true,
                GfxType::Rect(rect) => rect.changed = true,
                GfxType::Text(text) => text.changed = true,
            }
        }
    }

    pub fn set_color(&mut self, index: usize, color: Color) {
        if let Some(data) = self.collection.get_mut(index) {
            match &mut data.gfx {
                GfxType::Image(image) => {
                    image.color = color;
                    image.changed = true;
                }
                GfxType::Rect(rect) => {
                    rect.set_color(color);
                }
                GfxType::Text(text) => {
                    text.set_default_color(color);
                }
            }
        }
    }

    pub fn set_border_color(&mut self, index: usize, color: Color) {
        if let Some(data) = self.collection.get_mut(index) {
            if let GfxType::Rect(rect) = &mut data.gfx {
                rect.set_border_color(color);
            }
        }
    }

    pub fn set_border_width(&mut self, index: usize, width: f32) {
        if let Some(data) = self.collection.get_mut(index) {
            if let GfxType::Rect(rect) = &mut data.gfx {
                rect.set_border_width(width);
            }
        }
    }

    pub fn set_pos(&mut self, index: usize, pos: Vec3) {
        if let Some(data) = self.collection.get_mut(index) {
            match &mut data.gfx {
                GfxType::Image(image) => {
                    image.pos = pos;
                    image.changed = true;
                }
                GfxType::Rect(rect) => {
                    rect.set_position(pos);
                }
                GfxType::Text(text) => {
                    text.set_position(pos);
                }
            }
        }
    }

    pub fn set_size(&mut self, index: usize, size: Vec2) {
        if let Some(data) = self.collection.get_mut(index) {
            match &mut data.gfx {
                GfxType::Image(image) => {
                    image.hw = size;
                    image.changed = true;
                }
                GfxType::Rect(rect) => {
                    rect.set_size(size);
                }
                _ => {}
            }
        }
    }

    pub fn set_uv(&mut self, index: usize, uv: Vec4) {
        if let Some(data) = self.collection.get_mut(index) {
            if let GfxType::Image(image) = &mut data.gfx {
                image.uv = uv;
                image.changed = true;
            }
        }
    }

    pub fn set_text(
        &mut self,
        renderer: &mut GpuRenderer,
        index: usize,
        msg: &str,
    ) {
        if let Some(data) = self.collection.get_mut(index) {
            if let GfxType::Text(text) = &mut data.gfx {
                text.set_text(renderer, msg, Attrs::new(), Shaping::Advanced);
            }
        }
    }

    pub fn center_text(&mut self, index: usize) {
        if let Some(data) = self.collection.get_mut(index) {
            if let GfxType::Text(text) = &mut data.gfx {
                let size = text.measure();
                let bound = text.bounds.unwrap_or_default();
                let textbox_size = bound.right - bound.left;
                text.pos.x =
                    bound.left + ((textbox_size * 0.5) - (size.x * 0.5));
                text.changed = true;
            }
        }
    }

    pub fn get_pos(&mut self, index: usize) -> Vec3 {
        if let Some(data) = self.collection.get(index) {
            match &data.gfx {
                GfxType::Image(image) => image.pos,
                GfxType::Rect(rect) => rect.position,
                GfxType::Text(text) => text.pos,
            }
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        }
    }

    pub fn get_size(&mut self, index: usize) -> Vec2 {
        if let Some(data) = self.collection.get(index) {
            match &data.gfx {
                GfxType::Image(image) => image.hw,
                GfxType::Rect(rect) => rect.size,
                GfxType::Text(text) => text.size,
            }
        } else {
            Vec2::new(0.0, 0.0)
        }
    }

    pub fn get_uv(&mut self, index: usize) -> Vec4 {
        if let Some(data) = self.collection.get(index) {
            match &data.gfx {
                GfxType::Image(image) => image.uv,
                _ => Vec4::new(0.0, 0.0, 0.0, 0.0),
            }
        } else {
            Vec4::new(0.0, 0.0, 0.0, 0.0)
        }
    }

    pub fn get_color(&mut self, index: usize) -> Color {
        if let Some(data) = self.collection.get(index) {
            match &data.gfx {
                GfxType::Image(image) => image.color,
                GfxType::Rect(rect) => rect.color,
                GfxType::Text(text) => text.default_color,
            }
        } else {
            Color::rgba(0, 0, 0, 0)
        }
    }

    pub fn get_measure(&mut self, index: usize) -> Vec2 {
        if let Some(data) = self.collection.get(index) {
            match &data.gfx {
                GfxType::Text(text) => text.measure(),
                _ => Vec2::new(0.0, 0.0),
            }
        } else {
            Vec2::new(0.0, 0.0)
        }
    }
}
