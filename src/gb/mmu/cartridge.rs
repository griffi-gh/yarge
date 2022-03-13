#[allow(unused_variables)]
pub trait Cartridge {
    fn load(&mut self, rom: &[u8]);
    fn read(&self, addr: u16) -> u8;
    fn write(&self, addr: u16, value: u8) {}
}

pub type DynCartridge = Box<(dyn Cartridge + Send)>;

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

pub fn _parse_header(_rom: &[u8]) {
    // TODO Parse header
    todo!();
}
pub fn get_cartridge(_cart_type: u8) -> DynCartridge {
    // TODO Get cartridge
    Box::new(CartridgeNone::new())
}