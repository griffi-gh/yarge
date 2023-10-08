use crate::{Input, Timers, Ppu, Apu, Res, consts::BIOS, serial::Serial};
use std::fs;
pub mod cartridge;
use cartridge::{CartridgeImpl as _, RomHeader, Cartridge, MockCartridge};

pub struct MemBus {
  pub bios_disabled: bool,
  pub cart: Cartridge,
  cart_header: RomHeader,
  wram: Box<[u8; 0x2000]>,
  hram: [u8; 0x007F],
  //oam dma
  oam_value: u8,
  oam_transfer: u8,
  //interrupts
  pub iie: u8,
  pub iif: u8,
  //components
  pub ppu: Ppu,
  pub apu: Apu,
  pub timers: Timers,
  pub input: Input,
  pub serial: Serial,
}
impl MemBus {
  pub fn new() -> Self {
    Self {
      bios_disabled: false,
      cart: MockCartridge.into(),
      cart_header: RomHeader::default(),
      wram: Box::new([0; 0x2000]),
      hram: [0; 0x7F],
      //oam dma
      oam_value: 0,
      oam_transfer: 0,
      //interrupts
      iie: 0x00,
      iif: 0x00,
      //components
      ppu: Ppu::new(),
      apu: Apu::new(),
      timers: Timers::new(),
      input: Input::new(),
      serial: Serial::new(),
    }
  }

  pub fn rb(&self, addr: u16, blocking: bool) -> u8 {
    if blocking && self.check_oam_dma_block(addr) {
      return 0xFF
    }
    match addr {
      //BOOTROM/ROM
      0x0000..=0x00ff if !self.bios_disabled => BIOS[addr as usize],
      //ROM
      0x0000..=0x7fff => self.cart.read_rom(addr),
      //VRAM
      0x8000..=0x9FFF => self.ppu.read_vram(addr, blocking),
      //ERAM
      0xA000..=0xBFFF => self.cart.read_eram(addr, blocking),
      //WRAM/ECHO
      0xC000..=0xFDFF => self.wram[(addr & 0x1FFF) as usize],
      //OAM
      0xFE00..=0xFE9F => self.ppu.read_oam(addr, blocking),
      //IO REGISTERS
      0xFF00..=0xFF7F => match addr {
        0xFF00 => self.input.get_joyp(),
        0xFF01 => self.serial.read_sb(),
        0xFF02 => self.serial.read_sc(),
        0xFF04 => self.timers.get_div(),
        0xFF05 => self.timers.get_tima(),
        0xFF06 => self.timers.tma,
        0xFF07 => self.timers.get_tac(),
        0xFF0F => self.iif,
        //0xFF10..=0xFF26 => self.tmp_apu_reg[addr as usize - 0xFF10],
        0xFF10..=0xFF3F => self.apu.read(addr),
        0xFF40 => self.ppu.get_lcdc(), //LCDC
        0xFF41 => self.ppu.get_stat(), //STAT
        0xFF42 => self.ppu.scy,
        0xFF43 => self.ppu.scx,
        0xFF44 => { //LY
          #[cfg(feature = "dbg-ly-stub")] { 0x90 }
          #[cfg(not(feature = "dbg-ly-stub"))] { self.ppu.mmio_ly }
        },
        0xFF45 => self.ppu.lyc,
        0xFF46 => self.oam_value,
        0xFF47 => self.ppu.bgp,
        0xFF48 => self.ppu.obp.0,
        0xFF49 => self.ppu.obp.1,
        0xFF4A => self.ppu.wy,
        0xFF4B => self.ppu.wx,
        0xFF50 => 0xFE | (self.bios_disabled as u8),
        _ => 0xFF
      },
      //HRAM
      0xFF80..=0xFFFE => {
        self.hram[((addr - 0xFF80) & 0x7F) as usize]
      },
      0xFFFF => self.iie,
      _ => 0xFF
    }
  }
  
