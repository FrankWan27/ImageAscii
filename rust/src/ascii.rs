use godot::engine::{Font, ILabel, Image, Label, SubViewport, SubViewportContainer};
use godot::prelude::*;

use crate::draw::Draw;

pub const LOWEST_ASCII: char = ' ';
pub const HIGHEST_ASCII: char = '~';
pub const NUM_ASCII: usize = 95;

#[derive(GodotClass)]
#[class(base=Label)]
pub(crate) struct Ascii {
    base: Base<Label>,
    char_size: Vector2,
    draw: Option<Gd<Draw>>,
    viewport: Option<Gd<SubViewport>>,
    viewport_container: Option<Gd<SubViewportContainer>>,
    char_vec: Vec<Gd<Image>>,
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
            char_vec: vec![Image::new_gd(); NUM_ASCII],
        }
    }

    fn ready(&mut self) {}
}

#[godot_api]
impl Ascii {
    #[func]
    pub fn initialize(
        &mut self,
        draw: Gd<Draw>,
        viewport: Gd<SubViewport>,
        viewport_container: Gd<SubViewportContainer>,
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
            .set_font(font, font_size);

        let size = self.draw.as_mut().unwrap().get_size();
        self.viewport_container.as_mut().unwrap().set_size(size);
        self.viewport_container
            .as_mut()
            .unwrap()
            .set_position(Vector2::new(100.0, -100.0));
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

    pub fn convert_chunks_to_ascii(&mut self, chunks: &Vec<Vec<Gd<Image>>>) -> GString {
        let mut output = String::new();
        for row in chunks {
            for chunk in row {
                output.push(self.convert_chunk_to_char(&chunk));
            }
            output.push('\n');
        }
        output.into()
    }

    fn convert_chunk_to_char(&mut self, chunk: &Gd<Image>) -> char {
        let mut winning_char = '?';
        let mut top_score = 0.0;
        for (i, c) in (LOWEST_ASCII..=HIGHEST_ASCII).enumerate() {
            // if c == '`' {
            //     continue;
            // }
            let char_img = &self.char_vec[i];
            let curr_score = Ascii::get_compare_score(chunk, char_img);
            if curr_score > top_score {
                winning_char = c;
                top_score = curr_score;
            }
        }
        winning_char
    }

    fn get_compare_score(chunk: &Gd<Image>, char_img: &Gd<Image>) -> f32 {
        let size = chunk.get_size();
        let mut score = 0.0;

        for y in 0..size.y {
            for x in 0..size.x {
                score += Ascii::compare_pixel(&chunk.get_pixel(x, y), &char_img.get_pixel(x, y));
            }
        }
        score
    }

    fn compare_pixel(a: &Color, b: &Color) -> f32 {
        1.0 - (Ascii::color_magnitude(a) - Ascii::color_magnitude(b)).abs()
        //uncomment for inverse colors
        //(Ascii::color_magnitude(a) - Ascii::color_magnitude(b)).abs()
    }

    fn color_magnitude(c: &Color) -> f32 {
        let gray = (c.r + c.g + c.b) / 3.0;
        let inverse = 1.0 - gray;
        let alpha = inverse * c.a;
        alpha
    }

    pub fn populate_cache(&mut self) {
        for (i, _c) in (LOWEST_ASCII..=HIGHEST_ASCII).enumerate() {
            let char_image = self.capture_viewport().get_region(Rect2i::from_components(
                self.get_char_size().x as i32 * i as i32,
                0,
                self.get_char_size().x as i32,
                self.get_char_size().y as i32,
            ));
            self.char_vec[i] = char_image.unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_magnitude() {
        assert_eq!(Ascii::color_magnitude(&Color::BLACK), 1.0);
        assert_eq!(Ascii::color_magnitude(&Color::WHITE), 0.0);
        assert_eq!(
            Ascii::color_magnitude(&Color::from_rgba(0.0, 0.0, 0.0, 0.5)),
            0.5
        );
    }

    #[test]
    fn test_compare_pixel() {
        assert_eq!(Ascii::compare_pixel(&Color::BLACK, &Color::BLACK), 1.0);
        assert_eq!(Ascii::compare_pixel(&Color::BLACK, &Color::WHITE), 0.0);
        assert_eq!(
            Ascii::compare_pixel(&Color::BLACK, &Color::from_rgba(0.0, 0.0, 0.0, 0.5)),
            0.5
        );
        assert_eq!(
            Ascii::compare_pixel(&Color::WHITE, &Color::from_rgba(0.0, 0.0, 0.0, 0.5)),
            0.5
        );
    }
}
