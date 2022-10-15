#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SquareWaveChannelType {
  Channel1,
  Channel2
}

pub struct SquareWaveChannel {
  channel_type: SquareWaveChannelType
}
impl SquareWaveChannel {
  pub fn new(channel_type: SquareWaveChannelType) -> Self {
    Self {
      channel_type
    }
  }
  pub fn tick(&self) {
    //TODO SquareWaveChannel::tick
  }
}
