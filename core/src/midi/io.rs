use crate::midi::EventIo;

pub trait MidiInput {
  //  fn next(&self) -> Option<&EventIo>;
  //  fn iter(&self) -> &Iterator<Item=&EventIo>;
}

pub trait MidiOutput {
  fn push(&mut self, event: EventIo);
}
