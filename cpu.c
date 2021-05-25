#include "cpu.h"
#include "memory.h"
#include <stdio.h>
#include <stdlib.h>

struct registers_t registers = {0};

extern union memory_t memory;

extern uint32_t clock;

uint32_t cpuclock;

// Instruction information, including function pointers
struct instruction {
	char *disas;
	uint8_t opcodeLength;
    uint8_t cycles; // This is master clock cycles (aka dots i think). t mode on gbops table
	void *execute;
	//uint8_t ticks;
} extern const instructions[256];

uint8_t IME_flag = 1;

// https://izik1.github.io/gbops/
const struct instruction instructions[256] = {
    {"NOP", 1, 4, nop},                    // 0x00
    {"LD BC, 0x%04X", 3, 12, ld_bc_nn},    // 0x01
    {"LD (BC), A", 1, 8, ld_bcp_a},        // 0x02
    {"INC BC", 1, 8, inc_bc},              // 0x03
    {"INC B", 1, 4, inc_b},                // 0x04
    {"DEC B", 1, 4, dec_b},                // 0x05
    {"LD B, 0x%02X", 2, 8, ld_b_n},        // 0x06
    {"RLCA", 1, 4, rlca},                  // 0x07
    {"LD (0x%04X), SP", 3, 20, ld_nnp_sp},  // 0x08
    {"ADD HL, BC", 1, 8, add_hl_bc},       // 0x09
    {"LD A, (BC)", 1, 8, ld_a_bcp},        // 0x0A
    {"DEC BC", 1, 8, dec_bc},              // 0x0B
    {"INC C", 1, 4, inc_c},                // 0x0C
    {"DEC C", 1, 4, dec_c},                // 0x0D
    {"LD C, 0x%02X", 2, 8, ld_c_n},        // 0x0E
    {"RRCA", 1, 4, NULL},                  // 0x0F
    {"STOP", 1, 4, NULL},                  // 0x10
    {"LD DE, 0x%04X", 3, 12, ld_de_nn},     // 0x11
    {"LD (DE), A", 1, 8, ld_dep_a},        // 0x12
    {"INC DE", 1, 8, inc_de},              // 0x13
    {"INC D", 1, 4, inc_d},                // 0x14
    {"DEC D", 1, 4, dec_d},                // 0x15
    {"LD D, 0x%02X", 2, 8, ld_d_n},        // 0x16
    {"RLA", 1, 4, NULL},                   // 0x17
    {"JR 0x%02X", 2, 12, jr_nn},             // 0x18
    {"ADD HL, DE", 1, 8, add_hl_de},       // 0x19
    {"LD A, (DE)", 1, 8, ld_a_dep},        // 0x1A
    {"DEC DE", 1, 8, dec_de},              // 0x1B
    {"INC E", 1, 4, inc_e},                // 0x1C
    {"DEC E", 1, 4, dec_e},                // 0x1D
    {"LD E, 0x%02X", 2, 8, ld_e_n},        // 0x1E
    {"RRA", 1, 4, rra},                    // 0x1F
    {"JR NZ, 0x%02X", 2, 8, jr_nz},        // 0x20  CYCLES VARIER 8t-12t (4 additional cycles for branch)
    {"LD HL, 0x%04X", 3, 12, ld_hl_nn},     // 0x21
    {"LDI (HL), A", 1, 8, ldi_hlp_a},      // 0x22
    {"INC HL", 1, 8, inc_hl},              // 0x23
    {"INC H", 1, 4, inc_h},                // 0x24
    {"DEC H", 1, 4, dec_h},                // 0x25
    {"LD H, 0x%02X", 2, 8, ld_h_n},        // 0x26
    {"DAA", 1, 4, NULL},                   // 0x27
    {"JR Z, 0x%02X", 2, 8, jr_z},          // 0x28  8t-12t
    {"ADD HL, HL", 1, 8, add_hl_hl},       // 0x29
    {"LDI A, (HL)", 1, 8, ldi_a_hlp},      // 0x2A
    {"DEC HL", 1, 8, dec_hl},              // 0x2B
    {"INC L", 1, 4, inc_l},                // 0x2C
    {"DEC L", 1, 4, dec_l},                // 0x2D
    {"LD L, 0x%02X", 2, 8, ld_l_n},        // 0x2E
    {"CPL", 1, 4, NULL},                   // 0x2F
    {"JR NC, 0x%02X", 2, 8, NULL},         // 0x30 8t-12t
    {"LD SP,0x%04X", 3, 12, ld_sp_nn},      // 0x31
    {"LDD (HL), A", 1, 8, ldd_hlp_a},      // 0x32
    {"INC SP", 1, 8, inc_sp},              // 0x33
    {"INC (HL)", 1, 12, inc_hlp},           // 0x34
    {"DEC (HL)", 1, 12, dec_hlp},           // 0x35
    {"LD (HL), 0x%02X", 2, 12, ld_hlp_n},   // 0x36
    {"SCF", 1, 4, NULL},                   // 0x37
    {"JR C, 0x%02X", 2, 8, NULL},          // 0x38 8t-12t
    {"ADD HL, SP", 1, 8, add_hl_sp},       // 0x39
    {"LDD A,(HL)", 1, 8, ldd_a_hlp},       // 0x3A
    {"DEC SP", 1, 8, NULL},                // 0x3B
    {"INC A", 1, 4, inc_a},                // 0x3C
    {"DEC A", 1, 4, NULL},                 // 0x3D
    {"LD A, 0x%02X", 2, 8, ld_a_n},        // 0x3E
    {"CCF", 1, 4, NULL},                   // 0x3F
    {"LD B, B", 1, 4, ld_b_b},             // 0x40
    {"LD B, C", 1, 4, ld_b_c},             // 0x41
    {"LD B, D", 1, 4, ld_b_d},             // 0x42
    {"LD B, E", 1, 4, ld_b_e},             // 0x43
    {"LD B, H", 1, 4, ld_b_h},             // 0x44
    {"LD B, L", 1, 4, ld_b_l},             // 0x45
    {"LD B, (HL)", 1, 8, ld_b_hlp},        // 0x46
    {"LD B, A", 1, 4, ld_b_a},             // 0x47
    {"LD C, B", 1, 4, ld_c_b},             // 0x48
    {"LD C, C", 1, 4, ld_c_c},             // 0x49
    {"LD C, D", 1, 4, ld_c_d},             // 0x4A
    {"LD C, E", 1, 4, ld_c_e},             // 0x4B
    {"LD C, H", 1, 4, ld_c_h},             // 0x4C
    {"LD C, L", 1, 4, ld_c_l},             // 0x4D
    {"LD C, (HL)", 1, 8, ld_c_hlp},        // 0x4E
    {"LD C, A", 1, 4, ld_c_a},             // 0x4F
    {"LD D, B", 1, 4, ld_d_b},             // 0x50
    {"LD D, C", 1, 4, ld_d_c},             // 0x51
    {"LD D, D", 1, 4, ld_d_d},             // 0x52
    {"LD D, E", 1, 4, ld_d_e},             // 0x53
    {"LD D, H", 1, 4, ld_d_h},             // 0x54
    {"LD D, L", 1, 4, ld_d_l},             // 0x55
    {"LD D, (HL)", 1, 8, ld_d_hlp},        // 0x56
    {"LD D, A", 1, 4, ld_d_a},             // 0x57
    {"LD E, B", 1, 4, ld_e_b},             // 0x58
    {"LD E, C", 1, 4, ld_e_c},             // 0x59
    {"LD E, D", 1, 4, ld_e_d},             // 0x5A
    {"LD E, E", 1, 4, ld_e_e},             // 0x5B
    {"LD E, H", 1, 4, ld_e_h},             // 0x5C
    {"LD E, L", 1, 4, ld_e_l},             // 0x5D
    {"LD E, (HL)", 1, 8, ld_e_hlp},        // 0x5E
    {"LD E, A", 1, 4, ld_e_a},             // 0x5F
    {"LD H, B", 1, 4, ld_h_b},             // 0x60
    {"LD H, C", 1, 4, ld_h_c},             // 0x61
    {"LD H, D", 1, 4, ld_h_d},             // 0x62
    {"LD H, E", 1, 4, ld_h_e},             // 0x63
    {"LD H, H", 1, 4, ld_h_h},             // 0x64
    {"LD H, L", 1, 4, ld_h_l},             // 0x65
    {"LD H, (HL)", 1, 8, ld_h_hlp},        // 0x66
    {"LD H, A", 1, 4, ld_h_a},             // 0x67
    {"LD L, B", 1, 4, ld_l_b},             // 0x68
    {"LD L, C", 1, 4, ld_l_c},             // 0x69
    {"LD L, D", 1, 4, ld_l_d},             // 0x6A
    {"LD L, E", 1, 4, ld_l_e},             // 0x6B
    {"LD L, H", 1, 4, ld_l_h},             // 0x6C
    {"LD L, L", 1, 4, ld_l_l},             // 0x6D
    {"LD L, (HL)", 1, 8, ld_l_hlp},        // 0x6E
    {"LD L, A", 1, 4, ld_l_a},             // 0x6F
    {"LD (HL), B", 1, 8, ld_hlp_b},        // 0x70
    {"LD (HL), C", 1, 8, ld_hlp_c},        // 0x71
    {"LD (HL), D", 1, 8, ld_hlp_d},        // 0x72
    {"LD (HL), E", 1, 8, ld_hlp_e},        // 0x73
    {"LD (HL), H", 1, 8, ld_hlp_h},        // 0x74
    {"LD (HL), L", 1, 8, ld_hlp_l},        // 0x75
    {"HALT", 1, 4, NULL},                  // 0x76
    {"LD (HL), A", 1, 8, ld_hlp_a},        // 0x77
    {"LD A, B", 1, 4, ld_a_b},             // 0x78
    {"LD A, C", 1, 4, ld_a_c},             // 0x79
    {"LD A, D", 1, 4, ld_a_d},             // 0x7A
    {"LD A, E", 1, 4, ld_a_e},             // 0x7B
    {"LD A, H", 1, 4, ld_a_h},             // 0x7C
    {"LD A, L", 1, 4, ld_a_l},             // 0x7D
    {"LD A, (HL)", 1, 8, ld_a_hlp},        // 0x7E
    {"LD A, A", 1, 4, ld_a_a},             // 0x7F
    {"ADD A, B", 1, 4, add_a_b},           // 0x80
    {"ADD A, C", 1, 4, add_a_c},           // 0x81
    {"ADD A, D", 1, 4, add_a_d},           // 0x82
    {"ADD A, E", 1, 4, add_a_e},           // 0x83
    {"ADD A, H", 1, 4, add_a_h},           // 0x84
    {"ADD A, L", 1, 4, add_a_l},           // 0x85
    {"ADD A, (HL)", 1, 8, add_a_hlp},      // 0x86
    {"ADD A, A", 1, 4, add_a_a},           // 0x87
    {"ADC A, B", 1, 4, adc_a_b},           // 0x88
    {"ADC A, C", 1, 4, adc_a_c},           // 0x89
    {"ADC A, D", 1, 4, adc_a_d},           // 0x8A
    {"ADC A, E", 1, 4, adc_a_e},           // 0x8B
    {"ADC A, H", 1, 4, adc_a_h},           // 0x8C
    {"ADC A, L", 1, 4, adc_a_l},           // 0x8D
    {"ADC A, (HL)", 1, 8, adc_a_hlp},      // 0x8E
    {"ADC A, A", 1, 4, adc_a_a},           // 0x8F
    {"SUB A, B", 1, 4, sub_a_b},           // 0x90
    {"SUB A, C", 1, 4, sub_a_c},           // 0x91
    {"SUB A, D", 1, 4, sub_a_d},           // 0x92
    {"SUB A, E", 1, 4, sub_a_e},           // 0x93
    {"SUB A, H", 1, 4, sub_a_h},           // 0x94
    {"SUB A, L", 1, 4, sub_a_l},           // 0x95
    {"SUB A, (HL)", 1, 8, sub_a_hlp},      // 0x96
    {"SUB A, A", 1, 4, sub_a_a},           // 0x97
    {"SBC A, B", 1, 4, sbc_a_b},           // 0x98
    {"SBC A, C", 1, 4, sbc_a_c},           // 0x99
    {"SBC A, D", 1, 4, sbc_a_d},           // 0x9A
    {"SBC A, E", 1, 4, sbc_a_e},           // 0x9B
    {"SBC A, H", 1, 4, sbc_a_h},           // 0x9C
    {"SBC A, L", 1, 4, sbc_a_l},           // 0x9D
    {"SBC A, (HL)", 1, 8, sbc_a_hlp},      // 0x9E
    {"SBC A, A", 1, 4, sbc_a_a},           // 0x9F
    {"AND A, B", 1, 4, and_b},             // 0xA0
    {"AND A, C", 1, 4, and_c},             // 0xA1
    {"AND A, D", 1, 4, and_d},             // 0xA2
    {"AND A, E", 1, 4, and_e},             // 0xA3
    {"AND A, H", 1, 4, and_h},             // 0xA4
    {"AND A, L", 1, 4, and_l},             // 0xA5
    {"AND A, (HL)", 1, 8, and_hlp},        // 0xA6
    {"AND A, A", 1, 4, and_a},             // 0xA7
    {"XOR A, B", 1, 4, xor_b},             // 0xA8
    {"XOR A, C", 1, 4, xor_c},             // 0xA9
    {"XOR A, D", 1, 4, xor_d},             // 0xAA
    {"XOR A, E", 1, 4, xor_e},             // 0xAB
    {"XOR A, H", 1, 4, xor_h},             // 0xAC
    {"XOR A, L", 1, 4, xor_l},             // 0xAD
    {"XOR A, (HL)", 1, 8, xor_hlp},        // 0xAE
    {"XOR A, A", 1, 4, xor_a},             // 0xAF
    {"OR A, B", 1, 4, or_b},               // 0xB0
    {"OR A, C", 1, 4, or_c},               // 0xB1
    {"OR A, D", 1, 4, or_d},               // 0xB2
    {"OR A, E", 1, 4, or_e},               // 0xB3
    {"OR A, H", 1, 4, or_h},               // 0xB4
    {"OR A, L", 1, 4, or_l},               // 0xB5
    {"OR A, (HL)", 1, 8, or_hlp},          // 0xB6
    {"OR A, A", 1, 4, or_a},               // 0xB7
    {"CP A, B", 1, 4, cp_b},               // 0xB8
    {"CP A, C", 1, 4, cp_c},               // 0xB9
    {"CP A, D", 1, 4, cp_d},               // 0xBA
    {"CP A, E", 1, 4, cp_e},               // 0xBB
    {"CP A, H", 1, 4, cp_h},               // 0xBC
    {"CP A, L", 1, 4, cp_l},               // 0xBD
    {"CP A, (HL)", 1, 8, cp_hlp},          // 0xBE
    {"CP A, A", 1, 3, cp_a},               // 0xBF
    {"RET NZ", 1, 8, NULL},                // 0xC0  8t-20t
    {"POP BC", 1, 12, pop_bc},                // 0xC1
    {"JP NZ, 0x%04X", 3, 12, NULL},         // 0xC2 12t-16t
    {"JP 0x%04X", 3, 16, jp_nn},            // 0xC3
    {"CALL NZ", 3, 12, call_nz},               // 0xC4 12t-24t
    {"PUSH BC", 1, 16, push_bc},               // 0xC5
    {"ADD A, 0x%02X", 2, 8, add_a_n},      // 0xC6
    {"RST 00h", 1, 16, NULL},               // 0xC7
    {"RET Z", 1, 8, NULL},                 // 0xC8 8t-20t
    {"RET", 1, 16, ret},                   // 0xC9
    {"JP Z, 0x%04X", 3, 12, NULL},          // 0xCA 12t-16t
    {"PREFIX CB", 1, 4, NULL},             // 0xCB
    {"CALL Z, 0x%04X", 3, 12, call_z},        // 0xCC 12t-24t
    {"CALL 0x%04X", 3, 24, call_nn},           // 0xCD
    {"ADC A, 0x%02X", 2, 8, NULL},         // 0xCE
    {"RST 08h", 1, 16, NULL},               // 0xCF
    {"RET NC", 1, 8, NULL},                // 0xD0 8t-20t
    {"POP DE", 1, 12, pop_de},                // 0xD1
    {"JP NC, 0x%04X", 3, 12, jp_nc},        // 0xD2 12t-16t
    {"undefined", 1, 0, undefined},        // 0xD3
    {"CALL NC, 0x%04X", 3, 12, call_nc},       // 0xD4 12t-24t
    {"PUSH DE", 1, 16, push_de},               // 0xD5
    {"SUB A, 0x%02X", 2, 8, sub_a_n},      // 0xD6
    {"RST 10h", 1, 16, NULL},               // 0xD7
    {"RET C", 1, 8, NULL},                 // 0xD8 8t-20t
    {"RETI", 1, 16, NULL},                  // 0xD9
    {"JP C, 0x%04X", 3, 12, NULL},          // 0xDA 12t-24t
    {"undefined", 1, 0, undefined},        // 0xDB
    {"CALL C, 0x%04X", 3, 12, call_c},        // 0xDC 12t-24t
    {"undefined", 1, 0, undefined},        // 0xDD
    {"SBC A, 0x%02X", 2, 8, NULL},         // 0xDE
    {"RST 18h", 1, 16, NULL},               // 0xDF
    {"LD (FF00 + 0x%02X), A", 2, 12, ld_np_a}, // 0xE0
    {"POP HL", 1, 12, pop_hl},                // 0xE1
    {"LD (FF00+C), A", 1, 8, NULL},        // 0xE2
    {"undefined", 1, 0, undefined},        // 0xE3
    {"undefined", 1, 0, undefined},        // 0xE4
    {"PUSH HL", 1, 16, push_hl},               // 0xE5
    {"AND A, 0x%02X", 2, 8, and_n},        // 0xE6
    {"RST 20h", 1, 16, NULL},               // 0xE7
    {"ADD SP, 0x%02X", 2, 16, NULL},        // 0xE8
    {"JP HL", 1, 4, NULL},                 // 0xE9
    {"LD (0x%04X), A", 3, 16, ld_nnp_a},    // 0xEA
    {"undefined", 1, 0, undefined},        // 0xEB
    {"undefined", 1, 0, undefined},        // 0xEC
    {"undefined", 1, 0, undefined},        // 0xED
    {"XOR A, 0x%02X", 2, 8, xor_n},        // 0xEE
    {"RST 28h", 1, 16, NULL},               // 0xEF
    {"LD A, (FF00 + 0x%02X)", 2, 12, ld_a_np}, // 0xF0
    {"POP AF", 1, 12, pop_af},                // 0xF1
    {"LD A, (FF00 + C)", 1, 8, NULL}, // 0xF2
    {"DI", 1, 4, di},                      // 0xF3
    {"undefined", 1, 0, NULL},             // 0xF4
    {"PUSH AF", 1, 16, push_af},               // 0xF5
    {"OR A, 0x%02X", 2, 8, or_n},          // 0xF6
    {"RST 30h", 1, 16, NULL},               // 0xF7
    {"LD HL, SP + 0x%02X", 2, 12, NULL},    // 0xF8
    {"LD SP, HL", 1, 8, ld_sp_hl},         // 0xF9
    {"LD A, (0x%04X)", 3, 16, ld_a_nnp},    // 0xFA
    {"EI", 1, 4, ei},                      // 0xFB
    {"undefined", 1, 0, undefined},        // 0xFC
    {"undefined", 1, 0, undefined},        // 0xFD
    {"CP A, 0x%02X", 2, 8, cp_n},          // 0xFE
    {"RST 38h", 1, 16, NULL}                // 0xFF
};

