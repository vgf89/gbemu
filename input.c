#include "input.h"
#include "memory.h"
#include "raylib.h"

extern union memory_t memory;

struct joypadState_t joypadState = {1}; // Does this set all values to 1 or not?

void updateInput()
{
    uint8_t new_up = !IsKeyDown(KEY_UP);
    uint8_t new_down = !IsKeyDown(KEY_DOWN);
    uint8_t new_left = !IsKeyDown(KEY_LEFT);
    uint8_t new_right = !IsKeyDown(KEY_RIGHT);
    uint8_t new_a = !IsKeyDown(KEY_X);
    uint8_t new_b = !IsKeyDown(KEY_Z);
    uint8_t new_start = !IsKeyDown(KEY_ENTER);
    uint8_t new_select = !IsKeyDown(KEY_SPACE);


    if (
        new_up != joypadState.up ||
        new_down != joypadState.down ||
        new_left != joypadState.left ||
        new_right != joypadState.right ||
        new_a != joypadState.a ||
        new_b != joypadState.b ||
        new_start != joypadState.start ||
        new_select != joypadState.select
    ) {
        IF_SET(I_JOYPAD);
    }

    joypadState.up = new_up;
    joypadState.down = new_down;
    joypadState.left = new_left;
    joypadState.right = new_right;
    joypadState.a = new_a;
    joypadState.b = new_b;
    joypadState.start = new_start;
    joypadState.select = new_select;
}

uint8_t getInput()
{
    uint8_t input_register = 0;

    uint8_t select_action = memory.memory[0xff00] & (1 << 5);
    uint8_t select_dpad   = memory.memory[0xff00] & (1 << 4);
    
    if (select_dpad == 0) {
        input_register |= joypadState.down << 3;
        input_register |= joypadState.up << 2;
        input_register |= joypadState.left << 1;
        input_register |= joypadState.right << 0;
    } else { // Which takes priority? What if both are unselected?
        input_register |= joypadState.start << 3;
        input_register |= joypadState.select << 2;
        input_register |= joypadState.a << 1;
        input_register |= joypadState.b << 0;
    }

    input_register |= select_action;
    input_register |= select_dpad;

    return input_register;
}

uint8_t selectInput(uint8_t byte)
{
    memory.memory[0xff00] &= ~(1 << 4); // Clear bits 4 and 5
    memory.memory[0xff00] &= ~(1 << 5);
    memory.memory[0xff00] |= (byte & (1 << 5)); // Set new bits 4 and 5
    memory.memory[0xff00] |= (byte & (1 << 4));
}
