use godot::engine::ISprite2D;
use godot::engine::Image;
use godot::engine::ImageTexture;
use godot::engine::Sprite2D;
use godot::prelude::*;

use crate::utils::Utils;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct ImageRect {
    base: Base<Sprite2D>,
    image: Gd<Image>,
}

#[godot_api]
impl ISprite2D for ImageRect {
    fn init(base: Base<Sprite2D>) -> Self {
        Self {
            base,
            image: Image::new_gd(),
        }
    }
}

#[godot_api]
impl ImageRect {
    #[func]
    fn set_image(&mut self, image: Gd<Image>) {
        self.image = image;
        self.reset_texture();
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
        let tex = self.base_mut().get_texture().unwrap();
        let image = tex.get_image().unwrap();
        let gray = Utils::to_gray_scale(image);

        self.set_texture(gray);
    }

    #[func]
    fn reset_texture(&mut self) {
        let image = self.get_image();
        self.set_texture(image);
    }
}
