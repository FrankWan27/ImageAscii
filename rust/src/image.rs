use godot::engine::global::HorizontalAlignment;
use godot::engine::Font;
use godot::engine::ISprite2D;
use godot::engine::Image;
use godot::engine::ImageTexture;
use godot::engine::RichTextLabel;
use godot::engine::Sprite2D;
use godot::prelude::*;

use crate::ascii::Ascii;
use crate::utils::Utils;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct ImageRect {
    base: Base<Sprite2D>,
    image: Gd<Image>,
    ascii: Option<Gd<Ascii>>,
    text_label: Option<Gd<RichTextLabel>>,
}

#[godot_api]
impl ISprite2D for ImageRect {
    fn init(base: Base<Sprite2D>) -> Self {
        Self {
            base,
            image: Image::new_gd(),
            ascii: None,
            text_label: None,
        }
    }
}

#[godot_api]
impl ImageRect {
    #[func]
    fn set_ascii(&mut self, ascii: Gd<Ascii>) {
        self.ascii = Some(ascii);
    }

    #[func]
    fn set_text_label(&mut self, label: Gd<RichTextLabel>) {
        self.text_label = Some(label);
    }

    #[func]
    fn set_image(&mut self, image: Gd<Image>) {
        self.image = image;
        self.reset_texture();
    }

    #[func]
    fn set_font(&mut self, font: Gd<Font>, font_size: i32) {
        let label = self.text_label.as_mut().unwrap();
        label.clear();
        label.push_color(Color::BLACK);
        label.push_paragraph(HorizontalAlignment::CENTER);
        label.push_font_ex(font.clone()).font_size(font_size).done();
        self.ascii
            .as_mut()
            .unwrap()
            .bind_mut()
            .set_font(font, font_size);
    }

    fn set_texture(&mut self, image: Gd<Image>) {
        let mut tex = ImageTexture::new_gd();
        tex.set_image(image);
        self.base_mut().set_texture(tex.upcast());
    }

    #[func]
    fn get_image(&mut self) -> Gd<Image> {
        return self.image.clone();
    }

    #[func]
    fn convert_to_ascii(&mut self) {
        let image = self.base_mut().get_texture().unwrap().get_image().unwrap();
        //let gray = Utils::to_gray_scale(image);
        let chunks =
            Utils::split_image(image, &self.ascii.as_ref().unwrap().bind().get_char_size());
        let ascii_string = self
            .ascii
            .as_mut()
            .unwrap()
            .bind_mut()
            .convert_chunks_to_ascii(&chunks);

        let label = self.text_label.as_mut().unwrap();
        label.append_text(ascii_string);
        label.pop_all();
        godot_print!("{}", label.get_text());
    }

    #[func]
    fn reset_texture(&mut self) {
        let image = self.get_image();
        self.set_texture(image);
    }
}
