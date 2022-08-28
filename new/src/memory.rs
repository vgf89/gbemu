// MEMORY_MAP
pub const memory_size:usize = 0x10000;

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
pub const unusable:u16 = OAM + 0xA0;              // $fea0-$feff do not touch, just leave it blank unless needed

pub const IO:u16 = unusable + 0x0060;             // $ff00-$ff7f IO Registers
pub const JOYPAD:u16 = IO;                        // $ff00
pub const SIODATA:u16 = JOYPAD + 1;               // $ff01 [RW] Serial I/O Data
pub const SIOCONT:u16 = SIODATA + 1;              // $ff02 [RW] Serial I/O Control
pub const gap0:u16 = SIOCONT + 1;                 // $ff03
pub const DIV:u16 = gap0 + 1;                     // $ff04 [RW] Unconditional counter register (increases every 256 system clock)
pub const TIMA:u16 = DIV + 1;                     // $ff05 [RW] Timer Counter (constantly counts up, triggers timer interrupt on overflow)
pub const TMA:u16 = TIMA + 1;                     // $ff06 [RW] Timer Modulo (loaded into counter whenever counter overflows)
pub const TAC:u16 = TMA + 1;                      // $ff07 [RW] Timer Control
pub const gap1:u16 = TAC + 1;                     // $ff08-ff0e
pub const IFLAGS:u16 = gap1 + 0x0007;             // $ff0f [RW] Interrupt Flags
pub const sound:u16 = IFLAGS + 1;                 // $ff10-ff26
pub const gap2:u16 = sound + 0x0017;              // $ff27-29
pub const waveformRAM:u16 = gap2 + 0x0009;        // $FF30-$FF3F
pub const LCD:u16 = waveformRAM + 0x0010;         // $ff40-ff4b
pub const gap3:u16 = LCD + 0x000C;                
pub const VRAMBankSelect:u16 = gap3 + 0x0003;     // CGB $ff4f
pub const DisableBootRom:u16 = VRAMBankSelect + 1;// $ff50
pub const HDMA:u16 = DisableBootRom + 1;          // CGB $ff51
pub const gap4:u16 = HDMA + 0x0005;
pub const BCPOCP:u16 = gap4 + 0x0012;             // CGB $ff68
pub const WRAMBankSelect:u16 = BCPOCP + 0x0008;   // CGB $ff70
pub const gap5:u16 = WRAMBankSelect + 1;

pub const HRAM:u16 = gap5 + 0x000F;   // $ff80-$fffe High RAM
pub const IE:u16 = HRAM + 0x007F;     // $ffff Interrupts Enable Register


// IFLAGS interrupt Register
pub const I_VBLANK  :u8 = 0x1;
pub const I_LCD_STAT:u8 = 0x01;
pub const I_TIMER   :u8 = 0x001;
pub const I_SERIAL  :u8 = 0x0001;
pub const I_JOYPAD  :u8 = 0x00001;


// Mappers
pub const NoMBC:u8 = 0x00;
pub const MBC1:u8 = 0x01;


// Memory Implementation
#[derive(Default)]
pub struct Memory {
    pub ram:Vec<u8>, // 0x10000 addresses
    cartridgeMode:u8, // FIXME: Rename to cartridge_type
    MBC1BankNN:u8,
    MBC1Banks:Vec<[u8; 0x4000]>, // # 125 possible memory banks of 0x4000 size
}
impl Memory {
    fn default() -> Self {
        return Self{
            ram: vec![0; 0x10000],
            cartridgeMode:0u8,
            MBC1BankNN:0u8,
            MBC1Banks: vec![[0u8; 0x4000]; 125],
        }
    }

