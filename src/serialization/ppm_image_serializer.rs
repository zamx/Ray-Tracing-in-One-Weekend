use std::path::Path;
use std::fs::File;
use std::io::Write;

use image_serializer::ImageSerializer;
use crate::data::image::Image;
use crate::serialization::image_serializer;

pub struct PPMImageSerializer{}

impl PPMImageSerializer {
    pub(crate) fn new() -> Self {
        Self{}
    }
}

impl ImageSerializer for PPMImageSerializer {
    fn serialize(&self, image: &Image) {
        let path = Path::new("test.ppm");
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        let header = format!("P3\n{} {}\n255\n", image.width, image.height);

        match file.write_all(header.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => {},
        }

        for y in 0..image.height
        {
            for x in 0..image.width
            {
                let color = image.rgb_at(x, y);
                let line = format!("{} {} {}\n", color.r(), color.g(), color.b());

                match file.write_all(line.as_bytes()) {
                    Err(why) => panic!("couldn't write to {}: {}", display, why),
                    Ok(_) => {},
                }
            }
        }
    }
}