void reset_cpu_clock(uint16_t maxclock)
{
    cpuclock -= maxclock;
}

void cpuStep() {
    if (clock < cpuclock) {
        // Next cpu step shouldn't run yet...
        return;
    }
    uint8_t opcode = readChar(registers.pc);
    uint16_t operand = 0;

    struct instruction ins = instructions[opcode];

    if (ins.execute == NULL) {
        unimplemented(opcode);
        return;
    }


    static int bptriggered = 0;
    if (registers.pc == 0xc252) {
        bptriggered = 1;
        printf("Breakpoint hit. Press Enter to step execution.\n");
        fflush(stdout);
    }

    //print_registers();

    if (ins.opcodeLength == 2) operand = (uint16_t)readChar(registers.pc+1);
    if (ins.opcodeLength == 3) operand = readShort(registers.pc+1);

    //printf("0x%04X  ", registers.pc);
    switch(ins.opcodeLength) {
        case 1:
            //printf(ins.disas);
            //printf("\n");
            break;
        case 2:
            //printf(ins.disas, operand);
            //printf("\n");
            break;
        case 3:
            //printf(ins.disas, operand);
            //printf("  (%02x)\n", readChar(operand));
            break;
    }

    if (bptriggered)
    {
        getchar();
    }


    registers.pc += ins.opcodeLength;

    cpuclock += ins.cycles;

    switch(ins.opcodeLength) {
        case 1:
            ((void(*)())ins.execute)();
            break;
        case 2:
            ((void (*)(uint8_t))ins.execute)((uint8_t)operand);
            break;
        case 3:
            ((void (*)(uint16_t))ins.execute)(operand);
            break;
    }
}


