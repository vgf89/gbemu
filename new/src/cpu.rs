use crate::{memory::*, ops_table};
use std::{cell::RefCell, string, io::stdin};
use crate::ops_table::*;

#[derive(Default)]
pub struct CPU {
    // Registers
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,

    // Auxiiliary stuff
    pub cpu_clock: u32,
    pub ime_flag: bool,
    pub set_ei: u8,
    pub res_ei: u8 ,
    pub halted: bool,
    pub halt_bug: bool,

    pub memory: RefCell<Memory>,
}

pub const FLAGS_ZERO:u8 = 1 << 7;
pub const FLAGS_NEGATIVE:u8 = 1 << 6;
pub const FLAGS_HALFCARRY:u8 = 1 << 5;
pub const FLAGS_CARRY:u8 = 1 << 4;

//use crate gameboy::*;

impl CPU {
    pub fn new(mem: RefCell<Memory>) -> Self {
        return Self {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,

            cpu_clock: 0,
            ime_flag: true,
            set_ei: 0,
            res_ei: 0,
            halted: false,
            halt_bug: false,
            memory: mem,
        };
    }

    // Combined register getters/setters
    pub fn af(&self) -> u16 {
        return (self.a as u16) << 8 | (self.f as u16);
    }
    pub fn set_af(&mut self, af:u16) {
        self.a = ((af >> 8) & 0xff) as u8;
        self.f = (af & 0xff) as u8;
    }

    pub fn bc(&self) -> u16 {
        return (self.b as u16) << 8 | (self.c as u16);
    }
    pub fn set_bc(&mut self, value:u16) {
        self.b = ((value >> 8) & 0xff) as u8;
        self.c = (value & 0xff) as u8;
    }

    pub fn de(&self) -> u16 {
        return (self.d as u16) << 8 | (self.e as u16);
    }
    pub fn set_de(&mut self, value:u16) {
        self.d = ((value >> 8) & 0xff) as u8;
        self.e = (value & 0xff) as u8;
    }

    pub fn hl(&self) -> u16 {
        return (self.h as u16) << 8 | (self.l as u16);
    }
    pub fn set_hl(&mut self, value:u16) {
        self.h = ((value >> 8) & 0xff) as u8;
        self.l = (value & 0xff) as u8;
    }
    fn hlp(&self) -> u8 {
        return self.memory.borrow().read_byte(self.hl());
    }
    fn set_hlp(&mut self, val: u8) {
        self.memory.borrow_mut().write_byte(self.hl(), val);
    }

    pub fn flags_is_zero(&self) -> bool{
        return self.f & FLAGS_ZERO != 0;
    }
    pub fn flags_is_negative(&self) -> bool{
        return self.f & FLAGS_NEGATIVE != 0;
    }
    pub fn flags_is_halfcarry(&self) -> bool{
        return self.f & FLAGS_HALFCARRY != 0;
    }
    pub fn flags_is_carry(&self) -> bool{
        return self.f & FLAGS_CARRY != 0;
    }

    pub fn flags_isset(&self, x:u8) -> u8 {
        return self.f & x;
    }
    pub fn flags_set(&mut self, x:u8) {
        self.f = self.f | x;
    }
    pub fn flags_clear(&mut self, x:u8) {
        self.f = self.f & !x;
    }

    pub fn reset_cpu_clock(&mut self, maxclock: u16) {
        self.cpu_clock = self.cpu_clock - maxclock as u32;
    }

    // CPU stepper
    pub fn cpu_step(&mut self, system_clock: u32) {
        if system_clock < self.cpu_clock {
            return;
        }


        // Debug print & wait to step
        self.print_status();
        stdin().read_line(&mut String::new()).unwrap();

        // Handle Interrupts
        if self.ime_flag {
            if self.memory.borrow().ie_isset(I_VBLANK) && self.memory.borrow().if_isset(I_VBLANK) {
                self.cpu_clock += 20;
                self.halted = false;
                self.memory.borrow_mut().ie_clear(I_VBLANK);
                self.ime_flag = false;
                self.call_nn(0x0040);
                return;
            }
            if self.memory.borrow().ie_isset(I_LCD_STAT) && self.memory.borrow().if_isset(I_LCD_STAT) {
                self.cpu_clock += 20;
                self.halted = false;
                self.memory.borrow_mut().ie_clear(I_LCD_STAT);
                self.ime_flag = false;
                self.call_nn(0x0048);
                return;
            }
            if self.memory.borrow().ie_isset(I_TIMER) && self.memory.borrow().if_isset(I_TIMER) {
                self.cpu_clock += 20;
                self.halted = false;
                self.memory.borrow_mut().ie_clear(I_TIMER);
                self.ime_flag = false;
                self.call_nn(0x0050);
                return;
            }
            if self.memory.borrow().ie_isset(I_SERIAL) && self.memory.borrow().if_isset(I_SERIAL) {
                self.cpu_clock += 20;
                self.halted = false;
                self.memory.borrow_mut().ie_clear(I_SERIAL);
                self.ime_flag = false;
                self.call_nn(0x0058);
                return;
            }
            if self.memory.borrow().ie_isset(I_JOYPAD) && self.memory.borrow().if_isset(I_JOYPAD) {
                self.cpu_clock += 20;
                self.halted = false;
                self.memory.borrow_mut().ie_clear(I_JOYPAD);
                self.ime_flag = false;
                self.call_nn(0x0060);
                return;
            }
        } else if self.halted {
            if self.memory.borrow().if_isset(I_VBLANK) ||
               self.memory.borrow().if_isset(I_LCD_STAT) ||
               self.memory.borrow().if_isset(I_SERIAL) ||
               self.memory.borrow().if_isset(I_TIMER) ||
               self.memory.borrow().if_isset(I_JOYPAD) {
                self.halted = false;
            }
        }
        // FIXME: Implement

        let opcode_length;
        let mut operand8 = 0;
        let mut operand16 = 0;
        let opcode = self.memory.borrow().read_byte(self.pc);
        let inst: &Instruction = &ops_table::INSTRUCTIONS[opcode as usize];
        match &inst.execute {
            FnEnum::OpLen1(_) => { opcode_length = 1; },
            FnEnum::OpLen2(_) => {
                opcode_length = 2;
                operand8 = self.memory.borrow().read_byte(self.pc + 1);
            },
            FnEnum::OpLen2i(_) => {
                opcode_length = 2;
                operand8 = self.memory.borrow().read_byte(self.pc + 1);
            },
            FnEnum::OpLen3(_) => {
                opcode_length = 3;
                operand16 = self.memory.borrow().read_word(self.pc + 1);
            },
            FnEnum::STOP => todo!(),
            FnEnum::UNDEFINED => todo!(),
        }

        // FIXME: Handle breakpoints here

        if self.halt_bug {
            self.halt_bug = false;
        } else {
            if !self.halted {
                self.pc += opcode_length;
            }
        }

        if self.halted {
            self.cpu_clock += 4;
            return;
        }

        self.cpu_clock += inst.cycles;

        match &inst.execute {
            FnEnum::OpLen1(op) => {
                (op)(self);
            },
            FnEnum::OpLen2(op) => {
                (op)(self, operand8);
            },
            FnEnum::OpLen2i(op) => {
                (op)(self, operand8 as i8);
            },
            FnEnum::OpLen3(op) => {
                (op)(self, operand16);
            },
            FnEnum::STOP => todo!(),
            FnEnum::UNDEFINED => todo!(),
        }
    }

    pub fn print_status(&self) {
        //print!("REGS ");
        self.print_regs();
        print!(",  DISAS: ");
        self.print_disas(self.pc);
        //print!("\n");
    }

    pub fn print_regs(&self) {
        print!("pc:  {:#06X},  af: {:#06X},  bc: {:#06X},  de: {:#06X},  hl: {:#06X}->{:#04X}, flags: {:#010b}", self.pc, self.af(), self.bc(), self.de(), self.hl(), self.hlp(), self.f);
    }

    pub fn print_disas(&self, address: u16) {
        let opcode = self.memory.borrow().read_byte(address);
        let inst: &Instruction = &ops_table::INSTRUCTIONS[opcode as usize];
        println!("{}", self.print_op(inst));
    }

    fn op_format_helper_u8(opcode: &str, operand: u8) -> String {
        return opcode.replace(
            "{:#04X}",
            format!("{:#04X}", operand).as_str()
        );
    }

    fn op_format_helper_i8(opcode: &str, operand: i8) -> String {
        if operand >= 0 {
            return opcode.replace(
                "{:+#04X}",
                format!("{:#04X}", operand).as_str()
            );
        } else {
            return opcode.replace(
                "{:+#04X}",
                format!("-{:#04X}", -operand).as_str()
            );
        }
    }

    fn op_format_helper_u16(opcode: &str, operand: u16) -> String {
        return opcode.replace(
            "{:#06X}",
            format!("{:#06X}", operand).as_str()
        );
    }

