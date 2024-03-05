use iced::{widget::container, Background, Color, Theme, Border};

pub const SURFACE: Color = Color::from_rgb(33_f32 / 255.0, 38_f32 / 255.0, 45_f32 / 255.0);
pub const MAIN_SURFACE: Color = Color::from_rgb(13_f32 / 255.0, 17_f32 / 255.0, 23_f32 / 255.0);

pub struct UiContainer;

impl container::StyleSheet for UiContainer {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        let active: container::Appearance = Default::default();

        container::Appearance {
            background: Some(Background::Color(SURFACE)),
            border: Border {
                color: Color::from_rgba8(156, 166, 157, 1.0),
                width: 1.0,
                ..active.border
            },
            ..active
        }
    }
}

pub struct MainContainer;

impl container::StyleSheet for MainContainer {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        let active: container::Appearance = Default::default();
        
        container::Appearance {
            background: Some(Background::Color(MAIN_SURFACE)),
            border: Border {
                color: Color::from_rgba8(156, 166, 157, 1.0),
                width: 0.0,
                ..active.border
            },
            ..active
        }
    }
}

pub struct ImageContainer;

impl container::StyleSheet for ImageContainer {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        let active: container::Appearance = Default::default();

        container::Appearance {
            background: Some(Background::Color(SURFACE)),
            border: Border {
                color: Color::from_rgba8(156, 166, 157, 1.0),
                width: 1.0,
                ..active.border
            },
            ..active
        }
    }
}

pub struct ColorContainer {
    pub color: Color,
}

impl container::StyleSheet for ColorContainer {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        let active: container::Appearance = Default::default();

        container::Appearance {
            background: Some(Background::Color(self.color)),
            border: Border {
                color: Color::from_rgba8(156, 166, 157, 1.0),
                width: 1.0,
                ..active.border
            },
            ..active
        }
    }
}
