// Instruction information, including function pointers
pub enum FnEnum { // based on opcode length
    STOP, // end execution
    UNDEFINED, // undefined opcode: panic or nop?
    OpLen1(fn(&mut CPU)),
    OpLen2(fn(&mut CPU, u8)),
    OpLen2i(fn(&mut CPU, i8)),
    OpLen3(fn(&mut CPU, u16)),
}

pub struct Instruction {
	pub disas:&'static str,
    pub cycles:u32, // This is master clock cycles (aka dots i think). t mode on gbops table
	pub execute:FnEnum,
}

//extern struct registers_t registers;
use crate::cpu::*;

// https://izik1.github.io/gbops/
pub const INSTRUCTIONS: &'static [Instruction/*; 256*/] = &[
    Instruction {disas: "NOP", cycles: 4, execute: FnEnum::OpLen1(CPU::nop)},                      // 0x00
    Instruction {disas: "LD BC, {:x}", cycles: 12, execute: FnEnum::OpLen3(CPU::ld_bc_nn)},          // 0x01
    Instruction {disas: "LD (BC), A", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_bcp_a)},              // 0x02
    Instruction {disas: "INC BC", cycles: 8, execute: FnEnum::OpLen1(CPU::inc_bc)},                    // 0x03
    Instruction {disas: "INC B", cycles: 4, execute: FnEnum::OpLen1(CPU::inc_b)},                      // 0x04
    Instruction {disas: "DEC B", cycles: 4, execute: FnEnum::OpLen1(CPU::dec_b)},                      // 0x05
    Instruction {disas: "LD B, 0x%02X", cycles: 8, execute: FnEnum::OpLen2(CPU::ld_b_n)},              // 0x06
    Instruction {disas: "RLCA", cycles: 4, execute: FnEnum::OpLen1(CPU::rlca)},                        // 0x07
    Instruction {disas: "LD (0x%04X), SP", cycles: 20, execute: FnEnum::OpLen3(CPU::ld_nnp_sp)},       // 0x08
    Instruction {disas: "ADD HL, BC", cycles: 8, execute: FnEnum::OpLen1(CPU::add_hl_bc)},             // 0x09
    Instruction {disas: "LD A, (BC)", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_a_bcp)},              // 0x0A
    Instruction {disas: "DEC BC", cycles: 8, execute: FnEnum::OpLen1(CPU::dec_bc)},                    // 0x0B
    Instruction {disas: "INC C", cycles: 4, execute: FnEnum::OpLen1(CPU::inc_c)},                      // 0x0C
    Instruction {disas: "DEC C", cycles: 4, execute: FnEnum::OpLen1(CPU::dec_c)},                      // 0x0D
    Instruction {disas: "LD C, 0x%02X", cycles: 8, execute: FnEnum::OpLen2(CPU::ld_c_n)},              // 0x0E
    Instruction {disas: "RRCA", cycles: 4, execute: FnEnum::OpLen1(CPU::rrca)},                        // 0x0F
    Instruction {disas: "STOP", cycles: 4, execute: FnEnum::STOP},                        // 0x10
    Instruction {disas: "LD DE, 0x%04X", cycles: 12, execute: FnEnum::OpLen3(CPU::ld_de_nn)},           // 0x11
    Instruction {disas: "LD (DE), A", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_dep_a)},              // 0x12
    Instruction {disas: "INC DE", cycles: 8, execute: FnEnum::OpLen1(CPU::inc_de)},                    // 0x13
    Instruction {disas: "INC D", cycles: 4, execute: FnEnum::OpLen1(CPU::inc_d)},                      // 0x14
    Instruction {disas: "DEC D", cycles: 4, execute: FnEnum::OpLen1(CPU::dec_d)},                      // 0x15
    Instruction {disas: "LD D, 0x%02X", cycles: 8, execute: FnEnum::OpLen2(CPU::ld_d_n)},              // 0x16
    Instruction {disas: "RLA", cycles: 4, execute: FnEnum::OpLen1(CPU::rla)},                          // 0x17
    Instruction {disas: "JR 0x%02X", cycles: 12, execute: FnEnum::OpLen2i(CPU::jr_nn)},                 // 0x18
    Instruction {disas: "ADD HL, DE", cycles: 8, execute: FnEnum::OpLen1(CPU::add_hl_de)},             // 0x19
    Instruction {disas: "LD A, (DE)", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_a_dep)},              // 0x1A
    Instruction {disas: "DEC DE", cycles: 8, execute: FnEnum::OpLen1(CPU::dec_de)},                    // 0x1B
    Instruction {disas: "INC E", cycles: 4, execute: FnEnum::OpLen1(CPU::inc_e)},                      // 0x1C
    Instruction {disas: "DEC E", cycles: 4, execute: FnEnum::OpLen1(CPU::dec_e)},                      // 0x1D
    Instruction {disas: "LD E, 0x%02X", cycles: 8, execute: FnEnum::OpLen2(CPU::ld_e_n)},              // 0x1E
    Instruction {disas: "RRA", cycles: 4, execute: FnEnum::OpLen1(CPU::rra)},                          // 0x1F
    Instruction {disas: "JR NZ, 0x%02X", cycles: 8, execute: FnEnum::OpLen2i(CPU::jr_nz)},              // 0x20  CYCLES VARIER 8t-12t (4 additional cycles for branch)
    Instruction {disas: "LD HL, 0x%04X", cycles: 12, execute: FnEnum::OpLen3(CPU::ld_hl_nn)},          // 0x21
    Instruction {disas: "LDI (HL), A", cycles: 8, execute: FnEnum::OpLen1(CPU::ldi_hlp_a)},            // 0x22
    Instruction {disas: "INC HL", cycles: 8, execute: FnEnum::OpLen1(CPU::inc_hl)},                    // 0x23
    Instruction {disas: "INC H", cycles: 4, execute: FnEnum::OpLen1(CPU::inc_h)},                      // 0x24
    Instruction {disas: "DEC H", cycles: 4, execute: FnEnum::OpLen1(CPU::dec_h)},                      // 0x25
    Instruction {disas: "LD H, 0x%02X", cycles: 8, execute: FnEnum::OpLen2(CPU::ld_h_n)},              // 0x26
    Instruction {disas: "DAA", cycles: 4, execute: FnEnum::OpLen1(CPU::daa)},                          // 0x27
    Instruction {disas: "JR Z, 0x%02X", cycles: 8, execute: FnEnum::OpLen2i(CPU::jr_z)},                // 0x28  8t-12t
    Instruction {disas: "ADD HL, HL", cycles: 8, execute: FnEnum::OpLen1(CPU::add_hl_hl)},             // 0x29
    Instruction {disas: "LDI A, (HL)", cycles: 8, execute: FnEnum::OpLen1(CPU::ldi_a_hlp)},            // 0x2A
    Instruction {disas: "DEC HL", cycles: 8, execute: FnEnum::OpLen1(CPU::dec_hl)},                    // 0x2B
    Instruction {disas: "INC L", cycles: 4, execute: FnEnum::OpLen1(CPU::inc_l)},                      // 0x2C
    Instruction {disas: "DEC L", cycles: 4, execute: FnEnum::OpLen1(CPU::dec_l)},                      // 0x2D
    Instruction {disas: "LD L, 0x%02X", cycles: 8, execute: FnEnum::OpLen2(CPU::ld_l_n)},              // 0x2E
    Instruction {disas: "CPL", cycles: 4, execute: FnEnum::OpLen1(CPU::cpl)},                          // 0x2F
    Instruction {disas: "JR NC, 0x%02X", cycles: 8, execute: FnEnum::OpLen2i(CPU::jr_nc)},              // 0x30 8t-12t
    Instruction {disas: "LD SP,0x%04X", cycles: 12, execute: FnEnum::OpLen3(CPU::ld_sp_nn)},           // 0x31
    Instruction {disas: "LDD (HL), A", cycles: 8, execute: FnEnum::OpLen1(CPU::ldd_hlp_a)},            // 0x32
    Instruction {disas: "INC SP", cycles: 8, execute: FnEnum::OpLen1(CPU::inc_sp)},                    // 0x33
    Instruction {disas: "INC (HL)", cycles: 12, execute: FnEnum::OpLen1(CPU::inc_hlp)},                // 0x34
    Instruction {disas: "DEC (HL)", cycles: 12, execute: FnEnum::OpLen1(CPU::dec_hlp)},                // 0x35
    Instruction {disas: "LD (HL), 0x%02X", cycles: 12, execute: FnEnum::OpLen2(CPU::ld_hlp_n)},        // 0x36
    Instruction {disas: "SCF", cycles: 4, execute: FnEnum::OpLen1(CPU::scf)},                          // 0x37
    Instruction {disas: "JR C, 0x%02X", cycles: 8, execute: FnEnum::OpLen2i(CPU::jr_c)},                // 0x38 8t-12t
    Instruction {disas: "ADD HL, SP", cycles: 8, execute: FnEnum::OpLen1(CPU::add_hl_sp)},             // 0x39
    Instruction {disas: "LDD A,(HL)", cycles: 8, execute: FnEnum::OpLen1(CPU::ldd_a_hlp)},             // 0x3A
    Instruction {disas: "DEC SP", cycles: 8, execute: FnEnum::OpLen1(CPU::dec_sp)},                    // 0x3B
    Instruction {disas: "INC A", cycles: 4, execute: FnEnum::OpLen1(CPU::inc_a)},                      // 0x3C
    Instruction {disas: "DEC A", cycles: 4, execute: FnEnum::OpLen1(CPU::dec_a)},                      // 0x3D
    Instruction {disas: "LD A, 0x%02X", cycles: 8, execute: FnEnum::OpLen2(CPU::ld_a_n)},              // 0x3E
    Instruction {disas: "CCF", cycles: 4, execute: FnEnum::OpLen1(CPU::ccf)},                          // 0x3F
    Instruction {disas: "LD B, B", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_b_b)},                   // 0x40
    Instruction {disas: "LD B, C", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_b_c)},                   // 0x41
    Instruction {disas: "LD B, D", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_b_d)},                   // 0x42
    Instruction {disas: "LD B, E", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_b_e)},                   // 0x43
    Instruction {disas: "LD B, H", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_b_h)},                   // 0x44
    Instruction {disas: "LD B, L", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_b_l)},                   // 0x45
    Instruction {disas: "LD B, (HL)", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_b_hlp)},              // 0x46
    Instruction {disas: "LD B, A", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_b_a)},                   // 0x47
    Instruction {disas: "LD C, B", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_c_b)},                   // 0x48
    Instruction {disas: "LD C, C", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_c_c)},                   // 0x49
    Instruction {disas: "LD C, D", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_c_d)},                   // 0x4A
    Instruction {disas: "LD C, E", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_c_e)},                   // 0x4B
    Instruction {disas: "LD C, H", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_c_h)},                   // 0x4C
    Instruction {disas: "LD C, L", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_c_l)},                   // 0x4D
    Instruction {disas: "LD C, (HL)", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_c_hlp)},              // 0x4E
    Instruction {disas: "LD C, A", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_c_a)},                   // 0x4F
    Instruction {disas: "LD D, B", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_d_b)},                   // 0x50
    Instruction {disas: "LD D, C", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_d_c)},                   // 0x51
    Instruction {disas: "LD D, D", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_d_d)},                   // 0x52
    Instruction {disas: "LD D, E", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_d_e)},                   // 0x53
    Instruction {disas: "LD D, H", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_d_h)},                   // 0x54
    Instruction {disas: "LD D, L", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_d_l)},                   // 0x55
    Instruction {disas: "LD D, (HL)", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_d_hlp)},              // 0x56
    Instruction {disas: "LD D, A", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_d_a)},                   // 0x57
    Instruction {disas: "LD E, B", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_e_b)},                   // 0x58
    Instruction {disas: "LD E, C", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_e_c)},                   // 0x59
    Instruction {disas: "LD E, D", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_e_d)},                   // 0x5A
    Instruction {disas: "LD E, E", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_e_e)},                   // 0x5B
    Instruction {disas: "LD E, H", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_e_h)},                   // 0x5C
    Instruction {disas: "LD E, L", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_e_l)},                   // 0x5D
    Instruction {disas: "LD E, (HL)", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_e_hlp)},              // 0x5E
    Instruction {disas: "LD E, A", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_e_a)},                   // 0x5F
    Instruction {disas: "LD H, B", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_h_b)},                   // 0x60
    Instruction {disas: "LD H, C", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_h_c)},                   // 0x61
    Instruction {disas: "LD H, D", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_h_d)},                   // 0x62
    Instruction {disas: "LD H, E", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_h_e)},                   // 0x63
    Instruction {disas: "LD H, H", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_h_h)},                   // 0x64
    Instruction {disas: "LD H, L", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_h_l)},                   // 0x65
    Instruction {disas: "LD H, (HL)", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_h_hlp)},              // 0x66
    Instruction {disas: "LD H, A", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_h_a)},                   // 0x67
    Instruction {disas: "LD L, B", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_l_b)},                   // 0x68
    Instruction {disas: "LD L, C", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_l_c)},                   // 0x69
    Instruction {disas: "LD L, D", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_l_d)},                   // 0x6A
    Instruction {disas: "LD L, E", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_l_e)},                   // 0x6B
    Instruction {disas: "LD L, H", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_l_h)},                   // 0x6C
    Instruction {disas: "LD L, L", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_l_l)},                   // 0x6D
    Instruction {disas: "LD L, (HL)", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_l_hlp)},              // 0x6E
    Instruction {disas: "LD L, A", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_l_a)},                   // 0x6F
    Instruction {disas: "LD (HL), B", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_hlp_b)},              // 0x70
    Instruction {disas: "LD (HL), C", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_hlp_c)},              // 0x71
    Instruction {disas: "LD (HL), D", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_hlp_d)},              // 0x72
    Instruction {disas: "LD (HL), E", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_hlp_e)},              // 0x73
    Instruction {disas: "LD (HL), H", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_hlp_h)},              // 0x74
    Instruction {disas: "LD (HL), L", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_hlp_l)},              // 0x75
    Instruction {disas: "HALT", cycles: 4, execute: FnEnum::OpLen1(CPU::halt)},                        // 0x76
    Instruction {disas: "LD (HL), A", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_hlp_a)},              // 0x77
    Instruction {disas: "LD A, B", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_a_b)},                   // 0x78
    Instruction {disas: "LD A, C", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_a_c)},                   // 0x79
    Instruction {disas: "LD A, D", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_a_d)},                   // 0x7A
    Instruction {disas: "LD A, E", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_a_e)},                   // 0x7B
    Instruction {disas: "LD A, H", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_a_h)},                   // 0x7C
    Instruction {disas: "LD A, L", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_a_l)},                   // 0x7D
    Instruction {disas: "LD A, (HL)", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_a_hlp)},              // 0x7E
    Instruction {disas: "LD A, A", cycles: 4, execute: FnEnum::OpLen1(CPU::ld_a_a)},                   // 0x7F
    Instruction {disas: "ADD A, B", cycles: 4, execute: FnEnum::OpLen1(CPU::add_a_b)},                 // 0x80
    Instruction {disas: "ADD A, C", cycles: 4, execute: FnEnum::OpLen1(CPU::add_a_c)},                 // 0x81
    Instruction {disas: "ADD A, D", cycles: 4, execute: FnEnum::OpLen1(CPU::add_a_d)},                 // 0x82
    Instruction {disas: "ADD A, E", cycles: 4, execute: FnEnum::OpLen1(CPU::add_a_e)},                 // 0x83
    Instruction {disas: "ADD A, H", cycles: 4, execute: FnEnum::OpLen1(CPU::add_a_h)},                 // 0x84
    Instruction {disas: "ADD A, L", cycles: 4, execute: FnEnum::OpLen1(CPU::add_a_l)},                 // 0x85
    Instruction {disas: "ADD A, (HL)", cycles: 8, execute: FnEnum::OpLen1(CPU::add_a_hlp)},            // 0x86
    Instruction {disas: "ADD A, A", cycles: 4, execute: FnEnum::OpLen1(CPU::add_a_a)},                 // 0x87
    Instruction {disas: "ADC A, B", cycles: 4, execute: FnEnum::OpLen1(CPU::adc_a_b)},                 // 0x88
    Instruction {disas: "ADC A, C", cycles: 4, execute: FnEnum::OpLen1(CPU::adc_a_c)},                 // 0x89
    Instruction {disas: "ADC A, D", cycles: 4, execute: FnEnum::OpLen1(CPU::adc_a_d)},                 // 0x8A
    Instruction {disas: "ADC A, E", cycles: 4, execute: FnEnum::OpLen1(CPU::adc_a_e)},                 // 0x8B
    Instruction {disas: "ADC A, H", cycles: 4, execute: FnEnum::OpLen1(CPU::adc_a_h)},                 // 0x8C
    Instruction {disas: "ADC A, L", cycles: 4, execute: FnEnum::OpLen1(CPU::adc_a_l)},                 // 0x8D
    Instruction {disas: "ADC A, (HL)", cycles: 8, execute: FnEnum::OpLen1(CPU::adc_a_hlp)},            // 0x8E
    Instruction {disas: "ADC A, A", cycles: 4, execute: FnEnum::OpLen1(CPU::adc_a_a)},                 // 0x8F
    Instruction {disas: "SUB A, B", cycles: 4, execute: FnEnum::OpLen1(CPU::sub_a_b)},                 // 0x90
    Instruction {disas: "SUB A, C", cycles: 4, execute: FnEnum::OpLen1(CPU::sub_a_c)},                 // 0x91
    Instruction {disas: "SUB A, D", cycles: 4, execute: FnEnum::OpLen1(CPU::sub_a_d)},                 // 0x92
    Instruction {disas: "SUB A, E", cycles: 4, execute: FnEnum::OpLen1(CPU::sub_a_e)},                 // 0x93
    Instruction {disas: "SUB A, H", cycles: 4, execute: FnEnum::OpLen1(CPU::sub_a_h)},                 // 0x94
    Instruction {disas: "SUB A, L", cycles: 4, execute: FnEnum::OpLen1(CPU::sub_a_l)},                 // 0x95
    Instruction {disas: "SUB A, (HL)", cycles: 8, execute: FnEnum::OpLen1(CPU::sub_a_hlp)},            // 0x96
    Instruction {disas: "SUB A, A", cycles: 4, execute: FnEnum::OpLen1(CPU::sub_a_a)},                 // 0x97
    Instruction {disas: "SBC A, B", cycles: 4, execute: FnEnum::OpLen1(CPU::sbc_a_b)},                 // 0x98
    Instruction {disas: "SBC A, C", cycles: 4, execute: FnEnum::OpLen1(CPU::sbc_a_c)},                 // 0x99
    Instruction {disas: "SBC A, D", cycles: 4, execute: FnEnum::OpLen1(CPU::sbc_a_d)},                 // 0x9A
    Instruction {disas: "SBC A, E", cycles: 4, execute: FnEnum::OpLen1(CPU::sbc_a_e)},                 // 0x9B
    Instruction {disas: "SBC A, H", cycles: 4, execute: FnEnum::OpLen1(CPU::sbc_a_h)},                 // 0x9C
    Instruction {disas: "SBC A, L", cycles: 4, execute: FnEnum::OpLen1(CPU::sbc_a_l)},                 // 0x9D
    Instruction {disas: "SBC A, (HL)", cycles: 8, execute: FnEnum::OpLen1(CPU::sbc_a_hlp)},            // 0x9E
    Instruction {disas: "SBC A, A", cycles: 4, execute: FnEnum::OpLen1(CPU::sbc_a_a)},                 // 0x9F
    Instruction {disas: "AND A, B", cycles: 4, execute: FnEnum::OpLen1(CPU::and_b)},                   // 0xA0
    Instruction {disas: "AND A, C", cycles: 4, execute: FnEnum::OpLen1(CPU::and_c)},                   // 0xA1
    Instruction {disas: "AND A, D", cycles: 4, execute: FnEnum::OpLen1(CPU::and_d)},                   // 0xA2
    Instruction {disas: "AND A, E", cycles: 4, execute: FnEnum::OpLen1(CPU::and_e)},                   // 0xA3
    Instruction {disas: "AND A, H", cycles: 4, execute: FnEnum::OpLen1(CPU::and_h)},                   // 0xA4
    Instruction {disas: "AND A, L", cycles: 4, execute: FnEnum::OpLen1(CPU::and_l)},                   // 0xA5
    Instruction {disas: "AND A, (HL)", cycles: 8, execute: FnEnum::OpLen1(CPU::and_hlp)},              // 0xA6
    Instruction {disas: "AND A, A", cycles: 4, execute: FnEnum::OpLen1(CPU::and_a)},                   // 0xA7
    Instruction {disas: "XOR A, B", cycles: 4, execute: FnEnum::OpLen1(CPU::xor_b)},                   // 0xA8
    Instruction {disas: "XOR A, C", cycles: 4, execute: FnEnum::OpLen1(CPU::xor_c)},                   // 0xA9
    Instruction {disas: "XOR A, D", cycles: 4, execute: FnEnum::OpLen1(CPU::xor_d)},                   // 0xAA
    Instruction {disas: "XOR A, E", cycles: 4, execute: FnEnum::OpLen1(CPU::xor_e)},                   // 0xAB
    Instruction {disas: "XOR A, H", cycles: 4, execute: FnEnum::OpLen1(CPU::xor_h)},                   // 0xAC
    Instruction {disas: "XOR A, L", cycles: 4, execute: FnEnum::OpLen1(CPU::xor_l)},                   // 0xAD
    Instruction {disas: "XOR A, (HL)", cycles: 8, execute: FnEnum::OpLen1(CPU::xor_hlp)},              // 0xAE
    Instruction {disas: "XOR A, A", cycles: 4, execute: FnEnum::OpLen1(CPU::xor_a)},                   // 0xAF
    Instruction {disas: "OR A, B", cycles: 4, execute: FnEnum::OpLen1(CPU::or_b)},                     // 0xB0
    Instruction {disas: "OR A, C", cycles: 4, execute: FnEnum::OpLen1(CPU::or_c)},                     // 0xB1
    Instruction {disas: "OR A, D", cycles: 4, execute: FnEnum::OpLen1(CPU::or_d)},                     // 0xB2
    Instruction {disas: "OR A, E", cycles: 4, execute: FnEnum::OpLen1(CPU::or_e)},                     // 0xB3
    Instruction {disas: "OR A, H", cycles: 4, execute: FnEnum::OpLen1(CPU::or_h)},                     // 0xB4
    Instruction {disas: "OR A, L", cycles: 4, execute: FnEnum::OpLen1(CPU::or_l)},                     // 0xB5
    Instruction {disas: "OR A, (HL)", cycles: 8, execute: FnEnum::OpLen1(CPU::or_hlp)},                // 0xB6
    Instruction {disas: "OR A, A", cycles: 4, execute: FnEnum::OpLen1(CPU::or_a)},                     // 0xB7
    Instruction {disas: "CP A, B", cycles: 4, execute: FnEnum::OpLen1(CPU::cp_b)},                     // 0xB8
    Instruction {disas: "CP A, C", cycles: 4, execute: FnEnum::OpLen1(CPU::cp_c)},                     // 0xB9
    Instruction {disas: "CP A, D", cycles: 4, execute: FnEnum::OpLen1(CPU::cp_d)},                     // 0xBA
    Instruction {disas: "CP A, E", cycles: 4, execute: FnEnum::OpLen1(CPU::cp_e)},                     // 0xBB
    Instruction {disas: "CP A, H", cycles: 4, execute: FnEnum::OpLen1(CPU::cp_h)},                     // 0xBC
    Instruction {disas: "CP A, L", cycles: 4, execute: FnEnum::OpLen1(CPU::cp_l)},                     // 0xBD
    Instruction {disas: "CP A, (HL)", cycles: 8, execute: FnEnum::OpLen1(CPU::cp_hlp)},                // 0xBE
    Instruction {disas: "CP A, A", cycles: 3, execute: FnEnum::OpLen1(CPU::cp_a)},                     // 0xBF
    Instruction {disas: "RET NZ", cycles: 8, execute: FnEnum::OpLen1(CPU::ret_nz)},                    // 0xC0  8t-20t
    Instruction {disas: "POP BC", cycles: 12, execute: FnEnum::OpLen1(CPU::pop_bc)},                   // 0xC1
    Instruction {disas: "JP NZ, 0x%04X", cycles: 12, execute: FnEnum::OpLen3(CPU::jp_nz)},             // 0xC2 12t-16t
    Instruction {disas: "JP 0x%04X", cycles: 16, execute: FnEnum::OpLen3(CPU::jp_nn)},                 // 0xC3
    Instruction {disas: "CALL NZ", cycles: 12, execute: FnEnum::OpLen3(CPU::call_nz)},                 // 0xC4 12t-24t
    Instruction {disas: "PUSH BC", cycles: 16, execute: FnEnum::OpLen1(CPU::push_bc)},                 // 0xC5
    Instruction {disas: "ADD A, 0x%02X", cycles: 8, execute: FnEnum::OpLen2(CPU::add_a_n)},            // 0xC6
    Instruction {disas: "RST 00h", cycles: 16, execute: FnEnum::OpLen1(CPU::rst00)},                   // 0xC7
    Instruction {disas: "RET Z", cycles: 8, execute: FnEnum::OpLen1(CPU::ret_z)},                      // 0xC8 8t-20t
    Instruction {disas: "RET", cycles: 16, execute: FnEnum::OpLen1(CPU::ret)},                         // 0xC9
    Instruction {disas: "JP Z, 0x%04X", cycles: 12, execute: FnEnum::OpLen3(CPU::jp_z)},               // 0xCA 12t-16t
    Instruction {disas: "PREFIX CB", cycles: 0, execute: FnEnum::OpLen2(CPU::cb)},                     // 0xCB
    Instruction {disas: "CALL Z, 0x%04X", cycles: 12, execute: FnEnum::OpLen3(CPU::call_z)},           // 0xCC 12t-24t
    Instruction {disas: "CALL 0x%04X", cycles: 24, execute: FnEnum::OpLen3(CPU::call_nn)},             // 0xCD
    Instruction {disas: "ADC A, 0x%02X", cycles: 8, execute: FnEnum::OpLen2(CPU::adc_a_n)},            // 0xCE
    Instruction {disas: "RST 08h", cycles: 16, execute: FnEnum::OpLen1(CPU::rst08)},                   // 0xCF
    Instruction {disas: "RET NC", cycles: 8, execute: FnEnum::OpLen1(CPU::ret_nc)},                    // 0xD0 8t-20t
    Instruction {disas: "POP DE", cycles: 12, execute: FnEnum::OpLen1(CPU::pop_de)},                   // 0xD1
    Instruction {disas: "JP NC, 0x%04X", cycles: 12, execute: FnEnum::OpLen3(CPU::jp_nc)},             // 0xD2 12t-16t
    Instruction {disas: "UNDEFINED", cycles: 0, execute: FnEnum::UNDEFINED},              // 0xD3
    Instruction {disas: "CALL NC, 0x%04X", cycles: 12, execute: FnEnum::OpLen3(CPU::call_nc)},         // 0xD4 12t-24t
    Instruction {disas: "PUSH DE", cycles: 16, execute: FnEnum::OpLen1(CPU::push_de)},                 // 0xD5
    Instruction {disas: "SUB A, 0x%02X", cycles: 8, execute: FnEnum::OpLen2(CPU::sub_a_n)},            // 0xD6
    Instruction {disas: "RST 10h", cycles: 16, execute: FnEnum::OpLen1(CPU::rst10)},                   // 0xD7
    Instruction {disas: "RET C", cycles: 8, execute: FnEnum::OpLen1(CPU::ret_c)},                      // 0xD8 8t-20t
    Instruction {disas: "RETI", cycles: 16, execute: FnEnum::OpLen1(CPU::reti)},                       // 0xD9
    Instruction {disas: "JP C, 0x%04X", cycles: 12, execute: FnEnum::OpLen3(CPU::jp_c)},               // 0xDA 12t-24t
    Instruction {disas: "UNDEFINED", cycles: 0, execute: FnEnum::UNDEFINED},              // 0xDB
    Instruction {disas: "CALL C, 0x%04X", cycles: 12, execute: FnEnum::OpLen3(CPU::call_c)},           // 0xDC 12t-24t
    Instruction {disas: "UNDEFINED", cycles: 0, execute: FnEnum::UNDEFINED},              // 0xDD
    Instruction {disas: "SBC A, 0x%02X", cycles: 8, execute: FnEnum::OpLen2(CPU::sbc_a_n)},            // 0xDE
    Instruction {disas: "RST 18h", cycles: 16, execute: FnEnum::OpLen1(CPU::rst18)},                   // 0xDF
    Instruction {disas: "LD (FF00 + 0x%02X), A", cycles: 12, execute: FnEnum::OpLen2(CPU::ld_np_a)},   // 0xE0
    Instruction {disas: "POP HL", cycles: 12, execute: FnEnum::OpLen1(CPU::pop_hl)},                   // 0xE1
    Instruction {disas: "LD (FF00+C), A", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_ffcp_a)},         // 0xE2
    Instruction {disas: "UNDEFINED", cycles: 0, execute: FnEnum::UNDEFINED},              // 0xE3
    Instruction {disas: "UNDEFINED", cycles: 0, execute: FnEnum::UNDEFINED},              // 0xE4
    Instruction {disas: "PUSH HL", cycles: 16, execute: FnEnum::OpLen1(CPU::push_hl)},                 // 0xE5
    Instruction {disas: "AND A, 0x%02X", cycles: 8, execute: FnEnum::OpLen2(CPU::and_n)},              // 0xE6
    Instruction {disas: "RST 20h", cycles: 16, execute: FnEnum::OpLen1(CPU::rst20)},                   // 0xE7
    Instruction {disas: "ADD SP, 0x%02X", cycles: 16, execute: FnEnum::OpLen2(CPU::add_sp_n)},         // 0xE8
    Instruction {disas: "JP HL", cycles: 4, execute: FnEnum::OpLen1(CPU::jp_hl)},                      // 0xE9
    Instruction {disas: "LD (0x%04X), A", cycles: 16, execute: FnEnum::OpLen3(CPU::ld_nnp_a)},         // 0xEA
    Instruction {disas: "UNDEFINED", cycles: 0, execute: FnEnum::UNDEFINED},              // 0xEB
    Instruction {disas: "UNDEFINED", cycles: 0, execute: FnEnum::UNDEFINED},              // 0xEC
    Instruction {disas: "UNDEFINED", cycles: 0, execute: FnEnum::UNDEFINED},              // 0xED
    Instruction {disas: "XOR A, 0x%02X", cycles: 8, execute: FnEnum::OpLen2(CPU::xor_n)},              // 0xEE
    Instruction {disas: "RST 28h", cycles: 16, execute: FnEnum::OpLen1(CPU::rst28)},                   // 0xEF
    Instruction {disas: "LD A, (FF00 + 0x%02X)", cycles: 12, execute: FnEnum::OpLen2(CPU::ld_a_np)},   // 0xF0
    Instruction {disas: "POP AF", cycles: 12, execute: FnEnum::OpLen1(CPU::pop_af)},                   // 0xF1
    Instruction {disas: "LD A, (FF00 + C)", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_a_ffcp)},       // 0xF2
    Instruction {disas: "DI", cycles: 4, execute: FnEnum::OpLen1(CPU::di)},                            // 0xF3
    Instruction {disas: "UNDEFINED", cycles: 0, execute: FnEnum::UNDEFINED},                   // 0xF4
    Instruction {disas: "PUSH AF", cycles: 16, execute: FnEnum::OpLen1(CPU::push_af)},                 // 0xF5
    Instruction {disas: "OR A, 0x%02X", cycles: 8, execute: FnEnum::OpLen2(CPU::or_n)},                // 0xF6
    Instruction {disas: "RST 30h", cycles: 16, execute: FnEnum::OpLen1(CPU::rst30)},                   // 0xF7
    Instruction {disas: "LD HL, SP + 0x%02X", cycles: 12, execute: FnEnum::OpLen2i(CPU::ld_hl_spn)},    // 0xF8
    Instruction {disas: "LD SP, HL", cycles: 8, execute: FnEnum::OpLen1(CPU::ld_sp_hl)},               // 0xF9
    Instruction {disas: "LD A, (0x%04X)", cycles: 16, execute: FnEnum::OpLen3(CPU::ld_a_nnp)},         // 0xFA
    Instruction {disas: "EI", cycles: 4, execute: FnEnum::OpLen1(CPU::ei)},                            // 0xFB
    Instruction {disas: "UNDEFINED", cycles: 0, execute: FnEnum::UNDEFINED},              // 0xFC
    Instruction {disas: "UNDEFINED", cycles: 0, execute: FnEnum::UNDEFINED},              // 0xFD
    Instruction {disas: "CP A, 0x%02X", cycles: 8, execute: FnEnum::OpLen2(CPU::cp_n)},                // 0xFE
    Instruction {disas: "RST 38h", cycles: 16, execute: FnEnum::OpLen1(CPU::rst38)},                    // 0xFF
];