    fn print_op(&self, instr: &Instruction) -> String {
        match instr.execute {
            FnEnum::STOP => todo!(),
            FnEnum::UNDEFINED => todo!(),
            FnEnum::OpLen1(_) => {
                return instr.disas.to_string();
            },
            FnEnum::OpLen2(_) => {
                let operand = self.memory.borrow().read_byte(self.pc + 1);
                return CPU::op_format_helper_u8(instr.disas, operand);
            },
            FnEnum::OpLen2i(_) => {
                let operand = self.memory.borrow().read_byte(self.pc + 1) as i8;
                return CPU::op_format_helper_i8(instr.disas, operand);
            },
            FnEnum::OpLen3(_) => {
                let operand = self.memory.borrow().read_word(self.pc + 1);
                return CPU::op_format_helper_u16(instr.disas, operand);
            },
        }
    }



    // Opcodes
    pub fn nop(&mut self) 
    {
        // do nothing
    }

    pub fn inc_a(&mut self) { self.a = self.inc_n(self.a); }
    pub fn inc_b(&mut self) { self.b = self.inc_n(self.b); }
    pub fn inc_c(&mut self) { self.c = self.inc_n(self.c); }
    pub fn inc_d(&mut self) { self.d = self.inc_n(self.d); }
    pub fn inc_e(&mut self) { self.e = self.inc_n(self.e); }

    pub fn dec_a(&mut self) { self.a = self.dec_n(self.a); }
    pub fn dec_b(&mut self) { self.b = self.dec_n(self.b); }
    pub fn dec_c(&mut self) { self.c = self.dec_n(self.c); }
    pub fn dec_d(&mut self) { self.d = self.dec_n(self.d); }
    pub fn dec_e(&mut self) { self.e = self.dec_n(self.e); }

    pub fn inc_bc(&mut self) { self.set_bc(self.bc() + 1); }
    pub fn inc_de(&mut self) { self.set_de(self.de() + 1); }

    pub fn ld_bc_nn(&mut self, value: u16) { self.set_bc(value); }
    pub fn ld_bcp_a(&mut self) { self.memory.borrow_mut().write_byte(self.bc(), self.a); }

    pub fn ld_b_n(&mut self, value: u8) {self.b = value;}
    pub fn rlca(&mut self) {
        {
            self.a = self.rlc_r(self.a);
        }
        self.flags_clear(FLAGS_ZERO);
        self.flags_clear(FLAGS_NEGATIVE);
        self.flags_clear(FLAGS_HALFCARRY);
    }
    pub fn ld_nnp_sp(&mut self, address: u16) { self.memory.borrow_mut().write_word(address, self.sp); }
    pub fn add_hl_nn(&mut self, nn: u16) {
        self.set_hl(self.hl() + nn);
    }
    pub fn add_hl_bc(&mut self) { self.add_hl_nn(self.bc()); }
    pub fn ld_a_bcp(&mut self) { self.a = self.memory.borrow_mut().read_byte(self.bc()); }
    pub fn dec_bc(&mut self) { self.set_bc(self.bc() - 1); }
    pub fn ld_c_n(&mut self, value: u8) { self.c = value; }

    pub fn rrca(&mut self) {
        self.rrc_a();
        self.flags_clear(FLAGS_ZERO);
        self.flags_clear(FLAGS_NEGATIVE);
        self.flags_clear(FLAGS_HALFCARRY);
    }

    pub fn ld_de_nn(&mut self, value: u16) { self.set_de(value); }
    pub fn ld_dep_a(&mut self) { self.memory.borrow_mut().write_byte(self.de(), self.a); }

    pub fn ld_d_n(&mut self, value: u8) {self.d = value;}

    pub fn rla(&mut self) {
        self.rl_a();
        self.flags_clear(FLAGS_ZERO);
        self.flags_clear(FLAGS_NEGATIVE);
        self.flags_clear(FLAGS_HALFCARRY);
    }

    pub fn jr_nn(&mut self, offset: i8) { self.pc = self.pc.wrapping_add(offset as u16); }
    pub fn add_hl_de(&mut self) { self.add_hl_nn(self.de()); }
    pub fn ld_a_dep(&mut self) { self.a = self.memory.borrow_mut().read_byte(self.de()); }
    pub fn dec_de(&mut self) { self.set_de(self.de() - 1); }
    pub fn ld_e_n(&mut self, value: u8) { self.c = value; }

    
    pub fn rra(&mut self) {
        self.rr_a();
        self.flags_clear(FLAGS_ZERO);
        self.flags_clear(FLAGS_NEGATIVE);
        self.flags_clear(FLAGS_HALFCARRY);
    }
    pub fn jr_nz(&mut self, offset: i8) {
        if self.flags_is_zero() == false
        {
            self.jr_nn(offset);
            self.cpu_clock += 4;
        }
    }

    pub fn ld_hl_nn(&mut self, value: u16) {
        self.set_hl(value);
    }
    pub fn ldi_hlp_a(&mut self) {
        self.set_hlp(self.a);
        self.set_hl(self.hl() + 1);
    }
    pub fn inc_hl(&mut self) { self.set_hl(self.hl() + 1); }
    pub fn inc_h(&mut self) { self.h = self.h + 1; }
    pub fn dec_h(&mut self) { self.h = self.h - 1; }
    pub fn ld_h_n(&mut self, value: u8) { self.h = value; }
    pub fn daa(&mut self) {
        // https://ehaskins.com/2018-01-30%20Z80%20DAA/
        let mut correction: u8 = 0;
    
        let mut flag_c: bool = false;
        if self.flags_is_halfcarry() || (!self.flags_is_negative() && (self.a & 0x0f) > 0x9) {
            correction |= 0x6;
        }
    
        if self.flags_is_carry() || (!self.flags_is_negative() && self.a > 0x99) {
            correction |= 0x60;
            flag_c = true;
        }
    
        self.a = self.a + ( if self.flags_is_negative() { (-(correction as i8)) as u8 } else { correction });
    
        self.flags_clear(FLAGS_ZERO);
        self.flags_clear(FLAGS_HALFCARRY);
        self.flags_clear(FLAGS_CARRY);
    
        if self.a == 0 {
            self.flags_set(FLAGS_ZERO);
        }
        if flag_c {
            self.flags_set(FLAGS_CARRY);
        }
    }

    pub fn jr_z(&mut self, offset:i8) {
        if self.flags_is_zero() {
            self.pc = self.pc.wrapping_add(offset as u16);
            self.cpu_clock += 4;
        }
    }

    pub fn add_hl_hl(&mut self) {
        self.add_hl_nn(self.hl());
    }

    pub fn ldi_a_hlp(&mut self) {
        self.a = self.hlp();
        self.set_hl(self.hl() + 1);
    }

    pub fn dec_hl(&mut self) { self.set_hl(self.hl() - 1); }
    pub fn inc_l(&mut self) { self.l += 1; }
    pub fn dec_l(&mut self) { self.l -= 1; }
    pub fn ld_l_n(&mut self, value: u8) { self.l = value; }

    pub fn cpl(&mut self) {
        self.a = !self.a;
    }

    pub fn jr_nc(&mut self, offset: i8) {
        if !self.flags_is_carry() {
            self.pc = self.pc.wrapping_add(offset as u16);
            self.cpu_clock += 4;
        }
    }

    pub fn ld_sp_nn(&mut self, value: u16) { self.sp = value; }
    pub fn ldd_hlp_a(&mut self) {
        self.set_hlp(self.a);
        self.set_hl(self.hl() - 1);
    }
    pub fn inc_sp(&mut self) {
        self.sp += 1;
    }
    pub fn inc_hlp(&mut self) {
        self.set_hlp(self.hlp() + 1);
    }
    pub fn dec_hlp(&mut self) {
        self.set_hlp(self.hlp() - 1);
    }
    pub fn ld_hlp_n(&mut self, value: u8) {
        self.set_hlp(value);
    }
    pub fn scf(&mut self) {
        self.flags_clear(FLAGS_NEGATIVE);
        self.flags_clear(FLAGS_HALFCARRY);
        self.flags_set(FLAGS_CARRY);
    }
    pub fn jr_c(&mut self, offset: i8) {
        if self.flags_is_carry() {
            self.pc = self.pc.wrapping_add(offset as u16);
            self.cpu_clock += 4;
        }
    }
    pub fn add_hl_sp(&mut self) {
        self.set_hl(self.sp);
    }
    pub fn ldd_a_hlp(&mut self) {
        self.a = self.hlp();
        self.set_hl(self.hl() - 1);
    }
    pub fn dec_sp(&mut self) {
        self.sp -= 1;
    }

    // Sets flags and return the new register value for instruction inc_N
    // We return the new value instead of setting it directly due to borrow rules issues >:(\

    pub fn inc_n(&mut self, reg: u8) -> u8 {
        self.flags_clear(FLAGS_ZERO);
        self.flags_clear(FLAGS_NEGATIVE);
        self.flags_clear(FLAGS_HALFCARRY);
        // Half Carry
        if (((reg & 0xf).wrapping_add(1)) & 0x10) == 0x10 {
            self.flags_set(FLAGS_HALFCARRY);
        }
        // Zero Flag
        if reg.wrapping_add(1) == 0 {
            self.flags_set(FLAGS_ZERO);
        }
        return reg.wrapping_add(1);
    }

