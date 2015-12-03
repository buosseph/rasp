use std::f32::consts::PI;
use filter::Biquad;

/// A low-pass biquad filter.
pub struct LowPass {
  biquad: Biquad
}

impl LowPass {
  /// Creates a new `LowPass` biquad filter.
  pub fn new() -> Self {
    LowPass {
      biquad: Biquad::new()
    }
  }

  /// Set filter coefficients.
  ///
  /// `Biquad` coefficients are calculated from the `sample_rate`,
  /// `cutoff_frequency`, and `q` factor. These values are not
  /// validated.
  // TODO: Explain value ranges of parameters
  pub fn set_coefficients(&mut self,
                          sample_rate: f32,
                          cutoff_frequency: f32,
                          q: f32)
  {
    let w0 = 2f32 * PI * cutoff_frequency / sample_rate;
    let cos_w0  = w0.cos();
    let alpha   = w0.sin() / (2f32 * q);

    let mut b0  = (1f32 - cos_w0) / 2f32;
    let mut b1  =  1f32 - cos_w0;
    let mut b2  =  b0;
    let     a0  =  1f32 + alpha;
    let mut a1  = -2f32 * cos_w0;
    let mut a2  =  1f32 - alpha;

    b0 /= a0;
    b1 /= a0;
    b2 /= a0;
    a1 /= a0;
    a2 /= a0;

    self.biquad.set_coefficients(b0, b1, b2, a1, a2);
  }

  /// Processes and stores input sample into memory and outputs calculated
  /// sample.
  pub fn tick(&mut self, sample: f32) -> f32 {
    self.biquad.tick(sample)
  }

  /// Resets memory of all previous input and output to zero.
  pub fn clear(&mut self) {
    self.biquad.clear();
  }

  /// Returns the last computed output sample.
  pub fn last_out(&self) -> f32 {
    self.biquad.last_out()
  }
}

#[cfg(test)]
mod tests {
  use std::f32::EPSILON;
  use std::f32::consts::PI;
  use super::*;

  /*
   *  Octave input used to test, print all values to 12 decimal point for use in tests
   *
   *  input, output
   *  x, y
   *
   *  calc_intermids
   *  w0 = 2 * pi * cutoff / fs; cos_w0 = cos(w0); alpha = sin(w0) / (2 * q); printf("%.12f\n", w0), printf("%.12f\n", cos_w0), printf("%.12f\n", alpha)
   *
   *  calc_coeffs
   *  a0 = 1 + alpha; b0 = ((1-cos_w0)/2)/a0; b1 = (1-cos_w0)/a0; b2 = b0; a1 = (-2*cos_w0)/a0; a2 = (1-alpha)/a0;
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
  fn new() {
    let sample_rate = 44_100f32;
    let cutoff = 1_200f32;
    let q = 1f32;

    let mut filter = LowPass::new();
    filter.set_coefficients(sample_rate, cutoff, q);

    let w0      = 2f32 * PI * cutoff / sample_rate;
    let cos_w0  = w0.cos();
    let alpha   = w0.sin() / (2f32 * q);

    assert!(( 0.170_971_028_767f32 - w0)              .abs() <= EPSILON);
    assert!(( 0.985_420_021_355f32 - cos_w0)          .abs() <= EPSILON);
    assert!(( 0.085_069_650_158f32 - alpha)           .abs() <= EPSILON);
    assert!(( 0.006_718_452_886f32 - filter.biquad.b0).abs() <= EPSILON);
    assert!(( 0.013_436_905_772f32 - filter.biquad.b1).abs() <= EPSILON);
    assert!(( 0.006_718_452_886f32 - filter.biquad.b2).abs() <= EPSILON);
    assert!((-1.816_325_839_012f32 - filter.biquad.a1).abs() <= EPSILON);
    assert!(( 0.843_199_650_555f32 - filter.biquad.a2).abs() <= EPSILON);
  }

  #[test]
  fn tick() {
    let input = vec![0.5f32, 0.4f32, 0.3f32, 0.2f32, 0.1f32];
    let expected =
      vec![
        0.088_763_995_825f32,
        0.293_767_078_666f32,
        0.414_231_561_951f32,
        0.359_573_380_268f32,
        0.234_253_200_384f32
      ];

    let mut filter = LowPass::new();

    // No signal change on initialization
    let mut actual: f32;
    for i in 0..input.len() {
      actual = filter.tick(input[i]);
      assert!((input[i] - actual).abs() <= EPSILON);
    }

    filter.clear();
    filter.set_coefficients(44_100f32, 8_000f32, 0.71f32);

    for i in 0..input.len() {
      actual = filter.tick(input[i]);
      assert!((expected[i] - actual).abs() <= EPSILON);
    }
  }
}
