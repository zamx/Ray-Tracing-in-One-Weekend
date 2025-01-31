use crate::data::image::Image;
use crate::serialization::image_serializer::ImageSerializer;
use crate::serialization::ppm_image_serializer::PPMImageSerializer;

pub fn create_image() -> Image {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let mut image_height = image_width as f64 / aspect_ratio;
    image_height = if image_height < 1.0 { 1.0 } else { image_height };

    Image::new(image_width, image_height as u32)
}

pub fn serialize_image(image: &Image) {
    let serializer = PPMImageSerializer::new();

    serializer.serialize(&image);
}