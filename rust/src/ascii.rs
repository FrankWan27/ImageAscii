use godot::engine::control::GrowDirection;
use godot::engine::{
    viewport, Font, ILabel, Image, ImageTexture, Label, SubViewport, SubViewportContainer,
};
use godot::prelude::*;

use crate::draw::Draw;

pub const LOWEST_ASCII: char = ' ';
pub const HIGHEST_ASCII: char = '~';
pub const NUM_ASCII: usize = 95;

#[derive(GodotClass)]
#[class(base=Label)]
pub(crate) struct Ascii {
    char_size: Vector2,
    draw: Option<Gd<Draw>>,
    viewport: Option<Gd<SubViewport>>,
    viewport_container: Option<Gd<SubViewportContainer>>,
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
            viewport_container: None,
        }
    }
}

#[godot_api]
impl Ascii {
    #[func]
    pub fn initialize(
        &mut self,
        draw: Gd<Draw>,
        viewport: Gd<SubViewport>,
        mut viewport_container: Gd<SubViewportContainer>,
    ) {
        self.draw = Some(draw);
        self.viewport = Some(viewport);
        self.viewport_container = Some(viewport_container);
    }

    #[func]
    pub fn set_font(&mut self, font: Gd<Font>, font_size: i32) {
        self.base_mut()
            .add_theme_font_override("font".into(), font.clone());
        self.base_mut()
            .add_theme_font_size_override("font_size".into(), font_size);

        let char_size = self.measure_char_size();
        self.char_size = char_size.clone();
        self.draw
            .as_mut()
            .unwrap()
            .bind_mut()
            .set_font(font, font_size, char_size);
        godot_print!("changing viewport size {}", self.char_size);

        self.viewport_container
            .as_mut()
            .unwrap()
            .set_size(Vector2::new(char_size.x * NUM_ASCII as f32, char_size.y));
        self.viewport_container
            .as_mut()
            .unwrap()
            .set_position(Vector2::new(100.0, 100.0));
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
