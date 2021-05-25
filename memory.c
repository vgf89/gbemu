#include <stdio.h>
#include "memory.h"

union memory_t memory = {0};


// TODO: Implement ECHORAM 
uint8_t readChar(uint16_t address)
{
    return memory.memory[address];
}

void writeChar(uint16_t address, uint8_t val)
{
    if (address == 0xff02 && val == 0x81) { // Link Port
        //printf("captured Link Cable byte: ");
        printf("%c", (char)readChar(0xff01));
        //printf("\n");
        //return; // I assume it's basically discarded if no device is connected?
    }

    memory.memory[address] = val;
}


uint16_t readShort(uint16_t address)
{
    uint16_t c1 = (uint16_t)memory.memory[address];
    uint16_t c2 = (uint16_t)memory.memory[address + 1];
    return (c2 << 8) | c1;
}

void writeShort(uint16_t address, uint16_t val)
{
    // TODO: verify byte order
    uint8_t c1 = val & 0xff;
    uint8_t c2 = (val >> 8) & 0xff;

    writeChar(address, c1);
    writeChar(address + 1, c2);
}