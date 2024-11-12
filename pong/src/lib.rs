use avk_types::prelude::*;

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

#[no_mangle]
pub extern "C" fn avk_main() {
	let mut avk = {
		let mut palette: [Palette; 16] = [Palette::empty(); 16];
		palette[0] = Palette::empty();
		// generated with avk-convert
		palette[1] = Palette([0, 15, 65535, 64767, 27903, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

		let mut images: [Image; 256] = [Image::empty(); 256];
		images[0] = Image::empty();
		images[1] = Image::from_resource(include_bytes!("paddle0.avkres"));
		images[2] = Image::from_resource(include_bytes!("paddle1.avkres"));
		images[3] = Image::from_resource(include_bytes!("ball0.avkres"));

		Avk::init(images, palette)
	};

	let fg = avk.get_foreground();
	fg[0] = Sprite {
		image_id: 1,
		palette_transform: 0b0001 | Sprite::FLIP_Y_MASK,
		x: 0,
		y: 0
	};
	fg[1] = Sprite {
		image_id: 2,
		palette_transform: 0b0001,
		x: 0,
		y: 16,
	};
	fg[2] = Sprite {
		image_id: 1,
		palette_transform: 0b0001,
		x: 0,
		y: 32,
	};

	while avk.update() {
		let s_time = libm::roundf((libm::sinf(avk.get_time() as f32 / 1000.0) + 1.0) * (192.0 - (16.0 * 3.0)) / 2.0) as u16;
		let fg = avk.get_foreground();
		fg[0].y = s_time;
		fg[1].y = s_time + 16;
		fg[2].y = s_time + 32;
	}
}