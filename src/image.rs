// use std::fmt;

// pub struct Image {
//     width: usize,
//     height: usize,
//     pixels: Vec<u8>,
// }

// impl Image {
//     pub fn new(width: usize, height: usize, pixels: Vec<u8>) -> Self {
//         Image {
//             width,
//             height,
//             pixels,
//         }
//     }
//     pub fn save(&self, path: &str) -> Result<(), std::io::Error> {
//         //image::save_buffer(path, &self.pixels, self.width, self.height, image::ColorType::Rgb8)?;
//         unimplemented!()
//     }

//     pub fn compress_rle(&self) -> Vec<(u8, u32)> {
//         run_length_encode(&self.pixels)
//     }

//     pub fn decompress_rle(encoded: &[(u8, u32)]) -> Self {
//         let pixels = run_length_decode(encoded);
//         Image {
//             width: 0,
//             height: 0,
//             pixels,
//         }
//     }
//     pub fn compress_dct(&self, quality: f64) -> Vec<f64> {
//         compress_image(&self.pixels, self.width, self.height, quality)
//     }
//     pub fn decompress_dct(compressed: &[f64], width: usize, height: usize) -> Self {
//         let pixels = decompress_image(compressed, width, height);
//         Image {
//             width,
//             height,
//             pixels: pixels.iter().map(|&x| x as u8).collect(),
//         }
//     }
// }

// fn run_length_encode(pixels: &[u8]) -> Vec<(u8, u32)> {
//     //basically check if current pixel is same as prev pixel
//     //if yes, add count to u32, if no, push and go to next pixel
//     let mut encoded = Vec::new();
//     let mut count = 1;
//     let mut prev_pixel = pixels[0];
//     for &pixel in pixels.iter().skip(1) {
//         if pixel == prev_pixel {
//             count += 1;
//         } else {
//             encoded.push((prev_pixel, count));
//             prev_pixel = pixel;
//             count = 1;
//         }
//     }
//     encoded.push((prev_pixel, count));
//     encoded
// }

// fn run_length_decode(encoded: &[(u8, u32)]) -> Vec<u8> {
//     //fill count amt of pixels with u8 color
//     let mut decoded = Vec::new();
//     for &(pixel,count) in encoded {
//         decoded.extend(std::iter::repeat(pixel).take(count as usize));
//     }
//     decoded
// }

// fn compress_image(pixels: &[u8], width: usize, height: usize, quality: f64) -> Vec<f64> {
//     // Implementation of DCT compression
//     // ...
// }

// fn decompress_image(compressed: &[f64], width: usize, height: usize) -> Vec<f64> {
//     // Implementation of DCT decompression
//     // ...
// }