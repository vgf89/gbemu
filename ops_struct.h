#pragma once
#ifndef OPS_STRUCT_H
#define OPS_STRUCT_H

#include "cpu.h"

// Instruction information, including function pointers
struct instruction {
	char *disas;
	uint8_t opcodeLength;
    uint8_t cycles; // This is master clock cycles (aka dots i think). t mode on gbops table
	void *execute;
};

extern struct registers_t registers;


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
    {"RRCA", 1, 4, rrca},                  // 0x0F
    {"STOP", 1, 4, NULL},                  // 0x10
    {"LD DE, 0x%04X", 3, 12, ld_de_nn},     // 0x11
    {"LD (DE), A", 1, 8, ld_dep_a},        // 0x12
    {"INC DE", 1, 8, inc_de},              // 0x13
    {"INC D", 1, 4, inc_d},                // 0x14
    {"DEC D", 1, 4, dec_d},                // 0x15
    {"LD D, 0x%02X", 2, 8, ld_d_n},        // 0x16
    {"RLA", 1, 4, rla},                   // 0x17
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
    {"DAA", 1, 4, daa},                   // 0x27
    {"JR Z, 0x%02X", 2, 8, jr_z},          // 0x28  8t-12t
    {"ADD HL, HL", 1, 8, add_hl_hl},       // 0x29
    {"LDI A, (HL)", 1, 8, ldi_a_hlp},      // 0x2A
    {"DEC HL", 1, 8, dec_hl},              // 0x2B
    {"INC L", 1, 4, inc_l},                // 0x2C
    {"DEC L", 1, 4, dec_l},                // 0x2D
    {"LD L, 0x%02X", 2, 8, ld_l_n},        // 0x2E
    {"CPL", 1, 4, cpl},                   // 0x2F
    {"JR NC, 0x%02X", 2, 8, jr_nc},         // 0x30 8t-12t
    {"LD SP,0x%04X", 3, 12, ld_sp_nn},      // 0x31
    {"LDD (HL), A", 1, 8, ldd_hlp_a},      // 0x32
    {"INC SP", 1, 8, inc_sp},              // 0x33
    {"INC (HL)", 1, 12, inc_hlp},           // 0x34
    {"DEC (HL)", 1, 12, dec_hlp},           // 0x35
    {"LD (HL), 0x%02X", 2, 12, ld_hlp_n},   // 0x36
    {"SCF", 1, 4, NULL},                   // 0x37
    {"JR C, 0x%02X", 2, 8, jr_c},          // 0x38 8t-12t
    {"ADD HL, SP", 1, 8, add_hl_sp},       // 0x39
    {"LDD A,(HL)", 1, 8, ldd_a_hlp},       // 0x3A
    {"DEC SP", 1, 8, dec_sp},                // 0x3B
    {"INC A", 1, 4, inc_a},                // 0x3C
    {"DEC A", 1, 4, dec_a},                 // 0x3D
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
    {"RET NZ", 1, 8, ret_nz},                // 0xC0  8t-20t
    {"POP BC", 1, 12, pop_bc},                // 0xC1
    {"JP NZ, 0x%04X", 3, 12, jp_nz},         // 0xC2 12t-16t
    {"JP 0x%04X", 3, 16, jp_nn},            // 0xC3
    {"CALL NZ", 3, 12, call_nz},               // 0xC4 12t-24t
    {"PUSH BC", 1, 16, push_bc},               // 0xC5
    {"ADD A, 0x%02X", 2, 8, add_a_n},      // 0xC6
    {"RST 00h", 1, 16, NULL},               // 0xC7
    {"RET Z", 1, 8, ret_z},                 // 0xC8 8t-20t
    {"RET", 1, 16, ret},                   // 0xC9
    {"JP Z, 0x%04X", 3, 12, jp_z},          // 0xCA 12t-16t
    {"PREFIX CB", 2, 0, cb},             // 0xCB
    {"CALL Z, 0x%04X", 3, 12, call_z},        // 0xCC 12t-24t
    {"CALL 0x%04X", 3, 24, call_nn},           // 0xCD
    {"ADC A, 0x%02X", 2, 8, adc_a_n},         // 0xCE
    {"RST 08h", 1, 16, NULL},               // 0xCF
    {"RET NC", 1, 8, ret_nc},                // 0xD0 8t-20t
    {"POP DE", 1, 12, pop_de},                // 0xD1
    {"JP NC, 0x%04X", 3, 12, jp_nc},        // 0xD2 12t-16t
    {"undefined", 1, 0, undefined},        // 0xD3
    {"CALL NC, 0x%04X", 3, 12, call_nc},       // 0xD4 12t-24t
    {"PUSH DE", 1, 16, push_de},               // 0xD5
    {"SUB A, 0x%02X", 2, 8, sub_a_n},      // 0xD6
    {"RST 10h", 1, 16, NULL},               // 0xD7
    {"RET C", 1, 8, ret_c},                 // 0xD8 8t-20t
    {"RETI", 1, 16, NULL},                  // 0xD9
    {"JP C, 0x%04X", 3, 12, jp_c},          // 0xDA 12t-24t
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
    {"JP HL", 1, 4, jp_hl},                 // 0xE9
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


