// #[cfg(not(test))] 
// compile_error!("Not a test environment");

#[cfg(not(feature = "breakpoints"))]
compile_error!("Enable breakpoints feature to run tests");

macro_rules! define_test {
  ($name: tt, $path: literal, $callback: tt) => {
    #[test]
    fn $name () {
      use $crate::{Gameboy, YargeError};
      const ROM: &[u8] = include_bytes!(concat!("../../../roms/tests/", $path));
      let mut gb = Gameboy::new();
      gb.init();
      gb.load_rom(ROM).unwrap();
      gb.skip_bootrom();
      loop {
        match gb.step() {
          Ok(_) => {},
          Err(
            YargeError::LdBreakpoint { .. } | 
            YargeError::PcBreakpoint { .. } | 
            YargeError::MmuBreakpoint { .. }
          ) => {
            $callback(&mut gb);
            break;
          },
          Err(error) => Err(error).unwrap(),
        }
      }
    }
  }
}

macro_rules! define_test_mooneye {
  ($name: tt, $path: literal) => {
    define_test!($name, $path, (|gb: &mut $crate::Gameboy| {
      assert_eq!(gb.get_reg_b(), 3);
      assert_eq!(gb.get_reg_c(), 5);
      assert_eq!(gb.get_reg_d(), 8);
      assert_eq!(gb.get_reg_e(), 13);
      assert_eq!(gb.get_reg_h(), 21);
      assert_eq!(gb.get_reg_l(), 34);
    }));
  }
}

define_test_mooneye!(mooneye_oam_dma_basic, "mooneye/acceptance/oam_dma/basic.gb");
