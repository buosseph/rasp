use num;
use num::traits::Float;

use traits::Processor;

/// A single channel, one pole digital filter.
///
/// A `OnePole` filter uses the following equation:
///
/// `y[n] = b0*x[n] - a1*y[n-1]`
///
/// It has one feedback coefficient, `a1`.
#[repr(C)]
pub struct OnePole<T: Float> {
  y_z1: T,
  pub b0: T,
  pub a1: T
}

impl<T> OnePole<T> where T: Float {
  /// Creates a new `OnePole` filter.
  ///
  /// The filter will be initalized in a state that does not alter the input
  /// signal.
  ///
  /// # Examples
  ///
  /// ```
  /// # #![allow(unused_mut)]
  /// use rasp::filter::OnePole;
  ///
  /// let mut filter1: OnePole<f32> = OnePole::new();
  /// let mut filter2: OnePole<f64> = OnePole::new();
  /// let mut filter3 = OnePole::<f32>::new();
  /// let mut filter4 = OnePole::<f64>::new();
  /// ```
  pub fn new() -> Self {
    OnePole {
      y_z1: num::zero(),
      b0: num::one(),
      a1: num::zero()
    }
  }

  /// Sets all filter coefficients at once.
  ///
  /// `a1` is a feedback, or pole.
  pub fn set_coefficients(&mut self, b0: T, a1: T) {
    self.b0 = b0;
    self.a1 = a1;
  }
}

impl<T> Processor<T> for OnePole<T> where T: Float {
  fn process(&mut self, sample: T) -> T {
    let output = self.b0 * sample - self.a1 * self.y_z1;
    self.y_z1 = output;
    output
  }

  fn clear(&mut self) {
    self.y_z1 = num::zero();
  }

  fn last_out(&self) -> T {
    self.y_z1
  }
}

#[allow(dead_code)]
mod ffi {
  use super::*;
  use libc::c_float;
  use ::traits::Processor;

  #[no_mangle]
  pub extern fn filter_one_pole_new() -> *mut OnePole<c_float> {
    // Heap allocation
    Box::into_raw(Box::new(OnePole::<c_float>::new()))
  }

  #[no_mangle]
  pub extern fn filter_one_pole_destroy(ptr: *mut OnePole<c_float>) {
    if ptr.is_null() { return }
    unsafe { Box::from_raw(ptr); }
    // Drop
  }

  // How do you handle generics in an ffi?
  #[no_mangle]
  pub extern fn filter_one_pole_set_coefficients(ptr: *mut OnePole<c_float>, b0: c_float, a1: c_float) {
    let mut filter = unsafe {
      assert!(!ptr.is_null());
      &mut *ptr
    };
    filter.set_coefficients(b0, a1);
  }

  #[no_mangle]
  pub extern fn filter_one_pole_process(ptr: *mut OnePole<c_float>, sample: c_float) -> c_float {
    let mut filter = unsafe {
      assert!(!ptr.is_null());
      &mut *ptr
    };
    filter.process(sample)
  }

  #[no_mangle]
  pub extern fn filter_one_pole_clear(ptr: *mut OnePole<c_float>) {
    let mut filter = unsafe {
      assert!(!ptr.is_null());
      &mut *ptr
    };
    filter.clear();
  }

  #[no_mangle]
  pub extern fn filter_one_pole_last_out(ptr: *mut OnePole<c_float>) -> c_float {
    let filter = unsafe {
      assert!(!ptr.is_null());
      & *ptr
    };
    filter.last_out()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::f32::EPSILON;
  use ::traits::Processor;

  #[test]
  fn process() {
    let input = vec![0.55f32, -0.55f32, 0.55f32, -0.55f32, 0.25f32];
    let expected =
      vec![
         0.275_000_000_000f32,
        -0.302_500_000_000f32,
         0.305_250_000_000f32,
        -0.305_525_000_000f32,
         0.155_552_500_000f32
      ];
    let mut filter = OnePole::new();

    for sample in input.iter() {
      assert!((filter.process(*sample) - sample).abs() < EPSILON);
    }

    filter.clear();
    filter.set_coefficients(0.5f32, 0.1f32);

    for i in 0..input.len() {
      let output = filter.process(input[i]);
      println!("{:.12} - {:.12} = {:.12}", expected[i], output, expected[i] - output);
      assert!((expected[i] - output).abs() < EPSILON);
    }
  }

  #[test]
  fn process_block() {
    let input = vec![0.55f32, -0.55f32, 0.55f32, -0.55f32, 0.25f32];
    let expected =
      vec![
         0.275_000_000_000f32,
        -0.302_500_000_000f32,
         0.305_250_000_000f32,
        -0.305_525_000_000f32,
         0.155_552_500_000f32
      ];
    let mut filter = OnePole::new();

    let mut initial_input = input.clone();
    let mut last_processed = filter.process_block(&mut initial_input);
    assert!((last_processed - input.last().unwrap()).abs() < EPSILON);

    filter.clear();
    filter.set_coefficients(0.5f32, 0.1f32);

    let mut actual = input.clone();
    last_processed = filter.process_block(&mut actual);
    assert!((last_processed - expected.last().unwrap()).abs() < EPSILON);

    for i in 0..input.len() {
      println!("{:.12} - {:.12} = {:.12}", expected[i], actual[i], expected[i] - actual[i]);
      assert!((expected[i] - actual[i]).abs() < EPSILON);
    }
  }
}