// CB instructions are all 2 bytes longs so use a different type here
//pub const instructions: &'static [instruction/*; 256*/] = &
pub const CB_INSTRUCTIONS: &'static [Instruction/*; 256*/] = &
[
    Instruction {disas: "RLC B", cycles: 8, execute: FnEnum::OpLen1(CPU::rlc_b)},
    Instruction {disas: "RLC C", cycles: 8, execute: FnEnum::OpLen1(CPU::rlc_c)},
    Instruction {disas: "RLC D", cycles: 8, execute: FnEnum::OpLen1(CPU::rlc_d)},
    Instruction {disas: "RLC E", cycles: 8, execute: FnEnum::OpLen1(CPU::rlc_e)},
    Instruction {disas: "RLC H", cycles: 8, execute: FnEnum::OpLen1(CPU::rlc_h)},
    Instruction {disas: "RLC L", cycles: 8, execute: FnEnum::OpLen1(CPU::rlc_l)},
    Instruction {disas: "RLC (HL)", cycles: 16, execute: FnEnum::OpLen1(CPU::rlc_hlp)},
    Instruction {disas: "RLC A", cycles: 8, execute: FnEnum::OpLen1(CPU::rlc_a)},

    Instruction {disas: "RRC B", cycles: 8, execute: FnEnum::OpLen1(CPU::rrc_b)},
    Instruction {disas: "RRC C", cycles: 8, execute: FnEnum::OpLen1(CPU::rrc_c)},
    Instruction {disas: "RRC D", cycles: 8, execute: FnEnum::OpLen1(CPU::rrc_d)},
    Instruction {disas: "RRC E", cycles: 8, execute: FnEnum::OpLen1(CPU::rrc_e)},
    Instruction {disas: "RRC H", cycles: 8, execute: FnEnum::OpLen1(CPU::rrc_h)},
    Instruction {disas: "RRC L", cycles: 8, execute: FnEnum::OpLen1(CPU::rrc_l)},
    Instruction {disas: "RRC (HL)", cycles: 16, execute: FnEnum::OpLen1(CPU::rrc_hlp)},
    Instruction {disas: "RRC A", cycles: 8, execute: FnEnum::OpLen1(CPU::rrc_a)},

    Instruction {disas: "RL B", cycles: 8, execute: FnEnum::OpLen1(CPU::rl_b)},
    Instruction {disas: "RL C", cycles: 8, execute: FnEnum::OpLen1(CPU::rl_c)},
    Instruction {disas: "RL D", cycles: 8, execute: FnEnum::OpLen1(CPU::rl_d)},
    Instruction {disas: "RL E", cycles: 8, execute: FnEnum::OpLen1(CPU::rl_e)},
    Instruction {disas: "RL H", cycles: 8, execute: FnEnum::OpLen1(CPU::rl_h)},
    Instruction {disas: "RL L", cycles: 8, execute: FnEnum::OpLen1(CPU::rl_l)},
    Instruction {disas: "RL (HL)", cycles: 16, execute: FnEnum::OpLen1(CPU::rl_hlp)},
    Instruction {disas: "RL A", cycles: 8, execute: FnEnum::OpLen1(CPU::rl_a)},

    Instruction {disas: "RR B", cycles: 8, execute: FnEnum::OpLen1(CPU::rr_b)},
    Instruction {disas: "RR C", cycles: 8, execute: FnEnum::OpLen1(CPU::rr_c)},
    Instruction {disas: "RR D", cycles: 8, execute: FnEnum::OpLen1(CPU::rr_d)},
    Instruction {disas: "RR E", cycles: 8, execute: FnEnum::OpLen1(CPU::rr_e)},
    Instruction {disas: "RR H", cycles: 8, execute: FnEnum::OpLen1(CPU::rr_h)},
    Instruction {disas: "RR L", cycles: 8, execute: FnEnum::OpLen1(CPU::rr_l)},
    Instruction {disas: "RR (HL)", cycles: 16, execute: FnEnum::OpLen1(CPU::rr_hlp)},
    Instruction {disas: "RR A", cycles: 8, execute: FnEnum::OpLen1(CPU::rr_a)},

    Instruction {disas: "SLA B", cycles: 8, execute: FnEnum::OpLen1(CPU::sla_b)},
    Instruction {disas: "SLA C", cycles: 8, execute: FnEnum::OpLen1(CPU::sla_c)},
    Instruction {disas: "SLA D", cycles: 8, execute: FnEnum::OpLen1(CPU::sla_d)},
    Instruction {disas: "SLA E", cycles: 8, execute: FnEnum::OpLen1(CPU::sla_e)},
    Instruction {disas: "SLA H", cycles: 8, execute: FnEnum::OpLen1(CPU::sla_h)},
    Instruction {disas: "SLA L", cycles: 8, execute: FnEnum::OpLen1(CPU::sla_l)},
    Instruction {disas: "SLA (HL)", cycles: 16, execute: FnEnum::OpLen1(CPU::sla_hlp)},
    Instruction {disas: "SLA A", cycles: 8, execute: FnEnum::OpLen1(CPU::sla_a)},

    Instruction {disas: "SRA B", cycles: 8, execute: FnEnum::OpLen1(CPU::sra_b)},
    Instruction {disas: "SRA C", cycles: 8, execute: FnEnum::OpLen1(CPU::sra_c)},
    Instruction {disas: "SRA D", cycles: 8, execute: FnEnum::OpLen1(CPU::sra_d)},
    Instruction {disas: "SRA E", cycles: 8, execute: FnEnum::OpLen1(CPU::sra_e)},
    Instruction {disas: "SRA H", cycles: 8, execute: FnEnum::OpLen1(CPU::sra_h)},
    Instruction {disas: "SRA L", cycles: 8, execute: FnEnum::OpLen1(CPU::sra_l)},
    Instruction {disas: "SRA (HL)", cycles: 16, execute: FnEnum::OpLen1(CPU::sra_hlp)},
    Instruction {disas: "SRA A", cycles: 8, execute: FnEnum::OpLen1(CPU::sra_a)},

    Instruction {disas: "SWAP B", cycles: 8, execute: FnEnum::OpLen1(CPU::swap_b)},
    Instruction {disas: "SWAP C", cycles: 8, execute: FnEnum::OpLen1(CPU::swap_c)},
    Instruction {disas: "SWAP D", cycles: 8, execute: FnEnum::OpLen1(CPU::swap_d)},
    Instruction {disas: "SWAP E", cycles: 8, execute: FnEnum::OpLen1(CPU::swap_e)},
    Instruction {disas: "SWAP H", cycles: 8, execute: FnEnum::OpLen1(CPU::swap_h)},
    Instruction {disas: "SWAP L", cycles: 8, execute: FnEnum::OpLen1(CPU::swap_l)},
    Instruction {disas: "SWAP (HL)", cycles: 16, execute: FnEnum::OpLen1(CPU::swap_hlp)},
    Instruction {disas: "SWAP A", cycles: 8, execute: FnEnum::OpLen1(CPU::swap_a)},

    Instruction {disas: "SRL B", cycles: 8, execute: FnEnum::OpLen1(CPU::srl_b)},
    Instruction {disas: "SRL C", cycles: 8, execute: FnEnum::OpLen1(CPU::srl_c)},
    Instruction {disas: "SRL D", cycles: 8, execute: FnEnum::OpLen1(CPU::srl_d)},
    Instruction {disas: "SRL E", cycles: 8, execute: FnEnum::OpLen1(CPU::srl_e)},
    Instruction {disas: "SRL H", cycles: 8, execute: FnEnum::OpLen1(CPU::srl_h)},
    Instruction {disas: "SRL L", cycles: 8, execute: FnEnum::OpLen1(CPU::srl_l)},
    Instruction {disas: "SRL (HL)", cycles: 16, execute: FnEnum::OpLen1(CPU::srl_hlp)},
    Instruction {disas: "SRL A", cycles: 8, execute: FnEnum::OpLen1(CPU::srl_a)},

    Instruction {disas: "BIT 0 B", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_0_b)},
    Instruction {disas: "BIT 0 C", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_0_c)},
    Instruction {disas: "BIT 0 D", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_0_d)},
    Instruction {disas: "BIT 0 E", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_0_e)},
    Instruction {disas: "BIT 0 H", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_0_h)},
    Instruction {disas: "BIT 0 L", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_0_l)},
    Instruction {disas: "BIT 0 (HL)", cycles: 12, execute: FnEnum::OpLen1(CPU::bit_0_hlp)},
    Instruction {disas: "BIT 0 A", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_0_a)},
    Instruction {disas: "BIT 1 B", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_1_b)},
    Instruction {disas: "BIT 1 C", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_1_c)},
    Instruction {disas: "BIT 1 D", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_1_d)},
    Instruction {disas: "BIT 1 E", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_1_e)},
    Instruction {disas: "BIT 1 H", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_1_h)},
    Instruction {disas: "BIT 1 L", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_1_l)},
    Instruction {disas: "BIT 1 (HL)", cycles: 12, execute: FnEnum::OpLen1(CPU::bit_1_hlp)},
    Instruction {disas: "BIT 1 A", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_1_a)},
    Instruction {disas: "BIT 2 B", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_2_b)},
    Instruction {disas: "BIT 2 C", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_2_c)},
    Instruction {disas: "BIT 2 D", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_2_d)},
    Instruction {disas: "BIT 2 E", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_2_e)},
    Instruction {disas: "BIT 2 H", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_2_h)},
    Instruction {disas: "BIT 2 L", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_2_l)},
    Instruction {disas: "BIT 2 (HL)", cycles: 12, execute: FnEnum::OpLen1(CPU::bit_2_hlp)},
    Instruction {disas: "BIT 2 A", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_2_a)},
    Instruction {disas: "BIT 3 B", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_3_b)},
    Instruction {disas: "BIT 3 C", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_3_c)},
    Instruction {disas: "BIT 3 D", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_3_d)},
    Instruction {disas: "BIT 3 E", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_3_e)},
    Instruction {disas: "BIT 3 H", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_3_h)},
    Instruction {disas: "BIT 3 L", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_3_l)},
    Instruction {disas: "BIT 3 (HL)", cycles: 12, execute: FnEnum::OpLen1(CPU::bit_3_hlp)},
    Instruction {disas: "BIT 3 A", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_3_a)},
    Instruction {disas: "BIT 4 B", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_4_b)},
    Instruction {disas: "BIT 4 C", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_4_c)},
    Instruction {disas: "BIT 4 D", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_4_d)},
    Instruction {disas: "BIT 4 E", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_4_e)},
    Instruction {disas: "BIT 4 H", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_4_h)},
    Instruction {disas: "BIT 4 L", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_4_l)},
    Instruction {disas: "BIT 4 (HL).", cycles: 12, execute: FnEnum::OpLen1(CPU::bit_4_hlp)},
    Instruction {disas: "BIT 4 A", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_4_a)},
    Instruction {disas: "BIT 5 B", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_5_b)},
    Instruction {disas: "BIT 5 C", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_5_c)},
    Instruction {disas: "BIT 5 D", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_5_d)},
    Instruction {disas: "BIT 5 E", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_5_e)},
    Instruction {disas: "BIT 5 H", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_5_h)},
    Instruction {disas: "BIT 5 L", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_5_l)},
    Instruction {disas: "BIT 5 (HL)", cycles: 12, execute: FnEnum::OpLen1(CPU::bit_5_hlp)},
    Instruction {disas: "BIT 5 A", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_5_a)},
    Instruction {disas: "BIT 6 B", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_6_b)},
    Instruction {disas: "BIT 6 C", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_6_c)},
    Instruction {disas: "BIT 6 D", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_6_d)},
    Instruction {disas: "BIT 6 E", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_6_e)},
    Instruction {disas: "BIT 6 H", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_6_h)},
    Instruction {disas: "BIT 6 L", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_6_l)},
    Instruction {disas: "BIT 6 (HL)", cycles: 12, execute: FnEnum::OpLen1(CPU::bit_6_hlp)},
    Instruction {disas: "BIT 6 A", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_6_a)},
    Instruction {disas: "BIT 7 B", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_7_b)},
    Instruction {disas: "BIT 7 C", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_7_c)},
    Instruction {disas: "BIT 7 D", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_7_d)},
    Instruction {disas: "BIT 7 E", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_7_e)},
    Instruction {disas: "BIT 7 H", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_7_h)},
    Instruction {disas: "BIT 7 L", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_7_l)},
    Instruction {disas: "BIT 7 (HL)", cycles: 12, execute: FnEnum::OpLen1(CPU::bit_7_hlp)},
    Instruction {disas: "BIT 7 A", cycles: 8, execute: FnEnum::OpLen1(CPU::bit_7_a)},

    Instruction {disas: "RES 0 B", cycles: 8, execute: FnEnum::OpLen1(CPU::res_0_b)},
    Instruction {disas: "RES 0 C", cycles: 8, execute: FnEnum::OpLen1(CPU::res_0_c)},
    Instruction {disas: "RES 0 D", cycles: 8, execute: FnEnum::OpLen1(CPU::res_0_d)},
    Instruction {disas: "RES 0 E", cycles: 8, execute: FnEnum::OpLen1(CPU::res_0_e)},
    Instruction {disas: "RES 0 H", cycles: 8, execute: FnEnum::OpLen1(CPU::res_0_h)},
    Instruction {disas: "RES 0 L", cycles: 8, execute: FnEnum::OpLen1(CPU::res_0_l)},
    Instruction {disas: "RES 0 (HL)", cycles: 16, execute: FnEnum::OpLen1(CPU::res_0_hlp)},
    Instruction {disas: "RES 0 A", cycles: 8, execute: FnEnum::OpLen1(CPU::res_0_a)},
    Instruction {disas: "RES 1 B", cycles: 8, execute: FnEnum::OpLen1(CPU::res_1_b)},
    Instruction {disas: "RES 1 C", cycles: 8, execute: FnEnum::OpLen1(CPU::res_1_c)},
    Instruction {disas: "RES 1 D", cycles: 8, execute: FnEnum::OpLen1(CPU::res_1_d)},
    Instruction {disas: "RES 1 E", cycles: 8, execute: FnEnum::OpLen1(CPU::res_1_e)},
    Instruction {disas: "RES 1 H", cycles: 8, execute: FnEnum::OpLen1(CPU::res_1_h)},
    Instruction {disas: "RES 1 L", cycles: 8, execute: FnEnum::OpLen1(CPU::res_1_l)},
    Instruction {disas: "RES 1 (HL)", cycles: 16, execute: FnEnum::OpLen1(CPU::res_1_hlp)},
    Instruction {disas: "RES 1 A", cycles: 8, execute: FnEnum::OpLen1(CPU::res_1_a)},
    Instruction {disas: "RES 2 B", cycles: 8, execute: FnEnum::OpLen1(CPU::res_2_b)},
    Instruction {disas: "RES 2 C", cycles: 8, execute: FnEnum::OpLen1(CPU::res_2_c)},
    Instruction {disas: "RES 2 D", cycles: 8, execute: FnEnum::OpLen1(CPU::res_2_d)},
    Instruction {disas: "RES 2 E", cycles: 8, execute: FnEnum::OpLen1(CPU::res_2_e)},
    Instruction {disas: "RES 2 H", cycles: 8, execute: FnEnum::OpLen1(CPU::res_2_h)},
    Instruction {disas: "RES 2 L", cycles: 8, execute: FnEnum::OpLen1(CPU::res_2_l)},
    Instruction {disas: "RES 2 (HL)", cycles: 16, execute: FnEnum::OpLen1(CPU::res_2_hlp)},
    Instruction {disas: "RES 2 A", cycles: 8, execute: FnEnum::OpLen1(CPU::res_2_a)},
    Instruction {disas: "RES 3 B", cycles: 8, execute: FnEnum::OpLen1(CPU::res_3_b)},
    Instruction {disas: "RES 3 C", cycles: 8, execute: FnEnum::OpLen1(CPU::res_3_c)},
    Instruction {disas: "RES 3 D", cycles: 8, execute: FnEnum::OpLen1(CPU::res_3_d)},
    Instruction {disas: "RES 3 E", cycles: 8, execute: FnEnum::OpLen1(CPU::res_3_e)},
    Instruction {disas: "RES 3 H", cycles: 8, execute: FnEnum::OpLen1(CPU::res_3_h)},
    Instruction {disas: "RES 3 L", cycles: 8, execute: FnEnum::OpLen1(CPU::res_3_l)},
    Instruction {disas: "RES 3 (HL)", cycles: 16, execute: FnEnum::OpLen1(CPU::res_3_hlp)},
    Instruction {disas: "RES 3 A", cycles: 8, execute: FnEnum::OpLen1(CPU::res_3_a)},
    Instruction {disas: "RES 4 B", cycles: 8, execute: FnEnum::OpLen1(CPU::res_4_b)},
    Instruction {disas: "RES 4 C", cycles: 8, execute: FnEnum::OpLen1(CPU::res_4_c)},
    Instruction {disas: "RES 4 D", cycles: 8, execute: FnEnum::OpLen1(CPU::res_4_d)},
    Instruction {disas: "RES 4 E", cycles: 8, execute: FnEnum::OpLen1(CPU::res_4_e)},
    Instruction {disas: "RES 4 H", cycles: 8, execute: FnEnum::OpLen1(CPU::res_4_h)},
    Instruction {disas: "RES 4 L", cycles: 8, execute: FnEnum::OpLen1(CPU::res_4_l)},
    Instruction {disas: "RES 4 (HL)", cycles: 16, execute: FnEnum::OpLen1(CPU::res_4_hlp)},
    Instruction {disas: "RES 4 A", cycles: 8, execute: FnEnum::OpLen1(CPU::res_4_a)},
    Instruction {disas: "RES 5 B", cycles: 8, execute: FnEnum::OpLen1(CPU::res_5_b)},
    Instruction {disas: "RES 5 C", cycles: 8, execute: FnEnum::OpLen1(CPU::res_5_c)},
    Instruction {disas: "RES 5 D", cycles: 8, execute: FnEnum::OpLen1(CPU::res_5_d)},
    Instruction {disas: "RES 5 E", cycles: 8, execute: FnEnum::OpLen1(CPU::res_5_e)},
    Instruction {disas: "RES 5 H", cycles: 8, execute: FnEnum::OpLen1(CPU::res_5_h)},
    Instruction {disas: "RES 5 L", cycles: 8, execute: FnEnum::OpLen1(CPU::res_5_l)},
    Instruction {disas: "RES 5 (HL)", cycles: 16, execute: FnEnum::OpLen1(CPU::res_5_hlp)},
    Instruction {disas: "RES 5 A", cycles: 8, execute: FnEnum::OpLen1(CPU::res_5_a)},
    Instruction {disas: "RES 6 B", cycles: 8, execute: FnEnum::OpLen1(CPU::res_6_b)},
    Instruction {disas: "RES 6 C", cycles: 8, execute: FnEnum::OpLen1(CPU::res_6_c)},
    Instruction {disas: "RES 6 D", cycles: 8, execute: FnEnum::OpLen1(CPU::res_6_d)},
    Instruction {disas: "RES 6 E", cycles: 8, execute: FnEnum::OpLen1(CPU::res_6_e)},
    Instruction {disas: "RES 6 H", cycles: 8, execute: FnEnum::OpLen1(CPU::res_6_h)},
    Instruction {disas: "RES 6 L", cycles: 8, execute: FnEnum::OpLen1(CPU::res_6_l)},
    Instruction {disas: "RES 6 (HL)", cycles: 16, execute: FnEnum::OpLen1(CPU::res_6_hlp)},
    Instruction {disas: "RES 6 A", cycles: 8, execute: FnEnum::OpLen1(CPU::res_6_a)},
    Instruction {disas: "RES 7 B", cycles: 8, execute: FnEnum::OpLen1(CPU::res_7_b)},
    Instruction {disas: "RES 7 C", cycles: 8, execute: FnEnum::OpLen1(CPU::res_7_c)},
    Instruction {disas: "RES 7 D", cycles: 8, execute: FnEnum::OpLen1(CPU::res_7_d)},
    Instruction {disas: "RES 7 E", cycles: 8, execute: FnEnum::OpLen1(CPU::res_7_e)},
    Instruction {disas: "RES 7 H", cycles: 8, execute: FnEnum::OpLen1(CPU::res_7_h)},
    Instruction {disas: "RES 7 L", cycles: 8, execute: FnEnum::OpLen1(CPU::res_7_l)},
    Instruction {disas: "RES 7 (HL)", cycles: 16, execute: FnEnum::OpLen1(CPU::res_7_hlp)},
    Instruction {disas: "RES 7 A", cycles: 8, execute: FnEnum::OpLen1(CPU::res_7_a)},

    Instruction {disas: "SET 0 B", cycles: 8, execute: FnEnum::OpLen1(CPU::set_0_b)},
    Instruction {disas: "SET 0 C", cycles: 8, execute: FnEnum::OpLen1(CPU::set_0_c)},
    Instruction {disas: "SET 0 D", cycles: 8, execute: FnEnum::OpLen1(CPU::set_0_d)},
    Instruction {disas: "SET 0 E", cycles: 8, execute: FnEnum::OpLen1(CPU::set_0_e)},
    Instruction {disas: "SET 0 H", cycles: 8, execute: FnEnum::OpLen1(CPU::set_0_h)},
    Instruction {disas: "SET 0 L", cycles: 8, execute: FnEnum::OpLen1(CPU::set_0_l)},
    Instruction {disas: "SET 0 (HL)", cycles: 16, execute: FnEnum::OpLen1(CPU::set_0_hlp)},
    Instruction {disas: "SET 0 A", cycles: 8, execute: FnEnum::OpLen1(CPU::set_0_a)},
    Instruction {disas: "SET 1 B", cycles: 8, execute: FnEnum::OpLen1(CPU::set_1_b)},
    Instruction {disas: "SET 1 C", cycles: 8, execute: FnEnum::OpLen1(CPU::set_1_c)},
    Instruction {disas: "SET 1 D", cycles: 8, execute: FnEnum::OpLen1(CPU::set_1_d)},
    Instruction {disas: "SET 1 E", cycles: 8, execute: FnEnum::OpLen1(CPU::set_1_e)},
    Instruction {disas: "SET 1 H", cycles: 8, execute: FnEnum::OpLen1(CPU::set_1_h)},
    Instruction {disas: "SET 1 L", cycles: 8, execute: FnEnum::OpLen1(CPU::set_1_l)},
    Instruction {disas: "SET 1 (HL)", cycles: 16, execute: FnEnum::OpLen1(CPU::set_1_hlp)},
    Instruction {disas: "SET 1 A", cycles: 8, execute: FnEnum::OpLen1(CPU::set_1_a)},
    Instruction {disas: "SET 2 B", cycles: 8, execute: FnEnum::OpLen1(CPU::set_2_b)},
    Instruction {disas: "SET 2 C", cycles: 8, execute: FnEnum::OpLen1(CPU::set_2_c)},
    Instruction {disas: "SET 2 D", cycles: 8, execute: FnEnum::OpLen1(CPU::set_2_d)},
    Instruction {disas: "SET 2 E", cycles: 8, execute: FnEnum::OpLen1(CPU::set_2_e)},
    Instruction {disas: "SET 2 H", cycles: 8, execute: FnEnum::OpLen1(CPU::set_2_h)},
    Instruction {disas: "SET 2 L", cycles: 8, execute: FnEnum::OpLen1(CPU::set_2_l)},
    Instruction {disas: "SET 2 (HL)", cycles: 16, execute: FnEnum::OpLen1(CPU::set_2_hlp)},
    Instruction {disas: "SET 2 A", cycles: 8, execute: FnEnum::OpLen1(CPU::set_2_a)},
    Instruction {disas: "SET 3 B", cycles: 8, execute: FnEnum::OpLen1(CPU::set_3_b)},
    Instruction {disas: "SET 3 C", cycles: 8, execute: FnEnum::OpLen1(CPU::set_3_c)},
    Instruction {disas: "SET 3 D", cycles: 8, execute: FnEnum::OpLen1(CPU::set_3_d)},
    Instruction {disas: "SET 3 E", cycles: 8, execute: FnEnum::OpLen1(CPU::set_3_e)},
    Instruction {disas: "SET 3 H", cycles: 8, execute: FnEnum::OpLen1(CPU::set_3_h)},
    Instruction {disas: "SET 3 L", cycles: 8, execute: FnEnum::OpLen1(CPU::set_3_l)},
    Instruction {disas: "SET 3 (HL)", cycles: 16, execute: FnEnum::OpLen1(CPU::set_3_hlp)},
    Instruction {disas: "SET 3 A", cycles: 8, execute: FnEnum::OpLen1(CPU::set_3_a)},
    Instruction {disas: "SET 4 B", cycles: 8, execute: FnEnum::OpLen1(CPU::set_4_b)},
    Instruction {disas: "SET 4 C", cycles: 8, execute: FnEnum::OpLen1(CPU::set_4_c)},
    Instruction {disas: "SET 4 D", cycles: 8, execute: FnEnum::OpLen1(CPU::set_4_d)},
    Instruction {disas: "SET 4 E", cycles: 8, execute: FnEnum::OpLen1(CPU::set_4_e)},
    Instruction {disas: "SET 4 H", cycles: 8, execute: FnEnum::OpLen1(CPU::set_4_h)},
    Instruction {disas: "SET 4 L", cycles: 8, execute: FnEnum::OpLen1(CPU::set_4_l)},
    Instruction {disas: "SET 4 (HL)", cycles: 16, execute: FnEnum::OpLen1(CPU::set_4_hlp)},
    Instruction {disas: "SET 4 A", cycles: 8, execute: FnEnum::OpLen1(CPU::set_4_a)},
    Instruction {disas: "SET 5 B", cycles: 8, execute: FnEnum::OpLen1(CPU::set_5_b)},
    Instruction {disas: "SET 5 C", cycles: 8, execute: FnEnum::OpLen1(CPU::set_5_c)},
    Instruction {disas: "SET 5 D", cycles: 8, execute: FnEnum::OpLen1(CPU::set_5_d)},
    Instruction {disas: "SET 5 E", cycles: 8, execute: FnEnum::OpLen1(CPU::set_5_e)},
    Instruction {disas: "SET 5 H", cycles: 8, execute: FnEnum::OpLen1(CPU::set_5_h)},
    Instruction {disas: "SET 5 L", cycles: 8, execute: FnEnum::OpLen1(CPU::set_5_l)},
    Instruction {disas: "SET 5 (HL)", cycles: 16, execute: FnEnum::OpLen1(CPU::set_5_hlp)},
    Instruction {disas: "SET 5 A", cycles: 8, execute: FnEnum::OpLen1(CPU::set_5_a)},
    Instruction {disas: "SET 6 B", cycles: 8, execute: FnEnum::OpLen1(CPU::set_6_b)},
    Instruction {disas: "SET 6 C", cycles: 8, execute: FnEnum::OpLen1(CPU::set_6_c)},
    Instruction {disas: "SET 6 D", cycles: 8, execute: FnEnum::OpLen1(CPU::set_6_d)},
    Instruction {disas: "SET 6 E", cycles: 8, execute: FnEnum::OpLen1(CPU::set_6_e)},
    Instruction {disas: "SET 6 H", cycles: 8, execute: FnEnum::OpLen1(CPU::set_6_h)},
    Instruction {disas: "SET 6 L", cycles: 8, execute: FnEnum::OpLen1(CPU::set_6_l)},
    Instruction {disas: "SET 6 (HL)", cycles: 16, execute: FnEnum::OpLen1(CPU::set_6_hlp)},
    Instruction {disas: "SET 6 A", cycles: 8, execute: FnEnum::OpLen1(CPU::set_6_a)},
    Instruction {disas: "SET 7 B", cycles: 8, execute: FnEnum::OpLen1(CPU::set_7_b)},
    Instruction {disas: "SET 7 C", cycles: 8, execute: FnEnum::OpLen1(CPU::set_7_c)},
    Instruction {disas: "SET 7 D", cycles: 8, execute: FnEnum::OpLen1(CPU::set_7_d)},
    Instruction {disas: "SET 7 E", cycles: 8, execute: FnEnum::OpLen1(CPU::set_7_e)},
    Instruction {disas: "SET 7 H", cycles: 8, execute: FnEnum::OpLen1(CPU::set_7_h)},
    Instruction {disas: "SET 7 L", cycles: 8, execute: FnEnum::OpLen1(CPU::set_7_l)},
    Instruction {disas: "SET 7 (HL)", cycles: 16, execute: FnEnum::OpLen1(CPU::set_7_hlp)},
    Instruction {disas: "SET 7 A", cycles: 8, execute: FnEnum::OpLen1(CPU::set_7_a)}
];
