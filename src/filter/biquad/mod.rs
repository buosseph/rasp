//! Biquads are a popular choice for implementing
//! common audio filters.
pub mod lowpass;
pub mod highpass;

use Filter;
use std::f64::MIN_POSITIVE;

const MIN_SAMPLE_RATE: f64 = MIN_POSITIVE;
const MIN_FREQUENCY: f64 = 0f64;
const MIN_Q: f64 = MIN_POSITIVE;

/// Single channel, second-order filter.
///
/// A `Biquad` is a type of second-order `Filter` that uses the following equation:
/// > y[n] = b0*x[n] + b1*x[n-1] + b2*x[n-2] - a1*y[n-1] - a2*y[n-2]
pub struct Biquad {
  x_z1: f64,
  x_z2: f64,
  y_z1: f64,
  y_z2: f64,
  pub b0: f64,
  pub b1: f64,
  pub b2: f64,
  pub a1: f64,
  pub a2: f64
}

impl Biquad {
  /// Constructs a new `Biquad`.
  ///
  /// The filter will not alter the signal
  /// unitl the coefficients are changed.
  pub fn new() -> Biquad {
    Biquad {
      x_z1: 0f64,
      x_z2: 0f64,
      y_z1: 0f64,
      y_z2: 0f64,
      b0: 1f64,
      b1: 0f64,
      b2: 0f64,
      a1: 0f64,
      a2: 0f64
    }
  }

  /// Sets all filter coefficients at once.
  ///
  /// `b0`, `b1`, `b2` are feedforwards and `a1`, `a2` are feedbacks.
  #[allow(dead_code)]
  pub fn set_coefficients(&mut self, b0: f64, b1: f64, b2: f64, a1: f64, a2: f64) {
    self.b0 = b0;
    self.b1 = b1;
    self.b2 = b2;
    self.a1 = a1;
    self.a2 = a2;
  }
}

impl Filter for Biquad {
  fn tick(&mut self, sample: f64) -> f64 {
    let output = self.b0 * sample
      + self.b1 * self.x_z1 + self.b2 * self.x_z2
      - self.a1 * self.y_z1 - self.a2 * self.y_z2;
    self.x_z2 = self.x_z1;
    self.x_z1 = sample;
    self.y_z2 = self.y_z1;
    self.y_z1 = output;
    output
  }

  fn clear(&mut self) {
    self.x_z1 = 0f64;
    self.x_z2 = 0f64;
    self.y_z1 = 0f64;
    self.y_z2 = 0f64;
  }
}

#[cfg(test)]
mod tests {
  use Filter;
  use std::f64::EPSILON;
  use super::*;

  #[test]
  fn tick() {
    let input = vec![0.55f64, 0.55f64, 0.55f64, 0.55f64, 0.25f64];
    let mut biquad = Biquad::new();
    assert!((biquad.tick(input[0]) - 0.55f64).abs() < EPSILON);
    biquad.clear();
    biquad.set_coefficients(0.5f64, 0.4f64, 0.3f64, 0.2f64, 0.1f64);
    assert!((biquad.tick(input[1]) - 0.275f64).abs() < EPSILON);
    assert!((biquad.tick(input[2]) - 0.44f64).abs() < EPSILON);
    assert!((biquad.tick(input[3]) - 0.5445f64).abs() < EPSILON);
    assert!((biquad.tick(input[4]) - 0.3571f64).abs() < EPSILON);
  }
}
