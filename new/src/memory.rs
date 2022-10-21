use std::{fs, io::SeekFrom};

// MEMORY_MAP
pub const MEMORY_SIZE:usize = 0x10000;

pub const ROM00:u16 = 0x0;                        // $0000-$3fff 16 KiB ROM Bank 00. Usually fixed
pub const ROMNN:u16 = ROM00 + 0x4000;             // $4000-$7fff 16 KiB ROM Bank NN. Switchable via mapper (if any)

pub const VRAM:u16 = ROMNN + 0x4000;              // $8000-$9fff 8KiB Video RAM
pub const TILEDATA:u16 = VRAM;
pub const TILEDATA_B0:u16 = TILEDATA;             // $8000-$87FF, OBJs   0-127, BG/Win = (LCDC.4) ? 0-127 : null
pub const TILEDATA_B1:u16 = TILEDATA + 0x800;     // $8800-$8FFF, OBJs 128-255, BG/Win = (LCDC.4) ? 0-127 : 128-255 (or -127-0)
pub const TILEDATA_B2:u16 = TILEDATA_B1 + 0x800;  // $9000-$97FF, (Can't use *** ), if BG/Win if LCDC.4=0  is 0-127
    // NOTE: Sprites are always addressed with a base pointer of $8000. BG and Window tiles can also use $8800 as a base pointer, depending on LCDC.4
pub const EXRAM:u16 = VRAM + 0x2000;              // $a000-$bfff 8KiB External RAM (tends to be battery-backes SRAM)
pub const WRAM1:u16 = EXRAM + 0x2000;             // $c000-$cfff 4KiB Work RAM
pub const WRAM2:u16 = WRAM1 + 0x1000;             // $d000-$dfff 4KiB Work RAM (switchable banks 1~7 on CGB)
pub const ECHORAM:u16 = WRAM2 + 0x1000;           // $e000-$fdff An artifact of how the bus is connected. Mirrors C000~DDFF. Nintendo says use of this area is prohibited.
    // For accuracy, we can remap reads/writes from this location to C000~DFFF
pub const OAM:u16 = ECHORAM + 0x1E00;             // $fe00-$fe9f Sprite Attribute Table
pub const UNUSABLE:u16 = OAM + 0xA0;              // $fea0-$feff do not touch, just leave it blank unless needed

pub const IO:u16 = UNUSABLE + 0x0060;             // $ff00-$ff7f IO Registers
pub const JOYPAD:u16 = IO;                        // $ff00
pub const SIODATA:u16 = JOYPAD + 1;               // $ff01 [RW] Serial I/O Data
pub const SIOCONT:u16 = SIODATA + 1;              // $ff02 [RW] Serial I/O Control
pub const GAP0:u16 = SIOCONT + 1;                 // $ff03
pub const DIV:u16 = GAP0 + 1;                     // $ff04 [RW] Unconditional counter register (increases every 256 system clock)
pub const TIMA:u16 = DIV + 1;                     // $ff05 [RW] Timer Counter (constantly counts up, triggers timer interrupt on overflow)
pub const TMA:u16 = TIMA + 1;                     // $ff06 [RW] Timer Modulo (loaded into counter whenever counter overflows)
pub const TAC:u16 = TMA + 1;                      // $ff07 [RW] Timer Control
pub const GAP1:u16 = TAC + 1;                     // $ff08-ff0e
pub const IFLAGS:u16 = GAP1 + 0x0007;             // $ff0f [RW] Interrupt Flags
pub const SOUND:u16 = IFLAGS + 1;                 // $ff10-ff26
pub const GAP2:u16 = SOUND + 0x0017;              // $ff27-29
pub const WAVEFORM_RAM:u16 = GAP2 + 0x0009;        // $FF30-$FF3F
pub const LCD:u16 = WAVEFORM_RAM + 0x0010;         // $ff40-ff4b
pub const GAP3:u16 = LCD + 0x000C;                
pub const VRAM_BANK_SELECT:u16 = GAP3 + 0x0003;     // CGB $ff4f
pub const DISABLE_BOOT_ROM:u16 = VRAM_BANK_SELECT + 1;// $ff50
pub const HDMA:u16 = DISABLE_BOOT_ROM + 1;          // CGB $ff51
pub const GAP4:u16 = HDMA + 0x0005;
pub const BCPOCP:u16 = GAP4 + 0x0012;             // CGB $ff68
pub const WRAM_BANK_SELECT:u16 = BCPOCP + 0x0008;   // CGB $ff70
pub const GAP5:u16 = WRAM_BANK_SELECT + 1;

pub const HRAM:u16 = GAP5 + 0x000F;   // $ff80-$fffe High RAM
pub const IE:u16 = HRAM + 0x007F;     // $ffff Interrupts Enable Register


