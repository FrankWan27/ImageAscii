use godot::engine::{viewport, Font, ILabel, Image, ImageTexture, Label, SubViewport};
use godot::prelude::*;

use crate::draw::Draw;

const LOWEST_ASCII: char = ' ';
const HIGHEST_ASCII: char = '~';

#[derive(GodotClass)]
#[class(base=Label)]
pub(crate) struct Ascii {
    char_size: Vector2,
    draw: Option<Gd<Draw>>,
    viewport: Option<Gd<SubViewport>>,
    base: Base<Label>,
}

#[godot_api]
impl ILabel for Ascii {
    fn init(base: Base<Label>) -> Self {
        Self {
            base,
            char_size: Vector2::default(),
            draw: None,
            viewport: None,
        }
    }
}

#[godot_api]
impl Ascii {
    #[func]
    pub fn initialize(&mut self, draw: Gd<Draw>, viewport: Gd<SubViewport>) {
        self.draw = Some(draw);
        self.viewport = Some(viewport);
    }

    #[func]
    pub fn set_font(&mut self, font: Gd<Font>, font_size: i32) {
        self.base_mut().add_theme_font_override("font".into(), font);
        self.base_mut()
            .add_theme_font_size_override("font_size".into(), font_size);

        self.char_size = self.measure_char_size();
        godot_print!("char size {}", self.char_size);
        self.populate_cache();
    }

    #[func]
    pub fn get_char_size(&self) -> Vector2 {
        self.char_size
    }

    #[func]
    pub fn measure_char_size(&mut self) -> Vector2 {
        self.set_char('█');
        self.base().get_size()
    }

    fn set_char(&mut self, c: char) {
        self.base_mut().set_size(Vector2::ZERO);
        self.base_mut().set_text(c.to_string().into());
    }

    fn populate_cache(&mut self) {
        for code in LOWEST_ASCII..=HIGHEST_ASCII {
            let font: Gd<Font>;
            self.set_char(code);
            ImageTexture::new_gd();
            let image = Image::new_gd();
        }
    }

    #[func]
    fn capture_viewport(&mut self) -> Gd<Image> {
        self.viewport
            .as_ref()
            .unwrap()
            .get_texture()
            .unwrap()
            .get_image()
            .unwrap()
    }
}