    pub fn dec_n(&mut self, reg: u8) -> u8 {
        self.flags_clear(FLAGS_ZERO);
        self.flags_set(FLAGS_NEGATIVE);
        self.flags_clear(FLAGS_HALFCARRY);
        // Half Carry
        if (((reg & 0xf).wrapping_sub(1)) & 0x10) == 0x10 {
            self.flags_set(FLAGS_HALFCARRY);
        }
        // Zero Flag
        if reg.wrapping_sub(1) == 0 {
            self.flags_set(FLAGS_ZERO);
        }
        return reg.wrapping_sub(1);
    }
    
    pub fn ld_a_n(&mut self, value: u8) {
        self.a = value;
    }

    pub fn ccf(&mut self)
    {
        self.flags_clear(FLAGS_NEGATIVE);
        self.flags_clear(FLAGS_HALFCARRY);
        if self.flags_is_carry() {
            self.flags_clear(FLAGS_CARRY);
        } else {
            self.flags_set(FLAGS_CARRY);
        }
    }


    pub fn ld_b_b(&mut self) { self.b = self.b; }
    pub fn ld_b_c(&mut self) { self.b = self.c; }
    pub fn ld_b_d(&mut self) { self.b = self.d; }
    pub fn ld_b_e(&mut self) { self.b = self.e; }
    pub fn ld_b_h(&mut self) { self.b = self.h; }
    pub fn ld_b_l(&mut self) { self.b = self.l; }
    pub fn ld_b_hlp(&mut self) { self.b = self.hlp(); }
    pub fn ld_b_a(&mut self) { self.b = self.a; }

    pub fn ld_c_b(&mut self) { self.c = self.b; }
    pub fn ld_c_c(&mut self) { self.c = self.c; }
    pub fn ld_c_d(&mut self) { self.c = self.d; }
    pub fn ld_c_e(&mut self) { self.c = self.e; }
    pub fn ld_c_h(&mut self) { self.c = self.h; }
    pub fn ld_c_l(&mut self) { self.c = self.l; }
    pub fn ld_c_hlp(&mut self) { self.c = self.hlp(); }
    pub fn ld_c_a(&mut self) { self.c = self.a; }

    pub fn ld_d_b(&mut self) { self.d = self.b; }
    pub fn ld_d_c(&mut self) { self.d = self.c; }
    pub fn ld_d_d(&mut self) { self.d = self.d; }
    pub fn ld_d_e(&mut self) { self.d = self.e; }
    pub fn ld_d_h(&mut self) { self.d = self.h; }
    pub fn ld_d_l(&mut self) { self.d = self.l; }
    pub fn ld_d_hlp(&mut self) { self.d = self.hlp(); }
    pub fn ld_d_a(&mut self) { self.d = self.a; }

    pub fn ld_e_b(&mut self) { self.e = self.b; }
    pub fn ld_e_c(&mut self) { self.e = self.c; }
    pub fn ld_e_d(&mut self) { self.e = self.d; }
    pub fn ld_e_e(&mut self) { self.e = self.e; }
    pub fn ld_e_h(&mut self) { self.e = self.h; }
    pub fn ld_e_l(&mut self) { self.e = self.l; }
    pub fn ld_e_hlp(&mut self) { self.e = self.hlp(); }
    pub fn ld_e_a(&mut self) { self.e = self.a; }

    pub fn ld_h_b(&mut self) { self.h = self.b; }
    pub fn ld_h_c(&mut self) { self.h = self.c; }
    pub fn ld_h_d(&mut self) { self.h = self.d; }
    pub fn ld_h_e(&mut self) { self.h = self.e; }
    pub fn ld_h_h(&mut self) { self.h = self.h; }
    pub fn ld_h_l(&mut self) { self.h = self.l; }
    pub fn ld_h_hlp(&mut self) { self.h = self.hlp(); }
    pub fn ld_h_a(&mut self) { self.h = self.a; }

    pub fn ld_l_b(&mut self) { self.l = self.b; }
    pub fn ld_l_c(&mut self) { self.l = self.c; }
    pub fn ld_l_d(&mut self) { self.l = self.d; }
    pub fn ld_l_e(&mut self) { self.l = self.e; }
    pub fn ld_l_h(&mut self) { self.l = self.h; }
    pub fn ld_l_l(&mut self) { self.l = self.l; }
    pub fn ld_l_hlp(&mut self) { self.l = self.hlp(); }
    pub fn ld_l_a(&mut self) { self.l = self.a; }

    pub fn ld_hlp_b(&mut self) { self.set_hlp(self.b); }
    pub fn ld_hlp_c(&mut self) { self.set_hlp(self.c); }
    pub fn ld_hlp_d(&mut self) { self.set_hlp(self.d); }
    pub fn ld_hlp_e(&mut self) { self.set_hlp(self.e); }
    pub fn ld_hlp_h(&mut self) { self.set_hlp(self.h); }
    pub fn ld_hlp_l(&mut self) { self.set_hlp(self.l); }
    pub fn ld_hlp_a(&mut self) { self.set_hlp(self.a); }

    pub fn ld_a_b(&mut self) { self.a = self.b; }
    pub fn ld_a_c(&mut self) { self.a = self.c; }
    pub fn ld_a_d(&mut self) { self.a = self.d; }
    pub fn ld_a_e(&mut self) { self.a = self.e; }
    pub fn ld_a_h(&mut self) { self.a = self.h; }
    pub fn ld_a_l(&mut self) { self.a = self.l; }
    pub fn ld_a_hlp(&mut self) { self.a = self.hlp(); }
    pub fn ld_a_a(&mut self) { self.a = self.a; }

    pub fn add_a_n(&mut self, n: u8) {
        self.reset_flags();
        if (((self.a & 0xf) + (n & 0xf)) & 0x10) == 0x10 {
            self.flags_set(FLAGS_HALFCARRY);
        }
        if ((self.a as u16 + n as u16) & 0x100) == 0x100 {
            self.flags_set(FLAGS_CARRY);
        }
    
        self.a += n;
    
        if self.a == 0 {
            self.flags_set(FLAGS_ZERO)
        };
    }
    pub fn add_a_b(&mut self) { self.add_a_n(self.b); }
    pub fn add_a_c(&mut self) { self.add_a_n(self.c); }
    pub fn add_a_d(&mut self) { self.add_a_n(self.d); }
    pub fn add_a_e(&mut self) { self.add_a_n(self.e); }
    pub fn add_a_f(&mut self) { self.add_a_n(self.f); }
    pub fn add_a_h(&mut self) { self.add_a_n(self.h); }
    pub fn add_a_l(&mut self) { self.add_a_n(self.l); }
    pub fn add_a_hlp(&mut self) { 
        let hlp = self.hlp();
        self.add_a_n(hlp);
    }
    pub fn add_a_a(&mut self) { self.add_a_n(self.a); }

    pub fn adc_a_n(&mut self, n: u8) {
        let oldcarryflag: u8 = self.flags_is_carry() as u8;
        let half_result: u8 = (self.a & 0xf) +  (n & 0xf) + oldcarryflag;
        let full_result: u16 = self.a as u16 + n as u16 + oldcarryflag as u16;
        self.a = full_result as u8; // Cast truncates the overflow bits
        self.reset_flags();

        if half_result > 0xf {
            self.flags_set(FLAGS_HALFCARRY);
        }
        if full_result > 0xff { 
            self.flags_set(FLAGS_CARRY);
        }
        if self.a == 0 {
            self.flags_set(FLAGS_ZERO);
        }
    }
    pub fn adc_a_b(&mut self) { self.adc_a_n(self.b); }
    pub fn adc_a_c(&mut self) { self.adc_a_n(self.c); }
    pub fn adc_a_d(&mut self) { self.adc_a_n(self.d); }
    pub fn adc_a_e(&mut self) { self.adc_a_n(self.e); }
    pub fn adc_a_h(&mut self) { self.adc_a_n(self.h); }
    pub fn adc_a_l(&mut self) { self.adc_a_n(self.l); }
    pub fn adc_a_hlp(&mut self) {
        let hlp = self.hlp();
        self.adc_a_n(hlp);
    }
    pub fn adc_a_a(&mut self) { self.adc_a_n(self.a); }

