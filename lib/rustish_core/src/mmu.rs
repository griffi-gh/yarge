pub mod cartridge;

use cartridge::{DynCartridge, get_cartridge};

mod bios;
use bios::BIOS;
use super::PPU;

pub struct MMU {
  pub ppu: PPU,
  pub cart: DynCartridge,
  pub bios_disabled: bool,
  wram: [u8; 0x2000],
  hram: [u8; 0x007F],
  //MAYBE include IE here?
}
impl MMU {
  pub fn new() -> Self {
    Self {
      cart: get_cartridge(0),
      bios_disabled: false,
      wram: [0; 0x2000],
      hram: [0; 0x007F],
      ppu: PPU::new(),
    }
  }
  
  // MAYBE? rename to r16/w16/r8/w8 ?

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
        self.ppu.read_vram(addr)
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
        self.ppu.read_oam(addr)
      },
      //IO REGISTERS
      0xFF00..=0xFF7F => {
        match addr {
          0xFF40 => {
            self.ppu.lcdc.into_u8()
          }
          0xFF44 => {
            #[cfg(not(feature = "ly-stub"))] {
              self.ppu.ly
            }
            #[cfg(feature = "ly-stub")] {
              0x90
            }
          }
          _ => 0xff
        }
      },
      //HRAM
      0xFF80..=0xFFFE => {
        self.hram[((addr - 0xFF80) & 0x7F) as usize]
      },
      _ => 0
    }
  }
  
  pub fn wb(&mut self, addr: u16, value: u8) {
    match addr {
      //BOOTROM/ROM
      0..=0x7fff => {
        //nah it's not worth checking for "bios_disabled" here
        self.cart.write(addr, value);
      }
      //VRAM
      0x8000..=0x9FFF => {
        self.ppu.write_vram(addr, value);
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
        self.ppu.write_oam(addr, value);
      },
      //IO REGISTERS
      0xFF50 => {
        //TODO check the value?
        self.bios_disabled = true;
      }
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
