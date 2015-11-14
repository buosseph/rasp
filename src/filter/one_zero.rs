/// A single channel, one zero digital filter.
///
/// A `OneZero` filter uses the following equation:
///
/// `y[n] = b0*x[n] + b1*x[n-1]`
///
/// It has one feedforward coefficient, `b1`. 
pub struct OneZero {
  x_z1: f32,
  y_z1: f32, // Only necessary for last_out()
  pub b0: f32,
  pub b1: f32
}

impl OneZero {
  /// Creates a new `OneZero` filter.
  ///
  /// The filter will be initalized in a state that does not alter the input
  /// signal.
  pub fn new() -> OneZero {
    OneZero {
      x_z1: 0f32,
      y_z1: 0f32,
      b0: 1f32, b1: 0f32
    }
  }

  /// Sets all filter coefficients at once.
  ///
  /// `b1` is a feedforward, or zero.
  pub fn set_coefficients(&mut self, b0: f32, b1: f32) {
    self.b0 = b0;
    self.b1 = b1;
  }

  /// Processes and stores input sample into memory and outputs calculated
  /// sample.
  pub fn tick(&mut self, sample: f32) -> f32 {
    self.y_z1 = self.b0 * sample + self.b1 * self.x_z1;
    self.x_z1 = sample;
    self.y_z1
  }

  /// Resets memory of all previous input and output to zero.
  pub fn clear(&mut self) {
    self.x_z1 = 0f32;
  }

  /// Returns the last computed output sample.
  pub fn last_out(&self) -> f32 {
    self.y_z1
  }
}

#[cfg(test)]
mod tests {
  use std::f32::EPSILON;
  use super::*;

  /*
   *  Octave input used to test, print all values to 12 decimal point for use in tests
   *
   *  clear
   *  x_z1 = 0
   *
   *  tick (and print y)
   *  y = b0 * x + b1 * x_z1; x_z1 = x; printf("%.12f\n", y)
   *
   *  print to 12 decimal places
   *  printf("%.12f\n", y)
   */

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