// Opcode Implementation
void nop() 
{
    
}

void di() {
    IME_flag = 0;
}
void ei() {
    IME_flag = 1;
}

void reset_inc_flags()
{
    FLAGS_CLEAR(FLAGS_ZERO);
    FLAGS_CLEAR(FLAGS_NEGATIVE);
    FLAGS_CLEAR(FLAGS_HALFCARRY);
}

void inc_n(uint8_t* n)
{
    reset_inc_flags();
    if ((((*n & 0xf) + 1) & 0x10) == 0x10)
    {
        FLAGS_SET(FLAGS_HALFCARRY);
    }
    (*n)++;
    if (*n == 0)
    {
        FLAGS_SET(FLAGS_ZERO);
    }
}

void inc_a()
{
    inc_n(&registers.a);
}
void inc_b()
{
    inc_n(&registers.b);
}
void inc_c()
{
    inc_n(&registers.c);
}
void inc_d()
{
    inc_n(&registers.d);
}
void inc_e()
{
    inc_n(&registers.e);
}
void inc_h()
{
    inc_n(&registers.h);
}
void inc_l()
{
    inc_n(&registers.l);
}

void inc_hlp() {
    inc_n(&memory.memory[registers.hl]);
}

void inc_bc()
{
    registers.bc++;
}
void inc_de()
{
    registers.de++;
}
void inc_hl()
{
    registers.hl++;
}
void inc_sp()
{
    registers.sp++;
}


