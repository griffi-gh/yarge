/// This module exposes some getters/setters

use crate::{
  Gameboy, Res,
  apu::AudioDevice,
  cpu::CpuState,
  bus::cartridge::{RomHeader, CartridgeImpl},
  consts::FB_SIZE
};

impl Gameboy {
  #[inline] pub fn is_rendering(&self) -> bool {
    (self.cpu.bus.ppu.get_lcdc() & 0x80) != 0 &&
    self.cpu.state == CpuState::Running
  }

  #[inline] pub fn set_key_state_all(&mut self, key_state: u8) {
    self.cpu.bus.input.set_key_state_all(key_state);
  }
  #[inline] pub fn set_key_state(&mut self, key: crate::Key, state: bool) {
    self.cpu.bus.input.set_key_state(key, state);
  }

  #[inline] pub fn get_cpu_state(&self) -> CpuState {
    self.cpu.state
  }

  #[inline] pub fn get_display_data(&self) -> &[u8; FB_SIZE] {
    &self.cpu.bus.ppu.display
  }

  #[inline] pub fn read_mem(&self, addr: u16) -> u8 {
    self.cpu.bus.rb(addr, false)
  }
  #[inline] pub fn write_mem(&mut self, addr: u16, value: u8) {
    self.cpu.bus.wb(addr, value, false);
  }

  #[inline] pub fn read_mem_word(&self, addr: u16) -> u16 {
    self.cpu.bus.rw(addr, false)
  }
  #[inline] pub fn write_mem_word(&mut self, addr: u16, value: u16) {
    self.cpu.bus.ww(addr, value, false);
  }

  #[inline] pub fn render_tileset(&self) {
    self.cpu.bus.ppu.render_tileset();
  }

  #[inline] pub fn load_rom(&mut self, data: &[u8]) -> Res<()> {
    self.cpu.bus.load_rom(data)
  }
  #[inline] pub fn load_rom_force_mbc(&mut self, data: &[u8], mbc_type: u8) -> Res<()> {
    self.cpu.bus.load_rom_force_mbc(data, mbc_type)
  }
  #[allow(deprecated)]
  #[deprecated(note="Please handle file reading yourself")]
  #[inline] pub fn load_rom_file(&mut self, path: &str) -> Res<()> {
    self.cpu.bus.load_file(path)
  }
  #[allow(deprecated)]
  #[deprecated(note="Please handle file reading yourself")]
  #[inline] pub fn load_rom_file_force_mbc(&mut self, path: &str, mbc_type: u8) -> Res<()> {
    self.cpu.bus.load_file_force_mbc(path, mbc_type)
  }

  #[inline] pub fn get_mbc_name(&self) -> &str {
    self.cpu.bus.mbc_type_name()
  }
  #[inline] pub fn get_rom_header(&self) -> RomHeader {
    self.cpu.bus.header()
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

  #[inline] pub fn set_reg_a(&mut self, value: u8) {
    self.cpu.reg.set_a(value);
  }
  #[inline] pub fn set_reg_f(&mut self, value: u8) {
    self.cpu.reg.set_f(value);
  }
  #[inline] pub fn set_reg_b(&mut self, value: u8) {
    self.cpu.reg.set_b(value);
  }
  #[inline] pub fn set_reg_c(&mut self, value: u8) {
    self.cpu.reg.set_c(value);
  }
  #[inline] pub fn set_reg_d(&mut self, value: u8) {
    self.cpu.reg.set_d(value);
  }
  #[inline] pub fn set_reg_e(&mut self, value: u8) {
    self.cpu.reg.set_e(value);
  }
  #[inline] pub fn set_reg_h(&mut self, value: u8) {
    self.cpu.reg.set_h(value);
  }
  #[inline] pub fn set_reg_l(&mut self, value: u8) {
    self.cpu.reg.set_l(value);
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

  #[inline] pub fn set_reg_af(&mut self, value: u16) {
    self.cpu.reg.set_af(value);
  }
  #[inline] pub fn set_reg_bc(&mut self, value: u16) {
    self.cpu.reg.set_bc(value);
  }
  #[inline] pub fn set_reg_de(&mut self, value: u16) {
    self.cpu.reg.set_de(value);
  }
  #[inline] pub fn set_reg_hl(&mut self, value: u16) {
    self.cpu.reg.set_hl(value);
  }

  #[inline] pub fn get_reg_pc(&self) -> u16 {
    self.cpu.reg.pc
  }
  #[inline] pub fn get_reg_sp(&self) -> u16 {
    self.cpu.reg.sp
  }

  #[inline] pub fn set_reg_pc(&mut self, value: u16) {
    self.cpu.reg.pc = value;
  }
  #[inline] pub fn set_reg_sp(&mut self, value: u16) {
    self.cpu.reg.sp = value;
  }

  #[inline] pub fn get_bios_disabled(&self) -> bool {
    self.cpu.bus.bios_disabled
  }

  #[inline] pub fn reset_frame_ready(&mut self) {
    self.cpu.bus.ppu.frame_ready = false;
  }
  #[inline] pub fn get_frame_ready(&mut self) -> bool {
    self.cpu.bus.ppu.frame_ready
  }

  #[cfg(feature = "dbg-breakpoints")]
  #[inline] pub fn set_pc_breakpoint(&mut self, addr: u16, enable: bool) {
    self.cpu.pc_breakpoints[addr as usize] = enable;
  }
  #[cfg(feature = "dbg-breakpoints")]
  #[inline] pub fn get_pc_breakpoint(&mut self, addr: u16) -> bool {
    self.cpu.pc_breakpoints[addr as usize]
  }

  #[deprecated] #[inline] pub fn _set_audio_device_dyn(&mut self, device: Box<dyn AudioDevice>) {
    self.cpu.bus.apu.device = Some(device);
  }

  #[inline] pub fn set_audio_device(&mut self, device: impl AudioDevice + 'static) {
    self.cpu.bus.apu.device = Some(Box::new(device));
  }

  #[inline] pub fn get_audio_device(&mut self) -> Option<&dyn AudioDevice> {
    self.cpu.bus.apu.device.as_deref()
  }

  #[inline] pub fn get_audio_device_mut(&mut self) -> Option<&mut Box<dyn AudioDevice>> {
    self.cpu.bus.apu.device.as_mut()
  }

  #[inline] pub fn remove_audio_device(&mut self) -> Option<Box<dyn AudioDevice>> {
    self.cpu.bus.apu.device.take()
  }

  #[inline] pub fn has_save_data(&self) -> bool {
    self.cpu.bus.cart.has_save_data()
  }

  #[inline] pub fn set_save_data(&mut self, data: Vec<u8>) {
    self.cpu.bus.cart.load_data(data);
  }

  #[inline] pub fn get_save_data(&self) -> Option<Vec<u8>> {
    self.cpu.bus.cart.save_data()
  }
}
