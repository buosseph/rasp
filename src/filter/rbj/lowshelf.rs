use std::f32::consts::PI;
use filter::Biquad;

/// A low-shelf biquad filter.
pub struct LowShelf {
  biquad: Biquad
}

impl LowShelf {
  /// Creates a new `LowShelf` biquad filter.
  pub fn new() -> Self {
    LowShelf {
      biquad: Biquad::new()
    }
  }

  /// Set filter coefficients.
  ///
  /// `Biquad` coefficients are calculated from the `sample_rate`,
  /// `cutoff_frequency`, `db_gain`, and `shelf_slope` factor. These values
  /// are not validated.
  // TODO: Explain value ranges of parameters
  pub fn set_coefficients(&mut self,
                          sample_rate: f32,
                          cutoff_frequency: f32,
                          db_gain: f32,
                          shelf_slope: f32)
  {
    let a  = 10f32.powf(db_gain / 40f32);
    let w0 = 2f32 * PI * cutoff_frequency / sample_rate;
    let cos_w0 = w0.cos();
    let alpha = w0.sin() / 2f32
              * ((a + 1f32/a) * (1f32/shelf_slope - 1f32) + 2f32).sqrt();
    let sqrt_product = 2f32 * a.sqrt() * alpha;

    let mut b0  =        a * ((a + 1f32) - (a - 1f32) * cos_w0 + sqrt_product);
    let mut b1  = 2f32 * a * ((a - 1f32) - (a + 1f32) * cos_w0);
    let mut b2  =        a * ((a + 1f32) - (a - 1f32) * cos_w0 - sqrt_product);
    let     a0  =             (a + 1f32) + (a - 1f32) * cos_w0 + sqrt_product;
    let mut a1  =    -2f32 * ((a - 1f32) + (a + 1f32) * cos_w0);
    let mut a2  =             (a + 1f32) + (a - 1f32) * cos_w0 - sqrt_product;

    b0 /= a0;
    b1 /= a0;
    b2 /= a0;
    a1 /= a0;
    a2 /= a0;

    self.biquad.set_coefficients(b0, b1, b2, a1, a2);
    self.clear();
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
