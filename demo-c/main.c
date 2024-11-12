#include <stdio.h>
#include "../avk.h"

AvkRaw *init_rom() {
    Image images[MAX_IMAGES];
    Palette palettes[MAX_PALETTES];

    AvkRaw *avk = avk_init(images, palettes);
    return avk;
}

AVK_EXPORT void avk_main() {
    AvkRaw *avk = init_rom();

    while (avk_update(avk)) {
        puts("hi!");
    }
}