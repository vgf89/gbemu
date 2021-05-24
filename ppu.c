// 8KB memory.VRAM
// 160B memory.OAM


#include "ppu.h"

uint8_t LCD[144][160];
// 12B LCD registers memory.LCD
  
void ppuStep() {
    // Frames are defined by a few things...
    // Tiles:
        // 8x8 bitmaps in VRAM Tile Pattern Table
        // Tile Map
    // Background:
        // 32x32 tiles map (256x256 pixel)
        // Selected 160x144 rendering box
    // Window:
        // Used to show mostly static data (HUD, etc)
        // Static 20x18 tiles map, which draws over everything else
        // Visibility on different parts of screen controlled by LCDCONT register
    // Sprites
        // Tiles (objects) that can move independently.
        // Object Attribute Memory OAM stores which tiles will be used as sprites
        // Each entry in OAM specifies the following:
            // Tile index
            // X,Y position
            // Color palette
            // Priority (is this for order, or something else?)
            // Horizontal Vertical flip flags
        // Maximum of 10 sprites per scanline, 40 sprites per screen. Additional sprites are not rendered
    
    // Additional notes:
        // PPU spits out the Horizontal Blank Interrupt upon finishing a line.
        // PPU spits out Vertical Blank Interrupt upon finishing a line.
    

    // TODO: Implmement master clock/timing basics.
    // TODO: Implement CPU interrupts

    /* https://forums.nesdev.com/viewtopic.php?t=17754
    The Game Boy CPU and PPU run in parallel.
    The 4.2 MHz master clock is also the dot clock.
    It's divided by 2 to form the PPU's 2.1 MHz memory access clock,
    and divided by 4 to form a multi-phase 1.05 MHz clock used by the CPU.

    Each scanline is 456 dots (114 CPU cycles) long and consists of
    mode 2 (OAM search)
    mode 3 (active picture)
    and mode 0 (horizontal blanking).

    Mode 2 is 80 dots long (2 for each OAM entry),
    mode 3 is about 168 plus about 10 more for each sprite on a given line,
    and mode 0 is the rest.
    
    After 144 scanlines are drawn are 10 lines of mode 1 (vertical blanking),
    for a total of 154 lines or 70224 dots per screen.

    The CPU can't see VRAM (writes are ignored and reads are $FF) during mode 3,
    but it can during other modes.

    The CPU can't see OAM during modes 2 and 3,
    but it can during blanking modes (0 and 1).

    To make the estimate for mode 3 more precise,
    see "Nitty Gritty Gameboy Cycle Timing" by Kevin Horton.
    */
}