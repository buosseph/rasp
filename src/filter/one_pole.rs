// Use this to set up an example of a one-pole lowpass and highpass
// http://www.earlevel.com/main/2012/12/15/a-one-pole-filter/
use DspComponent;
use Filter;

/// Single channel, one pole digital filter.
///
/// A `OnePole` is a type of `Filter` that uses the following equation:
/// > y[n] = b0*x[n] - a1*y[n-1]
/// It only has one feedback coefficient `a1`. 
pub struct OnePole {
  y_z1: f32,
  pub b0: f32,
  pub a1: f32
}

impl OnePole {
  /// Sets all filter coefficients at once.
  ///
  /// `a1` is a feedback, or pole.
  pub fn set_coefficients(&mut self, b0: f32, a1: f32) {
    self.b0 = b0;
    self.a1 = a1;
  }
}

impl DspComponent for OnePole {
  fn new() -> OnePole {
    OnePole {
      y_z1: 0f32,
      b0: 1f32,
      a1: 0f32
    }
  }

  fn tick(&mut self, sample: f32) -> f32 {
    let output = self.b0 * sample - self.a1 * self.y_z1;
    self.y_z1 = output;
    output
  }
}

impl Filter for OnePole {
  fn clear(&mut self) {
    self.y_z1 = 0f32;
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
