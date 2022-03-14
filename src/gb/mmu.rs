pub mod cartridge;

use cartridge::{DynCartridge, get_cartridge};

mod bios;
use bios::BIOS;

pub struct MMU {
  cart: DynCartridge,
  bios_disabled: bool,
  wram: [u8; 0x2000],
  hram: [u8; 0x007F],
}
impl MMU {
  pub fn new() -> Self {
    Self {
      cart: get_cartridge(0),
      bios_disabled: false,
      wram: [0; 0x2000],
      hram: [0; 0x007F],
    }
  }
  
  // MAYBE? rename to r16/w16/r8/w8 ?

  #[inline(never)]
  pub fn rb(&self, addr: u16) -> u8 {
    match addr {
      //BOOTROM/ROM
      0x0000..=0x00ff => { 
        if self.bios_disabled {
          self.cart.read(addr)
        } else {
          BIOS[addr as usize]
        }
      },
      //ROM
      0x0100..=0x7fff => { 
        self.cart.read(addr)
      },
      //VRAM
      0x8000..=0x9FFF => {
        0 //TODO self.ppu.read(addr)
      }
      //ERAM
      0xA000..=0xBFFF => {
        self.cart.read(addr)
      }
      //WRAM/ECHO
      0xC000..=0xFDFF => {
        self.wram[(addr & 0x1FFF) as usize]
      },
      //OAM
      0xFE00..=0xFE9F => {
        0 //TODO self.ppu.read(addr)
      },
      //IO REGISTERS
      0xFF00..=0xFF7F => {
        0xff //TODO I/O Registers Read
      },
      //HRAM
      0xFF80..=0xFFFE => {
        self.hram[((addr - 0xFF80) & 0x7F) as usize]
      },
      _ => 0
    }
  }
  #[inline(never)]
  pub fn wb(&mut self, addr: u16, value: u8) {
    match addr {
      //BOOTROM/ROM
      0..=0xff => {
        if self.bios_disabled {
          self.cart.write(addr, value);
        }
      },
      //ROM
      0x100..=0x7fff => {
        self.cart.write(addr, value);
      }
      //VRAM
      0x8000..=0x9FFF => {
        //TODO self.ppu.write(addr, value);
      }
      //ERAM
      0xA000..=0xBFFF => {
        self.cart.write(addr, value);
      }
      //WRAM/ECHO
      0xC000..=0xFDFF => {
        self.wram[(addr & 0x1FFF) as usize] = value;
      },
      //OAM
      0xFE00..=0xFE9F => {
        //TODO self.ppu.write(addr, value);
      },
      //IO REGISTERS
      0xFF00..=0xFF7F => {
        //TODO I/O Registers Write
      },
      //HRAM
      0xFF80..=0xFFFE => {
        self.hram[((addr - 0xFF80) & 0x7F) as usize] = value;
      },
      _ => {}
    }
  }

  #[inline]
  pub fn rw(&self, addr: u16) -> u16 {
    self.rb(addr) as u16 | 
    ((self.rb(addr.wrapping_add(1)) as u16) << 8)
  }
  #[inline]
  pub fn ww(&mut self, addr: u16, value: u16) {
    self.wb(addr, (value & 0xFF) as u8);
    self.wb(addr.wrapping_add(1), (value >> 8) as u8);
  }
}
