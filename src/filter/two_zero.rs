use num;
use num::traits::Float;

use traits::Filter;

/// A single channel, two zero digital filter.
///
/// A `TwoZero` filter uses the following equation:
///
/// `y[n] = b0*x[n] + b1*x[n-1] + b2*x[n-2]`
///
/// It has two feedforward coefficients, `b1` and `b2`.
pub struct TwoZero<T> {
  x_z1: T,
  x_z2: T,
  // Only necessary for last_out()
  y_z1: T,
  pub b0: T,
  pub b1: T,
  pub b2: T
}

impl<T> TwoZero<T> where T: Float {
  /// Creates a new `TwoZero` filter.
  ///
  /// The filter will be initalized in a state that does not alter the input
  /// signal.
  ///
  /// # Examples
  ///
  /// ```
  /// # #![allow(unused_mut)]
  /// use rasp::filter::TwoZero;
  ///
  /// let mut filter1: TwoZero<f32> = TwoZero::new();
  /// let mut filter2: TwoZero<f64> = TwoZero::new();
  /// let mut filter3 = TwoZero::<f32>::new();
  /// let mut filter4 = TwoZero::<f64>::new();
  /// ```
  pub fn new() -> Self {
    TwoZero {
      x_z1: num::zero(),
      x_z2: num::zero(),
      y_z1: num::zero(),
      b0: num::one(),
      b1: num::zero(),
      b2: num::zero()
    }
  }

  /// Sets all filter coefficients at once.
  ///
  /// `b1` and `b2` are feedforwards, or zeroes.
  pub fn set_coefficients(&mut self, b0: T, b1: T, b2: T) {
    self.b0 = b0;
    self.b1 = b1;
    self.b2 = b2;
  }
}

impl<T> Filter<T> for TwoZero<T> where T: Float {
  fn tick(&mut self, sample: T) -> T {
    self.y_z1 = self.b0 * sample
      + self.b1 * self.x_z1 + self.b2 * self.x_z2;
    self.x_z2 = self.x_z1;
    self.x_z1 = sample;
    self.y_z1
  }

  fn clear(&mut self) {
    self.x_z1 = num::zero();
    self.x_z2 = num::zero();
  }

  fn last_out(&self) -> T {
    self.y_z1
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::f32::EPSILON;
  use ::traits::Filter;

  #[test]
  fn tick() {
    let input = vec![0.55f32, -0.55f32, 0.55f32, -0.55f32, 0.25f32];
    let expected =
      vec![
         0.495_000_000_000f32,
        -0.605_000_000_000f32,
         1.320_000_000_000f32,
        -1.320_000_000_000f32,
         1.050_000_000_000f32
      ];
    let mut two_zero = TwoZero::new();
    for sample in input.iter() {
      assert!((two_zero.tick(*sample) - sample).abs() <= EPSILON);
    }
    two_zero.clear();
    two_zero.set_coefficients(0.9f32, -0.2, 1.3f32);
    for i in 0..input.len() {
      let output = two_zero.tick(input[i]);
      println!("{:.12} - {:.12} = {:.12}", expected[i], output, expected[i] - output);
      assert!((expected[i] - output).abs() <= EPSILON);
    }
  }
}