void reset_dec_flags()
{
    FLAGS_CLEAR(FLAGS_ZERO);
    FLAGS_SET(FLAGS_NEGATIVE);
    FLAGS_CLEAR(FLAGS_HALFCARRY);
}

void dec_n(uint8_t* n)
{
    reset_dec_flags();
    if ((((*n & 0xf) - 1) & 0x10) == 0x10)
    {
        FLAGS_SET(FLAGS_HALFCARRY);
    }
    (*n)--;
    if (*n == 0)
    {
        FLAGS_SET(FLAGS_ZERO);
    }
}

void dec_a(){
    dec_n(&registers.a);
}
void dec_b()
{
    dec_n(&registers.b);
}
void dec_c()
{
    dec_n(&registers.c);
}
void dec_d() {
    dec_n(&registers.d);
}
void dec_e()
{
    dec_n(&registers.e);
}
void dec_h()
{
    dec_n(&registers.h);
}
void dec_l()
{
    dec_n(&registers.l);
}
void dec_hlp()
{
    dec_n(&memory.memory[registers.hl]);
}


void dec_bc() {
    registers.bc--;
    
}
void dec_de() {
    registers.de--;
    
}
void dec_hl() {
    registers.hl--;
    
}
void dec_sp() {
    registers.sp--;
    
}


