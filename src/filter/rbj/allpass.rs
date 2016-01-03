use num::traits::Float;

use filter::Biquad;
use traits::{Filter, FloatConst};

/// An all-pass biquad filter.
pub struct AllPass<T> {
  biquad: Biquad<T>
}

impl<T> AllPass<T> where T: Float + FloatConst {
  /// Creates a new `AllPass` biquad filter.
  pub fn new() -> Self {
    AllPass {
      biquad: Biquad::<T>::new()
    }
  }

  /// Set filter coefficients.
  ///
  /// `Biquad` coefficients are calculated from the `sample_rate`,
  /// `phase_frequency`, and `q` factor. These values are not
  /// validated.
  // TODO: Explain value ranges of parameters
  pub fn set_coefficients(&mut self,
                          sample_rate: T,
                          phase_frequency: T,
                          q: T)
  {
    let one: T = T::one();
    let two: T = T::two();

    let w0 = two * T::pi() * phase_frequency / sample_rate;
    let cos_w0  = w0.cos();
    let alpha   = w0.sin() / (two * q);

    let mut b0  =  one - alpha;
    let mut b1  = -two * cos_w0;
    let mut b2  =  one + alpha;
    let     a0  =  b2;
    let mut a1  =  b1;
    let mut a2  =  b0;

    b0 = b0 / a0;
    b1 = b1 / a0;
    b2 = b2 / a0;
    a1 = a1 / a0;
    a2 = a2 / a0;

    self.biquad.set_coefficients(b0, b1, b2, a1, a2);
    self.clear();
  }
}

impl<T> Filter<T> for AllPass<T> where T: Float {
  fn tick(&mut self, sample: T) -> T {
    self.biquad.tick(sample)
  }

  fn clear(&mut self) {
    self.biquad.clear();
  }

  fn last_out(&self) -> T {
    self.biquad.last_out()
  }
}
