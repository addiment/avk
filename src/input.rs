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