pub mod prelude;

/// Square pixel size of sprites and tiles.
pub const IMAGE_SIZE: u16 = 16; // px

/// Tiles per row of the canvas.
pub const CANVAS_WIDTH: u16 = 16; // tiles/screen
/// Tiles per column of the canvas.
pub const CANVAS_HEIGHT: u16 = 12; // tiles/screen
/// The total tile count of the canvas.
pub const CANVAS_SIZE: usize = CANVAS_WIDTH as usize * CANVAS_HEIGHT as usize; // 192

pub const BACKGROUND_CANVAS_WIDTH: u16 = CANVAS_WIDTH + 2;
pub const BACKGROUND_CANVAS_HEIGHT: u16 = CANVAS_HEIGHT + 2;
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
// TODO: pleeeeeeeeeeeeease fix the alignment and visibility...
#[derive(Default, Copy, Clone)]
pub struct Sprite {
    /// The tile ID. Every bit is needed, as there are 256 images max
    pub image_id: u8,
    // TODO: this is bad! icky!
    /// padding (2 bits) | flip-X (1 bit) | flip-Y (1 bit) | Palette ID (4 bits)
    pub palette_transform: u8,
    pub x: u16,
    pub y: u16,
}

impl Sprite {
    pub const FLIP_X_MASK: u8 = 0b0010_0000;
    pub const FLIP_Y_MASK: u8 = 0b0001_0000;
    pub const PALETTE_MASK: u8 = 0b1111;

    pub fn get_palette_id(&self) -> u8 {
        self.palette_transform & Self::PALETTE_MASK
    }

    pub fn get_flip_x(&self) -> bool {
        self.palette_transform & Self::FLIP_X_MASK != 0
    }

    pub fn get_flip_y(&self) -> bool {
        self.palette_transform & Self::FLIP_Y_MASK != 0
    }

    pub fn set_flip_x(&mut self, flip: bool) {
        if flip {
            self.palette_transform |= Self::FLIP_X_MASK;
        } else {
            self.palette_transform &= !Self::FLIP_X_MASK;
        }
    }

    pub fn set_flip_y(&mut self, flip: bool) {
        if flip {
            self.palette_transform |= Self::FLIP_Y_MASK;
        } else {
            self.palette_transform &= !Self::FLIP_Y_MASK;
        }
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

    TriggerLeft,
    TriggerRight,

    Select,
    Start,
}

pub fn rgba_to_u16(mut rgba: [u8; 4]) -> u16 {
    if rgba[3] > 7 {
        rgba[3] = 15;
        // red
        (rgba[0] as u16) << 12 |
            // green
            (rgba[1] as u16) << 8 |
            // blue
            (rgba[2] as u16) << 4 |
            // alpha
            (rgba[3] as u16)
    } else {
        0
    }
}

pub const fn u16_to_rgba(color: u16) -> [u8; 4] {
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