void set_or_flags()
{
    FLAGS_CLEAR(FLAGS_ZERO);
    FLAGS_CLEAR(FLAGS_NEGATIVE);
    FLAGS_CLEAR(FLAGS_HALFCARRY);
    FLAGS_CLEAR(FLAGS_CARRY);
    if (registers.a == 0)
        FLAGS_SET(FLAGS_ZERO);
}


void xor_n(uint8_t n) {
    registers.a ^= n;
    set_or_flags();
}
void xor_b() {
    registers.a ^= registers.b;
    set_or_flags();
}
void xor_c() {
    registers.a ^= registers.c;
    set_or_flags();
}
void xor_d() {
    registers.a ^= registers.d;
    set_or_flags();
}
void xor_e() {
    registers.a ^= registers.e;
    set_or_flags();
}
void xor_h() {
    registers.a ^= registers.h;
    set_or_flags();
}
void xor_l() {
    registers.a ^= registers.l;
    set_or_flags();
}
void xor_hlp() {
    registers.a ^= readChar(registers.hl);
    set_or_flags();
}
void xor_a() {
    registers.a ^= registers.a;
    set_or_flags();
}

void or_n(uint8_t n)
{
    registers.a |= n;
    set_or_flags();
}
void or_a() {
    registers.a |= registers.a;
    set_or_flags();
}
void or_b() {
    registers.a |= registers.b;
    set_or_flags();
}
void or_c() {
    registers.a |= registers.c;
    set_or_flags();
}
void or_d() {
    registers.a |= registers.d;
    set_or_flags();
}
void or_e() {
    registers.a |= registers.e;
    set_or_flags();
}
void or_h() {
    registers.a |= registers.h;
    set_or_flags();
}
void or_l() {
    registers.a |= registers.l;
    set_or_flags();
}
void or_hlp() {
    registers.a |= readChar(registers.hl);
}

void and_n(uint8_t n)
{
    registers.a &= n;
    set_or_flags();
}
void and_a()
{
    registers.a &= registers.a;
    set_or_flags();
}
void and_b()
{
    registers.a &= registers.b;
    set_or_flags();
}
void and_c()
{
    registers.a &= registers.c;
    set_or_flags();
}
void and_d()
{
    registers.a &= registers.d;
    set_or_flags();
}
void and_e()
{
    registers.a &= registers.e;
    set_or_flags();
}
void and_h()
{
    registers.a &= registers.h;
    set_or_flags();
}
void and_l()
{
    registers.a &= registers.l;
    set_or_flags();
}
void and_hlp()
{
    registers.a &= readChar(registers.hl);
}

void cp_n(uint8_t n)
{
    reset_flags();
    if ((((registers.a & 0xf) - (n & 0xf)) & 0x10) == 0x10)
    {
        FLAGS_SET(FLAGS_HALFCARRY);
    }
    if ((((uint16_t)registers.a - (uint16_t)n) & 0x100) == 0x100)
    {
        FLAGS_SET(FLAGS_CARRY);
    }
    if (registers.a == n)
        FLAGS_SET(FLAGS_ZERO);
}

void cp_a() {
    cp_n(registers.a);
}
void cp_b() {
    cp_n(registers.b);
}
void cp_c() {
    cp_n(registers.c);
}
void cp_d() {
    cp_n(registers.d);
}
void cp_e() {
    cp_n(registers.e);
}
void cp_h() {
    cp_n(registers.h);
}
void cp_l() {
    cp_n(registers.l);
}
void cp_hlp() {
    cp_n(memory.memory[registers.hl]);
}



void ld_a_n(uint8_t value) {
    registers.a = value;
    
}
void ld_b_n(uint8_t value) {
    registers.b = value;
    
}
void ld_c_n(uint8_t value) {
    registers.c = value;
    
}
void ld_d_n(uint8_t value) {
    registers.d = value;
    
}
void ld_e_n(uint8_t value) {
    registers.e = value;
    
}
void ld_h_n(uint8_t value) {
    registers.h = value;
    
}
void ld_l_n(uint8_t value) {
    registers.l = value;
    
}