    pub fn readByte(&mut self, address:u16) -> u8 {
        if self.cartridgeMode == MBC1 && address >= 0x4000 && address < 0x8000 {
            return self.MBC1Banks[self.MBC1BankNN as usize][address as usize - 0x4000];
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

    pub fn readWord(&mut self, address:u16) -> u16 {
        let c1 = self.readByte(address);
        let c2 = self.readByte(address + 1);
        return ((c2 as u16) << 8) | c1 as u16;
    }

    pub fn writeByte(&mut self, address:u16, val:u8) {
        if address == 0xff02 && val == 0x81 {

            print!("{}", self.readByte(0xff01) as char);
        } else if address >= 0xfea0 && address < 0xff00 {
            // Unusable memory, do nothing
        } else if address == 0xff00 {
            // FIXME
            // selectInput(avl)
        } else if self.cartridgeMode == MBC1 && address >= 0x2000 && address < 0x4000 {
            let mut newval = val & 0b11111;
            if newval == 0 { // FIXME: is this correct???
                newval = 1;
            }
            self.MBC1BankNN = newval;
        } else if address >= 0x8000 {
            self.ram[address as usize] = val;
        }
    }

    pub fn writeWord(&mut self, address:u16, val:u16) {
        let c1 = (val & 0xff) as u8;
        let c2 = ((val >> 8) & 0xff) as u8;
        self.writeByte(address, c1);
        self.writeByte(address + 1, c2);
    }


    pub fn IF_ISSET (&mut self, bitmask:u8) -> u8 {
        return self.ram[IFLAGS as usize] & bitmask;
    }
    pub fn IF_SET (&mut self, bitmask:u8) {
        self.ram[IFLAGS as usize] = self.ram[IFLAGS as usize] | bitmask;
    }
    pub fn IF_CLEAR (&mut self, bitmask:u8) {
        self.ram[IFLAGS as usize] = self.ram[IFLAGS as usize] & !bitmask;
    }

    // IE Interrupts Enable Register macros
    pub fn IE_ISSET (&mut self, bitmask:u8) -> u8{
        return self.ram[IE as usize] & bitmask;
    }
    pub fn IE_SET (&mut self, bitmask:u8) {
        self.ram[IE as usize] = self.ram[IE as usize] | bitmask;
    }
    pub fn IE_CLEAR (&mut self, bitmask:u8) {
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
        memory.writeByte(0x8500, 0b00110101);
        assert_eq!(memory.ram[0x8500], 0b00110101);
        assert_eq!(memory.readByte(0x8500), memory.ram[0x8500]);

        // FIXME: Verify that bytes are in the correct order
        memory.writeWord(0x8600, 0xf00d);
        assert_eq!(memory.readByte(0x8600), 0x0d);
        assert_eq!(memory.readByte(0x8601), 0xf0);
        assert_eq!(memory.readWord(0x8600), 0xf00d);
    }

    #[test]
    fn test_if_ie() {
        let mut memory: Memory = Memory::default();
        memory.ram[IFLAGS as usize] = 0u8;
        memory.ram[IE as usize] = 0u8;

        memory.IF_SET(0b0101);
        assert_eq!(memory.ram[IFLAGS as usize], 0b0101);
        memory.IF_SET(0b1010);
        assert_eq!(memory.ram[IFLAGS as usize], 0b1111);
        memory.IF_CLEAR(0b0011);
        assert_eq!(memory.ram[IFLAGS as usize], 0b1100);

        memory.IE_SET(0b0101);
        assert_eq!(memory.ram[IE as usize], 0b0101);
        memory.IE_SET(0b1010);
        assert_eq!(memory.ram[IE as usize], 0b1111);
        memory.IE_CLEAR(0b0011);
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
        assert_eq!(unusable, 0xfea0);
        assert_eq!(IO, 0xff00);
        assert_eq!(JOYPAD, 0xff00);
        assert_eq!(SIODATA, 0xff01);
        assert_eq!(SIOCONT, 0xff02);
        assert_eq!(gap0, 0xff03);
        assert_eq!(DIV, 0xff04);
        assert_eq!(TIMA, 0xff05);
        assert_eq!(TMA, 0xff06);
        assert_eq!(TAC, 0xff07);
        assert_eq!(gap1, 0xff08);
        assert_eq!(IFLAGS, 0xff0f);
        assert_eq!(sound, 0xff10);
        assert_eq!(gap2, 0xff27);
        assert_eq!(waveformRAM, 0xFF30);
        assert_eq!(LCD, 0xff40);
        assert_eq!(gap3, 0xff4c);
        assert_eq!(VRAMBankSelect, 0xff4f);
        assert_eq!(DisableBootRom, 0xff50);
        assert_eq!(HDMA, 0xff51);
        assert_eq!(gap4, 0xff56);
        assert_eq!(BCPOCP, 0xff68);
        assert_eq!(WRAMBankSelect, 0xff70);
        assert_eq!(gap5, 0xff71);
        assert_eq!(HRAM, 0xff80);
        assert_eq!(IE, 0xffff);
    }
}
