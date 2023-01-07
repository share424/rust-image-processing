use std::{fs::File, fmt::Display, cmp::min, cmp::max};
use std::io::BufReader;
use jpeg_decoder::Decoder;
use jpeg_encoder::{Encoder, ColorType, EncodingError};

#[derive(Clone)]
pub struct Image {
    pixels: Vec<u8>, // Assume pixel in CHW format
    pub height: u16,
    pub width: u16,
    pub color_mode: ColorMode
}

pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

#[derive(Clone)]
pub enum ColorMode {
    RGB,
    HSV,
    HCL,
    GRAY
}

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pixel")
            .field("R", &self.r)
            .field("G", &self.g)
            .field("B", &self.b)
            .finish()
    }
}

impl Image {
    pub fn load(path: &str) -> Self {
        let file = File::open(path).expect("failed to open file");
        let mut decoder = Decoder::new(BufReader::new(file));
        let pixels = decoder.decode().expect("failed to decode image");
        let metadata = decoder.info().unwrap();
        let height = metadata.height;
        let width = metadata.width;

        Image {
            pixels,
            height,
            width,
            color_mode: ColorMode::RGB
        }
    }

    pub fn save(&self, path: &str, quality: u8) -> Result<(), EncodingError> {
        let encoder = Encoder::new_file(path, quality).unwrap();
        encoder.encode(&self.pixels, self.width, self.height, ColorType::Rgb)
    }

    fn clamp(&self, x: usize, y: usize) -> (usize, usize) {
        return (
            min(max(x, 0), (self.width - 1) as usize),
            min(max(y, 0), (self.height - 1) as usize)
        );
    }

    fn coord_to_index(&self, x: usize, y: usize, c: usize) -> usize {
        return ((y * (self.width as usize)) * 3) + (x * 3) + c;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Pixel {
        let ( a, b ) = self.clamp(x, y);
        return Pixel {
            r: self.pixels[self.coord_to_index(a, b, 0)],
            g: self.pixels[self.coord_to_index(a, b, 1)],
            b: self.pixels[self.coord_to_index(a, b, 2)]
        };
    }

    pub fn get_individual_pixel(&self, x: usize, y: usize, mut channel: usize) -> u8 {
        let ( a, b ) = self.clamp(x, y);
        if channel >= 3 {
            channel = 2;
        }
        return self.pixels[self.coord_to_index(a, b, channel)];
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, p: Pixel) {
        self.set_individual_pixel(x, y, 0, p.r);
        self.set_individual_pixel(x, y, 1, p.g);
        self.set_individual_pixel(x, y, 2, p.b);
    }

    pub fn set_individual_pixel(&mut self, x: usize, y: usize, mut channel: usize, value: u8) {
        let ( a, b ) = self.clamp(x, y);
        if channel >= 3 {
            channel = 2;
        }
        let index = self.coord_to_index(a, b, channel);
        self.pixels[index] = value;
    }
}