const struct instruction CB_instructions[256] = {
    {"RLC B", 2, 8, rlc_b},
    {"RLC C", 2, 8, rlc_c},
    {"RLC D", 2, 8, rlc_d},
    {"RLC E", 2, 8, rlc_e},
    {"RLC H", 2, 8, rlc_h},
    {"RLC L", 2, 8, rlc_l},
    {"RLC (HL)", 2, 16, rlc_hlp},
    {"RLC A", 2, 8, rlc_a},

    {"RRC B", 2, 8, rrc_b},
    {"RRC C", 2, 8, rrc_c},
    {"RRC D", 2, 8, rrc_d},
    {"RRC E", 2, 8, rrc_e},
    {"RRC H", 2, 8, rrc_h},
    {"RRC L", 2, 8, rrc_l},
    {"RRC (HL)", 2, 16, rrc_hlp},
    {"RRC A", 2, 8, rrc_a},

    {"RL B", 2, 8, rl_b},
    {"RL C", 2, 8, rl_c},
    {"RL D", 2, 8, rl_d},
    {"RL E", 2, 8, rl_e},
    {"RL H", 2, 8, rl_h},
    {"RL L", 2, 8, rl_l},
    {"RL (HL)", 2, 16, rl_hlp},
    {"RL A", 2, 8, rl_a},

    {"RR B", 2, 8, rr_b},
    {"RR C", 2, 8, rr_c},
    {"RR D", 2, 8, rr_d},
    {"RR E", 2, 8, rr_e},
    {"RR H", 2, 8, rr_h},
    {"RR L", 2, 8, rr_l},
    {"RR (HL)", 2, 16, rr_hlp},
    {"RR A", 2, 8, rr_a},

    {"SLA B", 2, 8, sla_b},
    {"SLA C", 2, 8, sla_c},
    {"SLA D", 2, 8, sla_d},
    {"SLA E", 2, 8, sla_e},
    {"SLA H", 2, 8, sla_h},
    {"SLA L", 2, 8, sla_l},
    {"SLA (HL)", 2, 16, sla_hlp},
    {"SLA A", 2, 8, sla_a},

    {"SRA B", 2, 8, sra_b},
    {"SRA C", 2, 8, sra_c},
    {"SRA D", 2, 8, sra_d},
    {"SRA E", 2, 8, sra_e},
    {"SRA H", 2, 8, sra_h},
    {"SRA L", 2, 8, sra_l},
    {"SRA (HL)", 2, 16, sra_hlp},
    {"SRA A", 2, 8, sra_a},

    {"SWAP B", 2, 8, swap_b},
    {"SWAP C", 2, 8, swap_c},
    {"SWAP D", 2, 8, swap_d},
    {"SWAP E", 2, 8, swap_e},
    {"SWAP H", 2, 8, swap_h},
    {"SWAP L", 2, 8, swap_l},
    {"SWAP (HL)", 2, 16, swap_hlp},
    {"SWAP A", 2, 8, swap_a},

    {"SRL B", 2, 8, srl_b},
    {"SRL C", 2, 8, srl_c},
    {"SRL D", 2, 8, srl_d},
    {"SRL E", 2, 8, srl_e},
    {"SRL H", 2, 8, srl_h},
    {"SRL L", 2, 8, srl_l},
    {"SRL (HL)", 2, 16, srl_hlp},
    {"SRL A", 2, 8, srl_a},

    {"BIT 0 B", 2, 8, bit_0_b},
    {"BIT 0 C", 2, 8, bit_0_c},
    {"BIT 0 D", 2, 8, bit_0_d},
    {"BIT 0 E", 2, 8, bit_0_e},
    {"BIT 0 H", 2, 8, bit_0_h},
    {"BIT 0 L", 2, 8, bit_0_l},
    {"BIT 0 (HL)", 2, 12, bit_0_hlp},
    {"BIT 0 A", 2, 8, bit_0_a},
    {"BIT 1 B", 2, 8, bit_1_b},
    {"BIT 1 C", 2, 8, bit_1_c},
    {"BIT 1 D", 2, 8, bit_1_d},
    {"BIT 1 E", 2, 8, bit_1_e},
    {"BIT 1 H", 2, 8, bit_1_h},
    {"BIT 1 L", 2, 8, bit_1_l},
    {"BIT 1 (HL)", 2, 12, bit_1_hlp},
    {"BIT 1 A", 2, 8, bit_1_a},
    {"BIT 2 B", 2, 8, bit_2_b},
    {"BIT 2 C", 2, 8, bit_2_c},
    {"BIT 2 D", 2, 8, bit_2_d},
    {"BIT 2 E", 2, 8, bit_2_e},
    {"BIT 2 H", 2, 8, bit_2_h},
    {"BIT 2 L", 2, 8, bit_2_l},
    {"BIT 2 (HL)", 2, 12, bit_2_hlp},
    {"BIT 2 A", 2, 8, bit_2_a},
    {"BIT 3 B", 2, 8, bit_3_b},
    {"BIT 3 C", 2, 8, bit_3_c},
    {"BIT 3 D", 2, 8, bit_3_d},
    {"BIT 3 E", 2, 8, bit_3_e},
    {"BIT 3 H", 2, 8, bit_3_h},
    {"BIT 3 L", 2, 8, bit_3_l},
    {"BIT 3 (HL)", 2, 12, bit_3_hlp},
    {"BIT 3 A", 2, 8, bit_3_a},
    {"BIT 4 B", 2, 8, bit_4_b},
    {"BIT 4 C", 2, 8, bit_4_c},
    {"BIT 4 D", 2, 8, bit_4_d},
    {"BIT 4 E", 2, 8, bit_4_e},
    {"BIT 4 H", 2, 8, bit_4_h},
    {"BIT 4 L", 2, 8, bit_4_l},
    {"BIT 4 (HL).", 2, 12, bit_4_hlp},
    {"BIT 4 A", 2, 8, bit_4_a},
    {"BIT 5 B", 2, 8, bit_5_b},
    {"BIT 5 C", 2, 8, bit_5_c},
    {"BIT 5 D", 2, 8, bit_5_d},
    {"BIT 5 E", 2, 8, bit_5_e},
    {"BIT 5 H", 2, 8, bit_5_h},
    {"BIT 5 L", 2, 8, bit_5_l},
    {"BIT 5 (HL)", 2, 12, bit_5_hlp},
    {"BIT 5 A", 2, 8, bit_5_a},
    {"BIT 6 B", 2, 8, bit_6_b},
    {"BIT 6 C", 2, 8, bit_6_c},
    {"BIT 6 D", 2, 8, bit_6_d},
    {"BIT 6 E", 2, 8, bit_6_e},
    {"BIT 6 H", 2, 8, bit_6_h},
    {"BIT 6 L", 2, 8, bit_6_l},
    {"BIT 6 (HL)", 2, 12, bit_6_hlp},
    {"BIT 6 A", 2, 8, bit_6_a},
    {"BIT 7 B", 2, 8, bit_7_b},
    {"BIT 7 C", 2, 8, bit_7_c},
    {"BIT 7 D", 2, 8, bit_7_d},
    {"BIT 7 E", 2, 8, bit_7_e},
    {"BIT 7 H", 2, 8, bit_7_h},
    {"BIT 7 L", 2, 8, bit_7_l},
    {"BIT 7 (HL)", 2, 12, bit_7_hlp},
    {"BIT 7 A", 2, 8, bit_7_a},

    {"RES 0 B", 2, 8, res_0_b},
    {"RES 0 C", 2, 8, res_0_c},
    {"RES 0 D", 2, 8, res_0_d},
    {"RES 0 E", 2, 8, res_0_e},
    {"RES 0 H", 2, 8, res_0_h},
    {"RES 0 L", 2, 8, res_0_l},
    {"RES 0 (HL)", 2, 16, res_0_hlp},
    {"RES 0 A", 2, 8, res_0_a},
    {"RES 1 B", 2, 8, res_1_b},
    {"RES 1 C", 2, 8, res_1_c},
    {"RES 1 D", 2, 8, res_1_d},
    {"RES 1 E", 2, 8, res_1_e},
    {"RES 1 H", 2, 8, res_1_h},
    {"RES 1 L", 2, 8, res_1_l},
    {"RES 1 (HL)", 2, 16, res_1_hlp},
    {"RES 1 A", 2, 8, res_1_a},
    {"RES 2 B", 2, 8, res_2_b},
    {"RES 2 C", 2, 8, res_2_c},
    {"RES 2 D", 2, 8, res_2_d},
    {"RES 2 E", 2, 8, res_2_e},
    {"RES 2 H", 2, 8, res_2_h},
    {"RES 2 L", 2, 8, res_2_l},
    {"RES 2 (HL)", 2, 16, res_2_hlp},
    {"RES 2 A", 2, 8, res_2_a},
    {"RES 3 B", 2, 8, res_3_b},
    {"RES 3 C", 2, 8, res_3_c},
    {"RES 3 D", 2, 8, res_3_d},
    {"RES 3 E", 2, 8, res_3_e},
    {"RES 3 H", 2, 8, res_3_h},
    {"RES 3 L", 2, 8, res_3_l},
    {"RES 3 (HL)", 2, 16, res_3_hlp},
    {"RES 3 A", 2, 8, res_3_a},
    {"RES 4 B", 2, 8, res_4_b},
    {"RES 4 C", 2, 8, res_4_c},
    {"RES 4 D", 2, 8, res_4_d},
    {"RES 4 E", 2, 8, res_4_e},
    {"RES 4 H", 2, 8, res_4_h},
    {"RES 4 L", 2, 8, res_4_l},
    {"RES 4 (HL)", 2, 16, res_4_hlp},
    {"RES 4 A", 2, 8, res_4_a},
    {"RES 5 B", 2, 8, res_5_b},
    {"RES 5 C", 2, 8, res_5_c},
    {"RES 5 D", 2, 8, res_5_d},
    {"RES 5 E", 2, 8, res_5_e},
    {"RES 5 H", 2, 8, res_5_h},
    {"RES 5 L", 2, 8, res_5_l},
    {"RES 5 (HL)", 2, 16, res_5_hlp},
    {"RES 5 A", 2, 8, res_5_a},
    {"RES 6 B", 2, 8, res_6_b},
    {"RES 6 C", 2, 8, res_6_c},
    {"RES 6 D", 2, 8, res_6_d},
    {"RES 6 E", 2, 8, res_6_e},
    {"RES 6 H", 2, 8, res_6_h},
    {"RES 6 L", 2, 8, res_6_l},
    {"RES 6 (HL)", 2, 16, res_6_hlp},
    {"RES 6 A", 2, 8, res_6_a},
    {"RES 7 B", 2, 8, res_7_b},
    {"RES 7 C", 2, 8, res_7_c},
    {"RES 7 D", 2, 8, res_7_d},
    {"RES 7 E", 2, 8, res_7_e},
    {"RES 7 H", 2, 8, res_7_h},
    {"RES 7 L", 2, 8, res_7_l},
    {"RES 7 (HL)", 2, 16, res_7_hlp},
    {"RES 7 A", 2, 8, res_7_a},

    {"SET 0 B", 2, 8, set_0_b},
    {"SET 0 C", 2, 8, set_0_c},
    {"SET 0 D", 2, 8, set_0_d},
    {"SET 0 E", 2, 8, set_0_e},
    {"SET 0 H", 2, 8, set_0_h},
    {"SET 0 L", 2, 8, set_0_l},
    {"SET 0 (HL)", 2, 16, set_0_hlp},
    {"SET 0 A", 2, 8, set_0_a},
    {"SET 1 B", 2, 8, set_1_b},
    {"SET 1 C", 2, 8, set_1_c},
    {"SET 1 D", 2, 8, set_1_d},
    {"SET 1 E", 2, 8, set_1_e},
    {"SET 1 H", 2, 8, set_1_h},
    {"SET 1 L", 2, 8, set_1_l},
    {"SET 1 (HL)", 2, 16, set_1_hlp},
    {"SET 1 A", 2, 8, set_1_a},
    {"SET 2 B", 2, 8, set_2_b},
    {"SET 2 C", 2, 8, set_2_c},
    {"SET 2 D", 2, 8, set_2_d},
    {"SET 2 E", 2, 8, set_2_e},
    {"SET 2 H", 2, 8, set_2_h},
    {"SET 2 L", 2, 8, set_2_l},
    {"SET 2 (HL)", 2, 16, set_2_hlp},
    {"SET 2 A", 2, 8, set_2_a},
    {"SET 3 B", 2, 8, set_3_b},
    {"SET 3 C", 2, 8, set_3_c},
    {"SET 3 D", 2, 8, set_3_d},
    {"SET 3 E", 2, 8, set_3_e},
    {"SET 3 H", 2, 8, set_3_h},
    {"SET 3 L", 2, 8, set_3_l},
    {"SET 3 (HL)", 2, 16, set_3_hlp},
    {"SET 3 A", 2, 8, set_3_a},
    {"SET 4 B", 2, 8, set_4_b},
    {"SET 4 C", 2, 8, set_4_c},
    {"SET 4 D", 2, 8, set_4_d},
    {"SET 4 E", 2, 8, set_4_e},
    {"SET 4 H", 2, 8, set_4_h},
    {"SET 4 L", 2, 8, set_4_l},
    {"SET 4 (HL)", 2, 16, set_4_hlp},
    {"SET 4 A", 2, 8, set_4_a},
    {"SET 5 B", 2, 8, set_5_b},
    {"SET 5 C", 2, 8, set_5_c},
    {"SET 5 D", 2, 8, set_5_d},
    {"SET 5 E", 2, 8, set_5_e},
    {"SET 5 H", 2, 8, set_5_h},
    {"SET 5 L", 2, 8, set_5_l},
    {"SET 5 (HL)", 2, 16, set_5_hlp},
    {"SET 5 A", 2, 8, set_5_a},
    {"SET 6 B", 2, 8, set_6_b},
    {"SET 6 C", 2, 8, set_6_c},
    {"SET 6 D", 2, 8, set_6_d},
    {"SET 6 E", 2, 8, set_6_e},
    {"SET 6 H", 2, 8, set_6_h},
    {"SET 6 L", 2, 8, set_6_l},
    {"SET 6 (HL)", 2, 16, set_6_hlp},
    {"SET 6 A", 2, 8, set_6_a},
    {"SET 7 B", 2, 8, set_7_b},
    {"SET 7 C", 2, 8, set_7_c},
    {"SET 7 D", 2, 8, set_7_d},
    {"SET 7 E", 2, 8, set_7_e},
    {"SET 7 H", 2, 8, set_7_h},
    {"SET 7 L", 2, 8, set_7_l},
    {"SET 7 (HL)", 2, 16, set_7_hlp},
    {"SET 7 A", 2, 8, set_7_a}
};

#endif