    pub fn sub_a_n(&mut self, n: u8) {
        self.reset_flags();
        if (((self.a & 0xf) - (n & 0xf)) & 0x10) == 0x10 {
            self.flags_set(FLAGS_HALFCARRY);
        }
        if ((self.a as u16 - n as u16) & 0x100) == 0x100 {
            self.flags_set(FLAGS_CARRY);
        }
    
        self.a -= n;
    
        if self.a == 0 {
            self.flags_set(FLAGS_ZERO)
        };

        self.flags_set(FLAGS_NEGATIVE); // FIXME: Is this right?
    }
    pub fn sub_a_b(&mut self) { self.sub_a_n(self.b); }
    pub fn sub_a_c(&mut self) { self.sub_a_n(self.c); }
    pub fn sub_a_d(&mut self) { self.sub_a_n(self.d); }
    pub fn sub_a_e(&mut self) { self.sub_a_n(self.e); }
    pub fn sub_a_f(&mut self) { self.sub_a_n(self.f); }
    pub fn sub_a_h(&mut self) { self.sub_a_n(self.h); }
    pub fn sub_a_l(&mut self) { self.sub_a_n(self.l); }
    pub fn sub_a_hlp(&mut self) {
        let hlp = self.hlp();
        self.sub_a_n(hlp);
    }
    pub fn sub_a_a(&mut self) { self.sub_a_n(self.a); }

    pub fn sbc_a_n(&mut self, n: u8) {
        let oldcarryflag: u8 = self.flags_is_carry() as u8;
        let half_result: u8 = (self.a & 0xf) -  (n & 0xf) - oldcarryflag;
        let full_result: u16 = self.a as u16 - n as u16 - oldcarryflag as u16;
        self.a = full_result as u8; // Cast truncates the overflow bits
        self.reset_flags();

        self.flags_set(FLAGS_NEGATIVE);
        if half_result > 0xf {
            self.flags_set(FLAGS_HALFCARRY);
        }
        if full_result > 0xff { 
            self.flags_set(FLAGS_CARRY);
        }
        if self.a == 0 {
            self.flags_set(FLAGS_ZERO);
        }
    }
    pub fn sbc_a_b(&mut self) { self.sbc_a_n(self.b); }
    pub fn sbc_a_c(&mut self) { self.sbc_a_n(self.c); }
    pub fn sbc_a_d(&mut self) { self.sbc_a_n(self.d); }
    pub fn sbc_a_e(&mut self) { self.sbc_a_n(self.e); }
    pub fn sbc_a_h(&mut self) { self.sbc_a_n(self.h); }
    pub fn sbc_a_l(&mut self) { self.sbc_a_n(self.l); }
    pub fn sbc_a_hlp(&mut self) {
        let hlp = self.hlp();
        self.sbc_a_n(hlp);
    }
    pub fn sbc_a_a(&mut self) { self.sbc_a_n(self.a); }

    pub fn and_n(&mut self, n: u8) {
        self.a &= n;
        self.reset_flags();
        if self.a == 0 {
            self.flags_set(FLAGS_ZERO);
        }
        self.flags_set(FLAGS_HALFCARRY); // FIXME: Why is this always set?
    }
    pub fn and_b(&mut self) { self.and_n(self.b); }
    pub fn and_c(&mut self) { self.and_n(self.c); }
    pub fn and_d(&mut self) { self.and_n(self.d); }
    pub fn and_e(&mut self) { self.and_n(self.e); }
    pub fn and_h(&mut self) { self.and_n(self.h); }
    pub fn and_l(&mut self) { self.and_n(self.l); }
    pub fn and_hlp(&mut self) {
        let hlp = self.hlp();
        self.and_n(hlp);
    }
    pub fn and_a(&mut self) { self.and_n(self.a); }


    fn set_or_flags(&mut self) {
        self.flags_clear(FLAGS_ZERO);
        self.flags_clear(FLAGS_NEGATIVE);
        self.flags_clear(FLAGS_HALFCARRY);
        self.flags_clear(FLAGS_CARRY);
        if self.a == 0 {
            self.flags_set(FLAGS_ZERO);
        }
    }

    pub fn xor_n(&mut self, n: u8) {
        self.a ^= n;
        self.set_or_flags();
    }
    pub fn xor_b(&mut self) { self.xor_n(self.b); }
    pub fn xor_c(&mut self) { self.xor_n(self.c); }
    pub fn xor_d(&mut self) { self.xor_n(self.d); }
    pub fn xor_e(&mut self) { self.xor_n(self.e); }
    pub fn xor_h(&mut self) { self.xor_n(self.h); }
    pub fn xor_l(&mut self) { self.xor_n(self.l); }
    pub fn xor_hlp(&mut self) {
        let hlp = self.hlp();
        self.xor_n(hlp);
    }
    pub fn xor_a(&mut self) { self.xor_n(self.a); }

    pub fn or_n(&mut self, n: u8) {
        self.a |= n;
        self.set_or_flags();
    }
    pub fn or_b(&mut self) { self.or_n(self.b); }
    pub fn or_c(&mut self) { self.or_n(self.c); }
    pub fn or_d(&mut self) { self.or_n(self.d); }
    pub fn or_e(&mut self) { self.or_n(self.e); }
    pub fn or_h(&mut self) { self.or_n(self.h); }
    pub fn or_l(&mut self) { self.or_n(self.l); }
    pub fn or_hlp(&mut self) {
        let hlp = self.hlp();
        self.or_n(hlp);
    }
    pub fn or_a(&mut self) { self.or_n(self.a); }

    pub fn cp_n(&mut self, n: u8) {
        self.reset_flags();
        if (((self.a & 0xf) - (n & 0xf)) & 0x10) == 0x10 {
            self.flags_set(FLAGS_HALFCARRY);
        }
        if ((self.a as u16 - n as u16) & 0x100) == 0x100 {
            self.flags_set(FLAGS_CARRY);
        }
        if self.a == n {
            self.flags_set(FLAGS_ZERO);
        }
        self.flags_set(FLAGS_NEGATIVE);
    }
    pub fn cp_b(&mut self) { self.cp_n(self.b); }
    pub fn cp_c(&mut self) { self.cp_n(self.c); }
    pub fn cp_d(&mut self) { self.cp_n(self.d); }
    pub fn cp_e(&mut self) { self.cp_n(self.e); }
    pub fn cp_h(&mut self) { self.cp_n(self.h); }
    pub fn cp_l(&mut self) { self.cp_n(self.l); }
    pub fn cp_hlp(&mut self) {
        let hlp = self.hlp();
        self.cp_n(hlp);
    }
    pub fn cp_a(&mut self) { self.cp_n(self.a); }




    fn pop_rr(&mut self) -> (u8, u8) {
        let bigreg = self.memory.borrow().read_word(self.sp);
        let reg1_ret = ((bigreg >> 8) & 0xff) as u8;
        let reg2_ret = (bigreg & 0xff) as u8;
        self.sp += 2;
        return (reg1_ret, reg2_ret);
    }

    pub fn pop_af(&mut self) {
        (self.a, self.f) = self.pop_rr();
        self.f &= 0xf0;
    }
    pub fn pop_bc(&mut self) {
        (self.b, self.c) = self.pop_rr();
    }
    pub fn pop_de(&mut self) {
        (self.d, self.e) = self.pop_rr();
    }
    pub fn pop_hl(&mut self) {
        (self.h, self.l) = self.pop_rr();
    }



    pub fn di(&mut self) {
        self.ime_flag = false;
        //res_ei = 2;
    }
    pub fn ei(&mut self) {
        self.ime_flag = true;
        //set_ei = 2;
    }
    pub fn reti(&mut self)
    {
        self.ret();
        self.ei();
        //IME_flag = 1; // What?
    }
    
    pub fn halt(&mut self) {
        if self.ime_flag {
            self.halted = true;
        } else {
            if (self.memory.borrow_mut().read_byte(IE) & self.memory.borrow_mut().read_byte(IFLAGS) & 0x1f) == 0 {
                self.halted = true;
                // Halt mode is entered, but the interupt vector is not called
                // and IF isn't cleared (it instead just continue executing when an interrupt is received)
                // Can check this via (halted && (IME_flag == 0))
            } else {
                // Halt mode is not entered.
                // CPU does not increase pc on next instruction. IF flags aren't cleared
                self.halt_bug = true;
            }
        }
    }
    
    pub fn reset_flags(&mut self) {
        self.flags_clear(FLAGS_ZERO);
        self.flags_clear(FLAGS_NEGATIVE);
        self.flags_clear(FLAGS_HALFCARRY);
        self.flags_clear(FLAGS_CARRY);
    }


    pub fn jp_nn(&mut self, address: u16) {
        
        self.pc = address;
    }
    pub fn jp_z(&mut self, address: u16) {
        if self.flags_is_zero() {
            self.pc = address;
            self.cpu_clock += 4;
        }
    }
    pub fn jp_nz(&mut self, address: u16) {
        if !self.flags_is_zero() {
            self.pc = address;
            self.cpu_clock += 4;
        }
    }
    pub fn jp_nc(&mut self, address: u16) {
        if !self.flags_is_carry() {
            self.pc = address;
            self.cpu_clock += 4;
        }
    }
    pub fn jp_c(&mut self, address: u16) {
        if self.flags_is_carry() {
            self.pc = address;
            self.cpu_clock += 4;
        }
    }


