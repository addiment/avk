use avk::prelude::*;
use crate::SpriteIndex::PaddleEastNorth;

#[repr(u8)]
enum ImageIndex {
	Paddle0
}

#[repr(u8)]
enum SpriteIndex {
	PaddleWestNorth = 1,
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

		palette[0] = Palette::empty();
		// generated with avk-convert
		palette[1] = Palette([0, 15, 65535, 64767, 27903, 0, 0, 0]);
		// palette[0] = Palette([65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535]);

		let mut images: [Image; 256] = [Image::empty(); 256];
		images[1] = Image::from_resource(*include_bytes!("paddle0.avkres"));
		images[2] = Image::from_resource(*include_bytes!("paddle1.avkres"));
		images[3] = Image::from_resource(*include_bytes!("ball0.avkres"));

		Avk::init(images, palette)
	};

	avk.foreground[0] = Sprite {
		image_id: 1,
		palette_transform: 1,
		x: 0,
		y: 0
	};
	avk.foreground[0].set_flip_y(true);
	avk.foreground[1] = Sprite {
		image_id: 2,
		palette_transform: 1,
		x: 0,
		y: 16,
	};
	avk.foreground[2] = Sprite {
		image_id: 1,
		palette_transform: 1,
		x: 0,
		y: 32,
	};

	while avk.update() {
		let input = avk.get_input(Player::Alpha);
		let s_time = (((avk.get_time() as f32 / 1000.0).sin() + 1.0) * (192.0 - (16.0 * 3.0)) / 2.0).round() as u16;
		// let s_time = 192 - (16 * 3);
		avk.foreground[0].y = s_time;
		avk.foreground[1].y = s_time + 16;
		avk.foreground[2].y = s_time + 32;
	}
}