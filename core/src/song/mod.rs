use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use std::sync::{Arc, RwLock};

pub mod clips;
pub mod io;
pub mod metronome;
pub mod source;
pub mod track;
pub mod transport;

use crate::config::Config;
use crate::midi::bus::MidiBusLock;
use crate::studio::AudioTime;
use crate::time::{BarsTime, SampleRate, Signature, TicksTime};

use self::{
  metronome::Metronome,
  track::{Track, TrackMedia},
  transport::{Segment, Transport},
};

pub struct Song {
  name: String,
  transport: Transport,
  metronome: Metronome,
  tracks: Vec<Track>,
  midi_bus: MidiBusLock,
}

impl Song {
  pub fn new<T>(name: T, config: &Config, midi_bus: MidiBusLock) -> Song
  where
    T: Into<String>,
  {
    let sample_rate = config.audio.sample_rate;
    let metronome_config = config.metronome.clone();

    let transport = Transport::new(sample_rate);
    let signature = *transport.get_signature();

    let metronome = Metronome::new(metronome_config, signature, midi_bus.clone());

    Song {
      name: name.into(),
      transport: transport,
      metronome,
      tracks: Vec::new(),
      midi_bus,
    }
  }

  pub fn set_name<T>(&mut self, name: T)
  where
    T: Into<String>,
  {
    self.name = name.into();
  }

  pub fn get_name(&self) -> &str {
    self.name.as_str()
  }

  pub fn set_loop_enabled(&mut self, enabled: bool) {
    self.transport.set_loop_enabled(enabled);
  }

  pub fn set_loop_start(&mut self, position: BarsTime) {
    self.transport.set_loop_start(position);
  }

  pub fn set_loop_end(&mut self, position: BarsTime) {
    self.transport.set_loop_end(position)
  }

  pub fn play(&mut self, restart: bool) -> bool {
    self.transport.play(restart);
    self.transport.is_playing()
  }

  pub fn stop(&mut self) {
    self.transport.stop();
  }

  ///! Process song play
  pub fn process(&mut self, audio_time: AudioTime, samples: u32) {
    if self.transport.is_playing() {
      let mut segments = self.transport.segments_iterator(audio_time.output, samples);
      while let Some(segment) = segments.next(&self.transport) {
        self.process_segment(&segment);
      }
      self.transport.update_from_segments(&segments);
    }

    // TODO some devices might need to keep track of time even when not playing
  }

  fn process_segment(&mut self, segment: &Segment) {
    // println!(
    //   "=> Segment T [{:06?}, {:06?}) <{:06?}> C [{:010?}, {:010?}) <{:010?}> @ PT {:06?} PC {:010?}",
    //   u64::from(segment.start_ticks),
    //   u64::from(segment.end_ticks),
    //   u64::from(segment.segment_ticks),
    //   segment.start_time.units(),
    //   segment.end_time.units(),
    //   segment.segment_time.units(),
    //   u64::from(segment.play_ticks),
    //   segment.play_time.units()
    // );

    let signature = *self.transport.get_signature();
    let tempo = *self.transport.get_tempo();
    self.metronome.process_segment(segment, signature, tempo);

    for track in self.tracks.iter_mut() {
      // let clips = track.clips_in_range(start_ticks, end_ticks);
      match &track.media {
        TrackMedia::Midi(_midi_track) => {
          // prepare buffer for midi_track.sink

        }
        TrackMedia::Audio(_audio_track) => {}
        TrackMedia::Instrument(_instrument_track) => {}
      }
    }
  }
}
