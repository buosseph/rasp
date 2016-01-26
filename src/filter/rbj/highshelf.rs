use num;
use num::traits::Float;

use filter::Biquad2;
use traits::{FloatConst, Processor};

/// A high-shelf biquad filter.
#[repr(C)]
pub struct HighShelf<T> {
  biquad: Biquad2<T>
}

impl<T> HighShelf<T> where T: Float + FloatConst {
  /// Creates a new `HighShelf` biquad filter.
  pub fn new() -> Self {
    HighShelf {
      biquad: Biquad2::<T>::new()
    }
  }

  /// Set filter coefficients.
  ///
  /// `Biquad2` coefficients are calculated from the `sample_rate`,
  /// `cutoff_frequency`, `db_gain`, and `shelf_slope` factor. These values
  /// are not validated.
  // TODO: Explain value ranges of parameters
  pub fn set_coefficients(&mut self,
                          sample_rate: T,
                          cutoff_frequency: T,
                          db_gain: T,
                          shelf_slope: T)
  {
    let one: T = T::one();
    let two: T = T::two();
    let ten: T = num::cast(10f64).unwrap();
    let forty: T = num::cast(40f64).unwrap();

    let a  = ten.powf(db_gain / forty);
    let w0 = two * T::pi() * cutoff_frequency / sample_rate;
    let cos_w0 = w0.cos();
    let alpha = w0.sin() / two
              * ((a + one/a) * (one/shelf_slope - one) + two).sqrt();

    let a_plus_one = a + one;
    let a_subt_one = a - one;
    let sqrt_product = two * a.sqrt() * alpha;

    let mut b0  =        a * (a_plus_one + a_subt_one * cos_w0 + sqrt_product);
    let mut b1  = -two * a * (a_subt_one + a_plus_one * cos_w0);
    let mut b2  =        a * (a_plus_one + a_subt_one * cos_w0 - sqrt_product);
    let     a0  =             a_plus_one - a_subt_one * cos_w0 + sqrt_product;
    let mut a1  =      two * (a_subt_one - a_plus_one * cos_w0);
    let mut a2  =             a_plus_one - a_subt_one * cos_w0 - sqrt_product;

    b0 = b0 / a0;
    b1 = b1 / a0;
    b2 = b2 / a0;
    a1 = a1 / a0;
    a2 = a2 / a0;

    self.biquad.set_coefficients(b0, b1, b2, a1, a2);
    self.clear();
  }
}

impl<T> Processor<T> for HighShelf<T> where T: Float {
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
