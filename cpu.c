#include "cpu.h"
#include "memory.h"
#include <stdio.h>
#include <stdlib.h>

struct registers_t registers = {0};

extern union memory_t memory;

// Instruction information, including function pointers
struct instruction {
	char *disas;
	unsigned char opcodeLength;
	void *execute;
	//unsigned char ticks;
} extern const instructions[256];

// https://izik1.github.io/gbops/
const struct instruction instructions[256] = {
    {"NOP", 1, nop},                            // 0x00
    {"LD BC, 0x%04X", 3, undefined},            // 0x01
    {"LD (BC), A", 1, ld_bcp_a},                    // 0x02
    {"INC BC", 1, inc_bc},                        // 0x03
    {"INC B", 1, inc_b},                         // 0x04
    {"DEC B", 1, dec_b},                         // 0x05
    {"LD B, 0x%02X", 2, ld_b_n},                  // 0x06
    {"RLCA", 1, NULL},                          // 0x07
    {"LD (0x%04X), SP", 3, ld_nnp_sp},               // 0x08
    {"ADD HL, BC", 1, NULL},                    // 0x09
    {"LD A, (BC)", 1, ld_a_bcp},                    // 0x0A
    {"DEC BC", 1, dec_bc},                        // 0x0B
    {"INC C", 1, inc_c},                         // 0x0C
    {"DEC C", 1, dec_c},                         // 0x0D
    {"LD C, 0x%02X", 2, ld_c_n},                  // 0x0E
    {"RRCA", 1, NULL},                          // 0x0F
    {"STOP", 1, NULL},                          // 0x10
    {"LD DE, 0x%04X", 3, ld_de_nn},                 // 0x11
    {"LD (DE), A", 1, ld_dep_a},                    // 0x12
    {"INC DE", 1, inc_de},                        // 0x13
    {"INC D", 1, inc_d},                         // 0x14
    {"DEC D", 1, dec_d},                         // 0x15
    {"LD D, 0x%02X", 2, ld_d_n},                    // 0x16
    {"RLA", 1, NULL},                           // 0x17
    {"JR 0x%02X", 2, NULL},                     // 0x18
    {"ADD HL, DE", 1, NULL},                    // 0x19
    {"LD A, (DE)", 1, ld_a_dep},                    // 0x1A
    {"DEC DE", 1, dec_de},                        // 0x1B
    {"INC E", 1, inc_e},                         // 0x1C
    {"DEC E", 1, dec_e},                         // 0x1D
    {"LD E, 0x%02X", 2, ld_e_n},                  // 0x1E
    {"RRA", 1, NULL},                           // 0x1F
    {"JR NZ, 0x%02X", 2, jr_nz},                 // 0x20
    {"LD HL, 0x%04X", 3, ld_hl_nn},                 // 0x21
    {"LDI (HL), A", 1, ldi_hlp_a},                   // 0x22
    {"INC HL", 1, inc_hl},                        // 0x23
    {"INC H", 1, inc_h},                         // 0x24
    {"DEC H", 1, dec_h},                         // 0x25
    {"LD H, 0x%02X", 2, ld_h_n},                  // 0x26
    {"DAA", 1, NULL},                           // 0x27
    {"JR Z, 0x%02X", 2, NULL},                  // 0x28
    {"ADD HL, HL", 1, NULL},                    // 0x29
    {"LDI A, (HL)", 1, ldi_a_hlp},                   // 0x2A
    {"DEC HL", 1, dec_hl},                        // 0x2B
    {"INC L", 1, inc_l},                         // 0x2C
    {"DEC L", 1, dec_l},                         // 0x2D
    {"LD L, 0x%02X", 2, ld_l_n},                  // 0x2E
    {"CPL", 1, NULL},                           // 0x2F
    {"JR NC, 0x%02X", 2, NULL},                 // 0x30
    {"LD SP,0x%04X", 3, ld_sp_nn},                  // 0x31
    {"LDD (HL), A", 1, ldd_hlp_a},                   // 0x32
    {"INC SP", 1, inc_sp},                        // 0x33
    {"INC (HL)", 1, inc_hlp},                      // 0x34
    {"DEC (HL)", 1, dec_hlp},                      // 0x35
    {"LD (HL), 0x%02X", 2, ld_hlp_n},               // 0x36
    {"SCF", 1, NULL},           // 0x37
    {"JR C, 0x%02X", 2, NULL},           // 0x38
    {"ADD HL, SP", 1, NULL},           // 0x39
    {"LDD A,(HL)", 1, ldd_a_hlp},           // 0x3A
    {"DEC SP", 1, NULL},           // 0x3B
    {"INC A", 1, inc_a},           // 0x3C
    {"DEC A", 1, NULL},           // 0x3D
    {"LD A, 0x%02X", 2, ld_a_n},           // 0x3E
    {"CCF", 1, NULL},           // 0x3F
    {"LD B, B", 1, ld_b_b},           // 0x40
    {"LD B, C", 1, ld_b_c},           // 0x41
    {"LD B, D", 1, ld_b_d},           // 0x42
    {"LD B, E", 1, ld_b_e},           // 0x43
    {"LD B, H", 1, ld_b_h},           // 0x44
    {"LD B, L", 1, ld_b_l},           // 0x45
    {"LD B, (HL)", 1, ld_b_hlp},           // 0x46
    {"LD B, A", 1, ld_b_a},           // 0x47
    {"LD C, B", 1, ld_c_b},           // 0x48
    {"LD C, C", 1, ld_c_c},           // 0x49
    {"LD C, D", 1, ld_c_d},           // 0x4A
    {"LD C, E", 1, ld_c_e},           // 0x4B
    {"LD C, H", 1, ld_c_h},           // 0x4C
    {"LD C, L", 1, ld_c_l},           // 0x4D
    {"LD C, (HL)", 1, ld_c_hlp},           // 0x4E
    {"LD C, A", 1, ld_c_a},           // 0x4F
    {"LD D, B", 1, ld_d_b},           // 0x50
    {"LD D, C", 1, ld_d_c},           // 0x51
    {"LD D, D", 1, ld_d_d},           // 0x52
    {"LD D, E", 1, ld_d_e},           // 0x53
    {"LD D, H", 1, ld_d_h},           // 0x54
    {"LD D, L", 1, ld_d_l},           // 0x55
    {"LD D, (HL)", 1, ld_d_hlp},           // 0x56
    {"LD D, A", 1, ld_d_a},           // 0x57
    {"LD E, B", 1, ld_e_b},           // 0x58
    {"LD E, C", 1, ld_e_c},           // 0x59
    {"LD E, D", 1, ld_e_d},           // 0x5A
    {"LD E, E", 1, ld_e_e},           // 0x5B
    {"LD E, H", 1, ld_e_h},           // 0x5C
    {"LD E, L", 1, ld_e_l},           // 0x5D
    {"LD E, (HL)", 1, ld_e_hlp},           // 0x5E
    {"LD E, A", 1, ld_e_a},           // 0x5F
    {"LD H, B", 1, ld_h_b},           // 0x60
    {"LD H, C", 1, ld_h_c},           // 0x61
    {"LD H, D", 1, ld_h_d},           // 0x62
    {"LD H, E", 1, ld_h_e},           // 0x63
    {"LD H, H", 1, ld_h_h},           // 0x64
    {"LD H, L", 1, ld_h_l},           // 0x65
    {"LD H, (HL)", 1, ld_h_hlp},           // 0x66
    {"LD H, A", 1, ld_h_a},           // 0x67
    {"LD L, B", 1, ld_l_b},           // 0x68
    {"LD L, C", 1, ld_l_c},           // 0x69
    {"LD L, D", 1, ld_l_d},           // 0x6A
    {"LD L, E", 1, ld_l_e},           // 0x6B
    {"LD L, H", 1, ld_l_h},           // 0x6C
    {"LD L, L", 1, ld_l_l},           // 0x6D
    {"LD L, (HL)", 1, ld_l_hlp},           // 0x6E
    {"LD L, A", 1, ld_l_a},           // 0x6F
    {"LD (HL), B", 1, ld_hlp_b},           // 0x70
    {"LD (HL), C", 1, ld_hlp_c},           // 0x71
    {"LD (HL), D", 1, ld_hlp_d},           // 0x72
    {"LD (HL), E", 1, ld_hlp_e},           // 0x73
    {"LD (HL), H", 1, ld_hlp_h},           // 0x74
    {"LD (HL), L", 1, ld_hlp_l},           // 0x75
    {"HALT", 1, NULL},               // 0x76
    {"LD (HL), A", 1, ld_hlp_a},        // 0x77
    {"LD A, B", 1, ld_a_b},           // 0x78
    {"LD A, C", 1, ld_a_c},           // 0x79
    {"LD A, D", 1, ld_a_d},           // 0x7A
    {"LD A, E", 1, ld_a_e},           // 0x7B
    {"LD A, H", 1, ld_a_h},           // 0x7C
    {"LD A, L", 1, ld_a_l},           // 0x7D
    {"LD A, (HL)", 1, ld_a_hlp},           // 0x7E
    {"LD A, A", 1, ld_a_a},           // 0x7F
    {"ADD A, B", 1, NULL},           // 0x80
    {"ADD A, C", 1, NULL},           // 0x81
    {"ADD A, D", 1, NULL},           // 0x82
    {"ADD A, E", 1, NULL},           // 0x83
    {"ADD A, H", 1, NULL},           // 0x84
    {"ADD A, L", 1, NULL},           // 0x85
    {"ADD A, (HL)", 1, NULL},        // 0x86
    {"ADD A, A", 1, NULL},           // 0x87
    {"ADD A, B", 1, NULL},           // 0x88
    {"ADD A, C", 1, NULL},           // 0x89
    {"ADD A, D", 1, NULL},           // 0x8A
    {"ADD A, E", 1, NULL},           // 0x8B
    {"ADD A, H", 1, NULL},           // 0x8C
    {"ADD A, L", 1, NULL},           // 0x8D
    {"ADD A, (HL)", 1, NULL},        // 0x8E
    {"ADD A, A", 1, NULL},           // 0x8F
    {"SUB A, B", 1, NULL},           // 0x90
    {"SUB A, C", 1, NULL},           // 0x91
    {"SUB A, D", 1, NULL},           // 0x92
    {"SUB A, E", 1, NULL},           // 0x93
    {"SUB A, H", 1, NULL},           // 0x94
    {"SUB A, L", 1, NULL},           // 0x95
    {"SUB A, (HL)", 1, NULL},        // 0x96
    {"SUB A, A", 1, NULL},           // 0x97
    {"SUB A, B", 1, NULL},           // 0x98
    {"SUB A, C", 1, NULL},           // 0x99
    {"SUB A, D", 1, NULL},           // 0x9A
    {"SUB A, E", 1, NULL},           // 0x9B
    {"SUB A, H", 1, NULL},           // 0x9C
    {"SUB A, L", 1, NULL},           // 0x9D
    {"SUB A, (HL)", 1, NULL},        // 0x9E
    {"SUB A, A", 1, NULL},           // 0x9F
    {"AND A, B", 1, NULL},           // 0xA0
    {"AND A, C", 1, NULL},           // 0xA1
    {"AND A, D", 1, NULL},           // 0xA2
    {"AND A, E", 1, NULL},           // 0xA3
    {"AND A, H", 1, NULL},           // 0xA4
    {"AND A, L", 1, NULL},           // 0xA5
    {"AND A, (HL)", 1, NULL},        // 0xA6
    {"AND A, A", 1, NULL},           // 0xA7
    {"XOR A, B", 1, xor_b},           // 0xA8
    {"XOR A, C", 1, xor_c},           // 0xA9
    {"XOR A, D", 1, xor_d},           // 0xAA
    {"XOR A, E", 1, xor_e},           // 0xAB
    {"XOR A, H", 1, xor_h},           // 0xAC
    {"XOR A, L", 1, xor_l},           // 0xAD
    {"XOR A, (HL)", 1, xor_hlp},        // 0xAE
    {"XOR A, A", 1, xor_a},           // 0xAF
    {"OR A, B", 1, NULL},            // 0xB0
    {"OR A, C", 1, NULL},            // 0xB1
    {"OR A, D", 1, NULL},            // 0xB2
    {"OR A, E", 1, NULL},            // 0xB3
    {"OR A, H", 1, NULL},            // 0xB4
    {"OR A, L", 1, NULL},            // 0xB5
    {"OR A, (HL)", 1, NULL},         // 0xB6
    {"OR A, A", 1, NULL},            // 0xB7
    {"CP A, B", 1, NULL},            // 0xB8
    {"CP A, C", 1, NULL},            // 0xB9
    {"CP A, D", 1, NULL},            // 0xBA
    {"CP A, E", 1, NULL},            // 0xBB
    {"CP A, H", 1, NULL},            // 0xBC
    {"CP A, L", 1, NULL},            // 0xBD
    {"CP A, (HL)", 1, NULL},         // 0xBE
    {"CP A, A", 1, NULL},            // 0xBF
    {"RET NZ", 1, NULL},           // 0xC0
    {"POP BC", 1, NULL},           // 0xC1
    {"JP NZ, 0x%04X", 3, NULL},           // 0xC2
    {"JP 0x%04X", 3, jp_nn},           // 0xC3
    {"CALL NZ", 1, NULL},           // 0xC4
    {"PUSH BC", 1, NULL},           // 0xC5
    {"ADD A, 0x%02X", 2, NULL},           // 0xC6
    {"RST 00h", 1, NULL},           // 0xC7
    {"RET Z", 1, NULL},           // 0xC8
    {"RET", 1, NULL},           // 0xC9
    {"JP Z, 0x%04X", 3, NULL},           // 0xCA
    {"PREFIX CB", 1, NULL},           // 0xCB
    {"CALL Z, 0x%04X", 3, NULL},           // 0xCC
    {"CALL 0x%04X", 3, NULL},           // 0xCD
    {"ADC A, 0x%02X", 2, NULL},           // 0xCE
    {"RST 08h", 1, NULL},           // 0xCF
    {"RET NC", 1, NULL},           // 0xD0
    {"POP DE", 1, NULL},           // 0xD1
    {"JP NC, 0x%04X", 3, NULL},           // 0xD2
    {"undefined", 1,undefined},           // 0xD3
    {"CALL NC, 0x%04X", 3, NULL},           // 0xD4
    {"PUSH DE", 1, NULL},           // 0xD5
    {"SUB A, 0x%02X", 2, NULL},           // 0xD6
    {"RST 10h", 1, NULL},           // 0xD7
    {"RET C", 1, NULL},           // 0xD8
    {"RETI", 1, NULL},           // 0xD9
    {"JP C, 0x%04X", 3, NULL},           // 0xDA
    {"undefined",1 , undefined},           // 0xDB
    {"CALL C, 0x%04X", 3, NULL},           // 0xDC
    {"undefined",1 , NULL},           // 0xDD
    {"SBC A, 0x%02X", 2, NULL},           // 0xDE
    {"RST 18h", 1, NULL},           // 0xDF
    {"LD (FF00 + 0x%02X), A", 2, NULL},           // 0xE0
    {"POP HL", 1, NULL},           // 0xE1
    {"LD (FF00+C), A", 1, NULL},           // 0xE2
    {"undefined",  1, undefined},           // 0xE3
    {"undefined", 1, undefined},           // 0xE4
    {"PUSH HL", 1, NULL},           // 0xE5
    {"AND A, 0x%02X", 2, NULL},           // 0xE6
    {"RST 20h", 1, NULL},           // 0xE7
    {"ADD SP, 0x%02X", 2, NULL},           // 0xE8
    {"JP HL", 1, NULL},           // 0xE9
    {"LD (0x%04X), A", 3, ld_nnp_a},           // 0xEA
    {"undefined", 1, undefined},           // 0xEB
    {"undefined", 1, undefined},           // 0xEC
    {"undefined", 1, undefined},           // 0xED
    {"XOR A, 0x%02X", 2, NULL},           // 0xEE
    {"RST 28h", 1, NULL},           // 0xEF
    {"LD A, (FF00 + 0x%02X)", 2, NULL},           // 0xF0
    {"POP AF", 1, NULL},           // 0xF1
    {"LD A, (FF00 + 0x%02X)", 2, NULL},           // 0xF2
    {"DI", 1, NULL},           // 0xF3
    {"undefined", 1, NULL},           // 0xF4
    {"PUSH AF", 1, NULL},           // 0xF5
    {"OR A, 0x%02X", 2, NULL},           // 0xF6
    {"RST 30h", 1, NULL},           // 0xF7
    {"LD HL, SP + 0x%02X", 2, NULL},           // 0xF8
    {"LD SP, HL", 1, ld_sp_hl},           // 0xF9
    {"LD A, (0x%04X)", 3, ld_a_nnp},           // 0xFA
    {"EI", 1, NULL},           // 0xFB
    {"undefined", 1, undefined},           // 0xFC
    {"undefined", 1, undefined},           // 0xFD
    {"CP A, 0x%02X", 2, NULL},           // 0xFE
    {"RST 38h", 1, NULL}            // 0xFF
};

