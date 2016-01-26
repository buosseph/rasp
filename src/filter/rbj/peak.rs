use num;
use num::traits::Float;

use filter::Biquad2;
use traits::{FloatConst, Processor};

/// A peaking biquad filter.
pub struct Peak<T> {
  biquad: Biquad2<T>
}

impl<T> Peak<T> where T: Float + FloatConst {
  /// Creates a new `Peak` biquad filter.
  pub fn new() -> Self {
    Peak {
      biquad: Biquad2::new()
    }
  }

  /// Set filter coefficients.
  ///
  /// `Biquad2` coefficients are calculated from the `sample_rate`,
  /// `center_frequency`, `db_gain`, and `q` factor. These values are not
  /// validated.
  // TODO: Explain value ranges of parameters
  pub fn set_coefficients(&mut self,
                          sample_rate: T,
                          center_frequency: T,
                          db_gain: T,
                          q: T)
  {
    let one: T = T::one();
    let two: T = T::two();
    let ten: T = num::cast(10f64).unwrap();
    let forty: T = num::cast(40f64).unwrap();

    let a  = ten.powf(db_gain / forty);
    let w0 = two * T::pi() * center_frequency / sample_rate;
    let cos_w0  = w0.cos();
    let alpha   = w0.sin() / (two * q);

    let mut b0  =  one + alpha * a;
    let mut b1  = -two * cos_w0;
    let mut b2  =  one - alpha * a;
    let     a0  =  one + alpha / a;
    let mut a1  =  b1;
    let mut a2  =  one - alpha / a;

    b0 = b0 / a0;
    b1 = b1 / a0;
    b2 = b2 / a0;
    a1 = a1 / a0;
    a2 = a2 / a0;

    self.biquad.set_coefficients(b0, b1, b2, a1, a2);
    self.clear();
  }
}

impl<T> Processor<T> for Peak<T> where T: Float {
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
