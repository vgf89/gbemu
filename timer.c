#include "timer.h"
#include "cpu.h"
#include "memory.h"

extern union memory_t memory;

uint32_t timerClockSub = 0;
uint32_t timerClock = 0;

void timerStep()
{
    //memory.TIMA; // Timer Counter
    //memory.TMA; // Timer Modulo (decides when overflow, and thus interrupt, happens)
    //memory.TAC; // Timer Control.  2. Timer enable  1-0. Input Clock Select
    timerClockSub++;
    timerClock++;

    if (timerClockSub%256 == 0) {
        memory.DIV++;
        timerClockSub = 0;
    }


    if (!(memory.TAC & (1 << 2))) { // If timer not enabled
        return;
    }
    uint8_t clockSelect = memory.TAC & 0x3;
    uint16_t timerSpeed = 1;

    if (clockSelect == 0) {
        timerSpeed = 1024;
    } else if (clockSelect == 1) {
        timerSpeed = 16;
    } else if (clockSelect == 2) {
        timerSpeed = 64;
    } else if (clockSelect == 3) {
        timerSpeed = 256;
    }



    if (timerClock >= timerSpeed) {
        timerClock = 0;
        memory.TIMA++;
        if (memory.TIMA == 0) { // overflowed
            IF_SET(I_TIMER);
            memory.TIMA = memory.TMA;
        }
    }
}