// IFLAGS interrupt Register
pub const I_VBLANK  :u8 = 0x1;
pub const I_LCD_STAT:u8 = 0x01;
pub const I_TIMER   :u8 = 0x001;
pub const I_SERIAL  :u8 = 0x0001;
pub const I_JOYPAD  :u8 = 0x00001;


// Mappers
pub const NO_MBC:u8 = 0x00;
pub const MBC1:u8 = 0x01;


// Memory Implementation
#[derive(Default)]
pub struct Memory {
    pub ram:Vec<u8>, // 0x10000 addresses
    cartridge_type:u8,
    mbc_1_bank_nn:u8,
    mbc_1_banks:Vec<[u8; 0x4000]>, // # 125 possible memory banks of 0x4000 size
}
impl Memory {
    pub fn default() -> Self {
        return Self {
            ram: vec![0; 0x10000],
            cartridge_type:0u8,
            mbc_1_bank_nn:0u8,
            mbc_1_banks: vec![[0u8; 0x4000]; 125],
        }
    }

    pub fn load_rom(&mut self, filepath: String) {
        use std::fs::File;
        use std::path::Path;
        use std::io::prelude::*;

        let path = Path::new(&filepath);
        let metadata = match fs::metadata(path) {
            Err(why) => panic!("couldn't get file metadata for {}: {}", path.to_str().unwrap(), why),
            Ok(metadata) => metadata,
        };
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(why) => panic!("couldn't open file: {}", why),
        };
        //file.seek(SeekFrom::Start(0));
        let mut buf = vec![];
        match file.read_to_end(&mut buf) {
            Err(why) => panic!("couldn't read file: {}", why),
            Ok(_) => (),
        };

        let cartridge_mode :u8;
        match buf.get(0x0147) {
            Some(val) => cartridge_mode = *val,
            None => panic!("rom file buffer empy"),
        }

        match cartridge_mode {
            NO_MBC => {
                println!("Loading ROM-only Cartridge.");
                for (i, val) in buf.iter().enumerate() {
                    let bank = i/0x4000;
                    let row = i%0x4000;
                    //print!("{} {} {}    ", bank, row, val);
                    self.mbc_1_banks[bank][row] = *val;
                }
                //println!("\n");
            },
            MBC1 => {
                println!("Loading MBC1 Cartidge");
                self.cartridge_type = MBC1;
                file.seek(SeekFrom::Start(0)).unwrap();
                let mut buf = [0u8; 0x4000];
                let mut i = 0;
                while file.read_exact(&mut buf).is_ok() {
                    println!("Reading ROM Bank {}", i);
                    self.mbc_1_banks[i] = buf.clone();
                    i += 1;
                }
            },
            _ => panic!("cartidge mode not supported: {}", cartridge_mode),
        }
        
    }

    pub fn read_byte(&self, address:u16) -> u8 {
        if address < 0x4000 {
            return self.mbc_1_banks[0][address as usize];
        }
        if self.cartridge_type == MBC1 && address >= 0x4000 && address < 0x8000 {
            let bank = self.mbc_1_bank_nn + 1; // 0x4000-0x7fff Banks are mapped 1 higher. Hardware quirk
            return self.mbc_1_banks[bank as usize][address as usize - 0x4000];
        }
        if address >= 0xfea0 && address < 0xff00 {
            return 0xff;
        }
        if address == 0xff00 {
            // FIXME: Fix this
            //return getInput();
            return 0;
        }

        return self.ram[address as usize];
    }


    pub fn write_byte(&mut self, address:u16, val:u8) {
        if address == 0xff02  && val == 0x81 { // Print link cable output to terminal
            print!("{}", self.read_byte(0xff01) as char);
            use std::io::{self, Write};
            io::stdout().flush().unwrap();
        } else if address >= 0xfea0 && address < 0xff00 {
            // Unusable memory, do nothing
        } else if address == 0xff00 {
            // FIXME
            // selectInput(avl)
        } else if self.cartridge_type == MBC1 && address >= 0x2000 && address < 0x4000 {
            let mut newval = val & 0b11111;
            if newval == 0 { // FIXME: is this correct???
                newval = 1;
            }
            self.mbc_1_bank_nn = newval;
        } else if address >= 0x8000 {
            self.ram[address as usize] = val;
        }
    }

    // NOTE: Z80 is little endian (least significant byte is stored at the lower address)
    pub fn read_word(&self, address:u16) -> u16 {
        let lsb = self.read_byte(address);
        let msb = self.read_byte(address + 1);
        return ((msb as u16) << 8) | lsb as u16;
    }

    pub fn write_word(&mut self, address:u16, val:u16) {
        let lsb = (val & 0xff) as u8;
        let msb = ((val >> 8) & 0xff) as u8;
        self.write_byte(address, lsb);
        self.write_byte(address + 1, msb);
    }


    pub fn if_isset (&self, bitmask:u8) -> bool {
        return self.ram[IFLAGS as usize] & bitmask != 0;
    }
    pub fn if_set (&mut self, bitmask:u8) {
        self.ram[IFLAGS as usize] = self.ram[IFLAGS as usize] | bitmask;
    }
    pub fn if_clear (&mut self, bitmask:u8) {
        self.ram[IFLAGS as usize] = self.ram[IFLAGS as usize] & !bitmask;
    }

    // IE Interrupts Enable Register macros
    pub fn ie_isset (&self, bitmask:u8) -> bool{
        return self.ram[IE as usize] & bitmask != 0;
    }
    pub fn ie_set (&mut self, bitmask:u8) {
        self.ram[IE as usize] = self.ram[IE as usize] | bitmask;
    }
    pub fn ie_clear (&mut self, bitmask:u8) {
        self.ram[IE as usize] = self.ram[IE as usize] & !bitmask;
    } 
}




