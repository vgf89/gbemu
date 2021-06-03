// 8KB memory.VRAM
// 160B memory.OAM

#include "ppu.h"
#include "memory.h"
#include <stdlib.h>
#include <stdio.h>

Color LCD[144][160];
extern union memory_t memory;

extern uint32_t clock;
uint32_t ppuclock;

uint8_t line = 0;

uint8_t mode = 2;
/* Modes:
2: 80 dots.   Searching OAM for OBJs that render on this line. VRAM unlocked.
3: 168-291 dots depending on sprite count. Reading OAM and VRAM to generate picture. Locks VRAM, OAM, CGB Palette.
0: 85-208 dots. depending on mode 3. Horizontal Blanking. VRAM, OAM, CGB Palette unlocked.
1: 4560 dots (10 scanlines). Vertical Blanking. VRAM, OAM, CGB Palette unlocked

Total dot count: (80 + 376) * 144 + 4560 = 70224
*/


Color tiles_pixels1[192][128];

void init_ppu()
{
}

uint8_t lastLYCStatInterruptFlag = 0;
uint8_t lastMode0InterruptFlag = 0;
uint8_t lastMode1InterruptFlag = 0;
uint8_t lastMode2InterruptFlag = 0;

void reset_ppu_clock(uint16_t maxclock)
{
    ppuclock -= maxclock;
}

void updateLCDStatus()
{
    uint8_t result = 0;
    result |= mode;
    if (LY == LYC) {
        result |= (1 << 2);
    }
    result |= STAT & 0x3c;

    writeByte(0xff41, result);

    // TODO: Trigger STAT interrupts : Check for changes to interrupt sources in cpu.c

}

// Returns an array of pixels. 128x192 (16x24 tiles)
Color* TileData()
{
    
    // VRAM Tile Data:
    // Block    VRAM Address        Corresponding Tile IDs
    //                              OBJs        BG/Win if LCDC.4=1      BG/Win if LCDC.4=0
    // 0        $8000–$87FF 	    0–127 	    0–127
    // 1 	    $8800–$8FFF         128–255     128–255                 128–255 (or -127–0) 
    // 2        $9000–$97FF         (          Can't use         )      0-127
    
    // Each tile occupied 16 bytes, where each line is represented by 2 bytes:
    // Byte 0-1  Topmost line (Top 8 pixels)
    // Byte 2-3  Second Line
    // etc...
    // Each pixel is defined by 2 bits, stacked on top of each other

    // Loop tiles
    for (uint16_t tile = 0; tile < 0x180; tile++) {
        uint16_t tile_x = (tile % 16)*8; // top-left position of tile in bg_render
        uint16_t tile_y = (tile / 16)*8;
        // Loop y (2 bytes at a time)
        for (uint16_t y = 0; y < 8; y++) {
            uint8_t byte1 = memory.memory[0x8000 + tile * 16 + y * 2];
            uint8_t byte2 = memory.memory[0x8000 + tile * 16 + y * 2 + 1];
            // Loop x bits
            for (uint8_t x = 0; x < 8; x++) {
                char value = (((byte1 & (1 << (7 - x)))?0:1) << 1) | (((byte2 & (1 << (7 - x)))?0:1));
                value *= 85; // Map values to simple rgb values

                // TODO: Palette implementation

                uint16_t finalx = tile_x + x;
                uint16_t finaly = tile_y + y;
                
                tiles_pixels1[finaly][finalx].a = 255;
                tiles_pixels1[finaly][finalx].r = value;
                tiles_pixels1[finaly][finalx].g = value;
                tiles_pixels1[finaly][finalx].b = value;
            }
        }
    }

    return (Color*)tiles_pixels1;
}

Color bg_render[256][256] = {0};
// Background maps are 32x32 tiles. Memory area $9800-$9BFF or $9C00-$9FFF
// Returns pixel array of 256x256
Color* BG()
{
    for (uint16_t tile = 0; tile < 0x400; tile++) {
        uint16_t tile_x = (tile % 32)*8; // top-left position of tile in bg_render
        uint16_t tile_y = (tile / 32)*8;

        uint16_t BG_tile_map_area = (LCDC & (1 << 3))?0x9c00:0x9800; // tile map area
        uint16_t tile_address;
        if (LCDC & (1<<4)) { // tile data area address mode
            tile_address = memory.memory[BG_tile_map_area + tile] * 16 + 0x8000;
        } else {
            int8_t stile = (int8_t)memory.memory[BG_tile_map_area + tile];
            tile_address = stile * 16 + 0x9000;
        }

        for (uint16_t y = 0; y < 8; y++) { // 
            uint8_t byte1 = memory.memory[tile_address + y * 2];
            uint8_t byte2 = memory.memory[tile_address + y * 2 + 1];
            for (uint8_t x = 0; x < 8; x++) {
                char value = (((byte1 & (1 << (7 - x)))?0:1) << 1) | (((byte2 & (1 << (7 - x)))?0:1));
                value *= 85; // Map values to simple rgb values

                // TODO: Palette Implementation

                uint16_t finalx = tile_x + x;
                uint16_t finaly = tile_y + y;
                
                bg_render[finaly][finalx].a = 255;
                bg_render[finaly][finalx].r = value;
                bg_render[finaly][finalx].g = value;
                bg_render[finaly][finalx].b = value;
            }
        }
    }

    return (Color*)bg_render;
}

