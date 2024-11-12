#include <stdio.h>
#include "../avk.h"

AVK_EXPORT void avk_main() {
    Image images[MAX_IMAGES];
    Palette palettes[MAX_PALETTES];

    AvkRaw *avk = avk_init(images, palettes);

    while (avk_update(avk)) {
        puts("hi!");
    }
}