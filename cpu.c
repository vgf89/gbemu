#include "cpu.h"
#include "memory.h"
#include "ops_struct.h"
#include <stdio.h>
#include <stdlib.h>

struct registers_t registers = {0};

extern union memory_t memory;

extern uint32_t clock;

uint32_t cpuclock;

uint8_t IME_flag = 1;
uint8_t set_ei = 0;
uint8_t res_ei = 0;

uint8_t halted = 0;
uint8_t halt_bug = 0;


void reset_cpu_clock(uint16_t maxclock)
{
    cpuclock -= maxclock;
}

void cpuStep() {
    if (clock < cpuclock) {
        // Next cpu step shouldn't run yet...
        return;
    }


    if (IME_flag) {
        if (IE_ISSET(I_VBLANK) && IF_ISSET(I_VBLANK)) {
            cpuclock += 20; // This timinig may or may not be right
            if (halted || IME_flag) {
                halted = 0;
                IF_CLEAR(I_VBLANK);
                IME_flag = 0;
                call_nn(0x0040);
            }
            halted = 0;
            return;
        }
        else if (IE_ISSET(I_LCD_STAT) && IF_ISSET(I_LCD_STAT)) {
            cpuclock += 20; // This timinig may or may not be right
            if (halted || IME_flag) {
                halted = 0;
                IF_CLEAR(I_LCD_STAT);
                IME_flag = 0;
                call_nn(0x0048);
            }
            halted = 0;
            return;
        }
        else if (IE_ISSET(I_TIMER) && IF_ISSET(I_TIMER)) {
            cpuclock += 20; // This timing may or may not be right
            if (halted || IME_flag) {
                halted = 0;
                IF_CLEAR(I_TIMER);
                IME_flag = 0;
                call_nn(0x0050);
            }

            return;
        }
        else if (IE_ISSET(I_SERIAL) && IF_ISSET(I_SERIAL)) {
            cpuclock += 20; // This timinig may or may not be right
            if (halted || IME_flag) {
                halted = 0;
                IF_CLEAR(I_SERIAL);
                IME_flag = 0;
                call_nn(0x0058);
            }
            halted = 0;
            return;
        }
        else if (IE_ISSET(I_JOYPAD) && IF_ISSET(I_JOYPAD)) {
            cpuclock += 20; // This timinig may or may not be right
            if (halted || IME_flag) {
                halted = 0;
                IF_CLEAR(I_JOYPAD);
                IME_flag = 0;
                call_nn(0x0060);
            }
            halted = 0;
            return;
        }
    }

    else if (halted) {
        // Halted, but resumes execution on interrupt instead since IME == 0
        if (IF_ISSET(I_VBLANK) || IF_ISSET(I_LCD_STAT) || IF_ISSET(I_SERIAL) || IF_ISSET(I_TIMER) || IF_ISSET(I_JOYPAD)) {
            halted = 0;
        }
    }
    
    uint8_t opcode = readByte(registers.pc);
    struct instruction ins = instructions[opcode];

    uint16_t operand = 0;

    if (ins.execute == NULL) {
        unimplemented(opcode);
        return;
    }


    if (ins.opcodeLength == 2) operand = (uint16_t)readByte(registers.pc+1);
    if (ins.opcodeLength == 3) operand = readWord(registers.pc+1);


    static int bptriggered = 0;
    if (0){//opcode == 0x76) {
        bptriggered = 1;
        printf("0x%04X   Breakpoint hit. Press Enter to step execution.\n", registers.pc);
        fflush(stdout);
    }

    if (bptriggered) print_registers();

    if (bptriggered) printf("0x%04X  0x%02X  ", registers.pc, opcode);
    switch(ins.opcodeLength) {
        case 1:
            if (bptriggered) printf(ins.disas);
            if (bptriggered) printf("\n");
            break;
        case 2:
            if (bptriggered) printf(ins.disas, operand);
            if (bptriggered) printf("\n");
            break;
        case 3:
            if (bptriggered) printf(ins.disas, operand);
            if (bptriggered) printf("  (%02x)\n", readByte(operand));
            break;
    }

    if (bptriggered)
    {
        char c ;
        while ((c = getchar()) != '\n') {
            if (c == 'c') bptriggered = 0;
        }
    }

    if (halt_bug) {
        halt_bug = 0;
    } else {
        if (!halted) {
            registers.pc += ins.opcodeLength;
        }
    }

    if (halted)
    {
        cpuclock += 4;
        return;
    }

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
    //res_ei = 2;
}
void ei() {
    IME_flag = 1;
    //set_ei = 2;
}
void reti()
{
    ret();
    //ei();
    IME_flag = 1;
}

