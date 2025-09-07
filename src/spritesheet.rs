use super::{
    Cli,
    types::{Sprite, SpritePixel, SpritesheetInitial},
};
use clap::{CommandFactory, Error, error::ErrorKind};
use image::{GenericImageView, ImageReader, Rgba};
use postcard::to_allocvec;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// Encodes the spritesheet, not much else to say
pub fn encode_spritesheet(height: u32, input: PathBuf, output: PathBuf) -> Result<(), Error> {
    let img = ImageReader::open(&input)
        .map_err(|_| {
            Cli::command().error(
                ErrorKind::Io,
                format!("failed to open input file {:#?}", &input),
            )
        })?
        .decode()
        .map_err(|_| {
            Cli::command().error(
                ErrorKind::InvalidValue,
                "failed to decode image: invalid format",
            )
        })?;

    // Check dimensions of image, error if the image height isn't a multiple of sprite height
    let (img_width, img_height) = img.dimensions();

    println!("Using {img_width}px as sprite width");

    let height_remainder = img_height % height;
    if height_remainder != 0 {
        return Err(Cli::command().error(
            ErrorKind::InvalidValue,
            format!("invalid image: height must be a multiple of sprite height, is {height_remainder} pixel(s) too tall"),
        ));
    }

    let sprite_count = img_height / height;
    let mut spritesheet = SpritesheetInitial {
        sprites: Vec::new(),
    };

    for sprite_no in 0..sprite_count {
        let mut sprite = Sprite {
            height,
            width: img_width,
            pixels: Vec::new(),
        };
        println!("=== Sprite {sprite_no} ===");

        for y in 0..height {
            print!("{y}:  ");
            for x in 0..img_width {
                let pixel = match img.get_pixel(x, y + sprite_no * height) {
                    Rgba([_, _, _, 0]) => SpritePixel::Transparent,
                    Rgba([0, 0, 0, 255]) => SpritePixel::Black,
                    Rgba([255, 255, 255, 255]) => SpritePixel::White,
                    _ => return Err(Cli::command().error(
                        ErrorKind::InvalidValue,
                        format!("invalid pixel (coordinate {x}, {y}): pixels must be either transparent, black or white"),
                    )),
                };

                print!(
                    "{} ",
                    match pixel {
                        SpritePixel::Black => ".",
                        SpritePixel::White => "X",
                        SpritePixel::Transparent => " ",
                    }
                );

                sprite.pixels.push(pixel);
            }
            println!();
        }
        spritesheet.sprites.push(sprite);
    }

    let bytes: Vec<u8> = to_allocvec(&spritesheet).map_err(|_| {
        Cli::command().error(ErrorKind::InvalidValue, "failed to serialize sprite data")
    })?;

    let mut out_file = File::create(&output).map_err(|_| {
        Cli::command().error(
            ErrorKind::Io,
            format!("failed to open output file {:#?}", &output),
        )
    })?;

    out_file.write_all(&bytes).map_err(|_| {
        Cli::command().error(
            ErrorKind::Io,
            format!("failed to write to output file {:#?}", &output),
        )
    })?;

    println!();
    println!("Successfully wrote output to file: {:#?}", &output);

    Ok(())
}

pub fn validate_height(str: &str) -> Result<u32, String> {
    let height = str
        .parse::<u32>()
        .map_err(|_| format!("'{str}' is not a valid unsigned integer"))?;

    if height > 0 {
        Ok(height)
    } else {
        Err(String::from("height must be at least 1"))
    }
}
