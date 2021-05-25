#pragma once
#ifndef GAMEBOY_H
#define GAMEBOY_H

#include <stdint.h>

extern uint32_t clock;

void reset();
void loadRom(char* rompath);
void step();

#endif
