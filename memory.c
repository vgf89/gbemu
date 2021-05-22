#include "memory.h"

union memory_t memory = {0};


// TODO: Implement ECHORAM 
unsigned char readChar(unsigned short address)
{
    return memory.memory[address];
}

unsigned short readShort(unsigned short address)
{
    unsigned short c1 = (unsigned short)memory.memory[address];
    unsigned short c2 = (unsigned short)memory.memory[address + 1];
    return (c2 << 8) | c1;
}