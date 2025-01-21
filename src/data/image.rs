pub struct Image
{
    pub(crate) width: u32,
    pub(crate) height: u32,
    data: Vec<u8>
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        let data = vec![0; (width * height * 3) as usize];

        Image {
            width,
            height,
            data
        }
    }

    pub fn set_rgb(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8) {
        let index = self.get_index(x, y);

        self.data[index] = r;
        self.data[index + 1] = g;
        self.data[index + 2] = b;
    }
    pub fn rgb_at(&self, x: u32, y: u32) -> (u8, u8, u8) {
        let index = self.get_index(x, y);

        (self.data[index], self.data[index + 1], self.data[index + 2])
    }

    fn get_index(&self, x: u32, y: u32) -> usize {
        ((x + y * (self.width)) * 3) as usize
    }
}
