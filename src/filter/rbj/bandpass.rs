use num::traits::Float;

use filter::Biquad2;
use traits::{FloatConst, Processor};

/// A band-pass biquad filter.
///
/// This filter has a constant skirt gain with the peak gain controlled by the
/// Q factor.
#[repr(C)]
pub struct BandPass1<T> {
  biquad: Biquad2<T>
}

impl<T> BandPass1<T> where T: Float + FloatConst {
  /// Creates a new `BandPass1` biquad filter.
  pub fn new() -> Self {
    BandPass1 {
      biquad: Biquad2::<T>::new()
    }
  }

  /// Set filter coefficients.
  ///
  /// `Biquad2` coefficients are calculated from the `sample_rate`,
  /// `center_frequency`, and `q` factor. These values are not
  /// validated.
  // TODO: Explain value ranges of parameters
  pub fn set_coefficients(&mut self,
                          sample_rate: T,
                          center_frequency: T,
                          q: T)
  {
    let one: T = T::one();
    let two: T = T::two();

    let w0 = two * T::pi() * center_frequency / sample_rate;
    let cos_w0  = w0.cos();
    let alpha   = w0.sin() / (two * q);

    let mut b0  =  q * alpha;
    let mut b1  =  T::zero();
    let mut b2  = -q * alpha;
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

impl<T> Processor<T> for BandPass1<T> where T: Float {
  fn process(&mut self, sample: T) -> T {
    self.biquad.process(sample)
  }

  fn clear(&mut self) {
    self.biquad.clear();
  }

  fn last_out(&self) -> T {
    self.biquad.last_out()
  }
}

/// A band-pass biquad filter.
///
/// This filter has a constant peak gain at 0db.
#[repr(C)]
pub struct BandPass2<T> {
  biquad: Biquad2<T>
}

impl<T> BandPass2<T> where T: Float + FloatConst {
  /// Creates a new `BandPass2` biquad filter.
  pub fn new() -> Self {
    BandPass2 {
      biquad: Biquad2::<T>::new()
    }
  }

  /// Set filter coefficients.
  ///
  /// `Biquad2` coefficients are calculated from the `sample_rate`,
  /// `center_frequency`, and `q` factor. These values are not
  /// validated.
  // TODO: Explain value ranges of parameters
  pub fn set_coefficients(&mut self,
                          sample_rate: T,
                          center_frequency: T,
                          q: T)
  {
    let one: T = T::one();
    let two: T = T::two();

    let w0 = two * T::pi() * center_frequency / sample_rate;
    let cos_w0  = w0.cos();
    let alpha   = w0.sin() / (two * q);

    let mut b0  =  alpha;
    let mut b1  =  T::zero();
    let mut b2  = -alpha;
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

impl<T> Processor<T> for BandPass2<T> where T: Float {
  fn process(&mut self, sample: T) -> T {
    self.biquad.process(sample)
  }

  fn clear(&mut self) {
    self.biquad.clear();
  }

  fn last_out(&self) -> T {
    self.biquad.last_out()
  }
}
