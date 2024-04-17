use godot::engine::global::HorizontalAlignment;
use godot::engine::text_server::{Direction, JustificationFlag, Orientation};
use godot::engine::{Control, Font, IControl, ILabel, Image, ImageTexture, Label};
use godot::prelude::*;

use crate::ascii::{HIGHEST_ASCII, LOWEST_ASCII};

#[derive(GodotClass)]
#[class(base=Control)]
pub(crate) struct Draw {
    base: Base<Control>,
    font: Option<Gd<Font>>,
    font_size: i32,
    char_size: Vector2,
    text: GString,
}

#[godot_api]
impl IControl for Draw {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            font: None,
            font_size: 0,
            char_size: Vector2::ZERO,
            text: Draw::generate_ascii_string(LOWEST_ASCII, HIGHEST_ASCII),
        }
    }

    fn draw(&mut self) {
        godot_print!("drawing");
        match &self.font {
            Some(font) => self
                .base()
                .draw_string_ex(
                    font.clone(),
                    Vector2::new(0.0, self.char_size.y * 0.8),
                    self.text.clone(),
                )
                .font_size(self.font_size)
                .modulate(Color::BLACK)
                .done(),
            None => return,
        }
    }
}

#[godot_api]
impl Draw {
    #[func]
    pub fn set_font(&mut self, font: Gd<Font>, font_size: i32, char_size: Vector2) {
        self.font = Some(font);
        self.font_size = font_size;
        self.char_size = char_size;
        godot_print!("set fontsize {}", self.font_size);
        self.base_mut().queue_redraw();
    }
    fn generate_ascii_string(start: char, end: char) -> GString {
        let mut str = String::new();
        for c in start..=end {
            str.push(c);
            // str.push('█');
        }
        str.push('X');
        return str.into();
    }
}
