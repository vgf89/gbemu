pub const MAX_CLOCK:u32 = 69905; // Number of cycles per frame
// Magic number for 60FPS. 4194304 cycles per second / 60 = 69905
// Possible rates:
    // DMG: 69905
    // Super Game Boy 71590 (plus this one's divisible by 4), which might result in frame timing match?
// There are likely better ways to handle this, i.e. just waiting for VBLANK interrupt to break the step loop.
// If VBLANK timing is consistent though, we should be breaking based on that timing.
use crate::cpu::*;
use crate::memory::*;
use std::rc::Rc;
use std::cell::RefCell;
pub struct Gameboy {
    pub clock:u32,
    pub frames:u32,
    pub processor: CPU,
}

impl Gameboy {
    pub fn new() -> Self {
        let mem: RefCell<Memory> = RefCell::new(Memory::default());
        let mut proc = CPU::new(mem);

        return Self {
            clock: 0,
            frames: 0,
            processor: proc,
        }
    }
    fn reset() {

    }

    fn step() {
        
    }
}