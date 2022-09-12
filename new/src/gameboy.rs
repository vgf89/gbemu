pub const MAX_CLOCK:u32 = 69905; // Number of cycles per frame
// Magic number for 60FPS. 4194304 cycles per second / 60 = 69905
// Possible rates:
    // DMG: 69905
    // Super Game Boy 71590 (plus this one's divisible by 4), which might result in frame timing match?
// There are likely better ways to handle this, i.e. just waiting for VBLANK interrupt to break the step loop.
// If VBLANK timing is consistent though, we should be breaking based on that timing.
use crate::cpu::*;
use crate::memory::*;
use std::cell::RefCell;
use std::i16::MAX;
pub struct Gameboy {
    pub clock:u32,
    pub frames:u32,
    pub processor: CPU,
}

impl Gameboy {
    pub fn new() -> Self {
        let mem: RefCell<Memory> = RefCell::new(Memory::default());
        let mut proc = CPU::new(mem);
        mem.borrow_mut().load_rom("testroms/gb-test-roms-master/cpu_instrs/cpu_instrs.gb".to_string());
        

        return Self {
            clock: 0,
            frames: 0,
            processor: proc,
        }
    }
    pub fn reset(&mut self) {
        // BOOT
        self.processor.set_af(0x01b0);
        self.processor.set_bc(0x0013);
        self.processor.set_de(0x00d8);
        self.processor.set_hl(0x014d);
        self.processor.sp = 0xfffe;
    
        self.processor.memory.borrow_mut().write_byte(0xff05, 0x00);
        self.processor.memory.borrow_mut().write_byte(0xff06, 0x00);
        self.processor.memory.borrow_mut().write_byte(0xff07, 0x00);
        self.processor.memory.borrow_mut().write_byte(0xff10, 0x80);
        self.processor.memory.borrow_mut().write_byte(0xff11, 0xbf);
        self.processor.memory.borrow_mut().write_byte(0xff12, 0xf3);
        self.processor.memory.borrow_mut().write_byte(0xff14, 0xbf);
        self.processor.memory.borrow_mut().write_byte(0xff16, 0x3f);
        self.processor.memory.borrow_mut().write_byte(0xff17, 0x00);
        self.processor.memory.borrow_mut().write_byte(0xff19, 0xbf);
        self.processor.memory.borrow_mut().write_byte(0xff1a, 0x7f);
        self.processor.memory.borrow_mut().write_byte(0xff1b, 0xff);
        self.processor.memory.borrow_mut().write_byte(0xff1c, 0x9f);
        self.processor.memory.borrow_mut().write_byte(0xff1e, 0xbf);
        self.processor.memory.borrow_mut().write_byte(0xff20, 0xff);
        self.processor.memory.borrow_mut().write_byte(0xff21, 0x00);
        self.processor.memory.borrow_mut().write_byte(0xff22, 0x00);
        self.processor.memory.borrow_mut().write_byte(0xff23, 0xbf);
        self.processor.memory.borrow_mut().write_byte(0xff24, 0x77);
        self.processor.memory.borrow_mut().write_byte(0xff25, 0xf3);
        self.processor.memory.borrow_mut().write_byte(0xff26, 0xf1-1); //what does this mean [$FF26, $F1-GB, $F0-SGB ); NR52
        self.processor.memory.borrow_mut().write_byte(0xff40, 0x91);
        self.processor.memory.borrow_mut().write_byte(0xff42, 0x00);
        self.processor.memory.borrow_mut().write_byte(0xff43, 0x00);
        self.processor.memory.borrow_mut().write_byte(0xff45, 0x00);
        self.processor.memory.borrow_mut().write_byte(0xff47, 0xfc);
        self.processor.memory.borrow_mut().write_byte(0xff48, 0xff);
        self.processor.memory.borrow_mut().write_byte(0xff49, 0xff);
        self.processor.memory.borrow_mut().write_byte(0xff4a, 0x00);
        self.processor.memory.borrow_mut().write_byte(0xff4b, 0x00);
        self.processor.memory.borrow_mut().write_byte(0xffff, 0x00);
    
        self.processor.pc = 0x0100;
    
        // Other stuff
        self.processor.memory.borrow_mut().write_byte(0xff00, 0xff); // set default joypad to nothing pressed
    
        // FIXME: implment ppu
        //init_ppu();

    }

    pub fn step(&mut self) {
        // How to schedule CPU steps:
        // Number of dots (clock edges) per frame (60fps): 70000
        // CPU cycle is 4 dots
        // PPU cycle is 2 dots
        // CPU and PPU use their own cycle counters, and compare
        // against the master dot clock to schedule execution.

        self.clock = 0;

        while self.clock < MAX_CLOCK {
            //self.updateInput();
            self.processor.cpu_step(self.clock);
            //self.timerStep();
            //self.ppuStep();
            self.clock += 1;
        }

        self.frames += 1;

        //self.reset_cpu_clock(MAX_CLOCK);
        //self.reset_ppu_clock(MAX_CLOCK);
        
    }
}