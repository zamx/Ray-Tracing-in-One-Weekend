use crate::data::color::Color;

pub struct Image
{
    pub(crate) width: u32,
    pub(crate) height: u32,
    data: Vec<Color>
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        let data = vec![Color::black(); (width * height) as usize];

        Image {
            width,
            height,
            data
        }
    }

    pub fn set_rgb(&mut self, x: u32, y: u32, color: Color) {
        let index = self.get_index(x, y);

        self.data[index] = color;
    }
    pub fn rgb_at(&self, x: u32, y: u32) -> &Color {
        let index = self.get_index(x, y);

        &self.data[index]
    }

    const fn get_index(&self, x: u32, y: u32) -> usize {
        (x + y * (self.width)) as usize
    }
}
