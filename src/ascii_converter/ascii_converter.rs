use image::{DynamicImage, GenericImageView};
use rayon::prelude::*;

use strum_macros::{Display, EnumIter};

#[derive(EnumIter, Debug, PartialEq, Display)]
pub enum AsciiConversionMethod {
    SingleThreadedLoop,
    RayonThreadedLoop,
    RayonThreadedChunkedLoop,
}
#[allow(dead_code)]
pub struct AsciiConverter;
impl AsciiConverter {
    #[allow(dead_code)]
    const POSSIBLE_CHARS: [char; 29] = [
        '.', '`', '^', ',', ':', ';', '!', '>', '<', '~', '+', '_', '-', '?', ']', '[', '}', '{',
        ')', '(', '|', '\\', '/', '*', '#', '&', '%', '@', '$',
    ];

    #[allow(dead_code)]
    fn convert_pixel_to_ascii(pixel: u8) -> char {
        let index = (pixel as usize * Self::POSSIBLE_CHARS.len()) / 256;
        Self::POSSIBLE_CHARS[index]
    }

    #[allow(dead_code)]
    pub fn single_threaded_loop(img: &DynamicImage) -> Vec<char> {
        let char_vector = img
            .pixels()
            .map(|(_, _, pixel)| Self::convert_pixel_to_ascii(pixel[0]))
            .collect();
        char_vector
    }

    #[allow(dead_code)]
    pub fn rayon_threaded_loop(img: &DynamicImage) -> Vec<char> {
        let char_vector = img
            .pixels()
            .collect::<Vec<_>>()
            .par_iter()
            .map(|(_, _, pixel)| Self::convert_pixel_to_ascii(pixel[0]))
            .collect();
        char_vector
    }

    #[allow(dead_code)]
    pub fn rayon_threaded_chunked_loop(img: &DynamicImage) -> Vec<char> {
        let (width, _) = img.dimensions();
        let pixels = img.to_luma8().into_vec();

        pixels
            .par_chunks(width as usize)
            .flat_map(|chunk| {
                chunk
                    .iter()
                    .map(|&pixel| Self::convert_pixel_to_ascii(pixel))
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}
