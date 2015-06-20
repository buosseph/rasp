use Filter;
use std::f64::consts::PI;
use filter::biquad::Biquad;
use filter::biquad::{
  MIN_SAMPLE_RATE,
  MIN_FREQUENCY,
  MIN_Q
};

/// Highpass biquad filter.
pub struct Highpass {
  sample_rate: f64,
  cutoff: f64,
  q: f64,
  biquad: Biquad
}

impl Highpass {
  /// Constructs a new `Highpass`.
  ///
  /// Unlike a `Biquad`, the coefficients
  /// are immediately calculated.
  pub fn new(sample_rate: f64, cutoff: f64, q: f64) -> Self {
    let mut hpf =
      Highpass {
        sample_rate: sample_rate,
        cutoff: cutoff,
        q: q,
        biquad: Biquad::new()
      };
    hpf.update_coefficients();
    hpf
  }

  /// Sets the filter sample rate.
  ///
  /// Highpass filters require knowledge
  /// of the sample rate of the audio
  /// input in order to calculate
  /// the correct coefficients.
  ///
  /// The sample rate must be a positive non-zero value.
  /// If not, the value is clipped.
  pub fn set_sample_rate(&mut self, new_sample_rate: f64) {
    let mut fs = new_sample_rate;
    if fs < MIN_SAMPLE_RATE {
      fs = MIN_SAMPLE_RATE;
    }
    self.sample_rate = fs;
    self.update_coefficients();
  }

  /// Sets the filter frequency cutoff.
  ///
  /// The frequency must satisfy `0 Hz <= cutoff <= Fs/2`
  /// where `Fs/2` is the Nyquist frequency, or 
  /// half the sample rate of the input audio.
  /// If not, the value will be clipped.
  pub fn set_cutoff(&mut self, new_cutoff: f64) {
    let mut fc = new_cutoff;
    if fc < MIN_FREQUENCY {
      fc = MIN_FREQUENCY;
    }
    if fc > self.sample_rate / 2f64 {
      fc = self.sample_rate / 2f64;
    }
    self.cutoff = fc;
    self.update_coefficients();
  }

  /// Sets the filter Q value.
  ///
  /// The Q value must be a positive non-zero value.
  /// If not, the value is clipped.
  pub fn set_q(&mut self, new_q: f64) {
    let mut _q = new_q;
    if _q < MIN_Q { _q = MIN_Q; }
    self.q = _q;
    self.update_coefficients();
  }

  /// Returns the sample rate of the audio
  /// passed through the filter.
  pub fn sample_rate(&self) -> f64 { self.sample_rate }

  /// Returns the frequency cutoff of the filter.
  pub fn cutoff(&self) -> f64 { self.cutoff }

  /// Returns the Q value of the filter.
  pub fn q(&self) -> f64 { self.q }

  /// Updates `Biquad` coefficients.
  ///
  /// `Biquad` coefficients are calculated
  /// from the `sample_rate`, `cutoff`, and `q`.
  fn update_coefficients(&mut self) {
    let w0 = 2f64 * PI * self.cutoff / self.sample_rate;
    let cos_w0  = w0.cos();
    let alpha   = w0.sin() / (2f64 * self.q);
    let mut b0  = (1f64 + cos_w0) / 2f64;
    let mut b1  = -1f64 - cos_w0;
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

impl Filter for Highpass {
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
  use filter::biquad::{
    MIN_SAMPLE_RATE,
    MIN_FREQUENCY,
    MIN_Q
  };
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
   *  a0 = 1 + alpha; b0 = ((1+cos_w0)/2)/a0; b1 = -(1+cos_w0)/a0; b2 = b0; a1 = (-2*cos_w0)/a0; a2 = (1-alpha)/a0;
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
    let hpf = Highpass::new(44_100f64, 1_200f64, 1f64);
    assert!((hpf.sample_rate - 44_100f64).abs() < 1e-10);
    assert!((hpf.cutoff - 1_200f64      ).abs() < 1e-10);
    assert!((hpf.q - 1f64               ).abs() < 1e-10);
    let w0      = 2f64 * PI * hpf.cutoff / hpf.sample_rate;
    let cos_w0  = w0.cos();
    let alpha   = w0.sin() / (2f64 * hpf.q);
    assert!(( 0.170_971_028_767f64 - w0            ).abs() < 1e-10);
    assert!(( 0.985_420_021_355f64 - cos_w0        ).abs() < 1e-10);
    assert!(( 0.085_069_650_158f64 - alpha         ).abs() < 1e-10);
    assert!(( 0.914_881_372_392f64 - hpf.biquad.b0 ).abs() < 1e-10);
    assert!((-1.829_762_744_784f64 - hpf.biquad.b1 ).abs() < 1e-10);
    assert!(( 0.914_881_372_392f64 - hpf.biquad.b2 ).abs() < 1e-10);
    assert!((-1.816_325_839_012f64 - hpf.biquad.a1 ).abs() < 1e-10);
    assert!(( 0.843_199_650_555f64 - hpf.biquad.a2 ).abs() < 1e-10);
  }

  #[test]
  fn tick() {
    let input = vec![0.5f64, 0.4f64, 0.3f64, 0.2f64, 0.1f64];
    let mut hpf = Highpass::new(44_100f64, 8_000f64, 0.71f64);
    let expected =
      vec![
         0.216_146_415_734f64,
        -0.149_242_684_917f64,
        -0.123_518_332_014f64,
        -0.030_156_347_137f64,
         0.011_764_215_888f64
      ];
    let mut actual: f64;
    let mut abs_diff: f64;
    for i in 0..input.len() {
      actual = hpf.tick(input[i]);
      abs_diff = (expected[i] - actual).abs();
      assert!(abs_diff < 1e-10);
    }
  }

  #[test]
  fn accessors() {
    let mut filter = Highpass::new(44_100f64, 8_000f64, 0.71f64);
    filter.set_sample_rate(-2_000f64);
    assert_eq!(MIN_SAMPLE_RATE, filter.sample_rate());
    filter.set_sample_rate(192_000f64);
    assert_eq!(192_000f64, filter.sample_rate());
    filter.set_cutoff(-20f64);
    assert_eq!(MIN_FREQUENCY, filter.cutoff());
    filter.set_cutoff(200_000f64);
    assert_eq!(192_000f64 / 2f64, filter.cutoff());
    filter.set_q(-10f64);
    assert_eq!(MIN_Q, filter.q());
    filter.set_q(4f64);
    assert_eq!(4f64, filter.q());
  }
}