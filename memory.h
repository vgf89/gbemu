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
                uint8_t JOYPAD; // $ff00
                uint8_t SIODATA; // $ff01 [RW] Serial I/O Data
                uint8_t SIOCONT; // $ff02 [RW] Serial I/O Control
                uint8_t gap0; // $ff03
                uint8_t DIV; // $ff04 [RW] Unconditional counter register (increases every 256 system clock)
                uint8_t TIMA; // $ff05 [RW] Timer Counter (constantly counts up, triggers timer interrupt on overflow)
                uint8_t TMA; // $ff06 [RW] Timer Modulo (loaded into counter whenever counter overflows)
                uint8_t TAC; // $ff07 [RW] Timer Control
                uint8_t gap1[0x0007]; // $ff08-ff0e
                uint8_t IFLAGS; // $ff0f [RW] Interrupt Flags
                uint8_t sound[0x0017]; // $ff10-ff26
                uint8_t gap2[0x0003]; // $ff27-29
                uint8_t waveformRAM[0x0010]; // $FF30-$FF3F
                uint8_t LCD[0x000C]; // $ff40-$ff4b
                uint8_t gap3[0x0003];
                uint8_t VRAMBankSelect; // CGB
                uint8_t DisableBootRom;
                uint8_t HDMA[0x0017]; // CGB
                uint8_t gap4[0x000C];
                uint8_t BCPOCP[0x0002]; // CGB
                uint8_t WRAMBankSelect; // CGB
                uint8_t gap5[0x000F];
            };
            uint8_t IO[0x0080]; // $ff00-$ff7f IO Registers
        };
        
        uint8_t HRAM[0x007F]; // $ff80-$fffe High RAM
        uint8_t IE; // $ffff Interrupts Enable Register
    };
    uint8_t memory[0x10000]; // union
};

#define I_VBLANK (1)
#define I_LCD_STAT (1 << 1)
#define I_TIMER (1 << 2)
#define I_SERIAL (1 << 3)
#define I_JOYPAD (1 << 4)

#define IE_ISSET(x) (memory.IE & (x))
#define IE_SET(x) (memory.IE |= (x))
#define IE_CLEAR(x) (memory.IE &= ~(x))

#define IF_ISSET(x) (memory.IFLAGS & (x))
#define IF_SET(x) (memory.IFLAGS |= (x))
#define IF_CLEAR(x) (memory.IFLAGS &= ~(x))

uint8_t readByte(uint16_t address);
uint16_t readWord(uint16_t address);

void writeByte(uint16_t address, uint8_t val);
void writeWord(uint16_t address, uint16_t val);

#endif