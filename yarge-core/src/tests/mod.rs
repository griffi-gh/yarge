#[cfg(not(test))] 
compile_error!("Not a test environment");

#[cfg(not(feature = "dbg-breakpoint-on-ld-b-b"))]
compile_error!("Enable 'dbg-breakpoint-on-ld-b-b' feature to run tests");

#[cfg(not(feature = "dbg-breakpoints"))]
compile_error!("Enable 'dbg-breakpoints' feature to run tests");

macro_rules! define_test {
  ($name: tt, $path: literal) => {
    define_test!($name, $path, (|_:&mut Gameboy|{}));
  };
  ($name: tt, $path: literal, $callback: tt) => {
    define_test!($name, $path, $callback, (|gb: &mut Gameboy, rom: &[u8]| {
      gb.init();
      gb.load_rom(rom).unwrap();
      gb.skip_bootrom();
    }));
  };
  ($name: tt, $path: literal, $callback: tt, $setup: tt) => {
    define_test!($name, $path, $callback, $setup, (|_: &mut Gameboy, res: $crate::Res<usize>| -> bool {
      match res {
        Ok(_) => false,
        Err(
          YargeError::LdBreakpoint { .. } | 
          YargeError::PcBreakpoint { .. } | 
          YargeError::MmuBreakpoint { .. }
        ) => true,
        Err(error) => Err(error).unwrap(),
      }
    }));
  };
  ($name: tt, $path: literal, $callback: tt, $setup: tt, $predicate: tt) => {
    #[test]
    #[allow(non_snake_case)]
    fn $name () {
      use $crate::{Gameboy, YargeError};
      const ROM: &[u8] = include_bytes!(concat!("../../../roms/tests/", $path));
      let mut gb = Gameboy::new();
      gb.init();
      $setup(&mut gb, ROM);
      loop {
        let result = gb.step();
        if $predicate(&mut gb, result) {
          $callback(&mut gb);
          break;
        }
      }
    }
  };
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
  };
  ($name: tt, $path: literal, $setup: tt) => {
    define_test!($name, $path, (|gb: &mut $crate::Gameboy| {
      assert_eq!(gb.get_reg_b(), 3);
      assert_eq!(gb.get_reg_c(), 5);
      assert_eq!(gb.get_reg_d(), 8);
      assert_eq!(gb.get_reg_e(), 13);
      assert_eq!(gb.get_reg_h(), 21);
      assert_eq!(gb.get_reg_l(), 34);
    }), $setup);
  };
}

define_test_mooneye!(Mooneye___acceptance_oam_dma_basic, "mooneye/acceptance/oam_dma/basic.gb");
define_test_mooneye!(Mooneye___acceptance_oam_reg_read, "mooneye/acceptance/oam_dma/reg_read.gb");
define_test_mooneye!(Mooneye___acceptance_oam_sources_gs, "mooneye/acceptance/oam_dma/sources-GS.gb", (|gb: &mut Gameboy, rom: &[u8]| {
  gb.load_rom_force_mbc(rom, 0x02).unwrap(); //MBC1+RAM
  gb.skip_bootrom();
}));

// Mooneye tests
define_test_mooneye!(Mooneye___acceptance_timer_div_write, "mooneye/acceptance/timer/div_write.gb");
define_test_mooneye!(Mooneye___acceptance_timer_rapid_toggle, "mooneye/acceptance/timer/rapid_toggle.gb");
define_test_mooneye!(Mooneye___acceptance_timer_tim00_div_trigger, "mooneye/acceptance/timer/tim00_div_trigger.gb");
define_test_mooneye!(Mooneye___acceptance_timer_tim00, "mooneye/acceptance/timer/tim00.gb");
define_test_mooneye!(Mooneye___acceptance_timer_tim01_div_trigger, "mooneye/acceptance/timer/tim01_div_trigger.gb");
define_test_mooneye!(Mooneye___acceptance_timer_tim01, "mooneye/acceptance/timer/tim01.gb");
define_test_mooneye!(Mooneye___acceptance_timer_tim10_div_trigger, "mooneye/acceptance/timer/tim10_div_trigger.gb");
define_test_mooneye!(Mooneye___acceptance_timer_tim10, "mooneye/acceptance/timer/tim10.gb");
define_test_mooneye!(Mooneye___acceptance_timer_tim11_div_trigger, "mooneye/acceptance/timer/tim11_div_trigger.gb");
define_test_mooneye!(Mooneye___acceptance_timer_tim11, "mooneye/acceptance/timer/tim11.gb");
define_test_mooneye!(Mooneye___acceptance_timer_tima_reload, "mooneye/acceptance/timer/tima_reload.gb");
define_test_mooneye!(Mooneye___acceptance_timer_tima_write_reloading, "mooneye/acceptance/timer/tima_write_reloading.gb");
define_test_mooneye!(Mooneye___acceptance_timer_tma_write_reloading, "mooneye/acceptance/timer/tma_write_reloading.gb");

// Acid2
define_test!(Acid_2___dmg_acid2, "acid/dmg-acid2.gb", (|gb: &mut Gameboy| {
  let hash = fxhash::hash64(gb.get_display_data());
  assert_eq!(hash, 6523616297985761018);
}));
