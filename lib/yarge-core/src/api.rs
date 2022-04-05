/// This module exposes some getters/setters

use super::{
  Gameboy, Res,
  mmu::cartridge::RomHeader,
  consts::FB_SIZE
};

impl Gameboy {
  #[inline] pub fn get_display_data(&self) -> &[u8; FB_SIZE] {
    &self.cpu.mmu.ppu.display
  }

  #[inline] pub fn read_mem(&self, addr: u16) -> u8 {
    self.cpu.mmu.rb(addr)
  }
  #[inline] pub fn write_mem(&mut self, addr: u16, val: u8) {
    self.cpu.mmu.wb(addr, val);
  }

  #[inline] pub fn read_mem_word(&self, addr: u16) -> u16 {
    self.cpu.mmu.rw(addr)
  }
  #[inline] pub fn write_mem_word(&mut self, addr: u16, val: u16) {
    self.cpu.mmu.ww(addr, val);
  }

  #[inline] pub fn load_rom(&mut self, data: &[u8]) -> Res<()> {
    self.cpu.mmu.load_rom(data)
  }
  #[inline] pub fn load_rom_force_mbc(&mut self, data: &[u8], mbc_type: u8) -> Res<()> {
    self.cpu.mmu.load_rom_force_mbc(data, mbc_type)
  }
  #[inline] pub fn load_rom_file(&mut self, path: &str) -> Res<()> {
    self.cpu.mmu.load_file(path)
  }
  #[inline] pub fn load_rom_file_force_mbc(&mut self, path: &str, mbc_type: u8) -> Res<()> {
    self.cpu.mmu.load_file_force_mbc(path, mbc_type)
  }

  #[inline] pub fn get_mbc_name(&self) -> &str {
    self.cpu.mmu.mbc_type_name()
  }
  #[inline] pub fn get_mbc_type(&self) -> u8 {
    self.cpu.mmu.mbc_index()
  }

  #[inline] pub fn get_rom_header(&self) -> RomHeader {
    self.cpu.mmu.header()
  }

  #[inline] pub fn get_reg_a(&self) -> u8 {
    self.cpu.reg.a()
  }
  #[inline] pub fn get_reg_f(&self) -> u8 {
    self.cpu.reg.f()
  }
  #[inline] pub fn get_reg_b(&self) -> u8 {
    self.cpu.reg.b()
  }
  #[inline] pub fn get_reg_c(&self) -> u8 {
    self.cpu.reg.c()
  }
  #[inline] pub fn get_reg_d(&self) -> u8 {
    self.cpu.reg.d()
  }
  #[inline] pub fn get_reg_e(&self) -> u8 {
    self.cpu.reg.e()
  }
  #[inline] pub fn get_reg_h(&self) -> u8 {
    self.cpu.reg.h()
  }
  #[inline] pub fn get_reg_l(&self) -> u8 {
    self.cpu.reg.l()
  }

  #[inline] pub fn set_reg_a(&mut self, val: u8) {
    self.cpu.reg.set_a(val);
  }
  #[inline] pub fn set_reg_f(&mut self, val: u8) {
    self.cpu.reg.set_f(val);
  }
  #[inline] pub fn set_reg_b(&mut self, val: u8) {
    self.cpu.reg.set_b(val);
  }
  #[inline] pub fn set_reg_c(&mut self, val: u8) {
    self.cpu.reg.set_c(val);
  }
  #[inline] pub fn set_reg_d(&mut self, val: u8) {
    self.cpu.reg.set_d(val);
  }
  #[inline] pub fn set_reg_e(&mut self, val: u8) {
    self.cpu.reg.set_e(val);
  }
  #[inline] pub fn set_reg_h(&mut self, val: u8) {
    self.cpu.reg.set_h(val);
  }
  #[inline] pub fn set_reg_l(&mut self, val: u8) {
    self.cpu.reg.set_l(val);
  }

  #[inline] pub fn get_reg_af(&self) -> u16 {
    self.cpu.reg.af()
  }
  #[inline] pub fn get_reg_bc(&self) -> u16 {
    self.cpu.reg.bc()
  }
  #[inline] pub fn get_reg_de(&self) -> u16 {
    self.cpu.reg.de()
  }
  #[inline] pub fn get_reg_hl(&self) -> u16 {
    self.cpu.reg.hl()
  }

  #[inline] pub fn set_reg_af(&mut self, val: u16) {
    self.cpu.reg.set_af(val);
  }
  #[inline] pub fn set_reg_bc(&mut self, val: u16) {
    self.cpu.reg.set_bc(val);
  }
  #[inline] pub fn set_reg_de(&mut self, val: u16) {
    self.cpu.reg.set_de(val);
  }
  #[inline] pub fn set_reg_hl(&mut self, val: u16) {
    self.cpu.reg.set_hl(val);
  }

  #[inline] pub fn get_reg_pc(&self) -> u16 {
    self.cpu.reg.pc
  }
  #[inline] pub fn get_reg_sp(&self) -> u16 {
    self.cpu.reg.sp
  }

  #[inline] pub fn set_reg_pc(&mut self, val: u16) {
    self.cpu.reg.pc = val;
  }
  #[inline] pub fn set_reg_sp(&mut self, val: u16) {
    self.cpu.reg.sp = val;
  }

  #[inline] pub fn get_bios_disabled(&self) -> bool {
    self.cpu.mmu.bios_disabled
  }
}
