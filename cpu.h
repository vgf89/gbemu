#pragma once
#ifndef CPU_H
#define CPU_H

#include <stdint.h>

#define FLAGS_ZERO (1 << 7)
#define FLAGS_NEGATIVE (1 << 6)
#define FLAGS_HALFCARRY (1 << 5)
#define FLAGS_CARRY (1 << 4)

#define FLAGS_ISZERO (registers.f & FLAGS_ZERO)
#define FLAGS_ISNEGATIVE (registers.f & FLAGS_NEGATIVE)
#define FLAGS_ISCARRY (registers.f & FLAGS_CARRY)
#define FLAGS_ISHALFCARRY (registers.f & FLAGS_HALFCARRY)

#define FLAGS_ISSET(x) (registers.f & (x))
#define FLAGS_SET(x) (registers.f |= (x))
#define FLAGS_CLEAR(x) (registers.f &= ~(x))

struct registers_t {
	struct {
		union {
			struct {
				uint8_t f; // Flags
				uint8_t a; // Accumulator
			};
			uint16_t af;
		};
	};
	
	struct {
		union {
			struct {
				uint8_t c;
				uint8_t b;
			};
			uint16_t bc;
		};
	};
	
	struct {
		union {
			struct {
				uint8_t e;
				uint8_t d;
			};
			uint16_t de;
		};
	};
	
	struct {
		union {
			struct {
				uint8_t l;
				uint8_t h;
			};
			uint16_t hl;
		};
	};
	
	uint16_t sp;
	uint16_t pc;
};

extern uint8_t IME_flag;

void reset_cpu_clock(uint16_t maxclock);
void cpuStep();

void reset_flags();

void nop();

void di();
void ei();

void inc_bc();
void inc_de();
void inc_hl();
void inc_sp();

void inc_b();
void inc_d();
void inc_h();
void inc_hlp();

void reset_inc_flags();

void dec_b();
void dec_d();
void dec_h();
void dec_hlp();

void dec_bc();
void dec_de();
void dec_hl();
void dec_sp();

void inc_c();
void inc_e();
void inc_l();
void inc_a();

void dec_c();
void dec_e();
void dec_l();
void dec_a();

void reset_dec_flags();

void xor_n(uint8_t n);
void xor_b();
void xor_c();
void xor_d();
void xor_e();
void xor_h();
void xor_l();
void xor_hlp();
void xor_a();

void or_n(uint8_t n);
void or_a();
void or_b();
void or_c();
void or_d();
void or_e();
void or_h();
void or_l();
void or_hlp();

void set_or_flags();

void and_n(uint8_t n);
void and_a();
void and_b();
void and_c();
void and_d();
void and_e();
void and_h();
void and_l();
void and_hlp();

void cp_n(uint8_t);
void cp_a();
void cp_b();
void cp_c();
void cp_d();
void cp_e();
void cp_h();
void cp_l();
void cp_hlp();

void ld_bc_nn(uint16_t value);
void ld_bcp_a();
void ld_b_n(uint8_t value);
void ld_nnp_sp(uint16_t address);
void ld_nnp_a(uint16_t address);
void ld_a_nnp(uint16_t address);
void ld_np_a(uint8_t address);
void ld_a_np(uint8_t address);
void ld_a_bcp();
void ld_c_n(uint8_t value);
void ld_de_nn(uint16_t value);
void ld_dep_a();
void ld_d_n(uint8_t value);
void ld_a_dep();
void ld_e_n(uint8_t value);
void ld_hl_nn(uint16_t value);
void ldi_hlp_a();
void ld_h_n(uint8_t value);
void ldi_a_hlp();
void ld_l_n(uint8_t value);
void ld_sp_nn(uint16_t value);
void ldd_hlp_a();
void ld_hlp_n(uint8_t value);
void ldd_a_hlp();
void ld_a_n(uint8_t value);


void ld_a_a();  
void ld_a_b();  
void ld_a_c();  
void ld_a_d();  
void ld_a_e();  
void ld_a_h();  
void ld_a_l();  
void ld_b_a();  
void ld_b_b();  
void ld_b_c();  
void ld_b_d();  
void ld_b_e();  
void ld_b_h();  
void ld_b_l();  
void ld_c_a();  
void ld_c_b();  
void ld_c_c();  
void ld_c_d();  
void ld_c_e();  
void ld_c_h();  
void ld_c_l();  
void ld_d_a();  
void ld_d_b();  
void ld_d_c();  
void ld_d_d();  
void ld_d_e();  
void ld_d_h();  
void ld_d_l();  
void ld_e_a();  
void ld_e_b();  
void ld_e_c();  
void ld_e_d();  
void ld_e_e();  
void ld_e_h();  
void ld_e_l();  
void ld_h_a();  
void ld_h_b();  
void ld_h_c();  
void ld_h_d();  
void ld_h_e();  
void ld_h_h();  
void ld_h_l();  
void ld_l_a();  
void ld_l_b();  
void ld_l_c();  
void ld_l_d();  
void ld_l_e();  
void ld_l_h();  
void ld_l_l();  
void ld_sp_hl();
void ld_a_hlp();
void ld_b_hlp();
void ld_c_hlp();
void ld_d_hlp();
void ld_e_hlp();
void ld_h_hlp();
void ld_l_hlp();
void ld_hlp_a();
void ld_hlp_b();
void ld_hlp_c();
void ld_hlp_d();
void ld_hlp_e();
void ld_hlp_h();
void ld_hlp_l();
//    {"halt", 1, null},               // 0x76

void add_a_n(uint8_t n);
void add_a_a();
void add_a_b();
void add_a_c();
void add_a_d();
void add_a_e();
void add_a_h();
void add_a_l();
void add_a_hlp();

void add_hl_bc();
void add_hl_de();
void add_hl_hl();
void add_hl_sp();
void add_sp_n();

void adc_a_n(uint8_t n);
void adc_a_a();
void adc_a_b();
void adc_a_c();
void adc_a_d();
void adc_a_e();
void adc_a_h();
void adc_a_l();
void adc_a_hlp();

void sub_a_n(uint8_t n);
void sub_a_a();
void sub_a_b();
void sub_a_c();
void sub_a_d();
void sub_a_e();
void sub_a_h();
void sub_a_l();
void sub_a_hlp();

void sbc_a_n(uint8_t n);
void sbc_a_a();
void sbc_a_b();
void sbc_a_c();
void sbc_a_d();
void sbc_a_e();
void sbc_a_h();
void sbc_a_l();
void sbc_a_hlp();

void rlca();
void rra();

void jp_nn(uint16_t address);
void jp_nc(uint16_t address);
void jr_nz(int8_t offset);


void undefined();
void unimplemented(uint8_t opcode);

void print_registers();

#endif