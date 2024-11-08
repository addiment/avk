use std::collections::HashMap;
use std::env::args;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use image::{DynamicImage, GenericImageView};
use avk_types::IMAGE_SIZE;
use avk_types::prelude::*;

const fn rgba_to_u16(rgba: [u8; 4]) -> u16 {
    // red
    (rgba[0] as u16) << 12 |
        // green
        (rgba[1] as u16) << 8 |
        // blue
        (rgba[2] as u16) << 4 |
        // alpha
        (rgba[3] as u16)
}

const fn u16_to_rgba(color: u16) -> [u8; 4] {
    [
        // red
        ((color & 0b1111_0000_0000_0000) >> 12) as u8,
        // green
        ((color & 0b0000_1111_0000_0000) >> 8) as u8,
        // blue
        ((color & 0b0000_0000_1111_0000) >> 4) as u8,
        // alpha
        (color & 0b0000_0000_0000_1111) as u8,
    ]
}

fn generate_image_palette(img: &[[u8; 4]; IMAGE_SIZE as usize * IMAGE_SIZE as usize], gen_palette: &mut Palette, gen_palette_iter: &mut usize) -> Option<Image> {
    let mut gen_image = Image::empty();
    let mut gen_image_iter = 0;

    for pixel in img {
        let rounded = pixel.map(
            |c| ((c as f32) / 255.0 * 15.0).round() as u8 & 0b1111
        );
        let ru16 = rgba_to_u16(rounded);

        let palette_index = gen_palette.0.iter().position(|c| *c == ru16);
        if let Some(palette_index) = palette_index {
            // update the pixel
            gen_image.0[gen_image_iter] = palette_index as u8;
            gen_image_iter += 1;
        } else if *gen_palette_iter > 15 {
            eprintln!("Too many colors in the provided image!");
            return None;
        } else {
            // add the new color to the palette
            gen_palette.0[*gen_palette_iter] = ru16;
            // update the pixel
            gen_image.0[gen_image_iter] = *gen_palette_iter as u8;
            // increase the iterators
            gen_image_iter += 1;
            *gen_palette_iter += 1;
        }
    }

    Some(gen_image)
}

fn main() {
    let args: Vec<String> = args().collect();
    let filename = Path::new(&args[1]);
    let mut img = image::open(filename).unwrap();

    if img.width() % 16 != 0 || img.height() % 16 != 0
        || img.width() < 16 || img.height() < 16
    {
        eprintln!("Image size is not a multiple of 16!");
        return;
    }

    let w = img.width() / 16;
    let h = img.height() / 16;
    let num_slices = w * h;
    println!("num_slices: {num_slices}");

    // generated palettes are shared across the entire image-- for consistency.
    let mut gp = Palette::empty();
    let mut gpi = 0;

    for i in 0..num_slices {
        let x = i % w * IMAGE_SIZE as u32;
        let y = i / w * IMAGE_SIZE as u32;
        let cropped = img.crop(
            x, y,
            IMAGE_SIZE as u32,
            IMAGE_SIZE as u32
        );
        let a: Vec<[u8; 4]> = cropped.pixels().map(|(_, _, rgba)| {
            rgba.0
        }).collect();
        println!("{x}, {y}");

        let gi = generate_image_palette(&a.try_into().unwrap(), &mut gp, &mut gpi).unwrap();
        // println!("{:?}", gp.0.map(|c| u16_to_rgba(c)));
        println!("{:?}", gp.0);

        let output_filename = String::from(filename.file_stem().unwrap().to_str().unwrap()) + &*i.to_string() + ".avkres";
        let mut output_file = File::create(output_filename).unwrap();
        // output_file.write(b"avk\0img\0").unwrap();
        output_file.write(
            gi.0
                .chunks(2)
                .map(|c| (c[0] << 4) | c[1])
                .collect::<Vec<u8>>()
                .as_slice()
            // gi.0.iter().enumerate()
        ).unwrap();
    }
}
