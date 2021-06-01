#pragma once
#ifndef PPU_H
#define PPU_H

#include <stdint.h>
#include "raylib.h"

// Registers in memory
#define LCDC memory.memory[0xff40] // LCD Control (R/W)
#define LCDC_DISPLAY_ENABLE (1 << 7)
#define LCDC_WINDOW_TILE_MAP_ADDRESS (1 << 6)
#define LCDC_WINDOW_ENABLE (1 << 5)
#define LCDC_BG_AND_WINDOW_TILE_MAP_DATA (1 << 4)
#define LCDC_BG_TILE_MAP_ADDRESS (1 << 3)
#define LCDC_OBJ_SIZE (1 << 2)
#define LCDC_OBJ_ENABLE (1 << 1)
#define LCDC_BG_ENABLE (1 << 0)
// Example:
// if (LCDC & LCDC_DISPLAY_ENABLE) {
//     turn on display code
// }

#define STAT memory.memory[0xff41] // LCDC Status (R/W)
#define STAT_LYC_EQ_LY_INTERRUPT (1 << 6)
#define STAT_MODE2_OAM_INTERRPUPT (1 << 5)
#define STAT_MODE1_VBLANK_INTERRUPT (1 << 4)
#define STAT_MODE0_HBLANK_INTERRUPT (1 << 3)
#define STAT_LYC_EQ_LY_FLAG (1 << 2)
#define STAT_MODE (0x3)

#define SCY memory.memory[0xff42]  // Scroll Y (R/W)
#define SCX memory.memory[0xff43]  // Scroll X (R/W)
#define LY memory.memory[0xff44]   // LCDC Y-Coordinate (R)
#define LYC memory.memory[0xff45]  // LY Compare (R/W)
#define DMA memory.memory[0xff46]  // DMA Transfer and Start
#define BGP memory.memory[0xff47]  // BG Palette (R/W)
#define OBP0 memory.memory[0xff48] // Object Palette 0 (R/W)
#define OBP1 memory.memory[0xff49] // Objct Palette 1 (R/w)
#define WY memory.memory[0xff4a]   // Window Y Position (R/W)
#define WX memory.memory[0xff4b]   // Window X Position (R/W)




extern uint8_t LCD[144][160];


void reset_ppu_clock(uint16_t maxclock);

void ppuStep();

void init_ppu();

Color* TileData();
Color* BG();

#endif