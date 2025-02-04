mod data;
mod serialization;
mod render;
mod image_helpers;
mod visuals;
mod scene_factory;

fn main() {
    let scene = scene_factory::create_scene();
    let mut image = image_helpers::create_image();

    render::render_scene(&scene, &mut image);

    image_helpers::serialize_image(&image);
}
