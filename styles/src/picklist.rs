use iced::{pick_list, Background, Color};
use iced_style::menu;

const BACKGROUND: Color = Color {
    r: 48.0 / 255.0,
    g: 54.0 / 255.0,
    b: 61.0 / 255.0,
    a: 1.0,
};

const PRIMARY: Color = Color {
    r: 110.0 / 255.0,
    g: 118.0 / 255.0,
    b: 129.0 / 255.0,
    a: 1.0,
};

const HOVERED: Color = Color {
    r: 131.0 / 255.0,
    g: 139.0 / 255.0,
    b: 149.0 / 255.0,
    a: 1.0,
};

pub struct CustomPickList;
impl pick_list::StyleSheet for CustomPickList {
    fn menu(&self) -> menu::Style {
        menu::Style {
            text_color: Color::WHITE,
            background: Background::Color(BACKGROUND),
            border_color: PRIMARY,
            ..Default::default()
        }
    }

    fn active(&self) -> pick_list::Style {
        pick_list::Style {
            text_color: Color::WHITE,
            background: Background::Color(BACKGROUND),
            border_color: PRIMARY,
            ..Default::default()
        }
    }

    fn hovered(&self) -> pick_list::Style {
        pick_list::Style {
            border_color: HOVERED,
            ..self.active()
        }
    }
}
