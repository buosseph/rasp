use DspComponent;
use Filter;

/// Single channel, two pole digital filter.
///
/// A `TwoPole` is a type of `Filter` that uses the following equation:
///
/// > y[n] = b0*x[n] - a1*y[n-1] - a2*x[n-2]
///
/// It has two feedback coefficients `a1` and `a2`. 
pub struct TwoPole {
  y_z1: f32,
  y_z2: f32,
  pub b0: f32,
  pub a1: f32,
  pub a2: f32
}

impl TwoPole {
  /// Sets all filter coefficients at once.
  ///
  /// `a1` and `a2` are feedbacks, or poles.
  pub fn set_coefficients(&mut self, b0: f32, a1: f32, a2: f32) {
    self.b0 = b0;
    self.a1 = a1; self.a2 = a2;
  }
}

impl DspComponent for TwoPole {
  fn new() -> TwoPole {
    TwoPole {
      y_z1: 0f32, y_z2: 0f32,
      b0: 1f32,
      a1: 0f32, a2: 0f32
    }
  }

  fn tick(&mut self, sample: f32) -> f32 {
    let output = self.b0 * sample
      - self.a1 * self.y_z1 - self.a2 * self.y_z2;
    self.y_z2 = self.y_z1;
    self.y_z1 = output;
    output
  }
}

impl Filter for TwoPole {
  fn clear(&mut self) {
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
   *  clear
   *  y_z1 = y_z2 = 0
   *
   *  tick (and print y)
   *  y = b0 * x - a1 * y_z1 - a2 * y_z2; y_z2 = y_z1; y_z1 = y; printf("%.12f\n", y)
   *
   *  print to 12 decimal places
   *  printf("%.12f\n", y)
   */

  #[test]
  fn tick() {
    let input = vec![0.55f32, -0.55f32, 0.55f32, -0.55f32, 0.25f32];
    let expected =
      vec![
         0.495_000_000_000f32,
        -0.594_000_000_000f32,
         1.257_300_000_000f32,
        -1.518_660_000_000f32,
         2.163_222_000_000f32
      ];
    let mut two_pole = TwoPole::new();
    for sample in input.iter() {
      assert!((two_pole.tick(*sample) - sample).abs() < EPSILON);
    }
    two_pole.clear();
    two_pole.set_coefficients(0.9f32, 0.2, -1.3f32);
    for i in 0..input.len() {
      let output = two_pole.tick(input[i]);
      println!("{:.12} - {:.12} = {:.12}", expected[i], output, expected[i] - output);
      assert!((expected[i] - output).abs() < 1e-10);
    }
  }
}