    pub fn call_nn(&mut self, address: u16) {
        self.sp -= 2;
        self.memory.borrow_mut().write_word(self.sp, self.pc);
        self.pc = address;
    }
    pub fn call_z(&mut self, address: u16) {
        if self.flags_is_zero() {
            self.call_nn(address);
            self.cpu_clock += 12; // branch takes additional 12 cycles
        }
    }
    pub fn call_nz(&mut self, address: u16) {
        if !self.flags_is_zero() {
            self.call_nn(address);
            self.cpu_clock += 12;
        }
    }
    pub fn call_nc(&mut self, address: u16) {
        if !self.flags_is_carry() {
            self.call_nn(address);
            self.cpu_clock += 12;
        }
    }
    pub fn call_c(&mut self, address: u16) {
        if self.flags_is_carry() {
            self.call_nn(address);
            self.cpu_clock += 12;
        }
    }

    pub fn push_bc(&mut self) {
        self.push_nn(self.bc());
    }
    pub fn push_de(&mut self) {
        self.push_nn(self.de());
    }
    pub fn push_hl(&mut self) {
        self.push_nn(self.hl());
    }

    pub fn rst00(&mut self) {
        self.call_nn(0x00);
    }
    pub fn rst08(&mut self) {
        self.call_nn(0x08);
    }
    pub fn rst10(&mut self) {
        self.call_nn(0x10);
    }
    pub fn rst18(&mut self) {
        self.call_nn(0x18);
    }
    pub fn rst20(&mut self) {
        self.call_nn(0x20);
    }
    pub fn rst28(&mut self) {
        self.call_nn(0x28);
    }
    pub fn rst30(&mut self) {
        self.call_nn(0x30);
    }
    pub fn rst38(&mut self) {
        self.call_nn(0x38);
    }


    pub fn ret(&mut self) {
        self.pc = self.memory.borrow_mut().read_word(self.sp);
        self.sp += 2;
    }
    pub fn ret_z(&mut self) {
        if self.flags_is_zero() {
            self.cpu_clock += 12;
            self.ret();
        }
    }
    pub fn ret_nz(&mut self) {
        if !self.flags_is_zero() {
            self.cpu_clock += 12;
            self.ret();
        }
    }
    pub fn ret_nc(&mut self) {
        if !self.flags_is_carry() {
            self.cpu_clock += 12;
            self.ret();
        }
    }
    pub fn ret_c(&mut self) {
        if self.flags_is_carry() {
            self.cpu_clock += 12;
            self.ret();
        }
    }

    pub fn cb(&mut self, opcode: u8) {
        let inst: &Instruction = &ops_table::CB_INSTRUCTIONS[opcode as usize];
        match &inst.execute {
            FnEnum::OpLen1(op) => (op)(self),
            _ => println!("missing CB prefixed opcode: {}", opcode),
        }
    }

    pub fn ld_a_np(&mut self, address: u8) {
        self.a = self.memory.borrow_mut().read_byte(0xff00 + (address as u16));
    }
    pub fn ld_np_a(&mut self, address: u8) {
        self.memory.borrow_mut().write_byte(0xFF00 + address as u16, self.a);
    }
    pub fn ld_a_ffcp(&mut self) {
        let val = self.memory.borrow_mut().read_byte(0xff00 + self.c as u16);
        self.ld_a_n(val);
    }
    pub fn ld_ffcp_a(&mut self)
    {
        self.memory.borrow_mut().write_byte(0xff00 + self.c as u16, self.a);
    }

    pub fn add_sp_n(&mut self, n: u8) {
        self.sp += n as u16;
    }

    pub fn jp_hl(&mut self) {
        self.jp_nn(self.hl());
    }

    pub fn ld_nnp_a(&mut self, address: u16) {
        self.memory.borrow_mut().write_byte(address, self.a);
    }

    pub fn push_nn(&mut self, nn: u16) {
        self.sp -= 2;
        self.memory.borrow_mut().write_word(self.sp, nn);
    }
    pub fn push_af(&mut self) {
        self.push_nn(self.af());
    }

    pub fn ld_sp_hl(&mut self) {
        self.sp = self.hl();
    }

    pub fn ld_a_nnp(&mut self, address: u16) {
        self.a = self.memory.borrow_mut().read_byte(address);
    }

    pub fn ld_hl_spn(&mut self, value: i8) {
        self.reset_flags();
    
        if (((self.sp & 0xf) + (value & 0xf) as u16) & 0x10) > 0 {
            self.flags_set(FLAGS_HALFCARRY);
        }
    
        if (((self.sp & 0xff) + (value & 0xffu8 as i8) as u16) & 0x100) > 0 {
            self.flags_set(FLAGS_CARRY);
        }
        self.set_hl(self.sp + value as u16);
    }



    /***** CB Instructions *****/
    pub fn rlc_r(&mut self, r: u8) -> u8 {
        let mut ret_r = r;
        let msb: u8 = ((ret_r & (1<<7)) != 0) as u8;
        ret_r <<= 1;
        ret_r |= msb;
    
        self.flags_clear(FLAGS_ZERO);
        self.flags_clear(FLAGS_NEGATIVE);
        self.flags_clear(FLAGS_HALFCARRY);
        self.flags_clear(FLAGS_CARRY);
        if msb == 1 {
            self.flags_set(FLAGS_CARRY);
        }
        if ret_r == 0
        {
            self.flags_set(FLAGS_ZERO);
        }
        return ret_r;
    }
    pub fn rlc_a(&mut self) { self.a = self.rlc_r(self.a); }
    pub fn rlc_b(&mut self) { self.b = self.rlc_r(self.a); }
    pub fn rlc_c(&mut self) { self.c = self.rlc_r(self.a); }
    pub fn rlc_d(&mut self) { self.d = self.rlc_r(self.a); }
    pub fn rlc_e(&mut self) { self.e = self.rlc_r(self.a); }
    pub fn rlc_h(&mut self) { self.h = self.rlc_r(self.a); }
    pub fn rlc_l(&mut self) { self.l = self.rlc_r(self.a); }
    pub fn rlc_hlp(&mut self) {
        let address = self.hl();
        let cur_hlp = self.memory.borrow_mut().read_byte(address);
        let new_hlp = self.rlc_r(cur_hlp);
        self.memory.borrow_mut().write_byte(address, new_hlp);
    }

    pub fn rrc_r(&mut self, r: u8) -> u8 {
        let lsb: bool = (r & 1) != 0;
        let mut ret_r = r;
        ret_r >>= 1;
        ret_r |= (lsb as u8) << 7;
    
        self.flags_clear(FLAGS_ZERO);
        self.flags_clear(FLAGS_NEGATIVE);
        self.flags_clear(FLAGS_HALFCARRY);
        self.flags_clear(FLAGS_CARRY);
        if lsb {
            self.flags_set(FLAGS_CARRY);
        }
        if ret_r == 0 {
            self.flags_set(FLAGS_ZERO);
        }

        return ret_r;
    }
    pub fn rrc_a(&mut self) { self.a = self.rrc_r(self.a); }
    pub fn rrc_b(&mut self) { self.b = self.rrc_r(self.b); }
    pub fn rrc_c(&mut self) { self.c = self.rrc_r(self.c); }
    pub fn rrc_d(&mut self) { self.d = self.rrc_r(self.d); }
    pub fn rrc_e(&mut self) { self.e = self.rrc_r(self.e); }
    pub fn rrc_h(&mut self) { self.h = self.rrc_r(self.h); }
    pub fn rrc_l(&mut self) { self.l = self.rrc_r(self.l); }
    pub fn rrc_hlp(&mut self) {
        let address = self.hl();
        let cur_hlp = self.memory.borrow_mut().read_byte(address);
        let new_hlp = self.rrc_r(cur_hlp);
        self.memory.borrow_mut().write_byte(address, new_hlp);
    }

    pub fn rl_r(&mut self, r: u8) -> u8 {
        // tmp = MSB
        let tmp: u8 = r & (1 << 7);
        // r << 1
        let mut ret_r = r;
        ret_r <<= 1;
        // LSB = Carry
        ret_r |= self.flags_is_carry() as u8;
        // Carry = tmp
    
        self.flags_clear(FLAGS_ZERO);
        self.flags_clear(FLAGS_NEGATIVE);
        self.flags_clear(FLAGS_HALFCARRY);
        self.flags_clear(FLAGS_CARRY);
    
        if tmp != 0 {
            self.flags_set(FLAGS_CARRY);
        }
    
        if ret_r == 0 {
            self.flags_set(FLAGS_ZERO);
        }

        return ret_r
    }
    pub fn rl_a(&mut self) { self.a = self.rl_r(self.a); }
    pub fn rl_b(&mut self) { self.b = self.rl_r(self.b); }
    pub fn rl_c(&mut self) { self.c = self.rl_r(self.c); }
    pub fn rl_d(&mut self) { self.d = self.rl_r(self.d); }
    pub fn rl_e(&mut self) { self.e = self.rl_r(self.e); }
    pub fn rl_h(&mut self) { self.h = self.rl_r(self.h); }
    pub fn rl_l(&mut self) { self.l = self.rl_r(self.l); }
    pub fn rl_hlp(&mut self) {
        let address = self.hl();
        let cur_hlp = self.memory.borrow_mut().read_byte(address);
        let new_hlp = self.rl_r(cur_hlp);
        self.memory.borrow_mut().write_byte(address, new_hlp);
    }

