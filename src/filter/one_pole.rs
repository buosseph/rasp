// Use this to set up an example of a one-pole lowpass and highpass
// http://www.earlevel.com/main/2012/12/15/a-one-pole-filter/
use Filter;

/// Single channel, one pole digital filter.
///
/// A `OnePole` is a type of `Filter` that uses the following equation:
/// > y[n] = b0*x[n] - a1*y[n-1]
/// It only has one feedback coefficient `a1`. 
pub struct OnePole {
  y_z1: f64,
  pub b0: f64,
  pub a1: f64
}

impl OnePole {
  /// Constructs a new `OnePole`.
  ///
  /// The filter will not alter the signal
  /// unitl the coefficients are changed.
  pub fn new() -> OnePole {
    OnePole {
      y_z1: 0f64,
      b0: 1f64,
      a1: 0f64
    }
  }

  /// Sets all filter coefficients at once.
  ///
  /// `a1` is a feedback, or pole.
  #[allow(dead_code)]
  pub fn set_coefficients(&mut self, b0: f64, a1: f64) {
    self.b0 = b0;
    self.a1 = a1;
  }
}

impl Filter for OnePole {
  fn tick(&mut self, sample: f64) -> f64 {
    let output = self.b0 * sample - self.a1 * self.y_z1;
    self.y_z1 = output;
    output
  }

  fn clear(&mut self) {
    self.y_z1 = 0f64;
  }
}

#[cfg(test)]
mod tests {
  use Filter;
  use std::f64::EPSILON;
  use super::*;

  /*
   *  Octave input used to test, print all values to 12 decimal point for use in tests
   *
   *  clear
   *  y_z1 = 0
   *
   *  tick (and print y)
   *  y = b0 * x - a1 * y_z1; y_z1 = y; printf("%.12f\n", y)
   *
   *  print to 12 decimal places
   *  printf("%.12f\n", y)
   */

  #[test]
  fn tick() {
    let input = vec![0.55f64, -0.55f64, 0.55f64, -0.55f64, 0.25f64];
    let expected =
      vec![
         0.275_000_000_000f64,
        -0.302_500_000_000f64,
         0.305_250_000_000f64,
        -0.305_525_000_000f64,
         0.155_552_500_000f64
      ];
    let mut one_pole = OnePole::new();
    for sample in input.iter() {
      assert!((one_pole.tick(*sample) - sample).abs() < EPSILON);
    }
    one_pole.clear();
    one_pole.set_coefficients(0.5f64, 0.1f64);
    for i in 0..input.len() {
      let output = one_pole.tick(input[i]);
      println!("{:.12} - {:.12} = {:.12}", expected[i], output, expected[i] - output);
      assert!((expected[i] - output).abs() < EPSILON);
    }
  }
}
