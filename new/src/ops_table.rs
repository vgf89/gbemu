// Instruction information, including function pointers
pub enum FnEnum { // based on opcode length
    STOP, // end execution
    UNDEFINED, // undefined opcode: panic or nop?
    OpLen1(fn(&mut CPU)),
    OpLen2(fn(&mut CPU, u8)),
    OpLen2i(fn(&mut CPU, i8)),
    OpLen3(fn(&mut CPU, u16)),
}

pub struct instruction {
	pub disas:&'static str,
    pub cycles:u8, // This is master clock cycles (aka dots i think). t mode on gbops table
	pub execute:FnEnum,
}

//extern struct registers_t registers;
use crate::cpu::*;

// https://izik1.github.io/gbops/
pub const instructions: &'static [instruction/*; 256*/] = &[
    instruction {disas: "NOP", cycles: 4, execute: FnEnum::OpLen1(CPU::nop)},                      // 0x00
    instruction {disas: "LD BC, 0x%04X", cycles: 12, execute: FnEnum::OpLen3(CPU::ld_bc_nn)},          // 0x01
    instruction {disas: "LD (BC), A", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_bcp_a)},              // 0x02
    instruction {disas: "INC BC", cycles: 8, execute: FnEnum::OpLen1(CPU::inc_bc)},                    // 0x03
    instruction {disas: "INC B", cycles: 4, execute: FnEnum::OpLen1(CPU::inc_b)},                      // 0x04
    instruction {disas: "DEC B", cycles: 4, execute: FnEnum::OpLen1(CPU::dec_b)},                      // 0x05
    instruction {disas: "LD B, 0x%02X", cycles: 8, execute: FnEnum::OpLen2(CPU::ld_b_n)},              // 0x06
    instruction {disas: "RLCA", cycles: 4, execute: FnEnum::OpLen1(CPU::rlca)},                        // 0x07
    instruction {disas: "LD (0x%04X), SP", cycles: 20, execute: FnEnum::OpLen3(CPU::ld_nnp_sp)},       // 0x08
    instruction {disas: "ADD HL, BC", cycles: 8, execute: FnEnum::OpLen1(CPU::add_hl_bc)},             // 0x09
    instruction {disas: "LD A, (BC)", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_a_bcp)},              // 0x0A
    instruction {disas: "DEC BC", cycles: 8, execute: FnEnum::OpLen1(CPU::dec_bc)},                    // 0x0B
    instruction {disas: "INC C", cycles: 4, execute: FnEnum::OpLen1(CPU::inc_c)},                      // 0x0C
    instruction {disas: "DEC C", cycles: 4, execute: FnEnum::OpLen1(CPU::dec_c)},                      // 0x0D
    instruction {disas: "LD C, 0x%02X", cycles: 8, execute: FnEnum::OpLen2(CPU::ld_c_n)},              // 0x0E
    instruction {disas: "RRCA", cycles: 4, execute: FnEnum::OpLen1(CPU::rrca)},                        // 0x0F
    instruction {disas: "STOP", cycles: 4, execute: FnEnum::STOP},                        // 0x10
    instruction {disas: "LD DE, 0x%04X", cycles: 12, execute: FnEnum::OpLen3(CPU::ld_de_nn)},           // 0x11
    instruction {disas: "LD (DE), A", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_dep_a)},              // 0x12
    instruction {disas: "INC DE", cycles: 8, execute: FnEnum::OpLen1(CPU::inc_de)},                    // 0x13
    instruction {disas: "INC D", cycles: 4, execute: FnEnum::OpLen1(CPU::inc_d)},                      // 0x14
    instruction {disas: "DEC D", cycles: 4, execute: FnEnum::OpLen1(CPU::dec_d)},                      // 0x15
    instruction {disas: "LD D, 0x%02X", cycles: 8, execute: FnEnum::OpLen2(CPU::ld_d_n)},              // 0x16
    instruction {disas: "RLA", cycles: 4, execute: FnEnum::OpLen1(CPU::rla)},                          // 0x17
    instruction {disas: "JR 0x%02X", cycles: 12, execute: FnEnum::OpLen2i(CPU::jr_nn)},                 // 0x18
    instruction {disas: "ADD HL, DE", cycles: 8, execute: FnEnum::OpLen1(CPU::add_hl_de)},             // 0x19
    instruction {disas: "LD A, (DE)", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_a_dep)},              // 0x1A
    instruction {disas: "DEC DE", cycles: 8, execute: FnEnum::OpLen1(CPU::dec_de)},                    // 0x1B
    instruction {disas: "INC E", cycles: 4, execute: FnEnum::OpLen1(CPU::inc_e)},                      // 0x1C
    instruction {disas: "DEC E", cycles: 4, execute: FnEnum::OpLen1(CPU::dec_e)},                      // 0x1D
    instruction {disas: "LD E, 0x%02X", cycles: 8, execute: FnEnum::OpLen2(CPU::ld_e_n)},              // 0x1E
    instruction {disas: "RRA", cycles: 4, execute: FnEnum::OpLen1(CPU::rra)},                          // 0x1F
    instruction {disas: "JR NZ, 0x%02X", cycles: 8, execute: FnEnum::OpLen2i(CPU::jr_nz)},              // 0x20  CYCLES VARIER 8t-12t (4 additional cycles for branch)
    instruction {disas: "LD HL, 0x%04X", cycles: 12, execute: FnEnum::OpLen3(CPU::ld_hl_nn)},          // 0x21
    instruction {disas: "LDI (HL), A", cycles: 8, execute: FnEnum::OpLen1(CPU::ldi_hlp_a)},            // 0x22
    instruction {disas: "INC HL", cycles: 8, execute: FnEnum::OpLen1(CPU::inc_hl)},                    // 0x23
    instruction {disas: "INC H", cycles: 4, execute: FnEnum::OpLen1(CPU::inc_h)},                      // 0x24
    instruction {disas: "DEC H", cycles: 4, execute: FnEnum::OpLen1(CPU::dec_h)},                      // 0x25
    instruction {disas: "LD H, 0x%02X", cycles: 8, execute: FnEnum::OpLen2(CPU::ld_h_n)},              // 0x26
    instruction {disas: "DAA", cycles: 4, execute: FnEnum::OpLen1(CPU::daa)},                          // 0x27
    instruction {disas: "JR Z, 0x%02X", cycles: 8, execute: FnEnum::OpLen2i(CPU::jr_z)},                // 0x28  8t-12t
    instruction {disas: "ADD HL, HL", cycles: 8, execute: FnEnum::OpLen1(CPU::add_hl_hl)},             // 0x29
    instruction {disas: "LDI A, (HL)", cycles: 8, execute: FnEnum::OpLen1(CPU::ldi_a_hlp)},            // 0x2A
    instruction {disas: "DEC HL", cycles: 8, execute: FnEnum::OpLen1(CPU::dec_hl)},                    // 0x2B
    instruction {disas: "INC L", cycles: 4, execute: FnEnum::OpLen1(CPU::inc_l)},                      // 0x2C
    instruction {disas: "DEC L", cycles: 4, execute: FnEnum::OpLen1(CPU::dec_l)},                      // 0x2D
    instruction {disas: "LD L, 0x%02X", cycles: 8, execute: FnEnum::OpLen2(CPU::ld_l_n)},              // 0x2E
    instruction {disas: "CPL", cycles: 4, execute: FnEnum::OpLen1(CPU::cpl)},                          // 0x2F
    instruction {disas: "JR NC, 0x%02X", cycles: 8, execute: FnEnum::OpLen2i(CPU::jr_nc)},              // 0x30 8t-12t
    instruction {disas: "LD SP,0x%04X", cycles: 12, execute: FnEnum::OpLen3(CPU::ld_sp_nn)},           // 0x31
    instruction {disas: "LDD (HL), A", cycles: 8, execute: FnEnum::OpLen1(CPU::ldd_hlp_a)},            // 0x32
    instruction {disas: "INC SP", cycles: 8, execute: FnEnum::OpLen1(CPU::inc_sp)},                    // 0x33
    instruction {disas: "INC (HL)", cycles: 12, execute: FnEnum::OpLen1(CPU::inc_hlp)},                // 0x34
    instruction {disas: "DEC (HL)", cycles: 12, execute: FnEnum::OpLen1(CPU::dec_hlp)},                // 0x35
    instruction {disas: "LD (HL), 0x%02X", cycles: 12, execute: FnEnum::OpLen2(CPU::ld_hlp_n)},        // 0x36
    instruction {disas: "SCF", cycles: 4, execute: FnEnum::OpLen1(CPU::scf)},                          // 0x37
    instruction {disas: "JR C, 0x%02X", cycles: 8, execute: FnEnum::OpLen2i(CPU::jr_c)},                // 0x38 8t-12t
    instruction {disas: "ADD HL, SP", cycles: 8, execute: FnEnum::OpLen1(CPU::add_hl_sp)},             // 0x39
    instruction {disas: "LDD A,(HL)", cycles: 8, execute: FnEnum::OpLen1(CPU::ldd_a_hlp)},             // 0x3A
    instruction {disas: "DEC SP", cycles: 8, execute: FnEnum::OpLen1(CPU::dec_sp)},                    // 0x3B
    instruction {disas: "INC A", cycles: 4, execute: FnEnum::OpLen1(CPU::inc_a)},                      // 0x3C
    instruction {disas: "DEC A", cycles: 4, execute: FnEnum::OpLen1(CPU::dec_a)},                      // 0x3D
    instruction {disas: "LD A, 0x%02X", cycles: 8, execute: FnEnum::OpLen2(CPU::ld_a_n)},              // 0x3E
    instruction {disas: "CCF", cycles: 4, execute: FnEnum::OpLen1(CPU::ccf)},                          // 0x3F
    instruction {disas: "LD B, B", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_b_b)},                   // 0x40
    instruction {disas: "LD B, C", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_b_c)},                   // 0x41
    instruction {disas: "LD B, D", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_b_d)},                   // 0x42
    instruction {disas: "LD B, E", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_b_e)},                   // 0x43
    instruction {disas: "LD B, H", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_b_h)},                   // 0x44
    instruction {disas: "LD B, L", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_b_l)},                   // 0x45
    instruction {disas: "LD B, (HL)", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_b_hlp)},              // 0x46
    instruction {disas: "LD B, A", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_b_a)},                   // 0x47
    instruction {disas: "LD C, B", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_c_b)},                   // 0x48
    instruction {disas: "LD C, C", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_c_c)},                   // 0x49
    instruction {disas: "LD C, D", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_c_d)},                   // 0x4A
    instruction {disas: "LD C, E", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_c_e)},                   // 0x4B
    instruction {disas: "LD C, H", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_c_h)},                   // 0x4C
    instruction {disas: "LD C, L", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_c_l)},                   // 0x4D
    instruction {disas: "LD C, (HL)", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_c_hlp)},              // 0x4E
    instruction {disas: "LD C, A", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_c_a)},                   // 0x4F
    instruction {disas: "LD D, B", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_d_b)},                   // 0x50
    instruction {disas: "LD D, C", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_d_c)},                   // 0x51
    instruction {disas: "LD D, D", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_d_d)},                   // 0x52
    instruction {disas: "LD D, E", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_d_e)},                   // 0x53
    instruction {disas: "LD D, H", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_d_h)},                   // 0x54
    instruction {disas: "LD D, L", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_d_l)},                   // 0x55
    instruction {disas: "LD D, (HL)", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_d_hlp)},              // 0x56
    instruction {disas: "LD D, A", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_d_a)},                   // 0x57
    instruction {disas: "LD E, B", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_e_b)},                   // 0x58
    instruction {disas: "LD E, C", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_e_c)},                   // 0x59
    instruction {disas: "LD E, D", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_e_d)},                   // 0x5A
    instruction {disas: "LD E, E", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_e_e)},                   // 0x5B
    instruction {disas: "LD E, H", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_e_h)},                   // 0x5C
    instruction {disas: "LD E, L", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_e_l)},                   // 0x5D
    instruction {disas: "LD E, (HL)", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_e_hlp)},              // 0x5E
    instruction {disas: "LD E, A", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_e_a)},                   // 0x5F
    instruction {disas: "LD H, B", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_h_b)},                   // 0x60
    instruction {disas: "LD H, C", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_h_c)},                   // 0x61
    instruction {disas: "LD H, D", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_h_d)},                   // 0x62
    instruction {disas: "LD H, E", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_h_e)},                   // 0x63
    instruction {disas: "LD H, H", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_h_h)},                   // 0x64
    instruction {disas: "LD H, L", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_h_l)},                   // 0x65
    instruction {disas: "LD H, (HL)", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_h_hlp)},              // 0x66
    instruction {disas: "LD H, A", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_h_a)},                   // 0x67
    instruction {disas: "LD L, B", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_l_b)},                   // 0x68
    instruction {disas: "LD L, C", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_l_c)},                   // 0x69
    instruction {disas: "LD L, D", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_l_d)},                   // 0x6A
    instruction {disas: "LD L, E", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_l_e)},                   // 0x6B
    instruction {disas: "LD L, H", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_l_h)},                   // 0x6C
    instruction {disas: "LD L, L", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_l_l)},                   // 0x6D
    instruction {disas: "LD L, (HL)", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_l_hlp)},              // 0x6E
    instruction {disas: "LD L, A", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_l_a)},                   // 0x6F
    instruction {disas: "LD (HL), B", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_hlp_b)},              // 0x70
    instruction {disas: "LD (HL), C", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_hlp_c)},              // 0x71
    instruction {disas: "LD (HL), D", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_hlp_d)},              // 0x72
    instruction {disas: "LD (HL), E", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_hlp_e)},              // 0x73
    instruction {disas: "LD (HL), H", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_hlp_h)},              // 0x74
    instruction {disas: "LD (HL), L", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_hlp_l)},              // 0x75
    instruction {disas: "HALT", cycles: 4, execute: FnEnum::OpLen1(CPU::halt)},                        // 0x76
    instruction {disas: "LD (HL), A", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_hlp_a)},              // 0x77
    instruction {disas: "LD A, B", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_a_b)},                   // 0x78
    instruction {disas: "LD A, C", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_a_c)},                   // 0x79
    instruction {disas: "LD A, D", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_a_d)},                   // 0x7A
    instruction {disas: "LD A, E", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_a_e)},                   // 0x7B
    instruction {disas: "LD A, H", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_a_h)},                   // 0x7C
    instruction {disas: "LD A, L", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_a_l)},                   // 0x7D
    instruction {disas: "LD A, (HL)", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_a_hlp)},              // 0x7E
    instruction {disas: "LD A, A", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_a_a)},                   // 0x7F
    instruction {disas: "ADD A, B", cycles: 4, execute: FnEnum::OpLen1(CPU::add_a_b)},                 // 0x80
    instruction {disas: "ADD A, C", cycles: 4, execute: FnEnum::OpLen1(CPU::add_a_c)},                 // 0x81
    instruction {disas: "ADD A, D", cycles: 4, execute: FnEnum::OpLen1(CPU::add_a_d)},                 // 0x82
    instruction {disas: "ADD A, E", cycles: 4, execute: FnEnum::OpLen1(CPU::add_a_e)},                 // 0x83
    instruction {disas: "ADD A, H", cycles: 4, execute: FnEnum::OpLen1(CPU::add_a_h)},                 // 0x84
    instruction {disas: "ADD A, L", cycles: 4, execute: FnEnum::OpLen1(CPU::add_a_l)},                 // 0x85
    instruction {disas: "ADD A, (HL)", cycles: 8, execute: FnEnum::OpLen1(CPU::add_a_hlp)},            // 0x86
    instruction {disas: "ADD A, A", cycles: 4, execute: FnEnum::OpLen1(CPU::add_a_a)},                 // 0x87
    instruction {disas: "ADC A, B", cycles: 4, execute: FnEnum::OpLen1(CPU::adc_a_b)},                 // 0x88
    instruction {disas: "ADC A, C", cycles: 4, execute: FnEnum::OpLen1(CPU::adc_a_c)},                 // 0x89
    instruction {disas: "ADC A, D", cycles: 4, execute: FnEnum::OpLen1(CPU::adc_a_d)},                 // 0x8A
    instruction {disas: "ADC A, E", cycles: 4, execute: FnEnum::OpLen1(CPU::adc_a_e)},                 // 0x8B
    instruction {disas: "ADC A, H", cycles: 4, execute: FnEnum::OpLen1(CPU::adc_a_h)},                 // 0x8C
    instruction {disas: "ADC A, L", cycles: 4, execute: FnEnum::OpLen1(CPU::adc_a_l)},                 // 0x8D
    instruction {disas: "ADC A, (HL)", cycles: 8, execute: FnEnum::OpLen1(CPU::adc_a_hlp)},            // 0x8E
    instruction {disas: "ADC A, A", cycles: 4, execute: FnEnum::OpLen1(CPU::adc_a_a)},                 // 0x8F
    instruction {disas: "SUB A, B", cycles: 4, execute: FnEnum::OpLen1(CPU::sub_a_b)},                 // 0x90
    instruction {disas: "SUB A, C", cycles: 4, execute: FnEnum::OpLen1(CPU::sub_a_c)},                 // 0x91
    instruction {disas: "SUB A, D", cycles: 4, execute: FnEnum::OpLen1(CPU::sub_a_d)},                 // 0x92
    instruction {disas: "SUB A, E", cycles: 4, execute: FnEnum::OpLen1(CPU::sub_a_e)},                 // 0x93
    instruction {disas: "SUB A, H", cycles: 4, execute: FnEnum::OpLen1(CPU::sub_a_h)},                 // 0x94
    instruction {disas: "SUB A, L", cycles: 4, execute: FnEnum::OpLen1(CPU::sub_a_l)},                 // 0x95
    instruction {disas: "SUB A, (HL)", cycles: 8, execute: FnEnum::OpLen1(CPU::sub_a_hlp)},            // 0x96
    instruction {disas: "SUB A, A", cycles: 4, execute: FnEnum::OpLen1(CPU::sub_a_a)},                 // 0x97
    instruction {disas: "SBC A, B", cycles: 4, execute: FnEnum::OpLen1(CPU::sbc_a_b)},                 // 0x98
    instruction {disas: "SBC A, C", cycles: 4, execute: FnEnum::OpLen1(CPU::sbc_a_c)},                 // 0x99
    instruction {disas: "SBC A, D", cycles: 4, execute: FnEnum::OpLen1(CPU::sbc_a_d)},                 // 0x9A
    instruction {disas: "SBC A, E", cycles: 4, execute: FnEnum::OpLen1(CPU::sbc_a_e)},                 // 0x9B
    instruction {disas: "SBC A, H", cycles: 4, execute: FnEnum::OpLen1(CPU::sbc_a_h)},                 // 0x9C
    instruction {disas: "SBC A, L", cycles: 4, execute: FnEnum::OpLen1(CPU::sbc_a_l)},                 // 0x9D
    instruction {disas: "SBC A, (HL)", cycles: 8, execute: FnEnum::OpLen1(CPU::sbc_a_hlp)},            // 0x9E
    instruction {disas: "SBC A, A", cycles: 4, execute: FnEnum::OpLen1(CPU::sbc_a_a)},                 // 0x9F
    instruction {disas: "AND A, B", cycles: 4, execute: FnEnum::OpLen1(CPU::and_b)},                   // 0xA0
    instruction {disas: "AND A, C", cycles: 4, execute: FnEnum::OpLen1(CPU::and_c)},                   // 0xA1
    instruction {disas: "AND A, D", cycles: 4, execute: FnEnum::OpLen1(CPU::and_d)},                   // 0xA2
    instruction {disas: "AND A, E", cycles: 4, execute: FnEnum::OpLen1(CPU::and_e)},                   // 0xA3
    instruction {disas: "AND A, H", cycles: 4, execute: FnEnum::OpLen1(CPU::and_h)},                   // 0xA4
    instruction {disas: "AND A, L", cycles: 4, execute: FnEnum::OpLen1(CPU::and_l)},                   // 0xA5
    instruction {disas: "AND A, (HL)", cycles: 8, execute: FnEnum::OpLen1(CPU::and_hlp)},              // 0xA6
    instruction {disas: "AND A, A", cycles: 4, execute: FnEnum::OpLen1(CPU::and_a)},                   // 0xA7
    instruction {disas: "XOR A, B", cycles: 4, execute: FnEnum::OpLen1(CPU::xor_b)},                   // 0xA8
    instruction {disas: "XOR A, C", cycles: 4, execute: FnEnum::OpLen1(CPU::xor_c)},                   // 0xA9
    instruction {disas: "XOR A, D", cycles: 4, execute: FnEnum::OpLen1(CPU::xor_d)},                   // 0xAA
    instruction {disas: "XOR A, E", cycles: 4, execute: FnEnum::OpLen1(CPU::xor_e)},                   // 0xAB
    instruction {disas: "XOR A, H", cycles: 4, execute: FnEnum::OpLen1(CPU::xor_h)},                   // 0xAC
    instruction {disas: "XOR A, L", cycles: 4, execute: FnEnum::OpLen1(CPU::xor_l)},                   // 0xAD
    instruction {disas: "XOR A, (HL)", cycles: 8, execute: FnEnum::OpLen1(CPU::xor_hlp)},              // 0xAE
    instruction {disas: "XOR A, A", cycles: 4, execute: FnEnum::OpLen1(CPU::xor_a)},                   // 0xAF
    instruction {disas: "OR A, B", cycles: 4, execute: FnEnum::OpLen1(CPU::or_b)},                     // 0xB0
    instruction {disas: "OR A, C", cycles: 4, execute: FnEnum::OpLen1(CPU::or_c)},                     // 0xB1
    instruction {disas: "OR A, D", cycles: 4, execute: FnEnum::OpLen1(CPU::or_d)},                     // 0xB2
    instruction {disas: "OR A, E", cycles: 4, execute: FnEnum::OpLen1(CPU::or_e)},                     // 0xB3
    instruction {disas: "OR A, H", cycles: 4, execute: FnEnum::OpLen1(CPU::or_h)},                     // 0xB4
    instruction {disas: "OR A, L", cycles: 4, execute: FnEnum::OpLen1(CPU::or_l)},                     // 0xB5
    instruction {disas: "OR A, (HL)", cycles: 8, execute: FnEnum::OpLen1(CPU::or_hlp)},                // 0xB6
    instruction {disas: "OR A, A", cycles: 4, execute: FnEnum::OpLen1(CPU::or_a)},                     // 0xB7
    instruction {disas: "CP A, B", cycles: 4, execute: FnEnum::OpLen1(CPU::cp_b)},                     // 0xB8
    instruction {disas: "CP A, C", cycles: 4, execute: FnEnum::OpLen1(CPU::cp_c)},                     // 0xB9
    instruction {disas: "CP A, D", cycles: 4, execute: FnEnum::OpLen1(CPU::cp_d)},                     // 0xBA
    instruction {disas: "CP A, E", cycles: 4, execute: FnEnum::OpLen1(CPU::cp_e)},                     // 0xBB
    instruction {disas: "CP A, H", cycles: 4, execute: FnEnum::OpLen1(CPU::cp_h)},                     // 0xBC
    instruction {disas: "CP A, L", cycles: 4, execute: FnEnum::OpLen1(CPU::cp_l)},                     // 0xBD
    instruction {disas: "CP A, (HL)", cycles: 8, execute: FnEnum::OpLen1(CPU::cp_hlp)},                // 0xBE
    instruction {disas: "CP A, A", cycles: 3, execute: FnEnum::OpLen1(CPU::cp_a)},                     // 0xBF
    instruction {disas: "RET NZ", cycles: 8, execute: FnEnum::OpLen1(CPU::ret_nz)},                    // 0xC0  8t-20t
    instruction {disas: "POP BC", cycles: 12, execute: FnEnum::OpLen1(CPU::pop_bc)},                   // 0xC1
    instruction {disas: "JP NZ, 0x%04X", cycles: 12, execute: FnEnum::OpLen3(CPU::jp_nz)},             // 0xC2 12t-16t
    instruction {disas: "JP 0x%04X", cycles: 16, execute: FnEnum::OpLen3(CPU::jp_nn)},                 // 0xC3
    instruction {disas: "CALL NZ", cycles: 12, execute: FnEnum::OpLen3(CPU::call_nz)},                 // 0xC4 12t-24t
    instruction {disas: "PUSH BC", cycles: 16, execute: FnEnum::OpLen1(CPU::push_bc)},                 // 0xC5
    instruction {disas: "ADD A, 0x%02X", cycles: 8, execute: FnEnum::OpLen2(add_a_n)},            // 0xC6
    instruction {disas: "RST 00h", cycles: 16, execute: FnEnum::OpLen1(CPU::rst00)},                   // 0xC7
    instruction {disas: "RET Z", cycles: 8, execute: FnEnum::OpLen1(CPU::ret_z)},                      // 0xC8 8t-20t
    instruction {disas: "RET", cycles: 16, execute: FnEnum::OpLen1(CPU::ret)},                         // 0xC9
    instruction {disas: "JP Z, 0x%04X", cycles: 12, execute: FnEnum::OpLen3(CPU::jp_z)},               // 0xCA 12t-16t
    instruction {disas: "PREFIX CB", cycles: 0, execute: FnEnum::OpLen2(cb)},                     // 0xCB
    instruction {disas: "CALL Z, 0x%04X", cycles: 12, execute: FnEnum::OpLen3(CPU::call_z)},           // 0xCC 12t-24t
    instruction {disas: "CALL 0x%04X", cycles: 24, execute: FnEnum::OpLen3(CPU::call_nn)},             // 0xCD
    instruction {disas: "ADC A, 0x%02X", cycles: 8, execute: FnEnum::OpLen2(adc_a_n)},            // 0xCE
    instruction {disas: "RST 08h", cycles: 16, execute: FnEnum::OpLen1(CPU::rst08)},                   // 0xCF
    instruction {disas: "RET NC", cycles: 8, execute: FnEnum::OpLen1(CPU::ret_nc)},                    // 0xD0 8t-20t
    instruction {disas: "POP DE", cycles: 12, execute: FnEnum::OpLen1(CPU::pop_de)},                   // 0xD1
    instruction {disas: "JP NC, 0x%04X", cycles: 12, execute: FnEnum::OpLen3(CPU::jp_nc)},             // 0xD2 12t-16t
    instruction {disas: "UNDEFINED", cycles: 0, execute: FnEnum::OpLen1(CPU::UNDEFINED)},              // 0xD3
    instruction {disas: "CALL NC, 0x%04X", cycles: 12, execute: FnEnum::OpLen3(CPU::call_nc)},         // 0xD4 12t-24t
    instruction {disas: "PUSH DE", cycles: 16, execute: FnEnum::OpLen1(CPU::push_de)},                 // 0xD5
    instruction {disas: "SUB A, 0x%02X", cycles: 8, execute: FnEnum::OpLen2(sub_a_n)},            // 0xD6
    instruction {disas: "RST 10h", cycles: 16, execute: FnEnum::OpLen1(CPU::rst10)},                   // 0xD7
    instruction {disas: "RET C", cycles: 8, execute: FnEnum::OpLen1(CPU::ret_c)},                      // 0xD8 8t-20t
    instruction {disas: "RETI", cycles: 16, execute: FnEnum::OpLen1(CPU::reti)},                       // 0xD9
    instruction {disas: "JP C, 0x%04X", cycles: 12, execute: FnEnum::OpLen3(CPU::jp_c)},               // 0xDA 12t-24t
    instruction {disas: "UNDEFINED", cycles: 0, execute: FnEnum::OpLen1(CPU::UNDEFINED)},              // 0xDB
    instruction {disas: "CALL C, 0x%04X", cycles: 12, execute: FnEnum::OpLen3(CPU::call_c)},           // 0xDC 12t-24t
    instruction {disas: "UNDEFINED", cycles: 0, execute: FnEnum::OpLen1(CPU::UNDEFINED)},              // 0xDD
    instruction {disas: "SBC A, 0x%02X", cycles: 8, execute: FnEnum::OpLen2(sbc_a_n)},            // 0xDE
    instruction {disas: "RST 18h", cycles: 16, execute: FnEnum::OpLen1(CPU::rst18)},                   // 0xDF
    instruction {disas: "LD (FF00 + 0x%02X), A", cycles: 12, execute: FnEnum::OpLen2(ld_np_a)},   // 0xE0
    instruction {disas: "POP HL", cycles: 12, execute: FnEnum::OpLen1(CPU::pop_hl)},                   // 0xE1
    instruction {disas: "LD (FF00+C), A", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_ffcp_a)},         // 0xE2
    instruction {disas: "UNDEFINED", cycles: 0, execute: FnEnum::OpLen1(CPU::UNDEFINED)},              // 0xE3
    instruction {disas: "UNDEFINED", cycles: 0, execute: FnEnum::OpLen1(CPU::UNDEFINED)},              // 0xE4
    instruction {disas: "PUSH HL", cycles: 16, execute: FnEnum::OpLen1(CPU::push_hl)},                 // 0xE5
    instruction {disas: "AND A, 0x%02X", cycles: 8, execute: FnEnum::OpLen2(and_n)},              // 0xE6
    instruction {disas: "RST 20h", cycles: 16, execute: FnEnum::OpLen1(CPU::rst20)},                   // 0xE7
    instruction {disas: "ADD SP, 0x%02X", cycles: 16, execute: FnEnum::OpLen2(add_sp_n)},         // 0xE8
    instruction {disas: "JP HL", cycles: 4, execute: FnEnum::OpLen1(CPU::jp_hl)},                      // 0xE9
    instruction {disas: "LD (0x%04X), A", cycles: 16, execute: FnEnum::OpLen3(CPU::ld_nnp_a)},         // 0xEA
    instruction {disas: "UNDEFINED", cycles: 0, execute: FnEnum::OpLen1(CPU::UNDEFINED)},              // 0xEB
    instruction {disas: "UNDEFINED", cycles: 0, execute: FnEnum::OpLen1(CPU::UNDEFINED)},              // 0xEC
    instruction {disas: "UNDEFINED", cycles: 0, execute: FnEnum::OpLen1(CPU::UNDEFINED)},              // 0xED
    instruction {disas: "XOR A, 0x%02X", cycles: 8, execute: FnEnum::OpLen2(xor_n)},              // 0xEE
    instruction {disas: "RST 28h", cycles: 16, execute: FnEnum::OpLen1(CPU::rst28)},                   // 0xEF
    instruction {disas: "LD A, (FF00 + 0x%02X)", cycles: 12, execute: FnEnum::OpLen2(ld_a_np)},   // 0xF0
    instruction {disas: "POP AF", cycles: 12, execute: FnEnum::OpLen1(CPU::pop_af)},                   // 0xF1
    instruction {disas: "LD A, (FF00 + C)", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_a_ffcp)},       // 0xF2
    instruction {disas: "DI", cycles: 4, execute: FnEnum::OpLen1(CPU::di)},                            // 0xF3
    instruction {disas: "UNDEFINED", cycles: 0, execute: FnEnum::OpLen1(CPU::UNDEFINED)},                   // 0xF4
    instruction {disas: "PUSH AF", cycles: 16, execute: FnEnum::OpLen1(CPU::push_af)},                 // 0xF5
    instruction {disas: "OR A, 0x%02X", cycles: 8, execute: FnEnum::OpLen2(or_n)},                // 0xF6
    instruction {disas: "RST 30h", cycles: 16, execute: FnEnum::OpLen1(CPU::rst30)},                   // 0xF7
    instruction {disas: "LD HL, SP + 0x%02X", cycles: 12, execute: FnEnum::OpLen2(ld_hl_spn)},    // 0xF8
    instruction {disas: "LD SP, HL", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_sp_hl)},               // 0xF9
    instruction {disas: "LD A, (0x%04X)", cycles: 16, execute: FnEnum::OpLen3(CPU::ld_a_nnp)},         // 0xFA
    instruction {disas: "EI", cycles: 4, execute: FnEnum::OpLen1(CPU::ei)},                            // 0xFB
    instruction {disas: "UNDEFINED", cycles: 0, execute: FnEnum::OpLen1(CPU::UNDEFINED)},              // 0xFC
    instruction {disas: "UNDEFINED", cycles: 0, execute: FnEnum::OpLen1(CPU::UNDEFINED)},              // 0xFD
    instruction {disas: "CP A, 0x%02X", cycles: 8, execute: FnEnum::OpLen2(cp_n)},                // 0xFE
    instruction {disas: "RST 38h", cycles: 16, execute: FnEnum::OpLen1(CPU::rst38)},                    // 0xFF
];

/*
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
};*/