    pub fn rr_r(&mut self, r: u8) -> u8 {
        let mut ret_r = r;
        // lsb = r.0
        let lsb: bool = (ret_r & 1) == 1;
        // r >> 1
        ret_r >>= 1;
        // MSB = Carry
        ret_r |= (self.flags_is_carry() as u8) << 7;
        // Carry = tmp
    
        self.flags_clear(FLAGS_ZERO);
        self.flags_clear(FLAGS_NEGATIVE);
        self.flags_clear(FLAGS_HALFCARRY);
        self.flags_clear(FLAGS_CARRY);
    
        if lsb {
            self.flags_set(FLAGS_CARRY);
        }
    
        if ret_r == 0 {
            self.flags_set(FLAGS_ZERO);
        }

        return ret_r;
    }
    pub fn rr_a(&mut self) { self.a = self.rr_r(self.a); }
    pub fn rr_b(&mut self) { self.b = self.rr_r(self.b); }
    pub fn rr_c(&mut self) { self.c = self.rr_r(self.c); }
    pub fn rr_d(&mut self) { self.d = self.rr_r(self.d); }
    pub fn rr_e(&mut self) { self.e = self.rr_r(self.e); }
    pub fn rr_h(&mut self) { self.h = self.rr_r(self.h); }
    pub fn rr_l(&mut self) { self.l = self.rr_r(self.l); }
    pub fn rr_hlp(&mut self) {
        let address = self.hl();
        let cur_hlp = self.memory.borrow_mut().read_byte(address);
        let new_hlp = self.rr_r(cur_hlp);
        self.memory.borrow_mut().write_byte(address, new_hlp);
    }

    pub fn sla_r(&mut self, r: u8) -> u8 {
        // Left shift on gameboy is r.0 = 0. This should match *unsigned* shift left in C99.
        // TODO: Verify correct behavior.
        let mut ret_r = r;
        let carry = (ret_r & (1 << 7)) != 0;
        ret_r <<= 1;

        self.flags_clear(FLAGS_ZERO);
        self.flags_clear(FLAGS_NEGATIVE);
        self.flags_clear(FLAGS_HALFCARRY);
        self.flags_clear(FLAGS_CARRY);

        if ret_r == 0 {
            self.flags_set(FLAGS_ZERO);
        }
        if carry {
            self.flags_set(FLAGS_CARRY);
        }

        return ret_r;
    }
    pub fn sla_a(&mut self) { self.a = self.sla_r(self.a); }
    pub fn sla_b(&mut self) { self.b = self.sla_r(self.b); }
    pub fn sla_c(&mut self) { self.c = self.sla_r(self.c); }
    pub fn sla_d(&mut self) { self.d = self.sla_r(self.d); }
    pub fn sla_e(&mut self) { self.e = self.sla_r(self.e); }
    pub fn sla_h(&mut self) { self.h = self.sla_r(self.h); }
    pub fn sla_l(&mut self) { self.l = self.sla_r(self.l); }
    pub fn sla_hlp(&mut self) {
        let address = self.hl();
        let cur_hlp = self.memory.borrow_mut().read_byte(address);
        let new_hlp = self.sla_r(cur_hlp);
        self.memory.borrow_mut().write_byte(address, new_hlp);
    }


    pub fn sra_r(&mut self, r: u8) -> u8 {
        // SRA on gameboy is right shift with r.7 = old r.7
        // This should match *signed* right shift in C99.
        // TODO: Verify correct behavior.
        let mut ret_r = r;
        let carry = (ret_r & 1) != 0;
        ret_r >>= 1;

        self.flags_clear(FLAGS_ZERO);
        self.flags_clear(FLAGS_NEGATIVE);
        self.flags_clear(FLAGS_HALFCARRY);
        self.flags_clear(FLAGS_CARRY);

        if ret_r == 0 {
            self.flags_set(FLAGS_ZERO);
        }
        if carry {
            self.flags_set(FLAGS_CARRY);
        }

        return ret_r;
    }
    pub fn sra_a(&mut self) { self.a = self.sra_r(self.a); }
    pub fn sra_b(&mut self) { self.b = self.sra_r(self.b); }
    pub fn sra_c(&mut self) { self.c = self.sra_r(self.c); }
    pub fn sra_d(&mut self) { self.d = self.sra_r(self.d); }
    pub fn sra_e(&mut self) { self.e = self.sra_r(self.e); }
    pub fn sra_h(&mut self) { self.h = self.sra_r(self.h); }
    pub fn sra_l(&mut self) { self.l = self.sra_r(self.l); }
    pub fn sra_hlp(&mut self) {
        let address = self.hl();
        let cur_hlp = self.memory.borrow_mut().read_byte(address);
        let new_hlp = self.sra_r(cur_hlp);
        self.memory.borrow_mut().write_byte(address, new_hlp);
    }

    pub fn swap_r(&mut self, r: u8) -> u8 {
        let lsb_nibble = r & 0x0f;
        let msb_nibble = r & 0xf0;
        let ret_r = (lsb_nibble << 4) | (msb_nibble >> 4);

        self.flags_clear(FLAGS_ZERO);
        self.flags_clear(FLAGS_NEGATIVE);
        self.flags_clear(FLAGS_HALFCARRY);
        self.flags_clear(FLAGS_CARRY);

        if ret_r == 0 {
            self.flags_set(FLAGS_ZERO);
        }

        return ret_r;
    }
    pub fn swap_a(&mut self) { self.a = self.swap_r(self.a); }
    pub fn swap_b(&mut self) { self.b = self.swap_r(self.b); }
    pub fn swap_c(&mut self) { self.c = self.swap_r(self.c); }
    pub fn swap_d(&mut self) { self.d = self.swap_r(self.d); }
    pub fn swap_e(&mut self) { self.e = self.swap_r(self.e); }
    pub fn swap_h(&mut self) { self.h = self.swap_r(self.h); }
    pub fn swap_l(&mut self) { self.l = self.swap_r(self.l); }
    pub fn swap_hlp(&mut self) {
        let address = self.hl();
        let cur_hlp = self.memory.borrow_mut().read_byte(address);
        let new_hlp = self.swap_r(cur_hlp);
        self.memory.borrow_mut().write_byte(address, new_hlp);
    }


    pub fn srl_r(&mut self, r: u8) -> u8 {
        // SRL on gameboy is right shift with r.7 = 0
        // This should match *unsigned* shift right in C99.
        // TODO: Verify correct behavior.
        let mut ret_r = r;
        let carry = (ret_r & 1) != 0;
        ret_r >>= 1;

        self.flags_clear(FLAGS_ZERO);
        self.flags_clear(FLAGS_NEGATIVE);
        self.flags_clear(FLAGS_HALFCARRY);
        self.flags_clear(FLAGS_CARRY);

        if ret_r == 0 {
            self.flags_set(FLAGS_ZERO);
        }
        if carry {
            self.flags_set(FLAGS_CARRY);
        }

        return ret_r;
    }
    pub fn srl_a(&mut self) { self.a = self.srl_r(self.a); }
    pub fn srl_b(&mut self) { self.b = self.srl_r(self.b); }
    pub fn srl_c(&mut self) { self.c = self.srl_r(self.c); }
    pub fn srl_d(&mut self) { self.d = self.srl_r(self.d); }
    pub fn srl_e(&mut self) { self.e = self.srl_r(self.e); }
    pub fn srl_h(&mut self) { self.h = self.srl_r(self.h); }
    pub fn srl_l(&mut self) { self.l = self.srl_r(self.l); }
    pub fn srl_hlp(&mut self) {
        let address = self.hl();
        let cur_hlp = self.memory.borrow_mut().read_byte(address);
        let new_hlp = self.srl_r(cur_hlp);
        self.memory.borrow_mut().write_byte(address, new_hlp);
    }

    // Test if bit in register r is 0
    pub fn bit_n_r(&mut self, bit: u8, r: u8) {
        self.flags_clear(FLAGS_ZERO);
        self.flags_clear(FLAGS_NEGATIVE);
        self.flags_set(FLAGS_HALFCARRY);

        if (r & (1 << bit)) == 0 {
            self.flags_set(FLAGS_ZERO);
        }
    }