  pub fn wb(&mut self, addr: u16, value: u8, blocking: bool) {
    if blocking && self.check_oam_dma_block(addr) {
      return
    }
    match addr {
      //BOOTROM/ROM
      0x0000..=0x00ff if !self.bios_disabled => {},
      0x0000..=0x7fff => { self.cart.write_rom(addr, value) },
      //VRAM
      0x8000..=0x9FFF => { self.ppu.write_vram(addr, value, blocking) },
      //ERAM
      0xA000..=0xBFFF => { self.cart.write_eram(addr, value, blocking) },
      //WRAM/ECHO
      0xC000..=0xFDFF => { self.wram[(addr & 0x1FFF) as usize] = value },
      //OAM
      0xFE00..=0xFE9F => { self.ppu.write_oam(addr, value, blocking) },
      //IO REGISTERS
      0xFF00..=0xFF7F => match addr {
        0xFF00 => { self.input.set_joyp(value) },
        0xFF01 => { self.serial.write_sb(value) },
        0xFF02 => { self.serial.write_sc(value) },
        0xFF04 => { self.timers.reset_div() },
        0xFF05 => { self.timers.set_tima(value) },
        0xFF06 => { self.timers.tma = value },
        0xFF07 => { self.timers.set_tac(value) },
        0xFF0F => { self.iif = value },
        // 0xFF10..=0xFF26 => { self.tmp_apu_reg[addr as usize - 0xFF10] = value; }
        0xFF10..=0xFF3F => { self.apu.write(addr, value, blocking) }
        0xFF40 => { self.ppu.set_lcdc(value) },
        0xFF41 => { self.ppu.set_stat(value) },
        0xFF42 => { self.ppu.scy = value },
        0xFF43 => { self.ppu.scx = value },
        0xFF45 => { self.ppu.lyc = value },
        0xFF46 => { self.start_oam_dma(value) }
        0xFF47 => { self.ppu.bgp = value },
        0xFF48 => { self.ppu.obp.0 = value },
        0xFF49 => { self.ppu.obp.1 = value },
        0xFF4A => { self.ppu.wy  = value },
        0xFF4B => { self.ppu.wx  = value },
        0xFF50 => { self.bios_disabled = true },
        _ => ()
      }
      //HRAM
      0xFF80..=0xFFFE => {
        self.hram[((addr - 0xFF80) & 0x7F) as usize] = value;
      },
      //IE
      0xFFFF => { self.iie = value; },
      _ => ()
    }
  }

  pub fn rw(&self, addr: u16, blocking: bool) -> u16 {
    self.rb(addr, blocking) as u16 | 
    ((self.rb(addr.wrapping_add(1), blocking) as u16) << 8)
  }
  pub fn ww(&mut self, addr: u16, value: u16, blocking: bool) {
    self.wb(addr, (value & 0xFF) as u8, blocking);
    self.wb(addr.wrapping_add(1), (value >> 8) as u8, blocking);
  }
  
  fn check_oam_dma_block(&self, addr: u16) -> bool {
    //TODO THIS MAY CAUSE (gb game?) CRASHES
    (self.oam_transfer > 0) && !((0xFF80..=0xFFFE).contains(&addr) || (addr == 0xFF46))
  }
  fn start_oam_dma(&mut self, value: u8) {
    //Reset OAM transfer timer (Unlock memory if another oam is in progress)
    self.oam_transfer = 0;
    // Lock PPU OAM
    self.ppu.mmu_oam_locked = true;
    //Do OAM transfer
    let src_start = (value as u16) << 8;
    for i in 0..0xA0 {
      let mut src_addr = src_start + i;
      if src_addr > 0xC000 {
        src_addr = 0xC000 + (src_addr & 0x1FFF);
      }
      let src_value = self.rb(src_addr, true);
      let dest_addr = 0xFE00 | i;
      self.ppu.write_oam(dest_addr, src_value, false);
    }
    //Update variables
    self.oam_value = value;
    self.oam_transfer = 160;
  }
  fn tick_oam_dma(&mut self) {
    if self.oam_transfer > 0 {
      self.oam_transfer = self.oam_transfer.saturating_sub(4);
      if self.oam_transfer == 0 {
        self.ppu.mmu_oam_locked = false;
      }
    }
  }

  pub fn load_rom(&mut self, data: &[u8]) -> Res<()> {
    let header = RomHeader::parse(data);
    self.cart_header = header;
    self.cart = cartridge::get_cartridge(header)?;
    self.cart.load_rom(data)?;
    Ok(())
  }
  pub fn load_rom_force_mbc(&mut self, data: &[u8], mbc_type: u8) -> Res<()> {
    let mut header = RomHeader::parse(data);
    header.mbc_type = mbc_type;
    self.cart_header = header;
    self.cart = cartridge::get_cartridge(header).unwrap();
    self.cart.load_rom(data)?;
    Ok(())
  }
  #[deprecated(note = "load_file is deprecated")]
  pub fn load_file(&mut self, path: &str) -> Res<()> {
    let data: &[u8] = &(fs::read(path)?)[..];
    self.load_rom(data)?;
    Ok(())
  }
  #[deprecated(note = "load_file is deprecated")]
  pub fn load_file_force_mbc(&mut self, path: &str, mbc_type: u8) -> Res<()> {
    let data: &[u8] = &(fs::read(path)?)[..];
    self.load_rom_force_mbc(data, mbc_type)?;
    Ok(())
  }

  pub fn mbc_type_name(&self) -> &str {
    self.cart.name()
  }
  pub fn header(&self) -> RomHeader {
    self.cart_header
  }

  pub fn tick_components(&mut self) {
    self.tick_oam_dma();
    self.ppu.tick(&mut self.iif);
    self.timers.tick(&mut self.iif);
    self.apu.tick(self.timers.get_div_raw());
    self.input.tick(&mut self.iif);
    self.serial.tick(&mut self.iif);
    self.cart.tick();
  }
}