void ld_bc_nn(uint16_t value) {
    registers.bc = value;
    
}
void ld_de_nn(uint16_t value) {
    registers.de = value;
    
}
void ld_hl_nn(uint16_t value) {
    registers.hl = value;
}
void ld_sp_nn(uint16_t value) {
    registers.sp = value;
    
}


void ld_a_bcp() {
    registers.a = readChar(registers.bc);
}
void ld_a_dep() {
    registers.a = readChar(registers.de);
    
}
void ld_bcp_a() {
    writeChar(registers.bc, registers.a);
    
}
void ld_dep_a()
{
    writeChar(registers.de, registers.a);
}

void ld_hlp_n(uint8_t value) {
    writeChar(registers.hl, value);
    
}


void ldi_hlp_a() {
    writeChar(registers.hl, registers.a);
    registers.hl++;
    
}
void ldi_a_hlp() {
    registers.a = readChar(registers.hl);
    registers.hl++;
    
}

void ldd_hlp_a() {
    writeChar(registers.hl, registers.a);
    registers.hl--;
    
}
void ldd_a_hlp() {
    registers.a = readChar(registers.hl);
    registers.hl--;
    
}


void ld_nnp_sp(uint16_t address) {
    writeShort(address, registers.sp);
}
void ld_nnp_a(uint16_t address) {
    writeChar(address, registers.a);
}
void ld_a_nnp(uint16_t address) {
    registers.a = readChar(address);
}
void ld_np_a(uint8_t address)
{
    writeChar(0xFF00 + address, registers.a);
}
void ld_a_np(uint8_t address)
{
    registers.a = readChar(0xFF00 + address);
}

void ld_a_a() {
    registers.a = registers.a;
    
}  
void ld_a_b() {
    registers.a = registers.b;
    
}  
void ld_a_c() {
    registers.a = registers.c;
    
}  
void ld_a_d() {
    registers.a = registers.d;
    
}  
void ld_a_e() {
    registers.a = registers.e;
    
}  
void ld_a_h() {
    registers.a = registers.h;
    
}  
void ld_a_l() {
    registers.a = registers.l;
    
}  
void ld_b_a() {
    registers.b = registers.a;
    
}  
void ld_b_b() {
    registers.b = registers.b;
    
}  
void ld_b_c() {
    registers.b = registers.c;
    
}  
void ld_b_d() {
    registers.b = registers.d;
    
}  
void ld_b_e() {
    registers.b = registers.e;
    
}  
void ld_b_h() {
    registers.b = registers.h;
    
}  
void ld_b_l() {
    registers.b = registers.l;
    
}  
void ld_c_a() {
    registers.c = registers.a;
    
}  
void ld_c_b() {
    registers.c = registers.b;
    
}  
void ld_c_c() {
    registers.c = registers.c;
    
}  
void ld_c_d() {
    registers.c = registers.d;
    
}  
void ld_c_e() {
    registers.c = registers.e;
    
}  
void ld_c_h() {
    registers.c = registers.h;
    
}  
void ld_c_l() {
    registers.c = registers.l;
    
}  
void ld_d_a() {
    registers.d = registers.a;
    
}  
void ld_d_b() {
    registers.d = registers.b;
    
}  
void ld_d_c() {
    registers.d = registers.c;
    
}  
void ld_d_d() {
    registers.d = registers.d;
    
}  
void ld_d_e() {
    registers.d = registers.e;
    
}  
void ld_d_h() {
    registers.d = registers.h;
    
}  
void ld_d_l() {
    registers.d = registers.l;
    
}  
void ld_e_a() {
    registers.e = registers.a;
    
}  
void ld_e_b() {
    registers.e = registers.b;
    
}  
void ld_e_c() {
    registers.e = registers.c;
    
}  
void ld_e_d() {
    registers.e = registers.d;
    
}  
void ld_e_e() {
    registers.e = registers.e;
    
}  
void ld_e_h() {
    registers.e = registers.h;
    
}  
void ld_e_l() {
    registers.e = registers.l;
    
}  
void ld_h_a() {
    registers.h = registers.a;
    
}  
void ld_h_b() {
    registers.h = registers.b;
    
}  
void ld_h_c() {
    registers.h = registers.c;
    
}  
void ld_h_d() {
    registers.h = registers.d;
    
}  
void ld_h_e() {
    registers.h = registers.e;
    
}  
void ld_h_h() {
    registers.h = registers.h;
    
}  
void ld_h_l() {
    registers.h = registers.l;
    
}  
void ld_l_a() {
    registers.l = registers.a;
    
}  
void ld_l_b() {
    registers.l = registers.b;
    
}  
void ld_l_c() {
    registers.l = registers.c;
    
}  
void ld_l_d() {
    registers.l = registers.d;
    
}  
void ld_l_e() {
    registers.l = registers.e;
    
}  
void ld_l_h() {
    registers.l = registers.h;
    
}  
void ld_l_l() {
    registers.l = registers.l;
    
}
void ld_sp_hl() {
    registers.sp = registers.hl;
}


void ld_a_hlp() {
    registers.a = memory.memory[registers.hl];
    
}
void ld_b_hlp() {
    registers.b = memory.memory[registers.hl];
    
}
void ld_c_hlp() {
    registers.c = memory.memory[registers.hl];
    
}
void ld_d_hlp() {
    registers.d = memory.memory[registers.hl];
    
}
void ld_e_hlp() {
    registers.e = memory.memory[registers.hl];
    
}
void ld_h_hlp() {
    registers.h = memory.memory[registers.hl];
    
}
void ld_l_hlp() {
    registers.l = memory.memory[registers.hl];
    
}
void ld_hlp_a() {
    memory.memory[registers.hl] = registers.a;
    
}
void ld_hlp_b() {
    memory.memory[registers.hl] = registers.b;
    
}
void ld_hlp_c() {
    memory.memory[registers.hl] = registers.c;
    
}
void ld_hlp_d() {
    memory.memory[registers.hl] = registers.d;
    
}
void ld_hlp_e() {
    memory.memory[registers.hl] = registers.e;
    
}
void ld_hlp_h() {
    memory.memory[registers.hl] = registers.h;
    
}
void ld_hlp_l() {
    memory.memory[registers.hl] = registers.l;
    
}


