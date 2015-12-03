use std::f32::consts::PI;
use filter::Biquad;

/// A band-pass biquad filter.
///
/// This filter has a constant skirt gain with the peak gain controlled by the
/// Q factor.
pub struct BandPass1 {
  biquad: Biquad
}

impl BandPass1 {
  /// Creates a new `BandPass1` biquad filter.
  pub fn new() -> Self {
    BandPass1 {
      biquad: Biquad::new()
    }
  }

  /// Set filter coefficients.
  ///
  /// `Biquad` coefficients are calculated from the `sample_rate`,
  /// `center_frequency`, and `q` factor. These values are not
  /// validated.
  // TODO: Explain value ranges of parameters
  pub fn set_coefficients(&mut self,
                          sample_rate: f32,
                          center_frequency: f32,
                          q: f32)
  {
    let w0 = 2f32 * PI * center_frequency / sample_rate;
    let cos_w0  = w0.cos();
    let alpha   = w0.sin() / (2f32 * q);

    let mut b0  =  q * alpha;
    let mut b1  =  0f32;
    let mut b2  = -q * alpha;
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

/// A band-pass biquad filter.
///
/// This filter has a constant peak gain at 0db.
pub struct BandPass2 {
  biquad: Biquad
}

impl BandPass2 {
  /// Creates a new `BandPass2` biquad filter.
  pub fn new() -> Self {
    BandPass2 {
      biquad: Biquad::new()
    }
  }

  /// Set filter coefficients.
  ///
  /// `Biquad` coefficients are calculated from the `sample_rate`,
  /// `center_frequency`, and `q` factor. These values are not
  /// validated.
  // TODO: Explain value ranges of parameters
  pub fn set_coefficients(&mut self,
                          sample_rate: f32,
                          center_frequency: f32,
                          q: f32)
  {
    let w0 = 2f32 * PI * center_frequency / sample_rate;
    let cos_w0  = w0.cos();
    let alpha   = w0.sin() / (2f32 * q);

    let mut b0  =  alpha;
    let mut b1  =  0f32;
    let mut b2  = -alpha;
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
