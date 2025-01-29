use crate::data::color::Color;
use crate::serialization::ppm_image_serializer::PPMImageSerializer;
use crate::serialization::image_serializer::ImageSerializer;
use crate::data::image;

mod data;
mod serialization;

fn fill_image(image: &mut image::Image)
{
    for y in 0..image.height
    {
        for x in 0..image.width
        {
            let r = x as f64 / (image.height - 1) as f64;
            let g = y as f64 / (image.width - 1) as f64;
            let b = 0.0;

            let ir = (255.99 * r) as i32;
            let ig = (255.99 * g) as i32;
            let ib = (255.99 * b) as i32;

            image.set_rgb(x, y, Color::new(ir, ig, ib));
        }
    }
}

fn serialize_image(image: &image::Image) {
    let serializer = PPMImageSerializer::new();

    serializer.serialize(&image);
}

fn main() {
    let mut image = image::Image::new(256, 256);

    fill_image(&mut image);
    serialize_image(&image);
}
