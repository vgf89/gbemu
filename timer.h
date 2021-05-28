#pragma once
#ifndef TIMER_H
#define TIMER_H

#include <stdint.h>

void timerStep();

void resetDIV();

// return the bit in the LSB nibble selected by selector nibble
uint8_t logic_mux4b(uint8_t value, uint8_t selector);

uint8_t logic_and(uint8_t A, uint8_t B);

uint8_t logic_not(uint8_t A);

#endif