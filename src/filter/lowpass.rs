use Filter;
use filter::biquad::Biquad;
use std::f64::consts::PI;

/// Lowpass biquad filter.
pub struct Lowpass {
  pub sample_rate: f64,
  pub cutoff: f64,
  pub q: f64,
  biquad: Biquad
}

impl Lowpass {
  /// Constructs a new `Lowpass`.
  ///
  /// The filter will not alter the signal
  /// unitl the coefficients are changed.
  pub fn new(sample_rate: f64, cutoff: f64, q: f64) -> Self {
    let mut lpf =
      Lowpass {
        sample_rate: sample_rate,
        cutoff: cutoff,
        q: q,
        biquad: Biquad::new()
      };
    lpf.update_coefficients();
    lpf
  }

  /// Updates `Biquad` coefficients.
  ///
  /// `Biquad` coefficients are
  /// calculated from the `sample_rate`,
  /// `cutoff`, and `q`.
  pub fn update_coefficients(&mut self) {
    let w0 = 2f64 * PI * self.cutoff / self.sample_rate;
    let cos_w0  = w0.cos();
    let alpha   = w0.sin() / (2f64 * self.q);
    let mut b0  = (1f64 - cos_w0) / 2f64;
    let mut b1  =  1f64 - cos_w0;
    let mut b2  =  b0;
    let     a0  =  1f64 + alpha;
    let mut a1  = -2f64 * cos_w0;
    let mut a2  =  1f64 - alpha;
    b0 /= a0;
    b1 /= a0;
    b2 /= a0;
    a1 /= a0;
    a2 /= a0;
    self.biquad.set_coefficients(b0, b1, b2, a1, a2);
  }
}

impl Filter for Lowpass {
  fn tick(&mut self, sample: f64) -> f64 {
    self.biquad.tick(sample)
  }

  fn clear(&mut self) {
    self.biquad.clear();
  }
}

#[cfg(test)]
mod tests {
  use Filter;
  use std::f64::consts::PI;
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
   */

  #[test]
  fn new() {
    let lpf = Lowpass::new(44_100f64, 1_200f64, 1f64);
    assert!((lpf.sample_rate - 44_100f64).abs() < 1e-10);
    assert!((lpf.cutoff - 1_200f64      ).abs() < 1e-10);
    assert!((lpf.q - 1f64               ).abs() < 1e-10);
    let w0      = 2f64 * PI * lpf.cutoff / lpf.sample_rate;
    let cos_w0  = w0.cos();
    let alpha   = w0.sin() / (2f64 * lpf.q);
    assert!(( 0.170_971_028_767f64 - w0            ).abs() < 1e-10);
    assert!(( 0.985_420_021_355f64 - cos_w0        ).abs() < 1e-10);
    assert!(( 0.085_069_650_158f64 - alpha         ).abs() < 1e-10);
    assert!(( 0.006_718_452_886f64 - lpf.biquad.b0 ).abs() < 1e-10);
    assert!(( 0.013_436_905_772f64 - lpf.biquad.b1 ).abs() < 1e-10);
    assert!(( 0.006_718_452_886f64 - lpf.biquad.b2 ).abs() < 1e-10);
    assert!((-1.816_325_839_012f64 - lpf.biquad.a1 ).abs() < 1e-10);
    assert!(( 0.843_199_650_555f64 - lpf.biquad.a2 ).abs() < 1e-10);
  }

  #[test]
  fn tick() {
    let input = vec![0.5f64, 0.4f64, 0.3f64, 0.2f64, 0.1f64];
    let mut lowpass = Lowpass::new(44_100f64, 8_000f64, 0.71f64);
    let expected =
      vec![
        0.088_763_995_825f64,
        0.293_767_078_666f64,
        0.414_231_561_951f64,
        0.359_573_380_268f64,
        0.234_253_200_384f64
      ];
    let mut actual: f64;
    let mut abs_diff: f64;
    for i in 0..input.len() {
      actual = lowpass.tick(input[i]);
      abs_diff = (expected[i] - actual).abs();
      assert!(abs_diff < 1e-10);
    }
  }
}