void loadRom(char* rompath)
{
    // Load first ROM bank
    FILE* fp;
    fp = fopen(rompath, "r");
    fread(memory.ROM00, 0x8000, 1, fp);
    // Detect cartridge type (only support ROM ONLY for now)
    switch(memory.ROM00[0x0147]) {
        // ROM Only (read in Bank 1)
        case 0x00:
            fseek(fp, 0x8000, 0);
            fread(memory.ROMNN, 0x8000, 1, fp);
            break;
    }
}

void reset()
{
    registers.af = 0x01b0;
    registers.bc = 0x0013;
    registers.de = 0x00d8;
    registers.hl = 0x014d;
    registers.sp = 0x014d;

    memory.memory[0xff05] = 0x00;
    memory.memory[0xff06] = 0x00;
    memory.memory[0xff07] = 0x00;
    memory.memory[0xff10] = 0x80;
    memory.memory[0xff11] = 0xbf;
    memory.memory[0xff12] = 0xf3;
    memory.memory[0xff14] = 0xbf;
    memory.memory[0xff16] = 0x3f;
    memory.memory[0xff17] = 0x00;
    memory.memory[0xff19] = 0xbf;
    memory.memory[0xff1a] = 0x7f;
    memory.memory[0xff1b] = 0xff;
    memory.memory[0xff1c] = 0x9f;
    memory.memory[0xff1e] = 0xbf;
    memory.memory[0xff20] = 0xff;
    memory.memory[0xff21] = 0x00;
    memory.memory[0xff22] = 0x00;
    memory.memory[0xff23] = 0xbf;
    memory.memory[0xff24] = 0x77;
    memory.memory[0xff25] = 0xf3;
    memory.memory[0xff26] = 0xf1-1; //what does this mean [$FF26] = $F1-GB, $F0-SGB ; NR52
    memory.memory[0xff40] = 0x91;
    memory.memory[0xff42] = 0x00;
    memory.memory[0xff43] = 0x00;
    memory.memory[0xff45] = 0x00;
    memory.memory[0xff47] = 0xfc;
    memory.memory[0xff48] = 0xff;
    memory.memory[0xff49] = 0xff;
    memory.memory[0xff4a] = 0x00;
    memory.memory[0xff4b] = 0x00;
    memory.memory[0xffff] = 0x00;

    registers.pc = 0x0100;
}