// Tests
#[cfg(test)]
mod memory_tests {
    use super::*;

    #[test]
    fn test_read_write() {
        let mut memory = Memory::default();
        memory.write_byte(0x8500, 0b00110101);
        assert_eq!(memory.ram[0x8500], 0b00110101);
        assert_eq!(memory.read_byte(0x8500), memory.ram[0x8500]);

        // FIXME: Verify that bytes are in the correct order
        memory.write_word(0x8600, 0xf00d);
        assert_eq!(memory.read_byte(0x8600), 0x0d);
        assert_eq!(memory.read_byte(0x8601), 0xf0);
        assert_eq!(memory.read_word(0x8600), 0xf00d);
    }

    #[test]
    fn test_if_ie() {
        let mut memory: Memory = Memory::default();
        memory.ram[IFLAGS as usize] = 0u8;
        memory.ram[IE as usize] = 0u8;

        memory.if_set(0b0101);
        assert_eq!(memory.ram[IFLAGS as usize], 0b0101);
        memory.if_set(0b1010);
        assert_eq!(memory.ram[IFLAGS as usize], 0b1111);
        memory.if_clear(0b0011);
        assert_eq!(memory.ram[IFLAGS as usize], 0b1100);

        memory.ie_set(0b0101);
        assert_eq!(memory.ram[IE as usize], 0b0101);
        memory.ie_set(0b1010);
        assert_eq!(memory.ram[IE as usize], 0b1111);
        memory.ie_clear(0b0011);
        assert_eq!(memory.ram[IE as usize], 0b1100);
    }

    #[test]
    fn test_consts() {
        assert_eq!(ROM00, 0x0000);
        assert_eq!(ROMNN, 0x4000);
        assert_eq!(VRAM, 0x8000);
        assert_eq!(TILEDATA, 0x8000);
        assert_eq!(TILEDATA_B0, 0x8000);
        assert_eq!(TILEDATA_B1, 0x8800);
        assert_eq!(TILEDATA_B2, 0x9000);
        assert_eq!(EXRAM, 0xa000);
        assert_eq!(WRAM1, 0xc000);
        assert_eq!(WRAM2, 0xd000);
        assert_eq!(ECHORAM, 0xe000);
        assert_eq!(OAM, 0xfe00);
        assert_eq!(UNUSABLE, 0xfea0);
        assert_eq!(IO, 0xff00);
        assert_eq!(JOYPAD, 0xff00);
        assert_eq!(SIODATA, 0xff01);
        assert_eq!(SIOCONT, 0xff02);
        assert_eq!(GAP0, 0xff03);
        assert_eq!(DIV, 0xff04);
        assert_eq!(TIMA, 0xff05);
        assert_eq!(TMA, 0xff06);
        assert_eq!(TAC, 0xff07);
        assert_eq!(GAP1, 0xff08);
        assert_eq!(IFLAGS, 0xff0f);
        assert_eq!(SOUND, 0xff10);
        assert_eq!(GAP2, 0xff27);
        assert_eq!(WAVEFORM_RAM, 0xFF30);
        assert_eq!(LCD, 0xff40);
        assert_eq!(GAP3, 0xff4c);
        assert_eq!(VRAM_BANK_SELECT, 0xff4f);
        assert_eq!(DISABLE_BOOT_ROM, 0xff50);
        assert_eq!(HDMA, 0xff51);
        assert_eq!(GAP4, 0xff56);
        assert_eq!(BCPOCP, 0xff68);
        assert_eq!(WRAM_BANK_SELECT, 0xff70);
        assert_eq!(GAP5, 0xff71);
        assert_eq!(HRAM, 0xff80);
        assert_eq!(IE, 0xffff);
    }
}
