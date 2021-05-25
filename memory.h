#pragma once
#ifndef MEMORY_H
#define MEMORY_H

#include <stdint.h>

// Memory Map
union memory_t {
    struct{
        uint8_t ROM00[0x4000];   // $0000-$3fff 16 KiB ROM Bank 00. Usually fixed
        uint8_t ROMNN[0x4000];   // $4000-$7fff 16 KiB ROM Bank NN. Switchable via mapper (if any)
        union {
            struct {
                uint8_t TILEDATA_B0[0x800]; // $8000-$87FF, OBJs   0-127, BG/Win = (LCDC.4) ? 0-127 : null
                uint8_t TILEDATA_B1[0x800]; // $8800-$8FFF, OBJs 128-255, BG/Win = (LCDC.4) ? 0-127 : 128-255 (or -127-0)
                uint8_t TILEDATA_B2[0x800];  //$9000-$97FF, (Can't use                 ), if BG/Win if LCDC.4=0  is 0-127
                // NOTE: Sprites are always addressed with a base pointer of $8000. BG and Window tiles can also use $8800 as a base pointer, depending on LCDC.4
            };
            uint8_t VRAM[0x2000];    // $8000-$9fff 8KiB Video RAM
        };
        uint8_t EXRAM[0x2000];   // $a000-$bfff 8KiB External RAM
        uint8_t WRAM1[0x1000];   // $c000-$cfff 4KiB Work RAM
        uint8_t WRAM2[0x1000];   // $d000-$dfff 4KiB Work RAM (switchable banks 1~7 on CGB)
        uint8_t ECHORAM[0x1E00]; // $e000-$fdff An artifact of how the bus is connected. Mirrors C000~DDFF. Nintendo says use of this area is prohibited.
                                       // For accuracy, we can remap reads/writes from this location to C000~DFFF
        uint8_t OAM[0x00A0];     // $fe00-$fe9f Sprite Attribute Table
        uint8_t unusable[0x0060];// $fea0-$feff do not touch, just leave it blank unless needed
        union{
            struct{
                uint8_t controller[0x0001];
                uint8_t communication[0x0002];
                uint8_t gap0[0x0001];
                uint8_t dividerAndTimer[0x0004];
                uint8_t gap1[0x0008];
                uint8_t sound[0x0017];
                uint8_t gap2[0x0009];
                uint8_t waveformRAM[0x0010];
                uint8_t LCD[0x000C]; // $ff40-$ff4b
                uint8_t gap3[0x0003];
                uint8_t VRAMBankSelect[0x0001]; // CGB
                uint8_t DisableBootRom[0x0001];
                uint8_t HDMA[0x0017]; // CGB
                uint8_t gap4[0x000C];
                uint8_t BCPOCP[0x0002]; // CGB
                uint8_t WRAMBankSelect[0x0001]; // CGB
                uint8_t gap5[0x000F];
            };
            uint8_t IO[0x0080]; // $f000-$ff7f IO Registers
        };
        
        uint8_t HRAM[0x007F]; // $ff80-$fffe High RAM
        uint8_t IE[0x0001]; // $ffff Interrupts Enable Register
    };
    uint8_t memory[0x10000]; // union
};

uint8_t readChar(uint16_t address);
uint16_t readShort(uint16_t address);

void writeChar(uint16_t address, uint8_t val);
void writeShort(uint16_t address, uint16_t val);

#endif