void push_nn(uint16_t nn)
{
    registers.sp -= 2;
    writeShort(registers.sp, nn);
}

void push_af()
{
    push_nn(registers.af);
}

void push_bc()
{
    push_nn(registers.bc);
}

void push_de()
{
    push_nn(registers.de);
}

void push_hl()
{
    push_nn(registers.hl);
}

void pop_rr(uint16_t *rr)
{
    (*rr) = readShort(registers.sp);
    registers.sp += 2;
}

void pop_af()
{
    pop_rr(&registers.af);
}

void pop_bc()
{
    pop_rr(&registers.bc);
}

void pop_de()
{
    pop_rr(&registers.de);
}

void pop_hl()
{
    pop_rr(&registers.hl);
}

void reset_flags() {
    FLAGS_CLEAR(FLAGS_ZERO);
    FLAGS_CLEAR(FLAGS_NEGATIVE);
    FLAGS_CLEAR(FLAGS_HALFCARRY);
    FLAGS_CLEAR(FLAGS_CARRY);
}

void add_a_n(uint8_t n) {
    reset_flags();
    if ((((registers.a & 0xf) + (n & 0xf)) & 0x10) == 0x10)
    {
        FLAGS_SET(FLAGS_HALFCARRY);
    }
    if ((((uint16_t)registers.a + (uint16_t)n) & 0x100) == 0x100)
    {
        FLAGS_SET(FLAGS_CARRY);
    }

    registers.a += n;

    if (registers.a == 0) FLAGS_SET(FLAGS_ZERO);
}

void add_a_a() {
    add_a_n(registers.a);
}
void add_a_b() {
    add_a_n(registers.b);
}
void add_a_c() {
    add_a_n(registers.c);
}
void add_a_d() {
    add_a_n(registers.d);
}
void add_a_e() {
    add_a_n(registers.e);
}
void add_a_h() {
    add_a_n(registers.h);
}
void add_a_l() {
    add_a_n(registers.l);
}
void add_a_hlp() {
    add_a_n(memory.memory[registers.hl]);
}


void add_hl_nn(uint16_t nn)
{

    reset_flags();
    if ((((registers.hl & 0xff) + (nn & 0xff)) & 0x800) == 0x800)
    {
        FLAGS_SET(FLAGS_HALFCARRY);
    }
    if ((((uint16_t)registers.a + (uint16_t)nn) & 0x8000) == 0x8000)
    {
        FLAGS_SET(FLAGS_CARRY);
    }

    registers.hl += nn;

    if (registers.a == 0)
        FLAGS_SET(FLAGS_ZERO);
}
void add_hl_bc() {
    add_hl_nn(registers.bc);
}
void add_hl_de() {
    add_hl_nn(registers.de);
}
void add_hl_hl() {
    add_hl_nn(registers.hl);
}
void add_hl_sp() {
    add_hl_nn(registers.sp);
}
void add_sp_n();

void adc_a_n(uint8_t n) {
    uint8_t oldcarryflag = (FLAGS_ISCARRY != 0);
    reset_flags();
    if ((((registers.a & 0xf) + (n & 0xf) + oldcarryflag) & 0x10) == 0x10)
    {
        FLAGS_SET(FLAGS_HALFCARRY);
    }
    if ((((uint16_t)registers.a + (uint16_t)n + (uint16_t)oldcarryflag) & 0x100) == 0x100)
    {
        FLAGS_SET(FLAGS_CARRY);
    }
    registers.a += n + oldcarryflag;
    if (registers.a == 0) FLAGS_SET(FLAGS_ZERO);
}

void adc_a_a() {
    adc_a_n(registers.a);
}
void adc_a_b() {
    adc_a_n(registers.b);
}
void adc_a_c() {
    adc_a_n(registers.c);
}
void adc_a_d() {
    adc_a_n(registers.d);
}
void adc_a_e() {
    adc_a_n(registers.e);
}
void adc_a_h() {
    adc_a_n(registers.h);
}
void adc_a_l() {
    adc_a_n(registers.l);
}
void adc_a_hlp() {
    adc_a_n(memory.memory[registers.hl]);
}

void sub_a_n(uint8_t n)
{
    reset_flags();
    if ((((registers.a & 0xf) - (n & 0xf)) & 0x10) == 0x10)
    {
        FLAGS_SET(FLAGS_HALFCARRY);
    }
    if ((((uint16_t)registers.a - (uint16_t)n) & 0x100) == 0x100)
    {
        FLAGS_SET(FLAGS_CARRY);
    }

    registers.a -= n;

    if (registers.a == 0)
        FLAGS_SET(FLAGS_ZERO);
}

void sub_a_a() {
    sub_a_n(registers.a);
}
void sub_a_b() {
    sub_a_n(registers.b);
}
void sub_a_c() {
    sub_a_n(registers.c);
}
void sub_a_d() {
    sub_a_n(registers.d);
}
void sub_a_e() {
    sub_a_n(registers.e);
}
void sub_a_h() {
    sub_a_n(registers.h);
}
void sub_a_l() {
    sub_a_n(registers.l);
}
void sub_a_hlp() {
    sub_a_n(memory.memory[registers.hl]);
}


