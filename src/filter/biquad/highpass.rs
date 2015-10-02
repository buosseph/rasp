use DspComponent;
use Filter;
use std::f32::consts::PI;
use filter::biquad::Biquad;
use filter::biquad::{
  MIN_SAMPLE_RATE,
  MIN_FREQUENCY,
  MIN_Q
};

/// Highpass biquad filter.
pub struct Highpass {
  sample_rate: f32,
  cutoff: f32,
  q: f32,
  biquad: Biquad
}

impl Highpass {
  /// Constructs a new `Highpass`.
  ///
  /// Unlike a `Biquad`, the coefficients
  /// are immediately calculated.
  pub fn new(sample_rate: f32, cutoff: f32, q: f32) -> Self {
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
  pub fn set_sample_rate(&mut self, new_sample_rate: f32) {
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
  pub fn set_cutoff(&mut self, new_cutoff: f32) {
    let mut fc = new_cutoff;
    if fc < MIN_FREQUENCY {
      fc = MIN_FREQUENCY;
    }
    if fc > self.sample_rate / 2f32 {
      fc = self.sample_rate / 2f32;
    }
    self.cutoff = fc;
    self.update_coefficients();
  }

  /// Sets the filter Q value.
  ///
  /// The Q value must be a positive non-zero value.
  /// If not, the value is clipped.
  pub fn set_q(&mut self, new_q: f32) {
    let mut _q = new_q;
    if _q < MIN_Q { _q = MIN_Q; }
    self.q = _q;
    self.update_coefficients();
  }

  /// Returns the sample rate of the audio
  /// passed through the filter.
  pub fn sample_rate(&self) -> f32 { self.sample_rate }

  /// Returns the frequency cutoff of the filter.
  pub fn cutoff(&self) -> f32 { self.cutoff }

  /// Returns the Q value of the filter.
  pub fn q(&self) -> f32 { self.q }

  /// Updates `Biquad` coefficients.
  ///
  /// `Biquad` coefficients are calculated
  /// from the `sample_rate`, `cutoff`, and `q`.
  fn update_coefficients(&mut self) {
    let w0 = 2f32 * PI * self.cutoff / self.sample_rate;
    let cos_w0  = w0.cos();
    let alpha   = w0.sin() / (2f32 * self.q);
    let mut b0  = (1f32 + cos_w0) / 2f32;
    let mut b1  = -1f32 - cos_w0;
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
}

impl DspComponent for Highpass {
  fn new() -> Highpass {
    Highpass::new(44_100f32, 20_000f32, 0.71)
  }

  fn tick(&mut self, sample: f32) -> f32 {
    self.biquad.tick(sample)
  }
}

impl Filter for Highpass {
  fn clear(&mut self) {
    self.biquad.clear();
  }

  fn last_out(&self) -> f32 {
    self.biquad.last_out()
  }
}

#[cfg(test)]
mod tests {
  use DspComponent;
  use Filter;
  use std::f32::consts::PI;
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
    let hpf = Highpass::new(44_100f32, 1_200f32, 1f32);
    assert!((hpf.sample_rate - 44_100f32).abs() < 1e-10);
    assert!((hpf.cutoff - 1_200f32      ).abs() < 1e-10);
    assert!((hpf.q - 1f32               ).abs() < 1e-10);
    let w0      = 2f32 * PI * hpf.cutoff / hpf.sample_rate;
    let cos_w0  = w0.cos();
    let alpha   = w0.sin() / (2f32 * hpf.q);
    assert!(( 0.170_971_028_767f32 - w0            ).abs() < 1e-10);
    assert!(( 0.985_420_021_355f32 - cos_w0        ).abs() < 1e-10);
    assert!(( 0.085_069_650_158f32 - alpha         ).abs() < 1e-10);
    assert!(( 0.914_881_372_392f32 - hpf.biquad.b0 ).abs() < 1e-10);
    assert!((-1.829_762_744_784f32 - hpf.biquad.b1 ).abs() < 1e-10);
    assert!(( 0.914_881_372_392f32 - hpf.biquad.b2 ).abs() < 1e-10);
    assert!((-1.816_325_839_012f32 - hpf.biquad.a1 ).abs() < 1e-10);
    assert!(( 0.843_199_650_555f32 - hpf.biquad.a2 ).abs() < 1e-10);
  }

  #[test]
  fn tick() {
    let input = vec![0.5f32, 0.4f32, 0.3f32, 0.2f32, 0.1f32];
    let mut hpf = Highpass::new(44_100f32, 8_000f32, 0.71f32);
    let expected =
      vec![
         0.216_146_415_734f32,
        -0.149_242_684_917f32,
        -0.123_518_332_014f32,
        -0.030_156_347_137f32,
         0.011_764_215_888f32
      ];
    let mut actual: f32;
    let mut abs_diff: f32;
    for i in 0..input.len() {
      actual = hpf.tick(input[i]);
      abs_diff = (expected[i] - actual).abs();
      assert!(abs_diff < 1e-10);
    }
  }

  #[test]
  fn accessors() {
    let mut filter = Highpass::new(44_100f32, 8_000f32, 0.71f32);
    filter.set_sample_rate(-2_000f32);
    assert_eq!(MIN_SAMPLE_RATE, filter.sample_rate());
    filter.set_sample_rate(192_000f32);
    assert_eq!(192_000f32, filter.sample_rate());
    filter.set_cutoff(-20f32);
    assert_eq!(MIN_FREQUENCY, filter.cutoff());
    filter.set_cutoff(200_000f32);
    assert_eq!(192_000f32 / 2f32, filter.cutoff());
    filter.set_q(-10f32);
    assert_eq!(MIN_Q, filter.q());
    filter.set_q(4f32);
    assert_eq!(4f32, filter.q());
  }
}