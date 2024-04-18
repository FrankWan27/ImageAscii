use godot::engine::{Font, IControl, Label};
use godot::prelude::*;

use crate::ascii::{HIGHEST_ASCII, LOWEST_ASCII};

#[derive(GodotClass)]
#[class(base=Label)]
pub(crate) struct Draw {
    base: Base<Label>,
    text: GString,
}

#[godot_api]
impl IControl for Draw {
    fn init(base: Base<Label>) -> Self {
        Self {
            base,
            text: Draw::generate_ascii_string(LOWEST_ASCII, HIGHEST_ASCII),
        }
    }

    fn ready(&mut self) {
        self.base_mut()
            .add_theme_color_override("font_color".into(), Color::BLACK);
        let txt = self.text.clone();
        self.base_mut().set_text(txt);
    }
}

#[godot_api]
impl Draw {
    #[func]
    pub fn set_font(&mut self, font: Gd<Font>, font_size: i32) {
        self.base_mut()
            .add_theme_font_override("font".into(), font.clone());
        self.base_mut()
            .add_theme_font_size_override("font_size".into(), font_size);
    }

    fn generate_ascii_string(start: char, end: char) -> GString {
        let mut str = String::new();
        for c in start..=end {
            str.push(c);
        }
        return str.into();
    }
}
