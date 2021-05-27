#pragma once
#ifndef CPU_H
#define CPU_H

#include <stdint.h>
//#include "ops_struct.h"

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

extern struct registers_t registers;

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

void daa();
void cpl();

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
void ld_hl_spn(int8_t value);
void ldi_hlp_a();
void ld_h_n(uint8_t value);
void ldi_a_hlp();
void ld_l_n(uint8_t value);
void ld_sp_nn(uint16_t value);
void ldd_hlp_a();
void ld_hlp_n(uint8_t value);
void ldd_a_hlp();
void ld_a_n(uint8_t value);
void ld_a_ffcp();
void ld_ffcp_a();


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

void push_nn(uint16_t nn);
void push_af();
void push_bc();
void push_de();
void push_hl();
void pop_rr(uint16_t* rr);
void pop_af(); // flags (AF)
void pop_bc();
void pop_de();
void pop_hl();
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
void add_sp_n(int8_t n);

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

void rla();
void rlca();
void rra();
void rrca();

void jp_nn(uint16_t address);
void jp_hl();
void jp_z(uint16_t address);
void jp_nz(uint16_t address);
void jp_c(uint16_t address);
void jp_nc(uint16_t address);
void jr_nn(int8_t address);
void jr_c(int8_t offset);
void jr_nc(int8_t offset);
void jr_z(int8_t offset);
void jr_nz(int8_t offset);
void call_nn(uint16_t address);
void call_nz(uint16_t address);
void call_nc(uint16_t address);
void call_z(uint16_t address);
void call_c(uint16_t address);
void ret();
void ret_c();
void ret_nc();
void ret_z();
void ret_nz();




void cb(uint8_t opcode);
/////////// CB Prefix Instructions BEGIN

void rlc_r(uint8_t* r);
void rlc_a();
void rlc_b();
void rlc_c();
void rlc_d();
void rlc_e();
void rlc_h();
void rlc_l();
void rlc_hlp();

void rrc_r(uint8_t* r);
void rrc_a();
void rrc_b();
void rrc_c();
void rrc_d();
void rrc_e();
void rrc_h();
void rrc_l();
void rrc_hlp();

void rl_r(uint8_t* r);
void rl_a();
void rl_b();
void rl_c();
void rl_d();
void rl_e();
void rl_h();
void rl_l();
void rl_hlp();

void rr_r(uint8_t* r);
void rr_a();
void rr_b();
void rr_c();
void rr_d();
void rr_e();
void rr_h();
void rr_l();
void rr_hlp();

void sla_r(uint8_t* r);
void sla_a();
void sla_b();
void sla_c();
void sla_d();
void sla_e();
void sla_h();
void sla_l();
void sla_hlp();

void sra_r(uint8_t* r);
void sra_a();
void sra_b();
void sra_c();
void sra_d();
void sra_e();
void sra_h();
void sra_l();
void sra_hlp();

void swap_r(uint8_t* r);
void swap_a();
void swap_b();
void swap_c();
void swap_d();
void swap_e();
void swap_h();
void swap_l();
void swap_hlp();

void srl_r(uint8_t* r);
void srl_a();
void srl_b();
void srl_c();
void srl_d();
void srl_e();
void srl_h();
void srl_l();
void srl_hlp();


void bit_n_r(uint8_t bit, uint8_t* r);
void bit_0_a();
void bit_0_b();
void bit_0_c();
void bit_0_d();
void bit_0_e();
void bit_0_h();
void bit_0_l();
void bit_0_hlp();

void bit_1_a();
void bit_1_b();
void bit_1_c();
void bit_1_d();
void bit_1_e();
void bit_1_h();
void bit_1_l();
void bit_1_hlp();

void bit_2_a();
void bit_2_b();
void bit_2_c();
void bit_2_d();
void bit_2_e();
void bit_2_h();
void bit_2_l();
void bit_2_hlp();

void bit_3_a();
void bit_3_b();
void bit_3_c();
void bit_3_d();
void bit_3_e();
void bit_3_h();
void bit_3_l();
void bit_3_hlp();

void bit_4_a();
void bit_4_b();
void bit_4_c();
void bit_4_d();
void bit_4_e();
void bit_4_h();
void bit_4_l();
void bit_4_hlp();

void bit_5_a();
void bit_5_b();
void bit_5_c();
void bit_5_d();
void bit_5_e();
void bit_5_h();
void bit_5_l();
void bit_5_hlp();

void bit_6_a();
void bit_6_b();
void bit_6_c();
void bit_6_d();
void bit_6_e();
void bit_6_h();
void bit_6_l();
void bit_6_hlp();

void bit_7_a();
void bit_7_b();
void bit_7_c();
void bit_7_d();
void bit_7_e();
void bit_7_h();
void bit_7_l();
void bit_7_hlp();


void res_n_r(uint8_t bit, uint8_t* r);
void res_0_a();
void res_0_b();
void res_0_c();
void res_0_d();
void res_0_e();
void res_0_h();
void res_0_l();
void res_0_hlp();

void res_1_a();
void res_1_b();
void res_1_c();
void res_1_d();
void res_1_e();
void res_1_h();
void res_1_l();
void res_1_hlp();

void res_2_a();
void res_2_b();
void res_2_c();
void res_2_d();
void res_2_e();
void res_2_h();
void res_2_l();
void res_2_hlp();

void res_3_a();
void res_3_b();
void res_3_c();
void res_3_d();
void res_3_e();
void res_3_h();
void res_3_l();
void res_3_hlp();

void res_4_a();
void res_4_b();
void res_4_c();
void res_4_d();
void res_4_e();
void res_4_h();
void res_4_l();
void res_4_hlp();

void res_5_a();
void res_5_b();
void res_5_c();
void res_5_d();
void res_5_e();
void res_5_h();
void res_5_l();
void res_5_hlp();

void res_6_a();
void res_6_b();
void res_6_c();
void res_6_d();
void res_6_e();
void res_6_h();
void res_6_l();
void res_6_hlp();

void res_7_a();
void res_7_b();
void res_7_c();
void res_7_d();
void res_7_e();
void res_7_h();
void res_7_l();
void res_7_hlp();


void set_n_r(uint8_t bit, uint8_t* r);
void set_0_a();
void set_0_b();
void set_0_c();
void set_0_d();
void set_0_e();
void set_0_h();
void set_0_l();
void set_0_hlp();

void set_1_a();
void set_1_b();
void set_1_c();
void set_1_d();
void set_1_e();
void set_1_h();
void set_1_l();
void set_1_hlp();

void set_2_a();
void set_2_b();
void set_2_c();
void set_2_d();
void set_2_e();
void set_2_h();
void set_2_l();
void set_2_hlp();

void set_3_a();
void set_3_b();
void set_3_c();
void set_3_d();
void set_3_e();
void set_3_h();
void set_3_l();
void set_3_hlp();

void set_4_a();
void set_4_b();
void set_4_c();
void set_4_d();
void set_4_e();
void set_4_h();
void set_4_l();
void set_4_hlp();

void set_5_a();
void set_5_b();
void set_5_c();
void set_5_d();
void set_5_e();
void set_5_h();
void set_5_l();
void set_5_hlp();

void set_6_a();
void set_6_b();
void set_6_c();
void set_6_d();
void set_6_e();
void set_6_h();
void set_6_l();
void set_6_hlp();

void set_7_a();
void set_7_b();
void set_7_c();
void set_7_d();
void set_7_e();
void set_7_h();
void set_7_l();
void set_7_hlp();

/////////// CB Prefix Instructions END


void undefined();
void unimplemented(uint8_t opcode);

void print_registers();

#endif