    pub fn bit_0_a(&mut self) { self.bit_n_r(0, self.a); }
    pub fn bit_0_b(&mut self) { self.bit_n_r(0, self.b); }
    pub fn bit_0_c(&mut self) { self.bit_n_r(0, self.c); }
    pub fn bit_0_d(&mut self) { self.bit_n_r(0, self.d); }
    pub fn bit_0_e(&mut self) { self.bit_n_r(0, self.e); }
    pub fn bit_0_h(&mut self) { self.bit_n_r(0, self.h); }
    pub fn bit_0_l(&mut self) { self.bit_n_r(0, self.l); }
    pub fn bit_0_hlp(&mut self) { self.bit_n_r(0, self.hlp()); }
    pub fn bit_1_a(&mut self) { self.bit_n_r(1, self.a); }
    pub fn bit_1_b(&mut self) { self.bit_n_r(1, self.b); }
    pub fn bit_1_c(&mut self) { self.bit_n_r(1, self.c); }
    pub fn bit_1_d(&mut self) { self.bit_n_r(1, self.d); }
    pub fn bit_1_e(&mut self) { self.bit_n_r(1, self.e); }
    pub fn bit_1_h(&mut self) { self.bit_n_r(1, self.h); }
    pub fn bit_1_l(&mut self) { self.bit_n_r(1, self.l); }
    pub fn bit_1_hlp(&mut self) { self.bit_n_r(1, self.hlp()); }
    pub fn bit_2_a(&mut self) { self.bit_n_r(2, self.a); }
    pub fn bit_2_b(&mut self) { self.bit_n_r(2, self.b); }
    pub fn bit_2_c(&mut self) { self.bit_n_r(2, self.c); }
    pub fn bit_2_d(&mut self) { self.bit_n_r(2, self.d); }
    pub fn bit_2_e(&mut self) { self.bit_n_r(2, self.e); }
    pub fn bit_2_h(&mut self) { self.bit_n_r(2, self.h); }
    pub fn bit_2_l(&mut self) { self.bit_n_r(2, self.l); }
    pub fn bit_2_hlp(&mut self) { self.bit_n_r(2, self.hlp()); }
    pub fn bit_3_a(&mut self) { self.bit_n_r(3, self.a); }
    pub fn bit_3_b(&mut self) { self.bit_n_r(3, self.b); }
    pub fn bit_3_c(&mut self) { self.bit_n_r(3, self.c); }
    pub fn bit_3_d(&mut self) { self.bit_n_r(3, self.d); }
    pub fn bit_3_e(&mut self) { self.bit_n_r(3, self.e); }
    pub fn bit_3_h(&mut self) { self.bit_n_r(3, self.h); }
    pub fn bit_3_l(&mut self) { self.bit_n_r(3, self.l); }
    pub fn bit_3_hlp(&mut self) { self.bit_n_r(3, self.hlp()); }
    pub fn bit_4_a(&mut self) { self.bit_n_r(4, self.a); }
    pub fn bit_4_b(&mut self) { self.bit_n_r(4, self.b); }
    pub fn bit_4_c(&mut self) { self.bit_n_r(4, self.c); }
    pub fn bit_4_d(&mut self) { self.bit_n_r(4, self.d); }
    pub fn bit_4_e(&mut self) { self.bit_n_r(4, self.e); }
    pub fn bit_4_h(&mut self) { self.bit_n_r(4, self.h); }
    pub fn bit_4_l(&mut self) { self.bit_n_r(4, self.l); }
    pub fn bit_4_hlp(&mut self) { self.bit_n_r(4, self.hlp()); }
    pub fn bit_5_a(&mut self) { self.bit_n_r(5, self.a); }
    pub fn bit_5_b(&mut self) { self.bit_n_r(5, self.b); }
    pub fn bit_5_c(&mut self) { self.bit_n_r(5, self.c); }
    pub fn bit_5_d(&mut self) { self.bit_n_r(5, self.d); }
    pub fn bit_5_e(&mut self) { self.bit_n_r(5, self.e); }
    pub fn bit_5_h(&mut self) { self.bit_n_r(5, self.h); }
    pub fn bit_5_l(&mut self) { self.bit_n_r(5, self.l); }
    pub fn bit_5_hlp(&mut self) { self.bit_n_r(5, self.hlp()); }
    pub fn bit_6_a(&mut self) { self.bit_n_r(6, self.a); }
    pub fn bit_6_b(&mut self) { self.bit_n_r(6, self.b); }
    pub fn bit_6_c(&mut self) { self.bit_n_r(6, self.c); }
    pub fn bit_6_d(&mut self) { self.bit_n_r(6, self.d); }
    pub fn bit_6_e(&mut self) { self.bit_n_r(6, self.e); }
    pub fn bit_6_h(&mut self) { self.bit_n_r(6, self.h); }
    pub fn bit_6_l(&mut self) { self.bit_n_r(6, self.l); }
    pub fn bit_6_hlp(&mut self) { self.bit_n_r(6, self.hlp()); }
    pub fn bit_7_a(&mut self) { self.bit_n_r(7, self.a); }
    pub fn bit_7_b(&mut self) { self.bit_n_r(7, self.b); }
    pub fn bit_7_c(&mut self) { self.bit_n_r(7, self.c); }
    pub fn bit_7_d(&mut self) { self.bit_n_r(7, self.d); }
    pub fn bit_7_e(&mut self) { self.bit_n_r(7, self.e); }
    pub fn bit_7_h(&mut self) { self.bit_n_r(7, self.h); }
    pub fn bit_7_l(&mut self) { self.bit_n_r(7, self.l); }
    pub fn bit_7_hlp(&mut self) { self.bit_n_r(7, self.hlp()); }

    // reset bit in r to 0
    pub fn res_n_r(&self, bit: u8, r: u8) -> u8 {
        return r & !(1 << bit);
    }
    pub fn res_0_a(&mut self) { self.a = self.res_n_r(0, self.a); }
    pub fn res_0_b(&mut self) { self.b = self.res_n_r(0, self.b); }
    pub fn res_0_c(&mut self) { self.c = self.res_n_r(0, self.c); }
    pub fn res_0_d(&mut self) { self.d = self.res_n_r(0, self.d); }
    pub fn res_0_e(&mut self) { self.e = self.res_n_r(0, self.e); }
    pub fn res_0_h(&mut self) { self.h = self.res_n_r(0, self.h); }
    pub fn res_0_l(&mut self) { self.l = self.res_n_r(0, self.l); }
    pub fn res_0_hlp(&mut self) { self.set_hlp(self.res_n_r(0, self.hlp())); }
    pub fn res_1_a(&mut self) { self.a = self.res_n_r(1, self.a); }
    pub fn res_1_b(&mut self) { self.b = self.res_n_r(1, self.b); }
    pub fn res_1_c(&mut self) { self.c = self.res_n_r(1, self.c); }
    pub fn res_1_d(&mut self) { self.d = self.res_n_r(1, self.d); }
    pub fn res_1_e(&mut self) { self.e = self.res_n_r(1, self.e); }
    pub fn res_1_h(&mut self) { self.h = self.res_n_r(1, self.h); }
    pub fn res_1_l(&mut self) { self.l = self.res_n_r(1, self.l); }
    pub fn res_1_hlp(&mut self) { self.set_hlp(self.res_n_r(1, self.hlp())); }
    pub fn res_2_a(&mut self) { self.a = self.res_n_r(2, self.a); }
    pub fn res_2_b(&mut self) { self.b = self.res_n_r(2, self.b); }
    pub fn res_2_c(&mut self) { self.c = self.res_n_r(2, self.c); }
    pub fn res_2_d(&mut self) { self.d = self.res_n_r(2, self.d); }
    pub fn res_2_e(&mut self) { self.e = self.res_n_r(2, self.e); }
    pub fn res_2_h(&mut self) { self.h = self.res_n_r(2, self.h); }
    pub fn res_2_l(&mut self) { self.l = self.res_n_r(2, self.l); }
    pub fn res_2_hlp(&mut self) { self.set_hlp(self.res_n_r(2, self.hlp())); }
    pub fn res_3_a(&mut self) { self.a = self.res_n_r(3, self.a); }
    pub fn res_3_b(&mut self) { self.b = self.res_n_r(3, self.b); }
    pub fn res_3_c(&mut self) { self.c = self.res_n_r(3, self.c); }
    pub fn res_3_d(&mut self) { self.d = self.res_n_r(3, self.d); }
    pub fn res_3_e(&mut self) { self.e = self.res_n_r(3, self.e); }
    pub fn res_3_h(&mut self) { self.h = self.res_n_r(3, self.h); }
    pub fn res_3_l(&mut self) { self.l = self.res_n_r(3, self.l); }
    pub fn res_3_hlp(&mut self) { self.set_hlp(self.res_n_r(3, self.hlp())); }
    pub fn res_4_a(&mut self) { self.a = self.res_n_r(4, self.a); }
    pub fn res_4_b(&mut self) { self.b = self.res_n_r(4, self.b); }
    pub fn res_4_c(&mut self) { self.c = self.res_n_r(4, self.c); }
    pub fn res_4_d(&mut self) { self.d = self.res_n_r(4, self.d); }
    pub fn res_4_e(&mut self) { self.e = self.res_n_r(4, self.e); }
    pub fn res_4_h(&mut self) { self.h = self.res_n_r(4, self.h); }
    pub fn res_4_l(&mut self) { self.l = self.res_n_r(4, self.l); }
    pub fn res_4_hlp(&mut self) { self.set_hlp(self.res_n_r(4, self.hlp())); }
    pub fn res_5_a(&mut self) { self.a = self.res_n_r(5, self.a); }
    pub fn res_5_b(&mut self) { self.b = self.res_n_r(5, self.b); }
    pub fn res_5_c(&mut self) { self.c = self.res_n_r(5, self.c); }
    pub fn res_5_d(&mut self) { self.d = self.res_n_r(5, self.d); }
    pub fn res_5_e(&mut self) { self.e = self.res_n_r(5, self.e); }
    pub fn res_5_h(&mut self) { self.h = self.res_n_r(5, self.h); }
    pub fn res_5_l(&mut self) { self.l = self.res_n_r(5, self.l); }
    pub fn res_5_hlp(&mut self) { self.set_hlp(self.res_n_r(5, self.hlp())); }
    pub fn res_6_a(&mut self) { self.a = self.res_n_r(6, self.a); }
    pub fn res_6_b(&mut self) { self.b = self.res_n_r(6, self.b); }
    pub fn res_6_c(&mut self) { self.c = self.res_n_r(6, self.c); }
    pub fn res_6_d(&mut self) { self.d = self.res_n_r(6, self.d); }
    pub fn res_6_e(&mut self) { self.e = self.res_n_r(6, self.e); }
    pub fn res_6_h(&mut self) { self.h = self.res_n_r(6, self.h); }
    pub fn res_6_l(&mut self) { self.l = self.res_n_r(6, self.l); }
    pub fn res_6_hlp(&mut self) { self.set_hlp(self.res_n_r(6, self.hlp())); }
    pub fn res_7_a(&mut self) { self.a = self.res_n_r(7, self.a); }
    pub fn res_7_b(&mut self) { self.b = self.res_n_r(7, self.b); }
    pub fn res_7_c(&mut self) { self.c = self.res_n_r(7, self.c); }
    pub fn res_7_d(&mut self) { self.d = self.res_n_r(7, self.d); }
    pub fn res_7_e(&mut self) { self.e = self.res_n_r(7, self.e); }
    pub fn res_7_h(&mut self) { self.h = self.res_n_r(7, self.h); }
    pub fn res_7_l(&mut self) { self.l = self.res_n_r(7, self.l); }
    pub fn res_7_hlp(&mut self) { self.set_hlp(self.res_n_r(7, self.hlp())); }

