mod data;
mod serialization;
mod render;
mod image_helpers;



fn main() {
    let mut image = image_helpers::create_image();

    render::render_image(&mut image);

    image_helpers::serialize_image(&image);
}
