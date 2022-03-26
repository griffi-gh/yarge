mod oam;
mod ppu_registers;
mod bg_fetcher;
use bg_fetcher::BgFetcher;
use oam::OAMMemory;
use ppu_registers::LCDC;
use arraydeque::ArrayDeque;

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

pub struct PPU {
  pub display: [u8; WIDTH * HEIGHT],
  vram: [u8; 0x2000],
  oam: OAMMemory,
  _bg_fetcher: BgFetcher,
  _bg_fifo: ArrayDeque<[u8; 16]>,
  lcdc: LCDC,
  pub ly: u8,
}
impl PPU {
  pub fn new() -> Self {
    Self {
      display: [1; WIDTH * HEIGHT],
      vram: [0; 0x2000],
      oam: OAMMemory::new(),
      _bg_fetcher: BgFetcher::new(),
      _bg_fifo: ArrayDeque::default(),
      lcdc: LCDC::default(),
      ly: 0,
    }
  }

  #[inline] pub fn set_lcdc(&mut self, value: u8) {
    self.lcdc.set_from_u8(value);
  }
  #[inline] pub fn get_lcdc(&self) -> u8 {
    self.lcdc.into_u8()
  }

  #[inline]
  pub fn read_oam(&self, addr: u16) -> u8 {
    self.oam.read_oam(addr - 0xFE00)
  }
  #[inline]
  pub fn write_oam(&mut self, addr: u16, value: u8) {
    self.oam.write_oam(addr - 0xFE00, value);
  }

  #[inline]
  pub fn read_vram(&self, addr: u16) -> u8 {
    self.vram[(addr & 0x1FFF) as usize]
  }
  #[inline]
  pub fn write_vram(&mut self, addr: u16, value: u8) {
    self.vram[(addr & 0x1FFF) as usize] = value;
  }
  
  pub fn tick(&self) {
    //TODO ppu.tick()
  }
}
