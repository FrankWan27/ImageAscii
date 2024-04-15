use godot::engine::Image;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(no_init)]
pub(crate) struct Utils {}

#[godot_api]
impl Utils {
    #[func]
    pub fn fit_image_to_size(mut image: Gd<Image>, width: f32, height: f32) {
        let width_ratio = width / image.get_width() as f32;
        let height_ratio = height / image.get_height() as f32;
        // Choose the smaller ratio to ensure that the image fits within the specified bounds
        let ratio = width_ratio.min(height_ratio);

        let new_width = (image.get_width() as f32 * ratio) as i32;
        let new_height = (image.get_height() as f32 * ratio) as i32;

        image.resize(new_width, new_height)
    }

    pub fn to_gray_scale(mut image: Gd<Image>) -> Gd<Image> {
        for x in 0..image.get_width() {
            for y in 0..image.get_height() {
                let pixel = image.get_pixel(x, y);
                if pixel.a == 1.0 {
                    image.set_pixel(x, y, Utils::to_gray(pixel));
                }
            }
        }
        image
    }

    pub fn to_gray(c: Color) -> Color {
        let gray = (c.r + c.g + c.b) / 3.0;
        Color::from_hsv(0.0, 0.0, gray as f64)
    }
}
