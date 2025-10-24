use core::str;
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use image::{open, DynamicImage, ImageBuffer, Rgba};

// Encoding Image
// ==========================================================================

pub struct Encoder<'a> {
    img: ImageBuffer<Rgba<u8>, Vec<u8>>,
    input: &'a [u8],
}

#[allow(dead_code)]
impl<'a> Encoder<'a> {
    /// Creates a new encoder with a buffer to write and an image to write it to
    pub fn new(input: &[u8], img: DynamicImage) -> Encoder<'_> {
        let img_as_rgba: ImageBuffer<Rgba<u8>, Vec<u8>> = img.to_rgba8();
        Encoder {
            img: img_as_rgba,
            input,
        }
    }

    /// Encodes the buffer into the alpha channel of the destination image
    pub fn encode_alpha(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let (width, height) = self.img.dimensions();
        let bytes = width * height;

        if self.input.len() > bytes as usize {
            panic!("Input is too large for image size");
        }

        let mut out = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height);

        for (x, y, pixel) in self.img.enumerate_pixels() {
            let mut tmp_pixel = *pixel;

            let input_index = x + (y * width);

            if input_index < self.input.len() as u32 {
                tmp_pixel.0[3] = self.input[input_index as usize];
            }

            out.put_pixel(x, y, tmp_pixel);
        }

        out
    }

    /// Encodes the buffer into its own image using RGBA channels
    pub fn encode_image(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        //4 bytes per pixel
        let mut pixels = self.input.len() / 4;
        let padding = 4 - (self.input.len() % 4);
        //if the length falls on a pixel boundary so no padding needed
        if padding != 4 {
            pixels += padding;
        }

        //make it as close to a square as possible
        let width = (pixels as f64).sqrt().floor() as u32;
        let height = (pixels as f64 / width as f64).ceil() as u32;

        //create all the pixels
        let mut out = ImageBuffer::new(width, height);
        let mut out_pixels: Vec<(u32, u32, Rgba<u8>)> = Vec::new();
        for (x, y, pixel) in out.enumerate_pixels() {
            let tmp_pixel: &Rgba<u8> = pixel;
            let mut out_pixel = tmp_pixel.to_owned();
            let input_index = (x + (y * width)) * 4;
            if input_index < self.input.len() as u32 {
                let r: u8 = self.input[input_index as usize];
                let g: u8 = self.input[input_index as usize + 1];
                let b: u8 = self.input[input_index as usize + 2];
                let a: u8 = self.input[input_index as usize + 3];
                out_pixel.0 = [r, g, b, a];
            } else {
                out_pixel.0 = [0, 0, 0, 0];
            }
            out_pixels.push((x, y, out_pixel));
        }

        //write them to the output buffer
        for p in out_pixels {
            out.put_pixel(p.0, p.1, p.2);
        }

        out
    }
}
// Encoding Image
// ==========================================================================

// Decoding Image
// ==========================================================================
pub struct Decoder {
    img: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

#[allow(dead_code)]
impl Decoder {
    /// Creates a new decoder with an image to read from
    pub fn new(img: ImageBuffer<Rgba<u8>, Vec<u8>>) -> Decoder {
        Decoder { img }
    }

    /// Decodes the image by reading the alpha channel of each pixel
    pub fn decode_alpha(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();

        for (_, _, pixel) in self.img.enumerate_pixels() {
            out.push(pixel.0[3]);
        }

        out
    }

    /// Decodes the image by reading the bytes from each channel of each pixel
    pub fn decode_image(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();

        for (_, _, pixel) in self.img.enumerate_pixels() {
            out.push(pixel.0[0]);
            out.push(pixel.0[1]);
            out.push(pixel.0[2]);
            out.push(pixel.0[3]);
        }

        out
    }
}
// ==========================================================================

// more helper
// ===============================================================
#[allow(dead_code)]
pub fn str_to_bytes<'a>(word: &'a String) -> Option<&'a [u8]> {
    if word.is_empty() {
        None
    } else {
        let word_bytes = word.as_bytes();
        Some(word_bytes)
    }
}

#[allow(dead_code)]
pub fn file_to_bytes(mut file: File) -> Option<Vec<u8>> {
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .expect("Cant read file steg.rs");
    Some(buffer)
}

#[allow(dead_code)]
pub fn file_to_bytes_custom(file: &mut [&File]) -> Option<Vec<u8>> {
    let mut buffer = Vec::new();
    for fl in file {
        fl.read_to_end(&mut buffer).expect("Cant read file steg.rs");
    }
    if buffer.is_empty() {
        None
    } else {
        Some(buffer)
    }
}

#[allow(dead_code)]
pub fn bytes_to_str(bytes: &[u8]) -> Option<&str> {
    if bytes.is_empty() {
        None
    } else {
        let word = str::from_utf8(bytes).unwrap();
        Some(word)
    }
}

#[allow(dead_code)]
pub fn file_as_dynamic_image(filename: String) -> DynamicImage {
    let img = open(&Path::new(&filename)).unwrap();
    img
}

#[allow(dead_code)]
pub fn bytes_to_file<'a>(bytes: &[u8], mut file: &'a File) -> &'a File {
    let _ = file.write_all(bytes);
    &file
}

#[allow(dead_code)]
pub fn file_as_image_buffer(filename: String) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let img = open(&Path::new(&filename)).unwrap();
    img.to_rgba8()
}

#[allow(dead_code)]
pub fn save_image_buffer(img: ImageBuffer<Rgba<u8>, Vec<u8>>, filename: String) {
    let out_path = &Path::new(&filename);
    let _ = img.save(out_path).unwrap();
}
// ===============================================================
