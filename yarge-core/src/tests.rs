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
    define_test!($name, $path, ($callback), (|gb: &mut Gameboy, rom: &[u8]| {
      gb.init();
      gb.load_rom(rom).unwrap();
      gb.skip_bootrom();
    }));
  };
  ($name: tt, $path: literal, $callback: tt, $setup: tt) => {
    define_test!($name, $path, ($callback), ($setup), (|_: &mut Gameboy, res: $crate::Res<usize>| -> bool {
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
      const ROM: &[u8] = include_bytes!(concat!("./../../roms/tests/", $path));
      let mut gb = Gameboy::new();
      gb.init();
      ($setup)(&mut gb, ROM);
      loop {
        let result = gb.step();
        if ($predicate)(&mut gb, result) {
          ($callback)(&mut gb);
          break
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

mod mooneye {
  mod acceptance {
    mod oam_dma {
      define_test_mooneye!(basic, "mooneye/acceptance/oam_dma/basic.gb");
      define_test_mooneye!(reg_read, "mooneye/acceptance/oam_dma/reg_read.gb");
      define_test_mooneye!(sources_GS, "mooneye/acceptance/oam_dma/sources-GS.gb", (|gb: &mut Gameboy, rom: &[u8]| {
        gb.load_rom_force_mbc(rom, 0x02).unwrap(); //MBC1+RAM
        gb.skip_bootrom();
      }));
    }
    mod timer {
      define_test_mooneye!(div_write, "mooneye/acceptance/timer/div_write.gb");
      define_test_mooneye!(rapid_toggle, "mooneye/acceptance/timer/rapid_toggle.gb");
      define_test_mooneye!(tim00_div_trigger, "mooneye/acceptance/timer/tim00_div_trigger.gb");
      define_test_mooneye!(tim00, "mooneye/acceptance/timer/tim00.gb");
      define_test_mooneye!(tim01_div_trigger, "mooneye/acceptance/timer/tim01_div_trigger.gb");
      define_test_mooneye!(tim01, "mooneye/acceptance/timer/tim01.gb");
      define_test_mooneye!(tim10_div_trigger, "mooneye/acceptance/timer/tim10_div_trigger.gb");
      define_test_mooneye!(tim10, "mooneye/acceptance/timer/tim10.gb");
      define_test_mooneye!(tim11_div_trigger, "mooneye/acceptance/timer/tim11_div_trigger.gb");
      define_test_mooneye!(tim11, "mooneye/acceptance/timer/tim11.gb");
      define_test_mooneye!(tima_reload, "mooneye/acceptance/timer/tima_reload.gb");
      define_test_mooneye!(tima_write_reloading, "mooneye/acceptance/timer/tima_write_reloading.gb");
      define_test_mooneye!(tma_write_reloading, "mooneye/acceptance/timer/tma_write_reloading.gb");
    }
    mod bits {
      define_test_mooneye!(mem_oam, "mooneye/acceptance/bits/mem_oam.gb");
      define_test_mooneye!(reg_f, "mooneye/acceptance/bits/reg_f.gb");
      define_test_mooneye!(unused_hwio_GS, "mooneye/acceptance/bits/unused_hwio-GS.gb");
    }
    mod instr {
      define_test_mooneye!(daa, "mooneye/acceptance/instr/daa.gb");
    }
    mod interrupts {
      define_test_mooneye!(ie_push, "mooneye/acceptance/interrupts/ie_push.gb"); 
    }
  }
}

mod acid {
  define_test!(dmg_acid2, "acid/dmg-acid2.gb", (|gb: &mut Gameboy| {
    let hash = fxhash::hash64(gb.get_display_data());
    assert_eq!(hash, 6523616297985761018);
  }));
}
