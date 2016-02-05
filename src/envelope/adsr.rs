enum EnvState {
  Attack,
  Decay,
  Sustain,
  Release,
  Idle
}

/// An ADSR envelope generator
///
/// [Based on code by Nigel Redmon](http://www.earlevel.com/main/2013/06/03/envelope-generators-adsr-code/)
pub struct Adsr {
  sample_rate: f32,
  state: EnvState,
  target: f32,
  value: f32,

  attack_time: f32,
  decay_time: f32,
  sustain_level: f32,
  release_time: f32,

  // Rates at which the envelope is changing within a state
  attack_rate: f32,
  decay_rate: f32,
  release_rate: f32
}

impl Adsr {
  pub fn new(sample_rate: f32) -> Self {
    Adsr {
      sample_rate: sample_rate,
      state: EnvState::Idle,
      target: 0f32,
      value: 0f32,
      attack_time: 0f32,
      decay_time: 0f32,
      sustain_level: 1f32,
      release_time: 0f32,
      attack_rate: 0f32,
      decay_rate: 0f32,
      release_rate: 0f32
    }
  }

  /// Update sample rate of envelope
  pub fn set_sample_rate(&mut self, sample_rate: f32) {
    self.sample_rate = sample_rate;
    set_attack(self.attack_time);
    set_decay(self.decay_time);
    set_release(self.release_time);
  }

  /// `attack_time` is in seconds
  pub fn set_attack(&mut self, attack_time: f32) {
    self.attack_time = attack_time;
    self.attack_rate = 1f32 / (attack_time * self.sample_rate);
  }

  /// `decay_time` is in seconds
  pub fn set_decay(&mut self, decay_time: f32) {
    self.decay_time = decay_time;
    self.decay_rate = (1f32 - self.sustain_level) / (decay_time * self.sample_rate);
  }

  /// `sustain_level` is [0, 1]
  pub fn set_sustain(&mut self, sustain_level: f32) {
    self.sustain_level = sustain_level;
  }

  /// `release_time` is in seconds
  pub fn set_release(&mut self, release_time: f32) {
    self.release_time = release_time;
    self.release_rate = self.sustain_level / (release_time * self.sample_rate);
  }
}

pub trait Generator {
  fn tick(&mut self) -> f32;
  fn last_out(&self) -> f32;
  fn reset(&mut self);
}

impl Generator for Adsr {
  fn tick(&mut self) -> f32 {
    match self.state {
      EnvState::Attack => {
        self.value += self.attack_rate;
        if self.value >= self.target {
          self.value = self.target;
          self.target = self.sustain_level;
          self.state = EnvState::Decay;
        }
      },
      EnvState::Decay => {
        if self.value > self.sustain_level {
          self.value -= self.decay_rate;
          if self.value <= self.sustain_level {
            self.value = self.sustain_level;
            self.state = EnvState::Sustain;
          }
        }
        else {
          self.value += self.decay_rate; // attack target < sustain_level
          if self.value >= self.sustain_level {
            self.value = self.sustain_level;
            self.state = EnvState::Sustain;
          }
        }
      },
      EnvState::Release => {
        self.value -= self.release_rate;
        if self.value <= 0f32 {
          self.value = 0f32;
          self.state = EnvState::Idle;
        }
      },
      _ => continue,
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

pub trait EnvGenerator {
  fn gateOn(&mut self);
  fn gateOff(&mut self);
}

impl EnvGenerator for Adsr {
  // enter Attack state
  fn gateOn(&mut self) {
    // if target <= 0f32 {
    //   target = 1f32;
    // }
    self.state = EnvState::Attack;
  }

  // if not Idle, enter Release state
  fn gateOff(&mut self) {
    // self.target = 0f32;

    self.state =
      match self.state {
        EnvState::Idle => {},
        _ => EnvState::Release
      };

    // if self.release_time > 0f32 {
    //   self.release_rate = self.value / (self.release_rate / self.sample_rate);
    // }
  }
}