void halt() {
    if (IME_flag) {
        halted = 1;
    } else {
        if ((memory.IE & memory.IFLAGS & 0x1f) == 0) {
            halted = 1;
            // Halt mode is entered, but the interupt vector is not called
            // and IF isn't cleared (it instead just continue executing when an interrupt is received)
            // Can check this via (halted && (IME_flag == 0))
        } else {
            // Halt mode is not entered.
            // CPU does not increase pc on next instruction. IF flags aren't cleared
            halt_bug = 1;
        }
    }
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
    registers.a ^= readByte(registers.hl);
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
    or_n(registers.a);
}
void or_b() {
    or_n(registers.b);
}
void or_c() {
    or_n(registers.c);
}
void or_d() {
    or_n(registers.d);
}
void or_e() {
    or_n(registers.e);
}
void or_h() {
    or_n(registers.h);
}
void or_l() {
    or_n(registers.l);
}
void or_hlp() {
    or_n(readByte(registers.hl));
}

void and_n(uint8_t n)
{
    registers.a &= n;
    reset_flags();
    if (registers.a == 0) {
        FLAGS_SET(FLAGS_ZERO);
    }
    FLAGS_SET(FLAGS_HALFCARRY); // Why is this always set?
}
void and_a()
{
    and_n(registers.a);
}
void and_b()
{
    and_n(registers.b);
}
void and_c()
{
    and_n(registers.c);
}
void and_d()
{
    and_n(registers.d);
}
void and_e()
{
    and_n(registers.e);
}
void and_h()
{
    and_n(registers.h);
}
void and_l()
{
    and_n(registers.l);
}
void and_hlp()
{
    and_n(readByte(registers.hl));
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
    if (registers.a == n) {
        FLAGS_SET(FLAGS_ZERO);
    }
    FLAGS_SET(FLAGS_NEGATIVE);
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


void daa()
{
    // https://ehaskins.com/2018-01-30%20Z80%20DAA/
    uint8_t correction = 0;

    uint8_t flagC = 0;
    if (FLAGS_ISHALFCARRY || (!FLAGS_ISNEGATIVE && (registers.a & 0x0f) > 0x9)) {
        correction |= 0x6;
    }

    if (FLAGS_ISCARRY || (!FLAGS_ISNEGATIVE && registers.a > 0x99)) {
        correction |= 0x60;
        flagC = 1;
    }

    registers.a += FLAGS_ISNEGATIVE ? -correction : correction;

    FLAGS_CLEAR(FLAGS_ZERO);
    FLAGS_CLEAR(FLAGS_HALFCARRY);
    FLAGS_CLEAR(FLAGS_CARRY);

    if (registers.a == 0) {
        FLAGS_SET(FLAGS_ZERO);
    }
    if (flagC) {
        FLAGS_SET(FLAGS_CARRY);
    }
}

void cpl()
{
    registers.a = ~registers.a;
    FLAGS_SET(FLAGS_NEGATIVE);
    FLAGS_SET(FLAGS_HALFCARRY);
}

void scf()
{
    FLAGS_CLEAR(FLAGS_NEGATIVE);
    FLAGS_CLEAR(FLAGS_HALFCARRY);
    FLAGS_SET(FLAGS_CARRY);
}

void ccf()
{
    FLAGS_CLEAR(FLAGS_NEGATIVE);
    FLAGS_CLEAR(FLAGS_HALFCARRY);
    if (FLAGS_ISCARRY) {
        FLAGS_CLEAR(FLAGS_CARRY);
    } else {
        FLAGS_SET(FLAGS_CARRY);
    }
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
void ld_hl_spn(int8_t value)
{
    reset_flags();

    if (((registers.sp & 0xf) + (value & 0xf)) & 0x10) {
        FLAGS_SET(FLAGS_HALFCARRY);
    }

    if (((registers.sp & 0xff) + (value & 0xff)) & 0x100) {
        FLAGS_SET(FLAGS_CARRY);
    }
    registers.hl = registers.sp + value;
}


void ld_a_bcp() {
    registers.a = readByte(registers.bc);
}
void ld_a_dep() {
    registers.a = readByte(registers.de);
    
}
void ld_bcp_a() {
    writeByte(registers.bc, registers.a);
    
}
void ld_dep_a()
{
    writeByte(registers.de, registers.a);
}

void ld_hlp_n(uint8_t value) {
    writeByte(registers.hl, value);
    
}


void ldi_hlp_a() {
    writeByte(registers.hl, registers.a);
    registers.hl++;
    
}
void ldi_a_hlp() {
    registers.a = readByte(registers.hl);
    registers.hl++;
    
}

void ldd_hlp_a() {
    writeByte(registers.hl, registers.a);
    registers.hl--;
    
}
void ldd_a_hlp() {
    registers.a = readByte(registers.hl);
    registers.hl--;
    
}


void ld_nnp_sp(uint16_t address) {
    writeWord(address, registers.sp);
}
void ld_nnp_a(uint16_t address) {
    writeByte(address, registers.a);
}
void ld_a_nnp(uint16_t address) {
    registers.a = readByte(address);
}
void ld_np_a(uint8_t address)
{
    writeByte(0xFF00 + address, registers.a);
}
void ld_a_np(uint8_t address)
{
    registers.a = readByte(0xff00 + address);
}

void ld_a_ffcp()
{
    ld_a_n(readByte(0xff00 + registers.c));
}

void ld_ffcp_a()
{
    writeByte(0xff00 + registers.c, registers.a);
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
    writeWord(registers.sp, nn);
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
    (*rr) = readWord(registers.sp);
    registers.sp += 2;
}

void pop_af()
{
    pop_rr(&registers.af);
    registers.f &= 0xf0;
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

    FLAGS_CLEAR(FLAGS_NEGATIVE);
    FLAGS_CLEAR(FLAGS_HALFCARRY);
    FLAGS_CLEAR(FLAGS_CARRY);

    if ((((registers.hl & 0xfff) + (nn & 0xfff)) & 0x1000) == 0x1000)
    {
        FLAGS_SET(FLAGS_HALFCARRY);
    }
    if ((((uint32_t)registers.hl + nn) & 0x10000) == 0x10000)
    {
        FLAGS_SET(FLAGS_CARRY);
    }

    registers.hl += nn;
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

void add_sp_n(int8_t n) {
    reset_flags();

    if ((((registers.sp & 0x0f) + (n & 0x0f)) & 0x10) == 0x10)
    {
        FLAGS_SET(FLAGS_HALFCARRY);
    }
    if ((((registers.sp & 0xff) + (n & 0xff)) & 0x100) == 0x100)
    {
        FLAGS_SET(FLAGS_CARRY);
    }

    registers.sp += n;
}

void adc_a_n(uint8_t n) {
    uint8_t oldcarryflag = (FLAGS_ISCARRY != 0);
    uint8_t half_result = (registers.a & 0xf) +  (n & 0xf) + oldcarryflag;
    uint16_t full_result = registers.a + n + oldcarryflag;
    registers.a = (uint8_t)full_result; // Cast truncates the overflow bits
    reset_flags();

    if (half_result > 0xf) {
        FLAGS_SET(FLAGS_HALFCARRY);
    }
    if (full_result > 0xff) { 
        FLAGS_SET(FLAGS_CARRY);
    }
    if (registers.a == 0) {
        FLAGS_SET(FLAGS_ZERO);
    }
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

    FLAGS_SET(FLAGS_NEGATIVE);
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
    // NOTE: Carry flags are set on overflow, no matter what.
    uint8_t oldcarryflag = (FLAGS_ISCARRY != 0);
    uint8_t half_result = (registers.a & 0xf) -  (n & 0xf) - oldcarryflag;
    uint16_t full_result = registers.a - n - oldcarryflag;
    registers.a = (uint8_t)full_result; // Cast truncates the overflow bits
    reset_flags();
    FLAGS_SET(FLAGS_NEGATIVE);
    if (half_result > 0xf) {
        FLAGS_SET(FLAGS_HALFCARRY);
    }
    if (full_result > 0xff) {
        FLAGS_SET(FLAGS_CARRY);
    }
    if (registers.a == 0) {
        FLAGS_SET(FLAGS_ZERO);
    }
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


void rla()
{
    rl_a();
    FLAGS_CLEAR(FLAGS_ZERO);
    FLAGS_CLEAR(FLAGS_NEGATIVE);
    FLAGS_CLEAR(FLAGS_HALFCARRY);
}
void rlca()
{
    rlc_a();
    FLAGS_CLEAR(FLAGS_ZERO);
    FLAGS_CLEAR(FLAGS_NEGATIVE);
    FLAGS_CLEAR(FLAGS_HALFCARRY);
}
void rra() {
    rr_a();
    FLAGS_CLEAR(FLAGS_ZERO);
    FLAGS_CLEAR(FLAGS_NEGATIVE);
    FLAGS_CLEAR(FLAGS_HALFCARRY);

}
void rrca() {
    rrc_a();
    FLAGS_CLEAR(FLAGS_ZERO);
    FLAGS_CLEAR(FLAGS_NEGATIVE);
    FLAGS_CLEAR(FLAGS_HALFCARRY);
}


void jp_nn(uint16_t address)
{
    registers.pc = address;
}

void jp_hl()
{
    jp_nn(registers.hl);
}

void jp_z(uint16_t address)
{
    if (FLAGS_ISZERO) {
        registers.pc = address;
        cpuclock += 4;
    }
}

void jp_nz(uint16_t address)
{
    if (FLAGS_ISZERO == 0) {
        registers.pc = address;
        cpuclock += 4;
    }
}

void jp_c(uint16_t address)
{
    if (FLAGS_ISCARRY) {
        registers.pc = address;
        cpuclock += 4;
    }
}

void jp_nc(uint16_t address)
{
    if (FLAGS_ISCARRY == 0) {
        registers.pc = address;
        cpuclock += 4;
    }
}

void jr_c(int8_t offset)
{
    if (FLAGS_ISCARRY)
    {
        registers.pc += offset;
        cpuclock += 4;
    }
}

void jr_nc(int8_t offset)
{
    if (FLAGS_ISCARRY == 0)
    {
        registers.pc += offset;
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
    writeWord(registers.sp, registers.pc);
    registers.pc = address;
}

void call_nz(uint16_t address)
{
    if (FLAGS_ISZERO == 0) {
        call_nn(address);
        cpuclock += 12; // branch takes additional 12 cycles
    }
}

void call_nc(uint16_t address)
{
    if (FLAGS_ISCARRY == 0)
    {
        call_nn(address);
        cpuclock += 12;
    }
}

void call_z(uint16_t address)
{
    if (FLAGS_ISZERO)
    {
        call_nn(address);
        cpuclock += 12; // branch takes additional 12 cycles
    }
}

void call_c(uint16_t address)
{
    if (FLAGS_ISCARRY)
    {
        call_nn(address);
        cpuclock += 12;
    }
}

void ret(){
    registers.pc = readWord(registers.sp);
    registers.sp += 2;
}

void ret_c() {
    if (FLAGS_ISCARRY) {
        cpuclock += 12;
        ret();
    }
}

void ret_nc() {
    if (FLAGS_ISCARRY == 0) {
        cpuclock += 12;
        ret();
    }
}

void ret_z() {
    if (FLAGS_ISZERO) {
        cpuclock += 12;
        ret();
    }
}

void ret_nz() {
    if (FLAGS_ISZERO == 0) {
        cpuclock += 12;
        ret();
    }
}




void cb(uint8_t opcode)
{
    struct instruction ins = CB_instructions[opcode];
    //registers.pc += ins.opcodeLength;
    if (ins.execute == NULL) {
        //printf("unimplemented CB prefixed instruction: $%02X  ", opcode);
        //printf(ins.disas);
        exit(0);
    }

    ((void(*)())ins.execute)();
}



// CB Prefix Functions
void rlc_r(uint8_t* r) // Rotate Left
{
    uint8_t msb = ((*r) & (1<<7)) != 0;
    (*r) <<= 1;
    (*r) |= msb;

    FLAGS_CLEAR(FLAGS_ZERO);
    FLAGS_CLEAR(FLAGS_NEGATIVE);
    FLAGS_CLEAR(FLAGS_HALFCARRY);
    FLAGS_CLEAR(FLAGS_CARRY);
    if (msb) {
        FLAGS_SET(FLAGS_CARRY);
    }
    if ((*r) == 0)
    {
        FLAGS_SET(FLAGS_ZERO);
    }
}
void rl_r(uint8_t* r) // Rotate Left through carry
{
    // tmp = MSB
    uint8_t tmp = (*r) & (1 << 7);
    // r << 1
    (*r) <<= 1;
    // LSB = Carry
    (*r) |= (FLAGS_ISCARRY != 0);
    // Carry = tmp

    FLAGS_CLEAR(FLAGS_ZERO);
    FLAGS_CLEAR(FLAGS_NEGATIVE);
    FLAGS_CLEAR(FLAGS_HALFCARRY);
    FLAGS_CLEAR(FLAGS_CARRY);

    if (tmp) {
        FLAGS_SET(FLAGS_CARRY);
    }

    if ((*r) == 0) {
        FLAGS_SET(FLAGS_ZERO);
    }
}

void rrc_r(uint8_t* r) // Rotate Right
{
    uint8_t lsb = ((*r) & 1) != 0;
    (*r) >>= 1;
    (*r) |= lsb << 7;

    FLAGS_CLEAR(FLAGS_ZERO);
    FLAGS_CLEAR(FLAGS_NEGATIVE);
    FLAGS_CLEAR(FLAGS_HALFCARRY);
    FLAGS_CLEAR(FLAGS_CARRY);
    if (lsb) {
        FLAGS_SET(FLAGS_CARRY);
    }
    if ((*r) == 0)
    {
        FLAGS_SET(FLAGS_ZERO);
    }
}
void rr_r(uint8_t* r) // Rotate Right Through Carry
{
    // lsb = r.0
    uint8_t lsb = (*r) & 1;
    // r >> 1
    (*r) >>= 1;
    // MSB = Carry
    (*r) |= (FLAGS_ISCARRY != 0) << 7;
    // Carry = tmp

    FLAGS_CLEAR(FLAGS_ZERO);
    FLAGS_CLEAR(FLAGS_NEGATIVE);
    FLAGS_CLEAR(FLAGS_HALFCARRY);
    FLAGS_CLEAR(FLAGS_CARRY);

    if (lsb) {
        FLAGS_SET(FLAGS_CARRY);
    }

    if ((*r) == 0) {
        FLAGS_SET(FLAGS_ZERO);
    }
}

void sla_r(uint8_t* r)
{
    // Left shift on gameboy is r.0 = 0. This should match *unsigned* shift left in C99.
    // TODO: Verify correct behavior.
    uint8_t carry = (*r) & (1 << 7);
    (*r) <<= 1;

    FLAGS_CLEAR(FLAGS_ZERO);
    FLAGS_CLEAR(FLAGS_NEGATIVE);
    FLAGS_CLEAR(FLAGS_HALFCARRY);
    FLAGS_CLEAR(FLAGS_CARRY);

    if ((*r) == 0) {
        FLAGS_SET(FLAGS_ZERO);
    }
    if (carry) {
        FLAGS_SET(FLAGS_CARRY);
    }
}
void sra_r(uint8_t* r)
{
    // SRA on gameboy is right shift with r.7 = old r.7
    // This should match *signed* right shift in C99.
    // TODO: Verify correct behavior.
    uint8_t carry = (*r) & 1;
    *(int8_t*)r >>= 1;

    FLAGS_CLEAR(FLAGS_ZERO);
    FLAGS_CLEAR(FLAGS_NEGATIVE);
    FLAGS_CLEAR(FLAGS_HALFCARRY);
    FLAGS_CLEAR(FLAGS_CARRY);
    
    if ((*r) == 0) {
        FLAGS_SET(FLAGS_ZERO);
    }
    if (carry) {
        FLAGS_SET(FLAGS_CARRY);
    }
}
void srl_r(uint8_t* r)
{
    // SRL on gameboy is right shift with r.7 = 0
    // This should match *unsigned* shift right in C99.
    // TODO: Verify correct behavior.
    uint8_t carry = (*r) & 1;
    (*r) >>= 1;

    FLAGS_CLEAR(FLAGS_ZERO);
    FLAGS_CLEAR(FLAGS_NEGATIVE);
    FLAGS_CLEAR(FLAGS_HALFCARRY);
    FLAGS_CLEAR(FLAGS_CARRY);
    
    if ((*r) == 0) {
        FLAGS_SET(FLAGS_ZERO);
    }
    if (carry) {
        FLAGS_SET(FLAGS_CARRY);
    }
}
void swap_r(uint8_t* r)
{
    uint8_t lsb_nibble = (*r)& 0x0f;
    uint8_t msb_nibble = (*r)& 0xf0;
    (*r) = (lsb_nibble << 4) | (msb_nibble >> 4);

    FLAGS_CLEAR(FLAGS_ZERO);
    FLAGS_CLEAR(FLAGS_NEGATIVE);
    FLAGS_CLEAR(FLAGS_HALFCARRY);
    FLAGS_CLEAR(FLAGS_CARRY);

    if ((*r) == 0) {
        FLAGS_SET(FLAGS_ZERO);
    }
}
void bit_n_r(uint8_t bit, uint8_t* r)
{
    FLAGS_CLEAR(FLAGS_NEGATIVE);
    FLAGS_SET(FLAGS_HALFCARRY);

    FLAGS_CLEAR(FLAGS_ZERO);
    if (!((*r) & (1 << bit))) {
        FLAGS_SET(FLAGS_ZERO);
    }
}
void res_n_r(uint8_t bit, uint8_t* r)
{
    (*r) &= ~(1 << bit);
}
void set_n_r(uint8_t bit, uint8_t* r)
{
    (*r) |= (1 << bit);
}







void rlc_a() { rlc_r(&registers.a); }
void rlc_b() { rlc_r(&registers.b); }
void rlc_c() { rlc_r(&registers.c); }
void rlc_d() { rlc_r(&registers.d); }
void rlc_e() { rlc_r(&registers.e); }
void rlc_h() { rlc_r(&registers.h); }
void rlc_l() { rlc_r(&registers.l); }
void rlc_hlp() { rlc_r(&memory.memory[registers.hl]); }

void rrc_a() { rrc_r(&registers.a); }
void rrc_b() { rrc_r(&registers.b); }
void rrc_c() { rrc_r(&registers.c); }
void rrc_d() { rrc_r(&registers.d); }
void rrc_e() { rrc_r(&registers.e); }
void rrc_h() { rrc_r(&registers.h); }
void rrc_l() { rrc_r(&registers.l); }
void rrc_hlp() { rrc_r(&memory.memory[registers.hl]); }

void rl_a() { rl_r(&registers.a); }
void rl_b() { rl_r(&registers.b); }
void rl_c() { rl_r(&registers.c); }
void rl_d() { rl_r(&registers.d); }
void rl_e() { rl_r(&registers.e); }
void rl_h() { rl_r(&registers.h); }
void rl_l() { rl_r(&registers.l); }
void rl_hlp() { rl_r(&memory.memory[registers.hl]); }

void rr_a() { rr_r(&registers.a); }
void rr_b() { rr_r(&registers.b); }
void rr_c() { rr_r(&registers.c); }
void rr_d() { rr_r(&registers.d); }
void rr_e() { rr_r(&registers.e); }
void rr_h() { rr_r(&registers.h); }
void rr_l() { rr_r(&registers.l); }
void rr_hlp() { rr_r(&memory.memory[registers.hl]); }

void sla_a() { sla_r(&registers.a); }
void sla_b() { sla_r(&registers.b); }
void sla_c() { sla_r(&registers.c); }
void sla_d() { sla_r(&registers.d); }
void sla_e() { sla_r(&registers.e); }
void sla_h() { sla_r(&registers.h); }
void sla_l() { sla_r(&registers.l); }
void sla_hlp() { sla_r(&memory.memory[registers.hl]); }

void sra_a() { sra_r(&registers.a); }
void sra_b() { sra_r(&registers.b); }
void sra_c() { sra_r(&registers.c); }
void sra_d() { sra_r(&registers.d); }
void sra_e() { sra_r(&registers.e); }
void sra_h() { sra_r(&registers.h); }
void sra_l() { sra_r(&registers.l); }
void sra_hlp() { sra_r(&memory.memory[registers.hl]); }

void swap_a() { swap_r(&registers.a); }
void swap_b() { swap_r(&registers.b); }
void swap_c() { swap_r(&registers.c); }
void swap_d() { swap_r(&registers.d); }
void swap_e() { swap_r(&registers.e); }
void swap_h() { swap_r(&registers.h); }
void swap_l() { swap_r(&registers.l); }
void swap_hlp() { swap_r(&memory.memory[registers.hl]); }

void srl_a() { srl_r(&registers.a); }
void srl_b() { srl_r(&registers.b); }
void srl_c() { srl_r(&registers.c); }
void srl_d() { srl_r(&registers.d); }
void srl_e() { srl_r(&registers.e); }
void srl_h() { srl_r(&registers.h); }
void srl_l() { srl_r(&registers.l); }
void srl_hlp() { srl_r(&memory.memory[registers.hl]); }


void bit_n_hlp(uint8_t bit) {}

void bit_0_a() { bit_n_r(0, &registers.a); }
void bit_0_b() { bit_n_r(0, &registers.b); }
void bit_0_c() { bit_n_r(0, &registers.c); }
void bit_0_d() { bit_n_r(0, &registers.d); }
void bit_0_e() { bit_n_r(0, &registers.e); }
void bit_0_h() { bit_n_r(0, &registers.h); }
void bit_0_l() { bit_n_r(0, &registers.l); }
void bit_0_hlp() { bit_n_r(0, &memory.memory[registers.hl]); }

void bit_1_a() { bit_n_r(1, &registers.a); }
void bit_1_b() { bit_n_r(1, &registers.b); }
void bit_1_c() { bit_n_r(1, &registers.c); }
void bit_1_d() { bit_n_r(1, &registers.d); }
void bit_1_e() { bit_n_r(1, &registers.e); }
void bit_1_h() { bit_n_r(1, &registers.h); }
void bit_1_l() { bit_n_r(1, &registers.l); }
void bit_1_hlp() { bit_n_r(1, &memory.memory[registers.hl]); }

void bit_2_a() { bit_n_r(2, &registers.a); }
void bit_2_b() { bit_n_r(2, &registers.b); }
void bit_2_c() { bit_n_r(2, &registers.c); }
void bit_2_d() { bit_n_r(2, &registers.d); }
void bit_2_e() { bit_n_r(2, &registers.e); }
void bit_2_h() { bit_n_r(2, &registers.h); }
void bit_2_l() { bit_n_r(2, &registers.l); }
void bit_2_hlp() { bit_n_r(2, &memory.memory[registers.hl]); }

void bit_3_a() { bit_n_r(3, &registers.a); }
void bit_3_b() { bit_n_r(3, &registers.b); }
void bit_3_c() { bit_n_r(3, &registers.c); }
void bit_3_d() { bit_n_r(3, &registers.d); }
void bit_3_e() { bit_n_r(3, &registers.e); }
void bit_3_h() { bit_n_r(3, &registers.h); }
void bit_3_l() { bit_n_r(3, &registers.l); }
void bit_3_hlp() { bit_n_r(3, &memory.memory[registers.hl]); }

void bit_4_a() { bit_n_r(4, &registers.a); }
void bit_4_b() { bit_n_r(4, &registers.b); }
void bit_4_c() { bit_n_r(4, &registers.c); }
void bit_4_d() { bit_n_r(4, &registers.d); }
void bit_4_e() { bit_n_r(4, &registers.e); }
void bit_4_h() { bit_n_r(4, &registers.h); }
void bit_4_l() { bit_n_r(4, &registers.l); }
void bit_4_hlp() { bit_n_r(4, &memory.memory[registers.hl]); }

void bit_5_a() { bit_n_r(5, &registers.a); }
void bit_5_b() { bit_n_r(5, &registers.b); }
void bit_5_c() { bit_n_r(5, &registers.c); }
void bit_5_d() { bit_n_r(5, &registers.d); }
void bit_5_e() { bit_n_r(5, &registers.e); }
void bit_5_h() { bit_n_r(5, &registers.h); }
void bit_5_l() { bit_n_r(5, &registers.l); }
void bit_5_hlp() { bit_n_r(5, &memory.memory[registers.hl]); }

void bit_6_a() { bit_n_r(6, &registers.a); }
void bit_6_b() { bit_n_r(6, &registers.b); }
void bit_6_c() { bit_n_r(6, &registers.c); }
void bit_6_d() { bit_n_r(6, &registers.d); }
void bit_6_e() { bit_n_r(6, &registers.e); }
void bit_6_h() { bit_n_r(6, &registers.h); }
void bit_6_l() { bit_n_r(6, &registers.l); }
void bit_6_hlp() { bit_n_r(6, &memory.memory[registers.hl]); }

void bit_7_a() { bit_n_r(7, &registers.a); }
void bit_7_b() { bit_n_r(7, &registers.b); }
void bit_7_c() { bit_n_r(7, &registers.c); }
void bit_7_d() { bit_n_r(7, &registers.d); }
void bit_7_e() { bit_n_r(7, &registers.e); }
void bit_7_h() { bit_n_r(7, &registers.h); }
void bit_7_l() { bit_n_r(7, &registers.l); }
void bit_7_hlp() { bit_n_r(7, &memory.memory[registers.hl]); }


void res_n_hlp(uint8_t bit) {}

void res_0_a() { res_n_r(0, &registers.a); }
void res_0_b() { res_n_r(0, &registers.b); }
void res_0_c() { res_n_r(0, &registers.c); }
void res_0_d() { res_n_r(0, &registers.d); }
void res_0_e() { res_n_r(0, &registers.e); }
void res_0_h() { res_n_r(0, &registers.h); }
void res_0_l() { res_n_r(0, &registers.l); }
void res_0_hlp() { res_n_r(0, &memory.memory[registers.hl]); }

void res_1_a() { res_n_r(1, &registers.a); }
void res_1_b() { res_n_r(1, &registers.b); }
void res_1_c() { res_n_r(1, &registers.c); }
void res_1_d() { res_n_r(1, &registers.d); }
void res_1_e() { res_n_r(1, &registers.e); }
void res_1_h() { res_n_r(1, &registers.h); }
void res_1_l() { res_n_r(1, &registers.l); }
void res_1_hlp() { res_n_r(1, &memory.memory[registers.hl]); }

void res_2_a() { res_n_r(2, &registers.a); }
void res_2_b() { res_n_r(2, &registers.b); }
void res_2_c() { res_n_r(2, &registers.c); }
void res_2_d() { res_n_r(2, &registers.d); }
void res_2_e() { res_n_r(2, &registers.e); }
void res_2_h() { res_n_r(2, &registers.h); }
void res_2_l() { res_n_r(2, &registers.l); }
void res_2_hlp() { res_n_r(2, &memory.memory[registers.hl]); }

void res_3_a() { res_n_r(3, &registers.a); }
void res_3_b() { res_n_r(3, &registers.b); }
void res_3_c() { res_n_r(3, &registers.c); }
void res_3_d() { res_n_r(3, &registers.d); }
void res_3_e() { res_n_r(3, &registers.e); }
void res_3_h() { res_n_r(3, &registers.h); }
void res_3_l() { res_n_r(3, &registers.l); }
void res_3_hlp() { res_n_r(3, &memory.memory[registers.hl]); }

void res_4_a() { res_n_r(4, &registers.a); }
void res_4_b() { res_n_r(4, &registers.b); }
void res_4_c() { res_n_r(4, &registers.c); }
void res_4_d() { res_n_r(4, &registers.d); }
void res_4_e() { res_n_r(4, &registers.e); }
void res_4_h() { res_n_r(4, &registers.h); }
void res_4_l() { res_n_r(4, &registers.l); }
void res_4_hlp() { res_n_r(4, &memory.memory[registers.hl]); }

void res_5_a() { res_n_r(5, &registers.a); }
void res_5_b() { res_n_r(5, &registers.b); }
void res_5_c() { res_n_r(5, &registers.c); }
void res_5_d() { res_n_r(5, &registers.d); }
void res_5_e() { res_n_r(5, &registers.e); }
void res_5_h() { res_n_r(5, &registers.h); }
void res_5_l() { res_n_r(5, &registers.l); }
void res_5_hlp() { res_n_r(5, &memory.memory[registers.hl]); }

void res_6_a() { res_n_r(6, &registers.a); }
void res_6_b() { res_n_r(6, &registers.b); }
void res_6_c() { res_n_r(6, &registers.c); }
void res_6_d() { res_n_r(6, &registers.d); }
void res_6_e() { res_n_r(6, &registers.e); }
void res_6_h() { res_n_r(6, &registers.h); }
void res_6_l() { res_n_r(6, &registers.l); }
void res_6_hlp() { res_n_r(6, &memory.memory[registers.hl]); }

void res_7_a() { res_n_r(7, &registers.a); }
void res_7_b() { res_n_r(7, &registers.b); }
void res_7_c() { res_n_r(7, &registers.c); }
void res_7_d() { res_n_r(7, &registers.d); }
void res_7_e() { res_n_r(7, &registers.e); }
void res_7_h() { res_n_r(7, &registers.h); }
void res_7_l() { res_n_r(7, &registers.l); }
void res_7_hlp() { res_n_r(7, &memory.memory[registers.hl]); }

void set_n_hlp(uint8_t bit) {}
void set_0_a() { set_n_r(0, &registers.a); }
void set_0_b() { set_n_r(0, &registers.b); }
void set_0_c() { set_n_r(0, &registers.c); }
void set_0_d() { set_n_r(0, &registers.d); }
void set_0_e() { set_n_r(0, &registers.e); }
void set_0_h() { set_n_r(0, &registers.h); }
void set_0_l() { set_n_r(0, &registers.l); }
void set_0_hlp() { set_n_r(0, &memory.memory[registers.hl]); }

void set_1_a() { set_n_r(1, &registers.a); }
void set_1_b() { set_n_r(1, &registers.b); }
void set_1_c() { set_n_r(1, &registers.c); }
void set_1_d() { set_n_r(1, &registers.d); }
void set_1_e() { set_n_r(1, &registers.e); }
void set_1_h() { set_n_r(1, &registers.h); }
void set_1_l() { set_n_r(1, &registers.l); }
void set_1_hlp() { set_n_r(1, &memory.memory[registers.hl]); }

void set_2_a() { set_n_r(2, &registers.a); }
void set_2_b() { set_n_r(2, &registers.b); }
void set_2_c() { set_n_r(2, &registers.c); }
void set_2_d() { set_n_r(2, &registers.d); }
void set_2_e() { set_n_r(2, &registers.e); }
void set_2_h() { set_n_r(2, &registers.h); }
void set_2_l() { set_n_r(2, &registers.l); }
void set_2_hlp() { set_n_r(2, &memory.memory[registers.hl]); }

void set_3_a() { set_n_r(3, &registers.a); }
void set_3_b() { set_n_r(3, &registers.b); }
void set_3_c() { set_n_r(3, &registers.c); }
void set_3_d() { set_n_r(3, &registers.d); }
void set_3_e() { set_n_r(3, &registers.e); }
void set_3_h() { set_n_r(3, &registers.h); }
void set_3_l() { set_n_r(3, &registers.l); }
void set_3_hlp() { set_n_r(3, &memory.memory[registers.hl]); }

void set_4_a() { set_n_r(4, &registers.a); }
void set_4_b() { set_n_r(4, &registers.b); }
void set_4_c() { set_n_r(4, &registers.c); }
void set_4_d() { set_n_r(4, &registers.d); }
void set_4_e() { set_n_r(4, &registers.e); }
void set_4_h() { set_n_r(4, &registers.h); }
void set_4_l() { set_n_r(4, &registers.l); }
void set_4_hlp() { set_n_r(4, &memory.memory[registers.hl]); }

void set_5_a() { set_n_r(5, &registers.a); }
void set_5_b() { set_n_r(5, &registers.b); }
void set_5_c() { set_n_r(5, &registers.c); }
void set_5_d() { set_n_r(5, &registers.d); }
void set_5_e() { set_n_r(5, &registers.e); }
void set_5_h() { set_n_r(5, &registers.h); }
void set_5_l() { set_n_r(5, &registers.l); }
void set_5_hlp() { set_n_r(5, &memory.memory[registers.hl]); }

void set_6_a() { set_n_r(6, &registers.a); }
void set_6_b() { set_n_r(6, &registers.b); }
void set_6_c() { set_n_r(6, &registers.c); }
void set_6_d() { set_n_r(6, &registers.d); }
void set_6_e() { set_n_r(6, &registers.e); }
void set_6_h() { set_n_r(6, &registers.h); }
void set_6_l() { set_n_r(6, &registers.l); }
void set_6_hlp() { set_n_r(6, &memory.memory[registers.hl]); }

void set_7_a() { set_n_r(7, &registers.a); }
void set_7_b() { set_n_r(7, &registers.b); }
void set_7_c() { set_n_r(7, &registers.c); }
void set_7_d() { set_n_r(7, &registers.d); }
void set_7_e() { set_n_r(7, &registers.e); }
void set_7_h() { set_n_r(7, &registers.h); }
void set_7_l() { set_n_r(7, &registers.l); }
void set_7_hlp() { set_n_r(7, &memory.memory[registers.hl]); }

// End CB Prefix Functions


void undefined() {
    registers.pc--;
    printf("0x%04X  0x%02X  \n", registers.pc, readByte(registers.pc));
    exit(1);// ???? do we just NOP?
}

void unimplemented(uint8_t opcode) {
    print_registers();
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
    printf("  AF: %04X       \n  BC: %04X  (%02x)\n", registers.af, registers.bc, readByte(registers.bc));
    printf("  DE: %04X  (%02x)\n  HL: %04X  (%02x)\n", registers.de, readByte(registers.de),  registers.hl, readByte(registers.hl));
    printf("  SP: %04X  (%02x)\n  PC: %04X\n", registers.sp, readByte(registers.sp), registers.pc);

    printf("  F: 0b");
    print_byte_bits(registers.f);
    printf("\n");
    printf("  Halted: %01X\n", halted);
    printf("  haltbug: %01X\n", halt_bug);
    printf("  IME: %01X\n", IME_flag);
    printf("  IE: %02X\n  IF: %02X\n", memory.IE, memory.IFLAGS);
}