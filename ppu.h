#pragma once
#ifndef PPU_H
#define PPU_H

#include <stdint.h>

extern uint8_t LCD[144][160];

void ppuStep();

#endif