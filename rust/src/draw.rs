use godot::engine::global::HorizontalAlignment;
use godot::engine::text_server::{Direction, JustificationFlag, Orientation};
use godot::engine::{Control, Font, IControl, ILabel, Image, ImageTexture, Label};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Control)]
pub(crate) struct Draw {
    base: Base<Control>,
    font: Option<Gd<Font>>,
    font_size: i32,
    char_size: Vector2,
}

#[godot_api]
impl IControl for Draw {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            font: None,
            font_size: 0,
            char_size: Vector2::ZERO,
        }
    }

    fn draw(&mut self) {
        match &self.font {
            Some(font) => {
                self.base()
                    .draw_string(font.clone(), Vector2::ZERO, "4".into())
                // self.base().draw_string_full(
                //     font,
                //     Vector2::ZERO,
                //     "4",
                //     HorizontalAlignment::CENTER,
                //     self.char_size.x as i32,
                //     self.font_size,
                //     Color::BLACK,
                //     JustificationFlag::NONE,
                //     Direction::AUTO,
                //     Orientation::HORIZONTAL,
                // );
            }
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
    }
}
