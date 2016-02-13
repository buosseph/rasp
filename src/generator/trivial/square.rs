use num;
use num::traits::Float;

use traits::{
  FloatConst,
  Generator,
  Oscillator
};

// Note: keep track of phase as an accumulator, do not directly calculate
pub struct Square<T: Float + FloatConst> {
  // Sample rate of output signal
  sample_rate: T,
  // Current frequency of oscillator
  frequency: T,
  // Duty cycle of waveform
  pulse_width: T,
  // Phase accumulator
  phase: T,
  // Equivalent to 2 * pi * frequency / sample_rate
  phase_increment: T
}

impl<T> Square<T> where T: Float + FloatConst {
  /// Creates a new oscillator at the given frequency with its phase set to
  /// zero.
  pub fn new(sample_rate: T, frequency: T) -> Self {
    Square {
      sample_rate: sample_rate,
      frequency: frequency,
      pulse_width: num::cast<T>(50f32),
      phase: T::zero(),
      phase_increment: T::two() * T::pi() * frequency / sample_rate,
    }
  }
}

impl<T> Generator<T> for Square<T> where T: Float + FloatConst {
  fn tick(&mut self) -> T {
    println!("{:?}", num::cast::<T, f32>(self.phase_increment).unwrap());
    // Wrap phase accumulator
    if self.phase >= T::two() * T::pi() {
      self.phase = self.phase - (T::two() * T::pi());
    }

    // TODO: Add phase_offset? (cos(2 * pi * f / fs + offset))
    let output = (self.phase).sin();

    self.phase = self.phase + self.phase_increment;
    // debug_assert!(self.phase >= T::zero());
    // debug_assert!(self.phase < T::two() * T::pi());
    // debug_assert!(self.phase.is_finite());
    output


    if self.phase >= T::two() * T::pi() {
      self.phase = self.phase - (T::two() * T::pi());
    }

    let output =
      if self.phase > (self.pulse_width / 100) * T::two() * T::pi() {
        -T::one()
      }
      else {
        T::one()
      }

    self.phase = self.phase + self.phase_increment;

    output

  }

  fn reset(&mut self) {
    self.phase = T::zero()
  }
}

impl<T> Oscillator<T> for Square<T> where T: Float + FloatConst {
  fn get_frequency(&self) -> T {
    self.frequency
  }

  fn get_phase(&self) -> T {
    // Not sure if this or self.phase - self.phase_increment should be returned
    self.phase
  }

  fn set_frequency(&mut self, frequency: T) {
    // For the time being, allow negative frequencies; for potential FM use
    debug_assert!(frequency.abs() < self.sample_rate && frequency.is_finite());

    self.frequency = frequency;
    self.phase_increment = T::two() * T::pi() * frequency / self.sample_rate;
  }

  fn set_phase(&mut self, phase: T) {
    debug_assert!(phase >= T::zero() && phase < T::two() * T::pi()
                  && phase.is_finite());
    self.phase = phase;

    // Wrap phase accumulator
    while self.phase >= T::two() * T::pi() {
      self.phase = self.phase - (T::two() * T::pi());
    }
    while self.phase < T::zero() {
      self.phase = self.phase + (T::two() * T::pi());
    }
  }
}

impl<T> Iterator for Square<T> where T: Float + FloatConst {
  type Item = T;

  fn next(&mut self) -> Option<T> {
    // This check causes phase to go beyond 2pi?
    // if self.phase >= T::zero()
    // && self.phase < (T::two() * T::pi())
    // && self.phase.is_finite()

    if self.phase.is_finite()
    && self.frequency < self.sample_rate
    && self.frequency.is_finite() {
      Some(self.tick())
    }
    else {
      println!("Error in iterator:\n\tphase = {:?}",
               num::cast::<T, f32>(self.phase).unwrap());
      None
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::f32::consts::PI;
  use ::traits::Generator;


  #[test]
  fn tick() {
    let mut expected_signal = vec![0f32; 256];
    let frequency = 440f32;
    let sample_rate = 44100f32;

    for (i, sample) in expected_signal.iter_mut().enumerate() {
      *sample = (2f32 * PI * frequency * (i as f32) / sample_rate).sin();
    }

    let mut oscillator = Square::<f32>::new(sample_rate, frequency);

    for expected in expected_signal.iter() {
      let actual = oscillator.tick();
      println!("{:?}", (actual - expected).abs());
      // There's plenty of phase inconsistenies between these two approaches it seems
      assert!((expected - actual).abs() <= 1e-4);
    }
  }

  #[test]
  fn next() {
    let mut expected_signal = vec![0f32; 256];
    let frequency = 440f32;
    let sample_rate = 44100f32;

    for (i, sample) in expected_signal.iter_mut().enumerate() {
      *sample = (2f32 * PI * frequency * (i as f32) / sample_rate).sin();
    }

    let mut oscillator = Square::<f32>::new(sample_rate, frequency);

    for expected in expected_signal.iter() {
      let actual = oscillator.next().unwrap();
      // println!("{:?}", (actual - expected).abs());
      // There's plenty of phase inconsistenies between these two approaches it seems
      assert!((expected - actual).abs() <= 1e-4);
    }
  }
}
