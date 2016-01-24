use num::traits::Float;

use filter::Biquad2;
use traits::{Filter, FloatConst};

/// A high-pass biquad filter.
pub struct HighPass<T> {
  biquad: Biquad2<T>
}

impl<T> HighPass<T> where T: Float + FloatConst {
  /// Creates a new `HighPass` biquad filter.
  pub fn new() -> Self {
    HighPass {
      biquad: Biquad2::<T>::new()
    }
  }

  /// Set filter coefficients.
  ///
  /// `Biquad2` coefficients are calculated from the `sample_rate`,
  /// `cutoff_frequency`, and `q` factor. These values are not
  /// validated.
  // TODO: Explain value ranges of parameters
  pub fn set_coefficients(&mut self,
                          sample_rate: T,
                          cutoff_frequency: T,
                          q: T)
  {
    let one: T = T::one();
    let two: T = T::two();

    let w0 = two * T::pi() * cutoff_frequency / sample_rate;
    let cos_w0  = w0.cos();
    let alpha   = w0.sin() / (two * q);

    let mut b0  = (one + cos_w0) / two;
    let mut b1  = -one - cos_w0;
    let mut b2  =  b0;
    let     a0  =  one + alpha;
    let mut a1  = -two * cos_w0;
    let mut a2  =  one - alpha;

    b0 = b0 / a0;
    b1 = b1 / a0;
    b2 = b2 / a0;
    a1 = a1 / a0;
    a2 = a2 / a0;

    self.biquad.set_coefficients(b0, b1, b2, a1, a2);
    self.clear();
  }
}

impl<T> Filter<T> for HighPass<T> where T: Float {
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

#[cfg(test)]
mod tests {
  use super::*;
  use std::f32::EPSILON;
  use std::f32::consts::PI;
  use ::traits::Filter;

  #[test]
  fn new() {
    let sample_rate = 44_100f32;
    let cutoff = 1_200f32;
    let q = 1f32;

    let mut filter = HighPass::new();
    filter.set_coefficients(sample_rate, cutoff, q);

    let w0      = 2f32 * PI * cutoff / sample_rate;
    let cos_w0  = w0.cos();
    let alpha   = w0.sin() / (2f32 * q);

    assert!(( 0.170_971_028_767f32 - w0).abs()               <= EPSILON);
    assert!(( 0.985_420_021_355f32 - cos_w0).abs()           <= EPSILON);
    assert!(( 0.085_069_650_158f32 - alpha).abs()            <= EPSILON);
    assert!(( 0.914_881_372_392f32 - filter.biquad.b0).abs() <= EPSILON);
    assert!((-1.829_762_744_784f32 - filter.biquad.b1).abs() <= EPSILON);
    assert!(( 0.914_881_372_392f32 - filter.biquad.b2).abs() <= EPSILON);
    assert!((-1.816_325_839_012f32 - filter.biquad.a1).abs() <= EPSILON);
    assert!(( 0.843_199_650_555f32 - filter.biquad.a2).abs() <= EPSILON);
  }

  #[test]
  fn tick() {
    let input = vec![0.5f32, 0.4f32, 0.3f32, 0.2f32, 0.1f32];
    let expected =
      vec![
         0.216_146_415_734f32,
        -0.149_242_684_917f32,
        -0.123_518_332_014f32,
        -0.030_156_347_137f32,
         0.011_764_215_888f32
      ];

    let mut filter = HighPass::new();

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
