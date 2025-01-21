use crate::data::image::Image;

#[test]
fn test_new() {
    let image = Image::new(5, 5);
    assert_eq!(image.width, 5);
    assert_eq!(image.height, 5);
}

#[test]
fn test_set_rgb_rgb_at() {
    let mut image = Image::new(2, 2);

    image.set_rgb(0, 0, 0, 0, 0);
    image.set_rgb(0, 1, 255, 0, 0);
    image.set_rgb(1, 0, 0, 255, 0);
    image.set_rgb(1, 1, 255, 255, 0);

    assert_eq!(image.rgb_at(0,0), (0, 0, 0), "index 0, 0");
    assert_eq!(image.rgb_at(0,1), (255, 0, 0), "index 1, 0");
    assert_eq!(image.rgb_at(1,0), (0, 255, 0), "index 0, 1");
    assert_eq!(image.rgb_at(1,1), (255, 255, 0), "index 1, 1");
}