use std::f32::consts::PI;
use libm::{cosf, roundf, sinf};
use avk_types::{CANVAS_HEIGHT, CANVAS_WIDTH, IMAGE_SIZE, RESOLUTION_HEIGHT, RESOLUTION_WIDTH};
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
	left_y: f32,
	right_y: f32,
}

impl GameState {
	fn update(&mut self, avk: &mut Avk) {
	}
}

fn draw_ball(avk: &mut Avk, cx: i16, cy: i16) {
	let fg = avk.get_foreground();

	let tl = &mut fg[6];
	tl.x = cx - IMAGE_SIZE;
	tl.y = cy;

	let bl = &mut fg[7];
	bl.x = cx - IMAGE_SIZE;
	bl.y = cy - IMAGE_SIZE;

	let tr = &mut fg[8];
	tr.x = cx;
	tr.y = cy;

	let br = &mut fg[9];
	br.x = cx;
	br.y = cy - 16;

}

#[no_mangle]
pub extern "C" fn avk_main() {
	let mut avk = {
		let mut palette: [Palette; 16] = [Palette::empty(); 16];
		palette[0] = Palette::empty();
		// generated with avk-convert
		palette[1] = Palette([0, 15, 65535, 64767, 27903, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
		palette[2] = Palette([0, 65535, 8751, 58927, 17487, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);


		let mut images: [Image; 256] = [Image::empty(); 256];
		images[0] = Image::empty();
		images[1] = Image::from_resource(include_bytes!("paddle0.avkres"));
		images[2] = Image::from_resource(include_bytes!("paddle1.avkres"));
		images[3] = Image::from_resource(include_bytes!("icon0.avkres"));
		images[4] = Image::from_resource(include_bytes!("icon1.avkres"));

		Avk::init(images, palette)
	};

	{
		let fg = avk.get_foreground();

		let x = 8;
		fg[0] = Sprite {
			image_id: 1,
			palette_transform: 0b0001 | Sprite::FLIP_Y_MASK,
			x,
			y: 0
		};
		fg[1] = Sprite {
			image_id: 2,
			palette_transform: 0b0001,
			x,
			y: 16,
		};
		fg[2] = Sprite {
			image_id: 1,
			palette_transform: 0b0001,
			x,
			y: 32,
		};

		let x = RESOLUTION_WIDTH - 16 - 8;
		fg[3] = Sprite {
			image_id: 1,
			palette_transform: 0b0001 | Sprite::FLIP_X_MASK |Sprite::FLIP_Y_MASK,
			x,
			y: 0
		};
		fg[4] = Sprite {
			image_id: 2,
			palette_transform: 0b0001 | Sprite::FLIP_X_MASK,
			x,
			y: 16,
		};
		fg[5] = Sprite {
			image_id: 1,
			palette_transform: 0b0001 | Sprite::FLIP_X_MASK,
			x, y: 32,
		};

		let cx = RESOLUTION_WIDTH / 2;
		let cy = RESOLUTION_HEIGHT / 2;

		// logo!
		fg[6] = Sprite {
			image_id: 3,
			palette_transform: 0b0010,
			x: cx - 16,
			y: cy,
		};
		fg[7] = Sprite {
			image_id: 4,
			palette_transform: 0b0010,
			x: cx - 16,
			y: cy - 16,
		};
		fg[8] = Sprite {
			image_id: 3,
			palette_transform: 0b0010 | Sprite::FLIP_X_MASK,
			x: cx,
			y: cy,
		};
		fg[9] = Sprite {
			image_id: 4,
			palette_transform: 0b0010 | Sprite::FLIP_X_MASK,
			x: cx,
			y: cy - 16,
		};
	}

	let mut state = GameState {
		left_y: 0.0,
		right_y: 0.0,
	};

	let mut last_time = avk.get_time();

	let min_y = 0i16;
	let max_y = RESOLUTION_HEIGHT - (16 * 3);

	while avk.update() {
		let delta = (avk.get_time() - last_time) as f32 / 1000.0;
		last_time = avk.get_time();
		// let delta = 1.0;

		let s_time = avk.get_time() as f32 / 4000.0;
		let alpha_up = avk.get_input(Player::Alpha, GamepadInput::DirUp);
		let alpha_down = avk.get_input(Player::Alpha, GamepadInput::DirDown);

		let bravo_up = avk.get_input(Player::Bravo, GamepadInput::DirUp);
		let bravo_down = avk.get_input(Player::Bravo, GamepadInput::DirDown);

		let speed = 256.0;

		state.left_y += if alpha_up { 1.0 } else { 0.0 } * delta * speed;
		state.left_y += if alpha_down { -1.0 } else { 0.0 } * delta * speed;
		state.right_y += if bravo_up { 1.0 } else { 0.0 } * delta * speed;
		state.right_y += if bravo_down { -1.0 } else { 0.0 } * delta * speed;

		state.left_y = state.left_y.clamp(min_y as f32, max_y as f32);
		state.right_y = state.right_y.clamp(min_y as f32, max_y as f32);

		let fg = avk.get_foreground();

		let ly = roundf(state.left_y) as i16;
		let ry = roundf(state.right_y) as i16;

		fg[0].y = ly;
		fg[1].y = ly + 16;
		fg[2].y = ly + 32;

		fg[3].y = ry;
		fg[4].y = ry + 16;
		fg[5].y = ry + 32;


		let cx = (cosf(s_time * 2.0 * PI) * 32.0) as i16 + RESOLUTION_WIDTH / 2;
		let cy = (sinf(s_time * 2.0 * PI) * 32.0) as i16 + RESOLUTION_HEIGHT / 2;
		draw_ball(&mut avk, cx, cy);
	}
}