use num;
use num::traits::Float;

use traits::Filter;

/// A single channel, one zero digital filter.
///
/// A `OneZero` filter uses the following equation:
///
/// `y[n] = b0*x[n] + b1*x[n-1]`
///
/// It has one feedforward coefficient, `b1`. 
pub struct OneZero<T: Float> {
  x_z1: T,
  // Only necessary for last_out()
  y_z1: T,
  pub b0: T,
  pub b1: T
}

impl<T> OneZero<T> where T: Float {
  /// Creates a new `OneZero` filter.
  ///
  /// The filter will be initalized in a state that does not alter the input
  /// signal.
  ///
  /// # Examples
  ///
  /// ```
  /// # #![allow(unused_mut)]
  /// use rasp::filter::OneZero;
  ///
  /// let mut filter1: OneZero<f32> = OneZero::new();
  /// let mut filter2: OneZero<f64> = OneZero::new();
  /// let mut filter3 = OneZero::<f32>::new();
  /// let mut filter4 = OneZero::<f64>::new();
  /// ```
  pub fn new() -> Self {
    OneZero {
      x_z1: num::zero(),
      y_z1: num::zero(),
      b0: num::one(),
      b1: num::zero()
    }
  }

  /// Sets all filter coefficients at once.
  ///
  /// `b1` is a feedforward, or zero.
  pub fn set_coefficients(&mut self, b0: T, b1: T) {
    self.b0 = b0;
    self.b1 = b1;
  }
}

impl<T> Filter<T> for OneZero<T> where T: Float {
  fn tick(&mut self, sample: T) -> T {
    self.y_z1 = self.b0 * sample + self.b1 * self.x_z1;
    self.x_z1 = sample;
    self.y_z1
  }

  fn clear(&mut self) {
    self.x_z1 = num::zero();
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
        -0.165_000_000_000f32,
         0.605_000_000_000f32,
        -0.605_000_000_000f32,
         0.605_000_000_000f32,
        -0.515_000_000_000f32
      ];
    let mut one_zero = OneZero::new();
    for sample in input.iter() {
      assert!((one_zero.tick(*sample) - sample).abs() < EPSILON);
    }
    one_zero.clear();
    one_zero.set_coefficients(-0.3f32, 0.8f32);
    for i in 0..input.len() {
      let output = one_zero.tick(input[i]);
      println!("{:.12} - {:.12} = {:.12}", expected[i], output, expected[i] - output);
      assert!((expected[i] - output).abs() < EPSILON);
    }
  }
}
