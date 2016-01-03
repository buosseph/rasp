use std;
use num::traits::Float;

/// Necessary floating point constants
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

/// Represents a audio DSP filter.
///
/// The definition of a general `Filter` in DSP is different from the common
/// audio filter, such as a low-pass filter, you may already be familiar with.
/// A `Filter` represents a linear time-invariant system, that processes an
/// input signal.
pub trait Filter<T: Float> {
  /// Processes and stores input sample into memory and outputs calculated
  /// sample.
  fn tick(&mut self, sample: T) -> T;

  /// Resets memory of all previous input and output to zero.
  fn last_out(&self) -> T;

  /// Returns the last computed output sample.
  fn clear(&mut self);
}

/// Represents a tappable delay line.
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
