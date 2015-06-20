use Filter;

/// Single channel, two zero digital filter.
///
/// A `TwoZero` is a type of `Filter` that uses the following equation:
/// > y[n] = b0*x[n] + b1*x[n-1] + b2*x[n-2]
/// It has two feedforward coefficients `b1` and `b2`. 
pub struct TwoZero {
  x_z1: f64,
  x_z2: f64,
  pub b0: f64,
  pub b1: f64,
  pub b2: f64
}

impl TwoZero {
  /// Constructs a new `TwoZero`.
  ///
  /// The filter will not alter the signal
  /// unitl the coefficients are changed.
  pub fn new() -> TwoZero {
    TwoZero {
      x_z1: 0f64,
      x_z2: 0f64,
      b0: 1f64,
      b1: 0f64,
      b2: 0f64
    }
  }

  /// Sets all filter coefficients at once.
  ///
  /// `b1` and `b2` are feedforwards, or zeroes.
  #[allow(dead_code)]
  pub fn set_coefficients(&mut self, b0: f64, b1: f64, b2: f64) {
    self.b0 = b0;
    self.b1 = b1;
    self.b2 = b2;
  }
}

impl Filter for TwoZero {
  fn tick(&mut self, sample: f64) -> f64 {
    let output = self.b0 * sample
      + self.b1 * self.x_z1 + self.b2 * self.x_z2;
    self.x_z2 = self.x_z1;
    self.x_z1 = sample;
    output
  }

  fn clear(&mut self) {
    self.x_z1 = 0f64;
    self.x_z2 = 0f64;
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
   *  x_z1 = x_z2 = 0
   *
   *  tick (and print y)
   *  y = b0 * x + b1 * x_z1 + b2 * x_z2; x_z2 = x_z1; x_z1 = x; printf("%.12f\n", y)
   *
   *  print to 12 decimal places
   *  printf("%.12f\n", y)
   */

  #[test]
  fn tick() {
    let input = vec![0.55f64, -0.55f64, 0.55f64, -0.55f64, 0.25f64];
    let expected =
      vec![
         0.495_000_000_000f64,
        -0.605_000_000_000f64,
         1.320_000_000_000f64,
        -1.320_000_000_000f64,
         1.050_000_000_000f64
      ];
    let mut two_zero = TwoZero::new();
    for sample in input.iter() {
      assert!((two_zero.tick(*sample) - sample).abs() < EPSILON);
    }
    two_zero.clear();
    two_zero.set_coefficients(0.9f64, -0.2, 1.3f64);
    for i in 0..input.len() {
      let output = two_zero.tick(input[i]);
      println!("{:.12} - {:.12} = {:.12}", expected[i], output, expected[i] - output);
      assert!((expected[i] - output).abs() < 1e-10);
    }
  }
}
