#include <stdint.h>
#include <stdbool.h>

/// Square pixel size of sprites and tiles.
static const uint16_t IMAGE_SIZE = 16;

/// Tiles per row of the canvas.
static const uint16_t CANVAS_WIDTH = 16;

/// Tiles per column of the canvas.
static const uint16_t CANVAS_HEIGHT = 12;

/// The total tile count of the canvas.
static const uintptr_t CANVAS_SIZE = ((uintptr_t)CANVAS_WIDTH * (uintptr_t)CANVAS_HEIGHT);

static const uint16_t BACKGROUND_CANVAS_WIDTH = (CANVAS_WIDTH + 2);

static const uint16_t BACKGROUND_CANVAS_HEIGHT = (CANVAS_HEIGHT + 2);

static const uintptr_t BACKGROUND_CANVAS_SIZE = ((uintptr_t)BACKGROUND_CANVAS_WIDTH * (uintptr_t)BACKGROUND_CANVAS_HEIGHT);

/// Pixels per row of the canvas.
static const uint16_t RESOLUTION_WIDTH = (IMAGE_SIZE * CANVAS_WIDTH);

/// Pixels per column of the canvas
static const uint16_t RESOLUTION_HEIGHT = (IMAGE_SIZE * CANVAS_HEIGHT);

/// The total pixel count of the canvas.
static const uintptr_t RESOLUTION_SIZE = ((uintptr_t)RESOLUTION_WIDTH * (uintptr_t)RESOLUTION_HEIGHT);

static const uintptr_t MAX_IMAGES = 256;

static const uintptr_t MAX_PALETTES = 16;

static const uintptr_t MAX_SPRITES = 96;

static const uintptr_t IMAGE_PIXEL_COUNT = ((uintptr_t)IMAGE_SIZE * (uintptr_t)IMAGE_SIZE);

enum GamepadInput {
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

    Menu,
};

enum Player {
    Alpha = 0,
    Bravo = 1,
    Charlie = 2,
    Delta = 3,
};

typedef struct Tile {
    uint8_t image_id;
    uint8_t palette_id;
} Tile;

typedef struct Sprite {
	uint8_t image_id;
	uint8_t palette_transform;
	uint16_t x;
	uint16_t y;
} Sprite;

typedef struct AvkRaw {
    void *internal;
    Tile background[252];
    Sprite foreground[96];
} AvkRaw;

typedef uint8_t Player;
typedef uint8_t GamepadInput;
typedef uint8_t Image[256];
typedef uint16_t Palette[4];

AvkRaw *avk_init(const Image images[4], const Palette palettes[4]);
void avk_drop(AvkRaw *avk);
bool avk_update(AvkRaw *avk);
uint64_t avk_get_time(AvkRaw *avk);
bool avk_get_input(const AvkRaw *avk, Player player, GamepadInput input);
