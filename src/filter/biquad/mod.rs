//! Biquads are a popular choice for implementing common audio filters.

pub mod lowpass;
pub mod highpass;

use DspComponent;
use Filter;
use std::f32::MIN_POSITIVE;

// Used in common filter implementations
const MIN_SAMPLE_RATE: f32 = MIN_POSITIVE;
const MIN_FREQUENCY: f32 = 0f32;
const MIN_Q: f32 = MIN_POSITIVE;

/// Single channel, second-order filter.
///
/// A `Biquad` is a type of second-order `Filter` that uses the following
/// equation:
///
/// > y[n] = b0*x[n] + b1*x[n-1] + b2*x[n-2] - a1*y[n-1] - a2*y[n-2]
///
/// It has two feedforward coefficients `b1` and `b2`, and two feedback
/// coefficients `a1` and `a2`.
pub struct Biquad {
  x_z1: f32, x_z2: f32,
  y_z1: f32, y_z2: f32,
  pub b0: f32,
  pub b1: f32,
  pub b2: f32,
  pub a1: f32,
  pub a2: f32
}

impl Biquad {
  /// Sets all filter coefficients at once.
  ///
  /// `b1`, `b2` are feedforwards, or zeroes, and `a1`, `a2` are feedbacks,
  /// or poles.
  pub fn set_coefficients(&mut self, b0: f32, b1: f32, b2: f32, a1: f32, a2: f32) {
    self.b0 = b0; self.b1 = b1; self.b2 = b2;
    self.a1 = a1; self.a2 = a2;
  }
}

impl DspComponent for Biquad {
  fn new() -> Biquad {
    Biquad {
      x_z1: 0f32, x_z2: 0f32,
      y_z1: 0f32, y_z2: 0f32,
      b0: 1f32, b1: 0f32, b2: 0f32, a1: 0f32, a2: 0f32
    }
  }

  fn tick(&mut self, sample: f32) -> f32 {
    let output = self.b0 * sample
      + self.b1 * self.x_z1 + self.b2 * self.x_z2
      - self.a1 * self.y_z1 - self.a2 * self.y_z2;
    self.x_z2 = self.x_z1; self.x_z1 = sample;
    self.y_z2 = self.y_z1; self.y_z1 = output;
    output
  }
}

impl Filter for Biquad {
  fn clear(&mut self) {
    self.x_z1 = 0f32; self.x_z2 = 0f32;
    self.y_z1 = 0f32; self.y_z2 = 0f32;
  }

  fn last_out(&self) -> f32 {
    self.y_z1
  }
}

#[cfg(test)]
mod tests {
  use DspComponent;
  use Filter;
  use std::f32::EPSILON;
  use super::*;

  /*
   *  Octave input used to test, print all values to 12 decimal point for use in tests
   *
   *  input, output
   *  x, y
   *
   *  clear
   *  x_z1 = x_z2 = y_z1 = y_z2 = 0
   *
   *  tick (and print y)
   *  y = b0 * x + b1 * x_z1 + b2 * x_z2 - a1 * y_z1 - a2 * y_z2; x_z2 = x_z1; x_z1 = x; y_z2 = y_z1; y_z1 = y; printf("%.12f\n", y)
   *
   *  print to 12 decimal places
   *  printf("%.12f\n", y)
   *  printf("%.12f\n", b0), printf("%.12f\n", b1), printf("%.12f\n", b2), printf("%.12f\n", a1), printf("%.12f\n", a2)
   */

  #[test]
  fn tick() {
    let input = vec![0.55f32, -0.55f32, 0.55f32, -0.55f32, 0.25f32];
    let expected =
      vec![
         0.275_000_000_000f32,
        -0.110_000_000_000f32,
         0.214_500_000_000f32,
        -0.251_900_000_000f32,
         0.098_930_000_000f32
      ];
    let mut biquad = Biquad::new();
    for sample in input.iter() {
      assert!((biquad.tick(*sample) - sample).abs() < EPSILON);
    }
    biquad.clear();
    biquad.set_coefficients(0.5f32, 0.4f32, 0.3f32, 0.2f32, 0.1f32);
    for i in 0..input.len() {
      let output = biquad.tick(input[i]);
      println!("{:.12} - {:.12} = {:.12}", expected[i], output, expected[i] - output);
      assert!((expected[i] - output).abs() < EPSILON);
    }
  }
}
