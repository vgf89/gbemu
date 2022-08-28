use crate::{memory::*, ops_table};
use std::{rc::Rc, borrow::{Borrow, BorrowMut}};
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
    pub clock: u32,
    pub IME_flag: u8,
    pub set_ei: u8,
    pub res_ei: u8 ,
    pub halted: u8,
    pub halt_bug: u8,

    memory: Rc<Memory>,
}

pub const FLAGS_ZERO:u8 = 1 << 7;
pub const FLAGS_NEGATIVE:u8 = 1 << 6;
pub const FLAGS_HALFCARRY:u8 = 1 << 5;
pub const FLAGS_CARRY:u8 = 1 << 4;

impl CPU {
    pub fn new(mem: Rc<Memory>) -> Self {
        return Self{
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

            clock: 0,
            IME_flag: 1,
            set_ei: 0,
            res_ei: 0,
            halted: 0,
            halt_bug: 0,
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

    pub fn flags_is_zero(&self) -> u8{
        return self.f & FLAGS_ZERO;
    }
    pub fn flags_is_negative(&self) -> u8{
        return self.f & FLAGS_NEGATIVE;
    }
    pub fn flags_is_halfcarry(&self) -> u8{
        return self.f & FLAGS_HALFCARRY;
    }
    pub fn flags_is_carry(&self) -> u8{
        return self.f & FLAGS_CARRY;
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
        self.clock = self.clock - maxclock as u32;
    }

    // CPU stepper
    pub fn cpuStep(&mut self) {
        // FIXME: Implement

        match ops_table::instructions[0].execute {
            FnEnum::OpLen1(op) => (op)(self),
            FnEnum::OpLen2(op) => (op)(self, 0u8), // FIXME: Get value
            FnEnum::OpLen3(op) => (op)(self, 0u16), // FIXME: Get value
            NULL => (),
        }
    }



    // Opcodes
    pub fn nop(&mut self) 
    {
        // do nothing
    }

    pub fn ld_bc_nn(&mut self, value: u16) { self.set_bc(value); }
    pub fn ld_bcp_a(&mut self) { self.memory.writeByte(self.bc(), self.a); }
    pub fn inc_bc(&mut self) { self.set_bc(self.bc() + 1); }
    pub fn inc_b(&mut self) { self.b = self.b + 1; }
    pub fn dec_b(&mut self) { self.b = self.b - 1; }
    pub fn ld_b_n(&mut self, value: u8) {self.b = value;}
    pub fn rlca(&mut self) {
        self.rlc_a();
        self.flags_clear(FLAGS_ZERO);
        self.flags_clear(FLAGS_NEGATIVE);
        self.flags_clear(FLAGS_HALFCARRY);
    }
    pub fn ld_nnp_sp(&mut self, address: u16) { self.memory.writeWord(address, self.sp); }
    pub fn add_hl_bc(&mut self) { self.add_hl_nn(self.bc()); }
    pub fn ld_a_bcp(&mut self) { self.a = self.memory.readByte(self.bc()); }
    pub fn dec_bc(&mut self) { self.set_bc(self.bc() - 1); }
    pub fn inc_c(&mut self) { self.c = self.c + 1; }
    pub fn dec_c(&mut self) { self.c = self.c - 1; }
    pub fn ld_c_n(&mut self, value: u8) { self.c = value; }
    pub fn rrca(&mut self) {
        self.rrc_a();
        self.flags_clear(FLAGS_ZERO);
        self.flags_clear(FLAGS_NEGATIVE);
        self.flags_clear(FLAGS_HALFCARRY);
    }

    pub fn ld_de_nn(&mut self, value: u16) { self.set_de(value); }
    pub fn ld_dep_a(&mut self) { self.memory.writeByte(self.de(), self.a); }
    pub fn inc_de(&mut self) { self.set_de(self.de() + 1); }
    pub fn inc_d(&mut self) { self.d = self.d + 1; }
    pub fn dec_d(&mut self) { self.d = self.d - 1; }
    pub fn ld_d_n(&mut self, value: u8) {self.d = value;}
    pub fn rla(&mut self) {
        self.rl_a();
        self.flags_clear(FLAGS_ZERO);
        self.flags_clear(FLAGS_NEGATIVE);
        self.flags_clear(FLAGS_HALFCARRY);
    }
    pub fn jr_nn(&mut self, offset: i8) { self.pc = self.pc.wrapping_add(offset as u16); }
    pub fn add_hl_de(&mut self) { self.add_hl_nn(self.de()); }
    pub fn ld_a_dep(&mut self) { self.a = self.memory.readByte(self.de()); }
    pub fn dec_de(&mut self) { self.set_de(self.de() - 1); }
    pub fn inc_e(&mut self) { self.e = self.e + 1; }
    pub fn dec_e(&mut self) { self.e = self.e - 1; }
    pub fn ld_e_n(&mut self, value: u8) { self.c = value; }
    pub fn rra(&mut self) {
        self.rr_a();
        self.flags_clear(FLAGS_ZERO);
        self.flags_clear(FLAGS_NEGATIVE);
        self.flags_clear(FLAGS_HALFCARRY);
    }
    pub fn jr_nz(&mut self, offset: i8) {
        if self.flags_is_zero() == 0
        {
            self.jr_nn(offset);
            self.clock = self.clock + 4;
        }
    }





    pub fn di(&mut self) {
        self.IME_flag = 0;
        //res_ei = 2;
    }
    pub fn ei(&mut self) {
        self.IME_flag = 1;
        //set_ei = 2;
    }
    pub fn reti(&mut self)
    {
        //self.ret(); FIXME: Impleent ret
        self.ei();
        //IME_flag = 1;
    }
    
    pub fn halt(&mut self) {
        if self.IME_flag > 0 {
            self.halted = 1;
        } else {
            if (self.memory.ram[IE as usize] & self.memory.ram[IFLAGS as usize] & 0x1f) == 0 {
                self.halted = 1;
                // Halt mode is entered, but the interupt vector is not called
                // and IF isn't cleared (it instead just continue executing when an interrupt is received)
                // Can check this via (halted && (IME_flag == 0))
            } else {
                // Halt mode is not entered.
                // CPU does not increase pc on next instruction. IF flags aren't cleared
                self.halt_bug = 1;
            }
        }
    }
    
    pub fn reset_inc_flags(&mut self)
    {
        self.flags_clear(FLAGS_ZERO);
        self.flags_clear(FLAGS_NEGATIVE);
        self.flags_clear(FLAGS_HALFCARRY);
    }

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