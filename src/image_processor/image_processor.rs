use crate::ascii_converter::ascii_converter::{AsciiConversionMethod, AsciiConverter};
use image::{imageops, io::Reader as ImageReader, DynamicImage, GenericImageView};
use std::{
    io::{self, Write},
    time::Instant,
};

pub struct ImageProcessor {
    image_path: String,
    terminal_width: usize,
    img: Option<DynamicImage>,
}

impl ImageProcessor {
    pub fn new(image_path: String, terminal_width: usize) -> ImageProcessor {
        ImageProcessor {
            image_path: image_path,
            terminal_width: terminal_width,
            img: None,
        }
    }

    pub fn pre_process_image(&mut self) {
        let mut img = self.read_image();
        img = self.resize_image(&img);
        img = self.gray_scale_image(&img);

        self.img = Some(img);
    }

    fn read_image(&self) -> image::DynamicImage {
        let img = ImageReader::open(&self.image_path)
            .unwrap()
            .decode()
            .unwrap();
        img
    }

    fn resize_image(&self, img: &DynamicImage) -> DynamicImage {
        img.resize(
            self.terminal_width.try_into().unwrap(),
            img.dimensions().1,
            imageops::FilterType::Nearest,
        )
    }

    fn gray_scale_image(&self, img: &DynamicImage) -> DynamicImage {
        img.grayscale()
    }

    pub fn print_ascii(&self, conversion_method: AsciiConversionMethod) {
        let img = self
            .img
            .as_ref()
            .expect("Must have done image preprocessing first!");

        let start = Instant::now();
        let ascii = match conversion_method {
            AsciiConversionMethod::SingleThreadedLoop => AsciiConverter::single_threaded_loop(img),
            AsciiConversionMethod::RayonThreadedLoop => AsciiConverter::rayon_threaded_loop(img),
            AsciiConversionMethod::RayonThreadedChunkedLoop => {
                AsciiConverter::rayon_threaded_chunked_loop(img)
            }
        };
        let duration = start.elapsed();

        let mut buffer = String::with_capacity(ascii.len());
        for (index, char) in ascii.iter().enumerate() {
            buffer.push(*char);
            if index % self.terminal_width == 0 {
                buffer.push('\n');
            }
        }

        io::stdout().write_all(buffer.as_bytes()).unwrap();
        io::stdout().flush().unwrap();

        println!("\nConverted in {:?}", duration)
    }
}
