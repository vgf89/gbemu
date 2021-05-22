#pragma once
#ifndef MEMORY_H
#define MEMORY_H

// Memory Map
union memory_t {
    struct{
        unsigned char ROM00[0x4000];   // 0000-3fff 16 KiB ROM Bank 00. Usually fixed
        unsigned char ROMNN[0x4000];   // 4000-7fff 16 KiB ROM Bank NN. Switchable via mapper (if any)
        unsigned char VRAM[0x2000];    // 8000-9fff 8KiB Video RAM
        unsigned char EXRAM[0x2000];   // a000-bfff 8KiB External RAM
        unsigned char WRAM1[0x1000];   // c000-cfff 4KiB Work RAM
        unsigned char WRAM2[0x1000];   // d000-dfff 4KiB Work RAM (switchable banks 1~7 on CGB)
        unsigned char ECHORAM[0x1E00]; // e000-fdff An artifact of how the bus is connected. Mirrors C000~DDFF. Nintendo says use of this area is prohibited.
                                       // For accuracy, we can remap reads/writes from this location to C000~DFFF
        unsigned char OAM[0x00A0];     // fe00-fe9f Sprite Attribute Table
        unsigned char unusable[0x0060];// fea0-feff do not touch, just leave it blank unless needed
        union{
            struct{
                unsigned char controller[0x0001];
                unsigned char communication[0x0002];
                unsigned char gap0[0x0001];
                unsigned char dividerAndTimer[0x0004];
                unsigned char gap1[0x0008];
                unsigned char sound[0x0017];
                unsigned char gap2[0x0009];
                unsigned char waveformRAM[0x0010];
                unsigned char LCD[0x000C]; // ff40-ff4b
                unsigned char gap3[0x0003];
                unsigned char VRAMBankSelect[0x0001]; // CGB
                unsigned char DisableBootRom[0x0001];
                unsigned char HDMA[0x0017]; // CGB
                unsigned char gap4[0x000C];
                unsigned char BCPOCP[0x0002]; // CGB
                unsigned char WRAMBankSelect[0x0001]; // CGB
                unsigned char gap5[0x000F];
            };
            unsigned char IO[0x0080]; // f000-ff7f IO Registers
        };
        
        unsigned char HRAM[0x007F]; // ff80-fffe High RAM
        unsigned char IE[0x0001]; // ffff Interrupts Enable Register
    };
    unsigned char memory[0x10000]; // union
};

unsigned char readChar(unsigned short address);
unsigned short readShort(unsigned short address);

#endif