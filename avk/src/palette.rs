/// A 16-color palette usable by any tile or sprite.
/// Each color is a 16-bit integer-- 4 bits per channel.
// TODO: what should the alpha channel do?
#[derive(Copy, Clone)]
pub struct Palette(pub [u16; 16]);

impl Palette {
	pub const fn empty() -> Self {
		Self([0; 16])
	}
}
