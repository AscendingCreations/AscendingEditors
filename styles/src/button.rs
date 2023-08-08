use iced::{theme, widget::button, Background, Color, Theme, Vector};

#[allow(dead_code)]
pub enum Button {
    Primary,
    Secondary,
}

impl button::StyleSheet for Button {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        let default = style.active(&theme::Button::Primary);

        button::Appearance {
            background: Some(Background::Color(match self {
                Button::Primary => Color::from_rgb8(33, 38, 45),
                Button::Secondary => Color::from_rgb8(65, 75, 88),
            })),
            border_radius: 12.0.into(),
            border_color: Color::from_rgb8(45, 51, 58),
            shadow_offset: Vector::new(1.0, 1.0),
            text_color: Color::WHITE,
            ..default
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            text_color: Color::WHITE,
            border_color: Color::from_rgb8(201, 209, 217),
            shadow_offset: Vector::new(1.0, 2.0),
            ..self.active(style)
        }
    }
}
