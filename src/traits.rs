use num::traits::Float;

use std;

/// Common floating point constants
pub trait FloatConst {
  fn pi() -> Self;
  fn two() -> Self;
}

impl FloatConst for f32 {
  fn pi() -> Self {
    std::f32::consts::PI
  }

  fn two() -> Self {
    2f32
  }
}

impl FloatConst for f64 {
  fn pi() -> Self {
    std::f64::consts::PI
  }

  fn two() -> Self {
    2f64
  }
}

/// An audio processor.
pub trait Processor<T: Float> {
  /// Processes and stores input sample into memory and outputs calculated
  /// sample.
  fn process(&mut self, sample: T) -> T;

  /// Processes a contiguous sequence of samples, calling `process()` on each
  /// sample.
  fn process_block(&mut self, samples: &mut [T]) -> T {
    for sample in samples.iter_mut() {
      *sample = self.process(*sample);
    }
    *samples.last().unwrap()
  }

  /// Resets memory of all previous input and output to zero.
  fn clear(&mut self);

  /// Returns the last computed output sample.
  fn last_out(&self) -> T;
}

/// A tappable delay line.
///
/// A tappable delay line is able to access samples at a specified offset
/// from the internal write pointer.
pub trait TappableDelayLine<T: Float> {
  /// Returns the value at `tap_delay` samples from the current delay-line
  /// input.
  fn tap_out(&self, tap_delay: usize) -> T;

  /// Sets the value at `tap_delay` samples from the current delay-line
  /// input.
  fn tap_in(&mut self, value: T, tap_delay: usize);

  /// Adds to the value at `tap_delay` samples from the current delay-line
  /// input.
  fn add_to(&mut self, value: T, tap_delay: usize) -> T;
}
