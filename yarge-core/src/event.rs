//WIP: Event logging system
//TODO finish it and remove the "dbg-emit-ppu-events" feature

#[derive(Clone, Copy, Debug)]
pub enum Event {
  PpuLxInc {
    lx: u8,
    ly: u8,
    cycles: u16,
  }
}

pub struct EventRecorder {
  list: Vec<Event>
}
