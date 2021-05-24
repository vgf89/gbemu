// 8KB memory.VRAM
// 160B memory.OAM

#include "ppu.h"

uint8_t LCD[144][160];

extern uint32_t clock;
uint32_t ppuclock;

void reset_ppu_clock(uint16_t maxclock)
{
    ppuclock -= maxclock;
}

void ppuStep()
{
    if (clock < ppuclock)
    {
        return;
    }
    ppuclock += 2; // Minimum clock dots per ppu cycle

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