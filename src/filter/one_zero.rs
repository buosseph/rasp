use num;
use num::traits::Float;

use traits::Processor;

/// A single channel, one zero digital filter.
///
/// A `OneZero` filter uses the following equation:
///
/// `y[n] = b0*x[n] + b1*x[n-1]`
///
/// It has one feedforward coefficient, `b1`.
#[repr(C)]
pub struct OneZero<T: Float> {
  x_z1: T,
  output: T,
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
      output: num::zero(),
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

impl<T> Processor<T> for OneZero<T> where T: Float {
  fn process(&mut self, sample: T) -> T {
    self.output = self.b0 * sample + self.b1 * self.x_z1;
    self.x_z1 = sample;
    self.output
  }

  fn clear(&mut self) {
    self.x_z1 = num::zero();
  }

  fn last_out(&self) -> T {
    self.output
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::f32::EPSILON;
  use ::traits::Processor;

  #[test]
  fn process() {
    let input = vec![0.55f32, -0.55f32, 0.55f32, -0.55f32, 0.25f32];
    let expected =
      vec![
        -0.165_000_000_000f32,
         0.605_000_000_000f32,
        -0.605_000_000_000f32,
         0.605_000_000_000f32,
        -0.515_000_000_000f32
      ];
    let mut filter = OneZero::new();

    for sample in input.iter() {
      assert!((filter.process(*sample) - sample).abs() < EPSILON);
    }

    filter.clear();
    filter.set_coefficients(-0.3f32, 0.8f32);

    for i in 0..input.len() {
      let output = filter.process(input[i]);
      println!("{:.12} - {:.12} = {:.12}", expected[i], output, expected[i] - output);
      assert!((expected[i] - output).abs() < EPSILON);
    }
  }

  #[test]
  fn process_block() {
    let input = vec![0.55f32, -0.55f32, 0.55f32, -0.55f32, 0.25f32];
    let expected =
      vec![
        -0.165_000_000_000f32,
         0.605_000_000_000f32,
        -0.605_000_000_000f32,
         0.605_000_000_000f32,
        -0.515_000_000_000f32
      ];
    let mut filter = OneZero::new();

    let mut initial_input = input.clone();
    let mut last_processed = filter.process_block(&mut initial_input);
    assert!((last_processed - input.last().unwrap()).abs() < EPSILON);

    filter.clear();
    filter.set_coefficients(-0.3f32, 0.8f32);

    let mut actual = input.clone();
    last_processed = filter.process_block(&mut actual);
    assert!((last_processed - expected.last().unwrap()).abs() < EPSILON);

    for i in 0..input.len() {
      println!("{:.12} - {:.12} = {:.12}", expected[i], actual[i], expected[i] - actual[i]);
      assert!((expected[i] - actual[i]).abs() < EPSILON);
    }
  }
}
