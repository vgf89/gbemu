// gameboy.c high level interface for the emulated gameboy.
// Handles high level user actions such as loading roms, resetting
// emulation, and providing the final image in the desired format.
#include <stdio.h>
#include <stdlib.h>
#include "gameboy.h"
#include "memory.h"
#include "cpu.h"
#include "ppu.h"
#include "timer.h"
#include "input.h"

// externs
extern struct registers_t registers;
extern union memory_t memory;
extern Color LCD[144][160];


void loadRom(char* rompath)
{
    // Load first ROM bank
    FILE* fp;
    fp = fopen(rompath, "r");
    fread(memory.ROM00, 0x8000, 1, fp);
    // Detect cartridge type (only support ROM ONLY for now)
    switch(memory.ROM00[0x0147]) {
        // ROM Only (read in Bank 1)
        case 0x00:
            fseek(fp, 0x8000, 0);
            fread(memory.ROMNN, 0x8000, 1, fp);
            break;
        default:
            printf("Cartidge type 0x%02X not supported");
            exit(1);
    }
}

void reset()
{
    // BOOT
    registers.af = 0x01b0;
    registers.bc = 0x0013;
    registers.de = 0x00d8;
    registers.hl = 0x014d;
    registers.sp = 0xfffe;

    memory.memory[0xff05] = 0x00;
    memory.memory[0xff06] = 0x00;
    memory.memory[0xff07] = 0x00;
    memory.memory[0xff10] = 0x80;
    memory.memory[0xff11] = 0xbf;
    memory.memory[0xff12] = 0xf3;
    memory.memory[0xff14] = 0xbf;
    memory.memory[0xff16] = 0x3f;
    memory.memory[0xff17] = 0x00;
    memory.memory[0xff19] = 0xbf;
    memory.memory[0xff1a] = 0x7f;
    memory.memory[0xff1b] = 0xff;
    memory.memory[0xff1c] = 0x9f;
    memory.memory[0xff1e] = 0xbf;
    memory.memory[0xff20] = 0xff;
    memory.memory[0xff21] = 0x00;
    memory.memory[0xff22] = 0x00;
    memory.memory[0xff23] = 0xbf;
    memory.memory[0xff24] = 0x77;
    memory.memory[0xff25] = 0xf3;
    memory.memory[0xff26] = 0xf1-1; //what does this mean [$FF26] = $F1-GB, $F0-SGB ; NR52
    memory.memory[0xff40] = 0x91;
    memory.memory[0xff42] = 0x00;
    memory.memory[0xff43] = 0x00;
    memory.memory[0xff45] = 0x00;
    memory.memory[0xff47] = 0xfc;
    memory.memory[0xff48] = 0xff;
    memory.memory[0xff49] = 0xff;
    memory.memory[0xff4a] = 0x00;
    memory.memory[0xff4b] = 0x00;
    memory.memory[0xffff] = 0x00;

    registers.pc = 0x0100;

    // Other stuff
    memory.memory[0xff00] = 0xff; // set default joypad to nothing pressed

    init_ppu();
}


//const int MAX_CLOCK = 70224; // Number of cycles per frame
const int MAX_CLOCK = 70224; // Number of cycles per frame
// Magic number for 60FPS. 4194304 cycles per second / 60 = 69905
// Possible rates:
    // DMG: 69905
    // Super Game Boy 71590 (plus this one's divisible by 4), which might result in frame timing match?
    // PPU VBLANK rate: 70224, which should force emulated gameboy vblank to match target framerate (typically 60fps)

// There are likely better ways to handle this, i.e. just waiting for VBLANK interrupt to break the step loop.
// If VBLANK timing is consistent though, we should be breaking based on that timing.

uint32_t clock = 0;

int frames = 0;

void step() {
    // How to schedule CPU steps:
    // Number of dots (clock edges) per frame (60fps): 70000
    // CPU cycle is 4 dots
    // PPU cycle is 2 dots
    // CPU and PPU use their own cycle counters, and compare
    // to the master dot clock to schedule execution.
    

    clock = 0;

    while (clock < MAX_CLOCK)
    {
        updateInput();
        cpuStep();
        timerStep();
        ppuStep();
        clock++;
    }
    frames++;
    //printf("%d clock cycles completed!  frame/60: %f\n", MAX_CLOCK, (float)frames / 60.0);

    // render graphics, debug, etc from ppu to textures

    // Reset the clocks so counting can restart. Actually, these subtract the MAX_CLOCK from the internal clock trackers so we introduce extra cycles.
    reset_cpu_clock(MAX_CLOCK);
    reset_ppu_clock(MAX_CLOCK);
}