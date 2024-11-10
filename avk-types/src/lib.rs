pub mod prelude;

/// Square pixel size of sprites and tiles.
pub const IMAGE_SIZE: u16 = 16; // px

/// Tiles per row of the canvas.
pub const CANVAS_WIDTH: u16 = 16; // tiles/screen
/// Tiles per column of the canvas.
pub const CANVAS_HEIGHT: u16 = 12; // tiles/screen
/// The total tile count of the canvas.
pub const CANVAS_SIZE: usize = CANVAS_WIDTH as usize * CANVAS_HEIGHT as usize; // 192

pub const BACKGROUND_CANVAS_WIDTH: u16 = 18;
pub const BACKGROUND_CANVAS_HEIGHT: u16 = 14;
pub const BACKGROUND_CANVAS_SIZE: usize =
    BACKGROUND_CANVAS_WIDTH as usize * BACKGROUND_CANVAS_HEIGHT as usize;

/// Pixels per row of the canvas.
pub const RESOLUTION_WIDTH: u16 = IMAGE_SIZE * CANVAS_WIDTH;
/// Pixels per column of the canvas
pub const RESOLUTION_HEIGHT: u16 = IMAGE_SIZE * CANVAS_HEIGHT;
/// The total pixel count of the canvas.
pub const RESOLUTION_SIZE: usize = RESOLUTION_WIDTH as usize * RESOLUTION_HEIGHT as usize;

pub const MAX_IMAGES: usize = 256;
pub const MAX_PALETTES: usize = 16;
pub const MAX_SPRITES: usize = 96;

pub enum Player {
    Alpha,
    Bravo,
    Charlie,
    Delta,
}

impl Player {
    pub fn index(&self) -> usize {
        match self {
            Player::Alpha => 0,
            Player::Bravo => 1,
            Player::Charlie => 2,
            Player::Delta => 3,
        }
    }
}

// tile_id, palette_id, x, y
#[derive(Default, Copy, Clone)]
pub struct Sprite {
    /// The tile ID. Every bit is needed, as there are 256 images max
    image_id: u8,
    /// Palette ID (4 bits) | flip-X (1 bit) | flip-Y (1 bit) | scale (1 bit) | blend (1 bit)
    palette_transform: u8,
    x: u16,
    y: u16,
}

impl Sprite {
    pub fn get_image_id(&self) -> u8 {
        self.image_id
    }
    pub fn get_palette_id(&self) -> u8 {
        self.palette_transform
    }
}

#[derive(Default, Copy, Clone)]
pub struct Tile {
    pub image_id: u8,
    pub palette_id: u8,
}

/// Image data usable by any tile or sprite.
#[derive(Copy, Clone)]
pub struct Image(pub [u8; Image::PIXEL_COUNT]);

impl Image {
    pub const PIXEL_COUNT: usize = IMAGE_SIZE as usize * IMAGE_SIZE as usize;
    pub const fn empty() -> Self {
        Self([0; Image::PIXEL_COUNT])
    }
    pub const fn from_resource(avk_res: [u8; 128]) -> Self {
        let mut this = [0; Image::PIXEL_COUNT];
        let mut i = 0;
        while i < 128 {
            this[i * 2] = avk_res[i] >> 4;
            this[i * 2 + 1] = avk_res[i] & 0b1111;
            i += 1;
        }
        Self(this)
    }
}

/// An 8-color palette usable by any tile or sprite.
/// Each color is a 16-bit integer-- 4 bits per channel.
/// TODO: what should the alpha channel do?
#[derive(Copy, Clone)]
pub struct Palette(pub [u16; 8]);

impl Palette {
    pub const fn empty() -> Self {
        Self([0; 8])
    }
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum GamepadInput {
    DirUp,
    DirRight,
    DirDown,
    DirLeft,

    FaceUp,
    FaceRight,
    FaceDown,
    FaceLeft,

    BumperLeft,
    BumperRight,

    Select,
    Start,
}