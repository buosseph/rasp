use std::f32::consts::PI;
use filter::Biquad;

/// An all-pass biquad filter.
pub struct AllPass {
  biquad: Biquad
}

impl AllPass {
  /// Creates a new `AllPass` biquad filter.
  pub fn new() -> Self {
    AllPass {
      biquad: Biquad::new()
    }
  }

  /// Set filter coefficients.
  ///
  /// `Biquad` coefficients are calculated from the `sample_rate`,
  /// `phase_frequency`, and `q` factor. These values are not
  /// validated.
  // TODO: Explain value ranges of parameters
  pub fn set_coefficients(&mut self,
                          sample_rate: f32,
                          phase_frequency: f32,
                          q: f32)
  {
    let w0 = 2f32 * PI * phase_frequency / sample_rate;
    let cos_w0  = w0.cos();
    let alpha   = w0.sin() / (2f32 * q);

    let mut b0  =  1f32 - alpha;
    let mut b1  = -2f32 * cos_w0;
    let mut b2  =  1f32 + alpha;
    let     a0  =  b2;
    let mut a1  =  b1;
    let mut a2  =  b0;

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
