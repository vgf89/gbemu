#pragma once
#ifndef CPU_H
#define CPU_H

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
				unsigned char f; // Flags
				unsigned char a; // Accumulator
			};
			unsigned short af;
		};
	};
	
	struct {
		union {
			struct {
				unsigned char c;
				unsigned char b;
			};
			unsigned short bc;
		};
	};
	
	struct {
		union {
			struct {
				unsigned char e;
				unsigned char d;
			};
			unsigned short de;
		};
	};
	
	struct {
		union {
			struct {
				unsigned char l;
				unsigned char h;
			};
			unsigned short hl;
		};
	};
	
	unsigned short sp;
	unsigned short pc;
};

void loadRom(char* rompath);
void reset();
void cpuStep();

void nop();


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

void xor_b();
void xor_c();
void xor_d();
void xor_e();
void xor_h();
void xor_l();
void xor_hlp();
void xor_a();

void or_a_a();
void or_a_b();
void or_a_c();
void or_a_d();
void or_a_e();
void or_a_h();
void or_a_l();
void or_a_hlp();

void set_or_flags();


void ld_bc_nn(unsigned short value);
void ld_bcp_a();
void ld_b_n(unsigned char value);
void ld_nnp_sp(unsigned short address);
void ld_nnp_a(unsigned short address);
void ld_a_nnp(unsigned short address);
void ld_a_bcp();
void ld_c_n(unsigned char value);
void ld_de_nn(unsigned short value);
void ld_dep_a();
void ld_d_n(unsigned char value);
void ld_a_dep();
void ld_e_n(unsigned char value);
void ld_hl_nn(unsigned short value);
void ldi_hlp_a();
void ld_h_n(unsigned char value);
void ldi_a_hlp();
void ld_l_n(unsigned char value);
void ld_sp_nn(unsigned short value);
void ldd_hlp_a();
void ld_hlp_n(unsigned char value);
void ldd_a_hlp();
void ld_a_n(unsigned char value);


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


void rra();








void jp_nn(unsigned short address);
void jr_nz(unsigned char offset);


void undefined(unsigned char opcode, unsigned char pc);
void unimplemented(unsigned char opcode);

void print_registers();

#endif