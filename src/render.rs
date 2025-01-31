use crate::data::color::Color;
use crate::data::image::Image;
use crate::data::ray::Ray;
use crate::data::vec3::Vec3;

fn ray_color(_ray: &Ray) -> Color {
    Color::black()
}

pub fn render_image(image: &mut Image) {
    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image.width as f64 / image.height as f64);
    let camera_center = Vec3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image.width as f64;
    let pixel_delta_v = viewport_v / image.height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    for y in 0..image.height
    {
        for x in 0..image.width
        {
            let pixel_center = pixel00_loc + x * pixel_delta_u + y * pixel_delta_v;
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);
            let color = ray_color(&ray);

            image.set_rgb(x, y, color);
        }
    }
}