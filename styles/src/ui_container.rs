use iced::{theme, widget::container, Background, Color, Theme, Vector};

pub const SURFACE: Color = Color::from_rgb(33_f32 / 255.0, 38_f32 / 255.0, 45_f32 / 255.0);
pub const MAIN_SURFACE: Color = Color::from_rgb(13_f32 / 255.0, 17_f32 / 255.0, 23_f32 / 255.0);

pub struct UiContainer;

impl container::StyleSheet for UiContainer {
    fn style(&self) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(SURFACE)),
            border_width: 1.0,
            border_color: Color::from_rgba8(156, 166, 157, 1.0),
            ..Default::default()
        }
    }
}

pub struct MainContainer;

impl container::StyleSheet for MainContainer {
    fn style(&self) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(MAIN_SURFACE)),
            border_width: 0.0,
            border_color: Color::from_rgba8(156, 166, 157, 1.0),
            ..Default::default()
        }
    }
}

pub struct ImageContainer;

impl container::StyleSheet for ImageContainer {
    fn style(&self) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(SURFACE)),
            border_width: 1.0,
            border_color: Color::from_rgba8(156, 166, 157, 1.0),
            ..Default::default()
        }
    }
}

pub struct ColorContainer {
    pub color: Color,
}

impl container::StyleSheet for ColorContainer {
    fn style(&self) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(self.color)),
            border_width: 1.0,
            border_color: Color::from_rgba8(156, 166, 157, 1.0),
            ..Default::default()
        }
    }
}
