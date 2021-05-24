#include "memory.h"

union memory_t memory = {0};


// TODO: Implement ECHORAM 
uint8_t readChar(uint16_t address)
{
    return memory.memory[address];
}

uint16_t readShort(uint16_t address)
{
    uint16_t c1 = (uint16_t)memory.memory[address];
    uint16_t c2 = (uint16_t)memory.memory[address + 1];
    return (c2 << 8) | c1;
}