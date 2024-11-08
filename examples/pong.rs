use avk::prelude::*;


#[repr(u8)]
enum SpriteIndex {
	PaddleWestNorth,
	PaddleWestMid,
	PaddleWestSouth,

	PaddleEastNorth,
	PaddleEastMid,
	PaddleEastSouth,

	Ball,

	Num0,
	Num1,
	Num2,
	Num3,
	Num4,
	Num5,
	Num6,
	Num7,
	Num8,
	Num9,
}

enum Paddle {
	Left,
	Right
}

struct GameState {

}

impl GameState {
	fn update(&mut self, avk: &mut Avk) {

	}
}

fn main() {
	let mut avk = {

		let mut palette: [Palette; 16] = [Palette::empty(); 16];

		palette[0] = Palette([15, 65520, 65535, 64767, 27903, 0, 0, 0]);

		let mut images: [Image; 256] = [Image::empty(); 256];
		images[0] = Image::from_resource(*include_bytes!("paddle0.avkres"));
		images[1] = Image::from_resource(*include_bytes!("paddle1.avkres"));
		images[2] = Image::from_resource(*include_bytes!("ball0.avkres"));

		Avk::new(palette, images)
	};

	while avk.update() {
		
	}
}