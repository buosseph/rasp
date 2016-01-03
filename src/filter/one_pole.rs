// Use this to set up an example of a one-pole lowpass and highpass
// http://www.earlevel.com/main/2012/12/15/a-one-pole-filter/
use num;
use num::traits::Float;

use traits::Filter;

/// A single channel, one pole digital filter.
///
/// A `OnePole` filter uses the following equation:
///
/// `y[n] = b0*x[n] - a1*y[n-1]`
///
/// It has one feedback coefficient, `a1`. 
pub struct OnePole<T: Float> {
  y_z1: T,
  pub b0: T,
  pub a1: T
}

impl<T> OnePole<T> where T: Float {
  /// Creates a new `OnePole` filter.
  ///
  /// The filter will be initalized in a state that does not alter the input
  /// signal.
  ///
  /// # Examples
  ///
  /// ```
  /// # #![allow(unused_mut)]
  /// use rasp::filter::OnePole;
  ///
  /// let mut filter1: OnePole<f32> = OnePole::new();
  /// let mut filter2: OnePole<f64> = OnePole::new();
  /// let mut filter3 = OnePole::<f32>::new();
  /// let mut filter4 = OnePole::<f64>::new();
  /// ```
  pub fn new() -> Self {
    OnePole {
      y_z1: num::zero(),
      b0: num::one(),
      a1: num::zero()
    }
  }

  /// Sets all filter coefficients at once.
  ///
  /// `a1` is a feedback, or pole.
  pub fn set_coefficients(&mut self, b0: T, a1: T) {
    self.b0 = b0;
    self.a1 = a1;
  }
}

impl<T> Filter<T> for OnePole<T> where T: Float {
  fn tick(&mut self, sample: T) -> T {
    let output = self.b0 * sample - self.a1 * self.y_z1;
    self.y_z1 = output;
    output
  }

  fn clear(&mut self) {
    self.y_z1 = num::zero();
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
         0.275_000_000_000f32,
        -0.302_500_000_000f32,
         0.305_250_000_000f32,
        -0.305_525_000_000f32,
         0.155_552_500_000f32
      ];
    let mut one_pole = OnePole::new();
    for sample in input.iter() {
      assert!((one_pole.tick(*sample) - sample).abs() < EPSILON);
    }
    one_pole.clear();
    one_pole.set_coefficients(0.5f32, 0.1f32);
    for i in 0..input.len() {
      let output = one_pole.tick(input[i]);
      println!("{:.12} - {:.12} = {:.12}", expected[i], output, expected[i] - output);
      assert!((expected[i] - output).abs() < EPSILON);
    }
  }
}
