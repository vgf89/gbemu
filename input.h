#ifndef INPUT_H
#define INPUT_H

#include <stdint.h>


struct joypadState_t {
    uint8_t up;
    uint8_t down;
    uint8_t left;
    uint8_t right;
    uint8_t a;
    uint8_t b;
    uint8_t start;
    uint8_t select;
};

void updateInput();

uint8_t getInput();
uint8_t selectInput(uint8_t byte);

#endif