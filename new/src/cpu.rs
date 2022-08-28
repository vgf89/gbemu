use crate::{memory::*, ops_table};
use std::rc::Rc;
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
        match ops_table::instructions[0].execute {
            FnEnum::OpLen1(a) => {(a)(); println!("oplen1")},
            _ => println!("anything"),
        }
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
    pub fn set_bc(&mut self, af:u16) {
        self.b = ((af >> 8) & 0xff) as u8;
        self.c = (af & 0xff) as u8;
    }

    pub fn de(&self) -> u16 {
        return (self.d as u16) << 8 | (self.e as u16);
    }
    pub fn set_de(&mut self, af:u16) {
        self.d = ((af >> 8) & 0xff) as u8;
        self.e = (af & 0xff) as u8;
    }

    pub fn hl(&self) -> u16 {
        return (self.h as u16) << 8 | (self.l as u16);
    }
    pub fn set_hl(&mut self, af:u16) {
        self.h = ((af >> 8) & 0xff) as u8;
        self.l = (af & 0xff) as u8;
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
    pub fn cpuStep() {
        // FIXME: Implement
    }



    // Opcodes
    pub fn nop() 
    {
        println!("NOP");
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