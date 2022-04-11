pub mod cartridge;
use cartridge::{RomHeader, DynCartridge, get_cartridge};

use super::PPU;
use crate::{ Res, consts::BIOS };
use std::fs;

pub struct MMU {
  pub ppu: PPU,
  pub bios_disabled: bool,
  cart: DynCartridge,
  cart_header: RomHeader,
  wram: Box<[u8; 0x2000]>,
  hram: Box<[u8; 0x007F]>,
  //MAYBE include IE here?
}
impl MMU {
  pub fn new() -> Self {
    Self {
      ppu: PPU::new(),
      bios_disabled: false,
      cart: get_cartridge(0).unwrap(),
      cart_header: RomHeader::default(),
      wram: Box::new([0; 0x2000]),
      hram: Box::new([0; 0x007F]),
    }
  }

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
        self.cart.read_eram(addr)
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
          0xFF40 => { self.ppu.get_lcdc() } //LCDC
          0xFF41 => { self.ppu.get_stat() } //STAT
          0xFF42 => { self.ppu.scy },
          0xFF43 => { self.ppu.scx },
          0xFF44 => { //LY
            #[cfg(feature = "ly-stub")] { 0x90 }
            #[cfg(not(feature = "ly-stub"))] { self.ppu.get_ly() }
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
      //nah it's not worth checking for "bios_disabled" here
      0..=0x7fff => { self.cart.write(addr, value); },
      //VRAM
      0x8000..=0x9FFF => { self.ppu.write_vram(addr, value); },
      //ERAM
      0xA000..=0xBFFF => { self.cart.write_eram(addr, value); },
      //WRAM/ECHO
      0xC000..=0xFDFF => { self.wram[(addr & 0x1FFF) as usize] = value; },
      //OAM
      0xFE00..=0xFE9F => { self.ppu.write_oam(addr, value); },
      //IO REGISTERS
      0xFF40 => { self.ppu.set_lcdc(value); },
      0xFF41 => { self.ppu.set_stat(value); },
      0xFF42 => { self.ppu.scy = value; },
      0xFF43 => { self.ppu.scx = value; },
      0xFF50 => { self.bios_disabled = true; },
      //HRAM
      0xFF80..=0xFFFE => {
        self.hram[((addr - 0xFF80) & 0x7F) as usize] = value;
      },
      _ => {}
    }
  }

  #[inline] pub fn rw(&self, addr: u16) -> u16 {
    self.rb(addr) as u16 | 
    ((self.rb(addr.wrapping_add(1)) as u16) << 8)
  }
  #[inline] pub fn ww(&mut self, addr: u16, value: u16) {
    self.wb(addr, (value & 0xFF) as u8);
    self.wb(addr.wrapping_add(1), (value >> 8) as u8);
  }
  
  pub fn load_rom(&mut self, data: &[u8]) -> Res<()> {
    let header = RomHeader::parse(data);
    let mbc_type = header.mbc_type;
    self.cart_header = header;
    self.cart = cartridge::get_cartridge(mbc_type)?;
    self.cart.load(data)?;
    Ok(())
  }
  pub fn load_rom_force_mbc(&mut self, data: &[u8], mbc_type: u8) -> Res<()> {
    self.cart_header = RomHeader::parse(data);
    self.cart = cartridge::get_cartridge(mbc_type).unwrap();
    self.cart.load(data)?;
    Ok(())
  }
  pub fn load_file(&mut self, path: &str) -> Res<()> {
    let data: &[u8] = &(fs::read(path)?)[..];
    self.load_rom(data)?;
    Ok(())
  }
  pub fn load_file_force_mbc(&mut self, path: &str, mbc_type: u8) -> Res<()> {
    let data: &[u8] = &(fs::read(path)?)[..];
    self.load_rom_force_mbc(data, mbc_type)?;
    Ok(())
  }

  #[inline] pub fn mbc_type_name(&self) -> &str {
    self.cart.name()
  }
  #[inline] pub fn mbc_index(&self) -> u8 {
    self.cart.index()
  }
  #[inline] pub fn header(&self) -> RomHeader {
    self.cart_header
  }
}
