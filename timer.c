#include <stdio.h>
#include "timer.h"
#include "cpu.h"
#include "memory.h"

extern union memory_t memory;

uint16_t fullDIV = 0;
uint16_t timerClock = 0;

uint8_t preEdgeDetectDelay = 0;

extern uint32_t clock;

void timerStep()
{
    //memory.TIMA; // Timer Counter
    //memory.TMA; // Timer Modulo (decides when overflow, and thus interrupt, happens)
    //memory.TAC; // Timer Control.  2. Timer enable  1-0. Input Clock Select
    fullDIV++;
    memory.DIV = fullDIV >> 8;

    /* Timer increment circuit */
    uint8_t mux1v = ((memory.DIV & (1 << 1)) != 0);
    mux1v |= ((fullDIV & (1 << 7)) != 0) << 3;
    mux1v |= ((fullDIV & (1 << 5)) != 0) << 2;
    mux1v |= ((fullDIV & (1 << 3)) != 0 ) << 1;
    
    uint8_t tacEnable = (memory.TAC & (1 << 2)) != 0;
    uint8_t tacFreq = memory.TAC & 0x3;
    uint8_t preEdgeDetect = logic_and(logic_mux4b(mux1v,tacFreq),tacEnable);

    uint8_t edgeDetectNot = logic_not(preEdgeDetect);
    uint8_t edgeDetectResult = logic_and(edgeDetectNot, preEdgeDetectDelay);
    // Check if timer increment
    if (edgeDetectResult) {
        memory.TIMA++;
        if (memory.TIMA == 0) { // overflowed
            memory.TIMA = memory.TMA;
            if (IE_ISSET(I_TIMER) && !(IF_ISSET(I_TIMER))) {
                fflush(stdout);
                IF_SET(I_TIMER);
            }
        }
    }
    
    preEdgeDetectDelay = preEdgeDetect;
}

void resetDIV()
{
    fullDIV = 0;
    memory.DIV = 0;
}

// return the bit in the LSB nibble selected by selector nibble
uint8_t logic_mux4b(uint8_t value, uint8_t selector)
{
    value &= 0xf;
    selector &= 0x3;

    return (value & (1 << selector))?1:0;
}

uint8_t logic_and(uint8_t A, uint8_t B) {
    A &= 1;
    B &= 1;
    return A && B;
}

uint8_t logic_not(uint8_t A)
{
    A &= 1;
    return A?0:1;
}