void sbc_a_n(uint8_t n) {
    uint8_t oldcarryflag = (FLAGS_ISCARRY != 0);
    reset_flags();
    if ((((registers.a & 0xf) - (n & 0xf) - oldcarryflag) & 0x10) == 0x10)
    {
        FLAGS_SET(FLAGS_HALFCARRY);
    }
    if ((((uint16_t)registers.a - (uint16_t)n - (uint16_t)oldcarryflag) & 0x100) == 0x100)
    {
        FLAGS_SET(FLAGS_CARRY);
    }
    registers.a -= n - oldcarryflag;
    if (registers.a == 0)
        FLAGS_SET(FLAGS_ZERO);
}

void sbc_a_a() {
    sbc_a_n(registers.a);
}
void sbc_a_b() {
    sbc_a_n(registers.b);
}
void sbc_a_c() {
    sbc_a_n(registers.c);
}
void sbc_a_d() {
    sbc_a_n(registers.d);
}
void sbc_a_e() {
    sbc_a_n(registers.e);
}
void sbc_a_h() {
    sbc_a_n(registers.h);
}
void sbc_a_l() {
    sbc_a_n(registers.l);
}
void sbc_a_hlp() {
    sbc_a_n(memory.memory[registers.hl]);
}


void rlca()
{
    uint8_t msb = (registers.a & (1<<7)) != 0;
    registers.a <<= 1;
    registers.a |= msb;

    FLAGS_CLEAR(FLAGS_ZERO);
    FLAGS_CLEAR(FLAGS_NEGATIVE);
    FLAGS_CLEAR(FLAGS_HALFCARRY);
    FLAGS_CLEAR(FLAGS_CARRY);
    if (msb) FLAGS_SET(FLAGS_CARRY);
    if (registers.a == 0) FLAGS_SET(FLAGS_ZERO);
}
void rra() {
    uint8_t iscarry = FLAGS_ISSET(FLAGS_CARRY) != 0;
    uint8_t lsb = registers.a & (1);

    FLAGS_CLEAR(FLAGS_NEGATIVE);
    FLAGS_CLEAR(FLAGS_HALFCARRY);

    registers.a >>= 1;

    FLAGS_CLEAR(FLAGS_CARRY);

    if (lsb) FLAGS_SET(FLAGS_CARRY);
    registers.a ^= (-iscarry ^ registers.a) & (1 << 7);

}


void jp_nn(uint16_t address)
{
    registers.pc = address;
}

void jp_nc(uint16_t address)
{
    if (FLAGS_ISCARRY == 0) {
        registers.pc = address;
        cpuclock += 4;
    }
}

void jr_nn(int8_t address)
{
    registers.pc += address;
}

void jr_nz(int8_t offset)
{
    if (FLAGS_ISZERO == 0)
    {
        registers.pc += offset;
        cpuclock += 4;
    }
}

void jr_z(int8_t offset)
{
    if (FLAGS_ISZERO)
    {
        registers.pc += offset;
        cpuclock += 4;
    }
}

void call_nn(uint16_t address)
{
    registers.sp -= 2;
    writeShort(registers.sp, registers.pc);
    registers.pc = address;
}

void call_nz(uint16_t address)
{
    if (FLAGS_ISZERO == 0) {
        registers.sp -= 2;
        writeShort(registers.sp, registers.pc);
        registers.pc = address;
        cpuclock += 12; // branch takes additional 12 cycles
    }
}

void call_nc(uint16_t address)
{
    if (FLAGS_ISCARRY == 0)
    {
        registers.sp -= 2;
        writeShort(registers.sp, registers.pc);
        registers.pc = address;
        cpuclock += 12;
    }
}

void call_z(uint16_t address)
{
    if (FLAGS_ISZERO)
    {
        registers.sp -= 2;
        writeShort(registers.sp, registers.pc);
        registers.pc = address;
        cpuclock += 12; // branch takes additional 12 cycles
    }
}

void call_c(uint16_t address)
{
    if (FLAGS_ISCARRY)
    {
        registers.sp -= 2;
        writeShort(registers.sp, registers.pc);
        registers.pc = address;
        cpuclock += 12;
    }
}

void ret(){
    registers.pc = readShort(registers.sp);
    registers.sp += 2;
}


void undefined() {
    registers.pc--;
    printf("0x%04X  0x%02X  \n", registers.pc, readChar(registers.pc));
    exit(1);// ???? do we just NOP?
}

void unimplemented(uint8_t opcode) {
    printf("0x%04X  0x%02X  ", registers.pc, opcode);
    printf(instructions[opcode].disas);
    printf("   unimplemented.");
    //print_registers();
    fflush(stdout);
    exit(1);
}

void print_byte_bits(uint8_t b)
{
    printf("%d", (b & (1 << 7)) ? 1 : 0);
    printf("%d", (b & (1 << 6)) ? 1 : 0);
    printf("%d", (b & (1 << 5)) ? 1 : 0);
    printf("%d", (b & (1 << 4)) ? 1 : 0);
    printf("%d", (b & (1 << 3)) ? 1 : 0);
    printf("%d", (b & (1 << 2)) ? 1 : 0);
    printf("%d", (b & (1 << 1)) ? 1 : 0);
    printf("%d", (b & (1)) ? 1 : 0);
}

void print_registers()
{
    printf("  AF: %04X       \n  BC: %04X  (%02x)\n", registers.af, registers.bc, readChar(registers.bc));
    printf("  DE: %04X  (%02x)\n  HL: %04X  (%02x)\n", registers.de, readChar(registers.de),  registers.hl, readChar(registers.hl));
    printf("  SP: %04X  (%02x)\n  PC: %04X\n", registers.sp, readChar(registers.sp), registers.pc);

    printf("  F: 0b");
    print_byte_bits(registers.f);
    printf("\n");
}