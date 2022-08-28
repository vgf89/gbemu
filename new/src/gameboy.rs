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
pub struct Gameboy {
    pub clock:u32,
    pub frames:u32,
    pub ram: Rc<Memory>,
    pub processor: CPU,
}

impl Gameboy {
    pub fn new() -> Self {
        let mem: Rc<Memory> = Rc::new(Memory::default());
        let proc = CPU::new(Rc::clone(&mem));

        return Self {
            clock: 0,
            frames: 0,
            ram: mem,
            processor: proc,
        }
    }
    fn reset() {

    }

    fn step() {
        
    }
}