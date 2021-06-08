#include <stdio.h>
#include <stdlib.h>
#include "memory.h"
#include "timer.h"
#include "input.h"

union memory_t memory = {0};

uint8_t cartridgeMode = NoMBC;
uint8_t MBC1BankNN = 1;

uint8_t MBC1Banks[125][0x4000];// = {0};



void loadRom(char* rompath)
{
    FILE* fp;
    fp = fopen(rompath, "rb");
    fseek(fp, 0x0147, 0);
    fread(&cartridgeMode, sizeof(uint8_t), 1, fp);
    fseek(fp, 0, 0);
    switch(cartridgeMode) {
        case NoMBC: // ROM Only (read in Bank 0)
            printf("loading ROM ONLY game\n");
            fread(memory.ROM00, sizeof(uint8_t), 0x4000, fp);
            fseek(fp, 0x4000, SEEK_SET); // fread isn't advancing fp for some reason, so I need to seek manually
            fread(memory.ROMNN, sizeof(uint8_t), 0x4000, fp);
            break;
        case MBC1:
            printf("\tloading MBC1 cartidge\n");
            fseek(fp, 0, SEEK_SET);
            fread(memory.ROM00, sizeof(uint8_t), 0x4000, fp);
            // Dump all mapped banks into ram. Mapped in readByte/writeByte
            printf("\treading bank 0\n");
            fseek(fp, 0x4000, SEEK_SET);
            int i = 0;

            while(fread(MBC1Banks[i++], sizeof(uint8_t), 0x4000, fp) == 0x4000) {
                printf("\treading bank %d\n", i);
            }
            break;
        default:
            printf("Cartidge type 0x%02X not supported", cartridgeMode);
            exit(1);
    }
    fclose(fp);
}


// TODO: Implement ECHORAM 
uint8_t readByte(uint16_t address)
{
    if (cartridgeMode == MBC1 && (address >= 0x4000 && address <= 0x7fff)) {
        return MBC1Banks[MBC1BankNN-1][address - 0x4000];
    }

    if((address >= 0xfea0) && (address <= 0xfeff)) // Unusable RAM
    {
        return 0xff;
    }

    if(address == 0xff00) // joypad input
    {
        return getInput();
    }

    return memory.memory[address];
}

void writeByte(uint16_t address, uint8_t val)
{
    if (address == 0xff02 && val == 0x81) // Link Port
    {
        //printf("captured Link Cable byte: ");
        printf("%c", (char)readByte(0xff01));
        //printf("\n");
        //return; // I assume it's basically discarded if no device is connected?
    }

    if (&memory.memory[address] == &memory.DIV)
    {
        resetDIV();

    }
    else if((address >= 0xfea0) && (address <= 0xfeff)) {} // Unusable RAM
    else if(address == 0xff00) { // joypad input
        selectInput(val);
    } // Can't write to joypad input this way. use memory.memory[0xff00]
    else
    {
        if (cartridgeMode == MBC1 && (address >= 0x2000 && address <= 0x3fff)) // MBC1 Set ROM Bank Number
        {
            val &= 0x1f; // Mask to 5 bits
            if (val == 0) val = 1;
            // TODO: Mask to number number of bits that represents maximum bank number for cart
            MBC1BankNN = val;
        }
        else if (address <= 0x3fff)
        {
            memory.memory[address] = val;
        }
        else
        {
            memory.memory[address] = val;
        }
    }
}


uint16_t readWord(uint16_t address)
{
    uint16_t c1 = (uint16_t)readByte(address); // TODO: replace with readByte (should just be readByte(address); readByte(address+1), but verify with blarrg)
    uint16_t c2 = (uint16_t)readByte(address + 1);
    return (c2 << 8) | c1;
}

void writeWord(uint16_t address, uint16_t val)
{
    // TODO: verify byte order
    uint8_t c1 = val & 0xff;
    uint8_t c2 = (val >> 8) & 0xff;

    writeByte(address, c1);
    writeByte(address + 1, c2);
}