    pub fn set_n_r(&self, bit: u8, r: u8) -> u8 {
        return r | (1 << bit);
    }
    pub fn set_0_a(&mut self) { self.a = self.set_n_r(0, self.a); }
    pub fn set_0_b(&mut self) { self.b = self.set_n_r(0, self.b); }
    pub fn set_0_c(&mut self) { self.c = self.set_n_r(0, self.c); }
    pub fn set_0_d(&mut self) { self.d = self.set_n_r(0, self.d); }
    pub fn set_0_e(&mut self) { self.e = self.set_n_r(0, self.e); }
    pub fn set_0_h(&mut self) { self.h = self.set_n_r(0, self.h); }
    pub fn set_0_l(&mut self) { self.l = self.set_n_r(0, self.l); }
    pub fn set_0_hlp(&mut self) { self.set_hlp(self.set_n_r(0, self.hlp())); }
    pub fn set_1_a(&mut self) { self.a = self.set_n_r(1, self.a); }
    pub fn set_1_b(&mut self) { self.b = self.set_n_r(1, self.b); }
    pub fn set_1_c(&mut self) { self.c = self.set_n_r(1, self.c); }
    pub fn set_1_d(&mut self) { self.d = self.set_n_r(1, self.d); }
    pub fn set_1_e(&mut self) { self.e = self.set_n_r(1, self.e); }
    pub fn set_1_h(&mut self) { self.h = self.set_n_r(1, self.h); }
    pub fn set_1_l(&mut self) { self.l = self.set_n_r(1, self.l); }
    pub fn set_1_hlp(&mut self) { self.set_hlp(self.set_n_r(1, self.hlp())); }
    pub fn set_2_a(&mut self) { self.a = self.set_n_r(2, self.a); }
    pub fn set_2_b(&mut self) { self.b = self.set_n_r(2, self.b); }
    pub fn set_2_c(&mut self) { self.c = self.set_n_r(2, self.c); }
    pub fn set_2_d(&mut self) { self.d = self.set_n_r(2, self.d); }
    pub fn set_2_e(&mut self) { self.e = self.set_n_r(2, self.e); }
    pub fn set_2_h(&mut self) { self.h = self.set_n_r(2, self.h); }
    pub fn set_2_l(&mut self) { self.l = self.set_n_r(2, self.l); }
    pub fn set_2_hlp(&mut self) { self.set_hlp(self.set_n_r(2, self.hlp())); }
    pub fn set_3_a(&mut self) { self.a = self.set_n_r(3, self.a); }
    pub fn set_3_b(&mut self) { self.b = self.set_n_r(3, self.b); }
    pub fn set_3_c(&mut self) { self.c = self.set_n_r(3, self.c); }
    pub fn set_3_d(&mut self) { self.d = self.set_n_r(3, self.d); }
    pub fn set_3_e(&mut self) { self.e = self.set_n_r(3, self.e); }
    pub fn set_3_h(&mut self) { self.h = self.set_n_r(3, self.h); }
    pub fn set_3_l(&mut self) { self.l = self.set_n_r(3, self.l); }
    pub fn set_3_hlp(&mut self) { self.set_hlp(self.set_n_r(3, self.hlp())); }
    pub fn set_4_a(&mut self) { self.a = self.set_n_r(4, self.a); }
    pub fn set_4_b(&mut self) { self.b = self.set_n_r(4, self.b); }
    pub fn set_4_c(&mut self) { self.c = self.set_n_r(4, self.c); }
    pub fn set_4_d(&mut self) { self.d = self.set_n_r(4, self.d); }
    pub fn set_4_e(&mut self) { self.e = self.set_n_r(4, self.e); }
    pub fn set_4_h(&mut self) { self.h = self.set_n_r(4, self.h); }
    pub fn set_4_l(&mut self) { self.l = self.set_n_r(4, self.l); }
    pub fn set_4_hlp(&mut self) { self.set_hlp(self.set_n_r(4, self.hlp())); }
    pub fn set_5_a(&mut self) { self.a = self.set_n_r(5, self.a); }
    pub fn set_5_b(&mut self) { self.b = self.set_n_r(5, self.b); }
    pub fn set_5_c(&mut self) { self.c = self.set_n_r(5, self.c); }
    pub fn set_5_d(&mut self) { self.d = self.set_n_r(5, self.d); }
    pub fn set_5_e(&mut self) { self.e = self.set_n_r(5, self.e); }
    pub fn set_5_h(&mut self) { self.h = self.set_n_r(5, self.h); }
    pub fn set_5_l(&mut self) { self.l = self.set_n_r(5, self.l); }
    pub fn set_5_hlp(&mut self) { self.set_hlp(self.set_n_r(5, self.hlp())); }
    pub fn set_6_a(&mut self) { self.a = self.set_n_r(6, self.a); }
    pub fn set_6_b(&mut self) { self.b = self.set_n_r(6, self.b); }
    pub fn set_6_c(&mut self) { self.c = self.set_n_r(6, self.c); }
    pub fn set_6_d(&mut self) { self.d = self.set_n_r(6, self.d); }
    pub fn set_6_e(&mut self) { self.e = self.set_n_r(6, self.e); }
    pub fn set_6_h(&mut self) { self.h = self.set_n_r(6, self.h); }
    pub fn set_6_l(&mut self) { self.l = self.set_n_r(6, self.l); }
    pub fn set_6_hlp(&mut self) { self.set_hlp(self.set_n_r(6, self.hlp())); }
    pub fn set_7_a(&mut self) { self.a = self.set_n_r(7, self.a); }
    pub fn set_7_b(&mut self) { self.b = self.set_n_r(7, self.b); }
    pub fn set_7_c(&mut self) { self.c = self.set_n_r(7, self.c); }
    pub fn set_7_d(&mut self) { self.d = self.set_n_r(7, self.d); }
    pub fn set_7_e(&mut self) { self.e = self.set_n_r(7, self.e); }
    pub fn set_7_h(&mut self) { self.h = self.set_n_r(7, self.h); }
    pub fn set_7_l(&mut self) { self.l = self.set_n_r(7, self.l); }
    pub fn set_7_hlp(&mut self) { self.set_hlp(self.set_n_r(7, self.hlp())); }
    

    

}

/*struct registers_t {
    union:u16 {
        i: af_inner,
        af2: u16,
    },
}*/

// Tests
#[cfg(test)]
mod cpu_tests {
    use super::*;

    #[test]
    fn test_registers() {
        //let mut registers:registers_t = registers_t::default();
        //registers.af = 0xbeefu16;
        //assert_eq!(registers.af.f, 0xbe);
        //assert_eq!(registers.af.a, 0xef);
    }
}