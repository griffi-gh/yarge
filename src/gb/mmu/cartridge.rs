#[allow(unused_variables)]
pub trait Cartridge {
    fn read(&self, addr: u16) -> u8;
    fn write(&self, addr: u16, value: u8) {}
    fn load(&mut self, data: &[u8]) {}
    fn load_file(&mut self, path: &String) -> Result<(), Box<dyn std::error::Error + 'static>> {
        let data: &[u8] = &std::fs::read(path)?[..];
        self.load(data);
        Ok(())
    }
}
pub type DynCartridge = Box<(dyn Cartridge + Send)>;

pub struct CartridgeNone { rom: [u8; 0x8000] }
impl CartridgeNone {
    pub fn new() -> Self {
        Self { rom: [0; 0x8000] }
    }
}
impl Cartridge for CartridgeNone {
    fn load(&mut self, rom: &[u8]) {
        for (place, data) in self.rom.iter_mut().zip(rom.iter()) {
            *place = *data;
        }
    }
    #[inline]
    fn read(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }
}

pub fn _parse_header(_rom: &[u8]) {
    todo!(); // TODO parse_header()
}
pub fn get_cartridge(cart_type: u8) -> DynCartridge {
    match cart_type {
        0x00 => Box::new(CartridgeNone::new()),
        _ => panic!("Cartridge type not supported {:#04X}", cart_type)
    }
}