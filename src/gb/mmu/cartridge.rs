pub trait Cartridge {
    fn load(&mut self, rom: &[u8]);
    fn read(&self, addr: u16) -> u8;
    #[allow(unused_variables)]
    fn write(&self, addr: u16, value: u8) {}
}

pub struct CartridgeNone {
    rom: [u8; 0x8000]
}
impl CartridgeNone {
    pub fn new() -> Self {
        Self {
            rom: [0; 0x8000]
        }
    }
}
impl Cartridge for CartridgeNone {
    fn load(&mut self, rom: &[u8]) {
        for (place, data) in self.rom.iter_mut().zip(rom.iter()) {
            *place = *data
        }
    }
    fn read(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }
}