void cpuStep() {
    unsigned short oldpc = registers.pc;
    unsigned char opcode = readChar(registers.pc);
    unsigned short operand = 0;

    struct instruction ins = instructions[opcode];

    if (ins.execute == NULL) {
        unimplemented(opcode);
        return;
    }


    if (ins.opcodeLength == 2) operand = (unsigned short)readChar(registers.pc+1);
    if (ins.opcodeLength == 3) operand = readShort(registers.pc+1);
    registers.pc += ins.opcodeLength;
    

    switch(ins.opcodeLength) {
        case 1:
            printf("0x%04X  ", registers.pc);
            printf(ins.disas);
            printf("\n");
            ((void(*)())ins.execute)();
            break;
        case 2:
            printf("0x%04X  ", registers.pc);
            printf(ins.disas, operand);
            printf("\n");
            ((void (*)(unsigned char))ins.execute)((unsigned char)operand);
            break;
        case 3:
            printf("0x%04X  ", registers.pc);
            printf(ins.disas, operand);
            printf("\n");
            ((void (*)(unsigned short))ins.execute)(operand);
            break;
    }
    
}


// Opcode Implementation
void nop() 
{
    
}


void inc_bc() {
    registers.bc ++;
    
}
void inc_de() {
    registers.de++;
    
}
void inc_hl() {
    registers.hl++;
    
}
void inc_sp() {
    registers.sp++;
    
}

