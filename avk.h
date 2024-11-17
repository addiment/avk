#ifndef AVK_H
#define AVK_H
#include <stdint.h>
#include <stdbool.h>

#define AVK_EXPORT __attribute__(( visibility("default") ))

/// Square pixel size of sprites and tiles.
static const int16_t IMAGE_SIZE = 16;

/// Tiles per row of the canvas.
static const int16_t CANVAS_WIDTH = 16;

/// Tiles per column of the canvas.
static const int16_t CANVAS_HEIGHT = 12;

/// The total tile count of the canvas.
static const uintptr_t CANVAS_SIZE = ((uintptr_t)CANVAS_WIDTH * (uintptr_t)CANVAS_HEIGHT);

static const int16_t BACKGROUND_CANVAS_WIDTH = (CANVAS_WIDTH + 2);

static const int16_t BACKGROUND_CANVAS_HEIGHT = (CANVAS_HEIGHT + 2);

static const uintptr_t BACKGROUND_CANVAS_SIZE = ((uintptr_t)BACKGROUND_CANVAS_WIDTH * (uintptr_t)BACKGROUND_CANVAS_HEIGHT);

/// Pixels per row of the canvas.
static const int16_t RESOLUTION_WIDTH = (IMAGE_SIZE * CANVAS_WIDTH);

/// Pixels per column of the canvas
static const int16_t RESOLUTION_HEIGHT = (IMAGE_SIZE * CANVAS_HEIGHT);

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
	int16_t x;
	int16_t y;
} Sprite;

typedef struct AvkRaw {
    void *internal;
    Tile background[252];
    Sprite foreground[96];
    int8_t pan_x,
    int8_t pan_y,
} AvkRaw;

typedef uint8_t Image[256];
typedef uint16_t Palette[4];

// Function pointers to be filled in  by the AVK loader.
AVK_EXPORT void *AVK_INIT = 0;
AVK_EXPORT void *AVK_DROP = 0;
AVK_EXPORT void *AVK_UPDATE = 0;
AVK_EXPORT void *AVK_GET_TIME = 0;
AVK_EXPORT void *AVK_GET_INPUT = 0;

inline static AvkRaw *avk_init(const Image images[4], const Palette palettes[4]) {
    AvkRaw *(*fp)(const Image[4], const Palette[4]) = (AvkRaw *(*)(const Image *, const Palette *))AVK_INIT;
    return fp(images, palettes);
}

inline static void avk_drop(AvkRaw *avk) {
    void (*fp)(AvkRaw *avk) = (void (*)(AvkRaw *avk))AVK_DROP;
    return fp(avk);
}

inline static bool avk_update(AvkRaw *avk) {
    bool (*fp)(AvkRaw *avk) = (bool (*)(AvkRaw *avk))AVK_UPDATE;
    return fp(avk);
}

inline static uint64_t avk_get_time(const AvkRaw *avk) {
    uint64_t (*fp)(const AvkRaw *avk) = (uint64_t (*)(const AvkRaw *avk))AVK_GET_TIME;
    return fp(avk);
}

inline static bool avk_get_input(const AvkRaw *avk, enum Player player, enum GamepadInput input) {
    bool (*fp)(const AvkRaw*, enum Player, enum GamepadInput) = (bool (*)(const AvkRaw*, enum Player, enum GamepadInput))AVK_GET_INPUT;
    return fp(avk, player, input);
}

#endif // AVK_H