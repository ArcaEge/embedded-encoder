use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SpritesheetInitial {
    pub sprites: Vec<Sprite>,
}

/// Sprite
/// TODO: Make this more memory efficient (SpritePixels only need 2 bits of memory but currently use 8)
#[derive(Serialize, Deserialize)]
pub struct Sprite {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<SpritePixel>,
}

/// Pixel of a sprite, Black, White or Transparent
#[repr(u8)]
#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum SpritePixel {
    Black = 0,
    White = 1,
    Transparent = 2,
}