void inc_b() {
    registers.b++;
    
}
void inc_d() {
    registers.d++;
    
}
void inc_h() {
    registers.h++;
    
}
void inc_hlp() {
    memory.memory[registers.h]++;
    
}

void dec_b() {
    registers.b--;
    
}
void dec_d() {
    registers.d--;
    
}
void dec_h() {
    registers.h--;
    
}
void dec_hlp() {
    memory.memory[registers.h]--;
    
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

void inc_c() {
    registers.c++;
    
}
void inc_e() {
    registers.e++;
    
}
void inc_l() {
    registers.l++;
    
}
void inc_a() {
    registers.a++;
    
}

void dec_c() {
    registers.c--;
    
}
void dec_e() {
    registers.e--;
    
}
void dec_l() {
    registers.l--;
    
}
void dec_a() {
    registers.a--;
    
}


void xor_b() {
    registers.a ^= registers.b;
    
}
void xor_c() {
    registers.a ^= registers.c;
    
}
void xor_d() {
    registers.a ^= registers.d;
    
}
void xor_e() {
    registers.a ^= registers.e;
    
}
void xor_h() {
    registers.a ^= registers.h;
    
}
void xor_l() {
    registers.a ^= registers.l;
    
}
void xor_hlp() {
    registers.a ^= memory.memory[registers.hl];
    
}
void xor_a() {
    registers.a ^= registers.a;
    
}



void ld_a_n(unsigned char value) {
    registers.a = value;
    
}
void ld_b_n(unsigned char value) {
    registers.b = value;
    
}
void ld_c_n(unsigned char value) {
    registers.c = value;
    
}
void ld_d_n(unsigned char value) {
    registers.d = value;
    
}
void ld_e_n(unsigned char value) {
    registers.e = value;
    
}
void ld_h_n(unsigned char value) {
    registers.h = value;
    
}
void ld_l_n(unsigned char value) {
    registers.l = value;
    
}

void ld_bc_nn(unsigned short value) {
    registers.bc = value;
    
}
void ld_de_nn(unsigned short value) {
    registers.de = value;
    
}
void ld_hl_nn(unsigned short value) {
    registers.hl = value;
}
void ld_sp_nn(unsigned short value) {
    registers.sp = value;
    
}


void ld_a_bcp() {
    registers.a = memory.memory[registers.bc];
    
}
void ld_a_dep() {
    registers.a = memory.memory[registers.de];
    
}
void ld_bcp_a() {
    memory.memory[registers.bc] = registers.a;
    
}
void ld_dep_a() {
    memory.memory[registers.de] = registers.a;
    
}

void ld_hlp_n(unsigned char value) {
    memory.memory[registers.hl] = value;
    
}


void ldi_hlp_a() {
    memory.memory[registers.hl] = registers.a;
    registers.hl++;
    
}
void ldi_a_hlp() {
    registers.a = memory.memory[registers.hl];
    registers.hl++;
    
}

void ldd_hlp_a() {
    memory.memory[registers.hl] = registers.a;
    registers.hl--;
    
}
void ldd_a_hlp() {
    registers.a = memory.memory[registers.hl];
    registers.hl--;
    
}


void ld_nnp_sp(unsigned short address) {
    memory.memory[address] = registers.sp;
}
void ld_nnp_a(unsigned short address) {
    memory.memory[address] = registers.a;
}
void ld_a_nnp(unsigned short address) {
    registers.a = memory.memory[address];
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












void jp_nn(unsigned short address)
{
    registers.pc = address;
}

void jr_nz(unsigned char offset)
{
    registers.pc += offset;
}









void undefined(unsigned char opcode, unsigned char pc) {
    printf("0x%04X  0x%02X  ", pc, opcode);
    printf("0x%02X undefined opcode: ", opcode);
    exit(1);
}

void unimplemented(unsigned char opcode) {
    printf("0x%04X  0x%02X  ", registers.pc, opcode);
    printf(instructions[opcode].disas);
    printf("   unimplemented.");
    //print_registers();
    exit(1);
}

void print_registers()
{
    printf("pc: 0x%04X\n", registers.pc);
}
