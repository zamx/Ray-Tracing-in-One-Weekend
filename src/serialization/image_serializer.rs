use crate::data::image::Image;

pub trait ImageSerializer {
    fn serialize(&self, image: &Image);
}