uint8_t TileMaps_Textures[2][256][256] = {0};
uint8_t*** TileMaps()
{
    return NULL;
}

void ppuStep()
{
    if (clock < ppuclock)
    {
        return;
    }
    line = readByte(0xff44);

    switch(mode) {
        case 2: // OAM search for sprite indexes to render
            ppuclock += 20; // early return from ppuStep() until line rendering is ready to start
            mode = 3;
        break;
        case 3: // Rendering line
            ppuclock += 72; // FIXME: This is not accurate.
            mode = 0;
        break;
        case 0:              // Horizontal Blanking
            ppuclock += 22; // FIXME: this is not accurate
            if (line == 143) {
                mode = 1;
            }

            writeByte(0xff44, ++line); //
            // Set HBlank interrupt/register/whatever
            // Set ppuclock to end of Mode 0 time
        break;
        case 1: // Vertical Blanking. Similar to horizontal blanking, but long.
            // Spit out the final image
            // Set VBlank interrupt etc
            if (line == 144) {
                IF_SET(I_VBLANK);
                printf("vblank   ");
            }

            ppuclock += 114;

            if (line == 153) {
                mode = 2;
                line = 0;
                printf("((gb vblank, ppuclock = %d ))", ppuclock);
            }
            writeByte(0xff44, ++line);
        break;
    }

    updateLCDStatus();

    // Frames are defined by a few things...
    // Tiles:
        // 8x8 bitmaps in VRAM Tile Pattern Table
        // Tile Map (256 tiles)
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


    // TODO: Debugging visualizations:
        // Tile Pattern Table
        // Background
        // Sprites
}

// Tiles:
    // Each 8x8 pixels
        // Each pixel is 1 of 4 colors:
        // 0b00 == white
        // 0b01 = light gray
        // 0b10 = dark grey
        // 0b11 = black
        // Palette mapping lets this be changed, colors can be reused as well. Good for effects

    // Tile storage:
    // Pixel color is defined vertically? https://www.youtube.com/watch?v=HyzD8pNlpwI&t=1725s
    // i.e....
        // 0 0 0 0 0 0 1 0
        // 1 1 1 1 1 1 1 1  //One horizontal line of pixels of light gray, exept bit 6 which is white

//

// Background Layer
// Background Tile Data
    // 256 background tiles
// Background Map
    // 32x32 tile indexes
    // Viewable portion is controlled by SCY and SCX
    // Scrolling wraps around the edges of the map

// Window layer
    // No translucency
    // enabled by 0xff40.5
    // position controlled by WY and WX

// Sprites
// Enabled by 0xff40.1
// Sprite size: 1 or 2 (vertically stacked 8x16) tiles
// OAM Entry:
// 0x0: Position X :  0x__   NOTE: sprite origin is bottom-right
// 0x1: Position Y :  0x__
// 0x2: Tile Number : 0x__
// 0x3.7: Priority  0b_  // 0: Draw on top of everything. 1: Draws on top of BG color 0b00, but not other colors (except for transparent pixels)
// 0x3.6: Flip Y
// 0x3.5: Flip X
// 0x3.4: Palette
// 0x3.3: Tile VRAM Bank (CGB only)
// 0x3.2-0: Palette number (CGB Only)
// color 0b00 is translucent
// 256 sprite tiles



/* LCDC  LCD Control ($FF40)
Bit Name                            Usage notes
7   LCD and PPU enable	            0=Off, 1=On
6   Window tile map area	        0=9800-9BFF, 1=9C00-9FFF
5   Window enable	                0=Off, 1=On
4   BG and Window tile data area	0=8800-97FF, 1=8000-8FFF
3   BG tile map area	            0=9800-9BFF, 1=9C00-9FFF
2   OBJ size	                    0=8x8, 1=8x16
1   OBJ enable	                    0=Off, 1=On
0   BG and Window enable/priority	0=Off, 1=On
*/

/* STAT  LCD Status ($FF41)
Bit 6 - LYC=LY STAT Interrupt source         (1=Enable) (Read/Write)
Bit 5 - Mode 2 OAM STAT Interrupt source     (1=Enable) (Read/Write)
Bit 4 - Mode 1 VBlank STAT Interrupt source  (1=Enable) (Read/Write)
Bit 3 - Mode 0 HBlank STAT Interrupt source  (1=Enable) (Read/Write)
Bit 2 - LYC=LY Flag                          (0=Different, 1=Equal) (Read Only)
Bit 1-0 - Mode Flag                          (Mode 0-3, see below) (Read Only)
          0: In HBlank
          1: In VBlank
          2: Searching OAM
          3: Transferring Data to LCD Controller
*/

/* SCY  Scroll Y ($FF42) R/W */

/* SCX  Scroll X ($FF43) R/W */

/* LY  LCDC Y-Coordinate ($FF44) R */ /* If this isn't implemented we get a loop */