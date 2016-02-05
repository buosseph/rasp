// TODO: Move EnvState to mod.rs
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EnvState {
  Attack,
  Sustain,
  Release,
  Idle
}

// TODO: Move traits to mod traits
pub trait Generator {
  fn tick(&mut self) -> f32;
  fn last_out(&self) -> f32;
  fn reset(&mut self);
}

pub trait EnvGenerator : Generator {
  fn gate_on(&mut self);
  fn gate_off(&mut self);
}

pub struct Ar {
  // The sample rate of all signals to be processed
  sample_rate: f32,
  // The current state of the envelope
  state: EnvState,
  // The current value of the envelope
  value: f32,


  // The stored attack time of the envelope
  attack_time: f32,
  // The stored release time of the envelope
  release_time: f32,

  // Rates at which the envelope is changing within a state
  attack_gain: f32,
  release_gain: f32
}

impl Ar {
  pub fn new(sample_rate: f32) -> Self {
    Ar {
      sample_rate: sample_rate,
      state: EnvState::Idle,
      value: 0f32,

      attack_time: 0f32,
      release_time: 0f32,
      attack_gain: 0f32,
      release_gain: 0f32
    }
  }

  // Move this to Generator triat?
  pub fn set_sample_rate(&mut self, sample_rate: f32) {
    self.sample_rate = sample_rate;
    self.reset();
    // set attack and release times
  }

  // // TODO: Determine how to apply attack_time to 
  // /// `attack_time` is in seconds
  // pub fn set_attack(&mut self, attack_time: f32) {
  //   self.attack_time = attack_time;
  //   self.attack_gain = 1f32 / (attack_time * self.sample_rate);
  // }

  // pub fn set_attack_time(&mut self, attack_time: f32) {
  //   debug_assert!(attack_time > 0f32);
  //   self.attack_time = attack_time;
  //   self.attack_gain = 1f32 / (attack_time * sample_rate);
  // }
}

impl Generator for Ar {
  fn tick(&mut self) -> f32 {
    match self.state {
      EnvState::Idle | EnvState::Sustain => {},
      EnvState::Attack => {
        self.value += self.value * self.attack_gain; // DEBUG
        if self.value >= 1f32 {
          self.value = 1f32;
          self.state = EnvState::Sustain;
        }
      },
      EnvState::Release => {
        self.value -= self.value * self.release_gain; // DEBUG
        if self.value <= 0f32 {
          self.value = 0f32;
          self.state = EnvState::Idle;
        }
      }
    }

    self.value
  }

  fn last_out(&self) -> f32 {
    self.value
  }

  fn reset(&mut self) {
    self.state = EnvState::Idle;
    self.value = 0f32;
  }
}

impl EnvGenerator for Ar {
  fn gate_on(&mut self) {
    self.state = EnvState::Attack;
  }

  fn gate_off(&mut self) {
    if self.state != EnvState::Idle {
      self.state = EnvState::Release;
    }
  }
}
