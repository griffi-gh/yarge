macro_rules! cpu_instructions {
  ($self: expr, $op: expr) => {
    match($op) {
      0x00 => {},
      _ => panic!("Invalid instruction")
    }
  };
}
pub(crate) use cpu_instructions;

macro_rules! cpu_instructions_cb {
  ($self: expr, $op: expr) => {
    match($op) {
      _ => panic!("Invalid instruction")
    }
  };
}
pub(crate) use cpu_instructions_cb;
