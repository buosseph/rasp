//! A biquad is a second-order recursive filter.

use num;
use num::traits::Float;

use traits::Filter;

/* Notes on biquads
  - A biquad is a recursive second-order IIR filter and is often used as a
    building component for more complex filters
  - The direct implementation of a biquad is known as direct form I
    - It's a straight forward and easy to understand
    - This realization prevents overflow, so it's good for interger
      type signals
  - There's also direct form II and transposed direct form II
    - This realization uses less memory, but can overflow, which makes
      it good for floating point signals since values can't overflow.
*/

/// A biquad filter in direct from 1.
///
/// This implementation uses a [Direct Form I](https://en.wikipedia.org/wiki/Digital_biquad_filter#Direct_Form_1)
/// realization using the following equation:
///
/// `y[n] = b0*x[n] + b1*x[n-1] + b2*x[n-2] - a1*y[n-1] - a2*y[n-2]`
///
/// It has two feedforward coefficients, `b1` and `b2`, and two feedback
/// coefficients, `a1` and `a2`.
pub struct Biquad1<T> {
  x_z1: T,
  x_z2: T,
  y_z1: T,
  y_z2: T,
  pub b0: T,
  pub b1: T,
  pub b2: T,
  pub a1: T,
  pub a2: T
}

impl<T> Biquad1<T> where T: Float {
  /// Creates a new `Biquad1` filter.
  ///
  /// The filter will be initalized in a state that does not alter the input
  /// signal.
  ///
  /// # Examples
  ///
  /// ```
  /// # #![allow(unused_mut)]
  /// use rasp::filter::Biquad1;
  ///
  /// let mut filter1: Biquad1<f32> = Biquad1::new();
  /// let mut filter2: Biquad1<f64> = Biquad1::new();
  /// let mut filter3 = Biquad1::<f32>::new();
  /// let mut filter4 = Biquad1::<f64>::new();
  /// ```
  pub fn new() -> Self {
    Biquad1 {
      x_z1: num::zero(),
      x_z2: num::zero(),
      y_z1: num::zero(),
      y_z2: num::zero(),
      b0: num::one(),
      b1: num::zero(),
      b2: num::zero(),
      a1: num::zero(),
      a2: num::zero()
    }
  }

  /// Sets all filter coefficients at once.
  ///
  /// `b1`, `b2` are feedforwards, or zeroes, and `a1`, `a2` are feedbacks,
  /// or poles.
  pub fn set_coefficients(&mut self, b0: T, b1: T, b2: T, a1: T, a2: T) {
    self.b0 = b0;
    self.b1 = b1;
    self.b2 = b2;
    self.a1 = a1;
    self.a2 = a2;
  }
}

impl<T> Filter<T> for Biquad1<T> where T: Float {
  fn tick(&mut self, sample: T) -> T {
    let output = self.b0 * sample
      + self.b1 * self.x_z1 + self.b2 * self.x_z2
      - self.a1 * self.y_z1 - self.a2 * self.y_z2;
    self.x_z2 = self.x_z1;
    self.x_z1 = sample;
    self.y_z2 = self.y_z1;
    self.y_z1 = output;
    output
  }

  fn clear(&mut self) {
    self.x_z1 = num::zero();
    self.x_z2 = num::zero();
    self.y_z1 = num::zero();
    self.y_z2 = num::zero();
  }

  fn last_out(&self) -> T {
    self.y_z1
  }
}

/// A biquad filter in transposed direct form 2.
///
/// This implementation uses a Transposed [Direct Form II](https://en.wikipedia.org/wiki/Digital_biquad_filter#Direct_Form_2)
/// realization using the following equations:
///
/// `y[n] = b0*x[n] + w[n-1]; w[n-1] = b1*x[n] + w[n-2] - a1*y[n]; w[n-2] = b2*x[n] - a2*y[n];`
///
/// It has two feedforward coefficients, `b1` and `b2`, and two feedback
/// coefficients, `a1` and `a2`.
pub struct Biquad2<T> {
  z1: T,
  z2: T,
  output: T,
  pub b0: T,
  pub b1: T,
  pub b2: T,
  pub a1: T,
  pub a2: T
}

impl<T> Biquad2<T> where T: Float {
  /// Creates a new `Biquad2` filter.
  ///
  /// The filter will be initalized in a state that does not alter the input
  /// signal.
  ///
  /// # Examples
  ///
  /// ```
  /// # #![allow(unused_mut)]
  /// use rasp::filter::Biquad2;
  ///
  /// let mut filter1: Biquad2<f32> = Biquad2::new();
  /// let mut filter2: Biquad2<f64> = Biquad2::new();
  /// let mut filter3 = Biquad2::<f32>::new();
  /// let mut filter4 = Biquad2::<f64>::new();
  /// ```
  pub fn new() -> Self {
    Biquad2 {
      z1: num::zero(),
      z2: num::zero(),
      output: num::zero(),
      b0: num::one(),
      b1: num::zero(),
      b2: num::zero(),
      a1: num::zero(),
      a2: num::zero()
    }
  }

  /// Sets all filter coefficients at once.
  ///
  /// `b1`, `b2` are feedforwards, or zeroes, and `a1`, `a2` are feedbacks,
  /// or poles.
  pub fn set_coefficients(&mut self, b0: T, b1: T, b2: T, a1: T, a2: T) {
    self.b0 = b0;
    self.b1 = b1;
    self.b2 = b2;
    self.a1 = a1;
    self.a2 = a2;
  }
}

impl<T> Filter<T> for Biquad2<T> where T: Float {
  fn tick(&mut self, sample: T) -> T {
    self.output = self.b0 * sample + self.z1;
    self.z1 = self.b1 * sample + self.z2 - self.a1 * self.output;
    self.z2 = self.b2 * sample - self.a2 * self.output;
    self.output
  }

  fn clear(&mut self) {
    self.z1 = num::zero();
    self.z2 = num::zero();
    self.output = num::zero();
  }

  fn last_out(&self) -> T {
    self.output
  }
}

#[cfg(test)]
mod form1 {
  use super::*;
  use std::f32::EPSILON;
  use ::traits::Filter;

  #[test]
  fn tick() {
    let input = vec![0.55f32, -0.55f32, 0.55f32, -0.55f32, 0.25f32];
    let expected =
      vec![
         0.275_000_000_000f32,
        -0.110_000_000_000f32,
         0.214_500_000_000f32,
        -0.251_900_000_000f32,
         0.098_930_000_000f32
      ];
    let mut biquad = Biquad1::new();
    for sample in input.iter() {
      assert!((biquad.tick(*sample) - sample).abs() < EPSILON);
    }
    biquad.clear();
    biquad.set_coefficients(0.5f32, 0.4f32, 0.3f32, 0.2f32, 0.1f32);
    for i in 0..input.len() {
      let output = biquad.tick(input[i]);
      println!("{:.12} - {:.12} = {:.12}", expected[i], output, expected[i] - output);
      assert!((expected[i] - output).abs() < EPSILON);
    }
  }
}

#[cfg(test)]
mod form2 {
  use super::*;
  use std::f32::EPSILON;
  use ::traits::Filter;

  #[test]
  fn tick() {
    let input = vec![0.55f32, -0.55f32, 0.55f32, -0.55f32, 0.25f32];
    let expected =
      vec![
         0.275_000_000_000f32,
        -0.110_000_000_000f32,
         0.214_500_000_000f32,
        -0.251_900_000_000f32,
         0.098_930_000_000f32
      ];
    let mut biquad = Biquad2::new();
    for sample in input.iter() {
      assert!((biquad.tick(*sample) - sample).abs() < EPSILON);
    }
    biquad.clear();
    biquad.set_coefficients(0.5f32, 0.4f32, 0.3f32, 0.2f32, 0.1f32);
    for i in 0..input.len() {
      let output = biquad.tick(input[i]);
      println!("{:.12} - {:.12} = {:.12}", expected[i], output, expected[i] - output);
      assert!((expected[i] - output).abs() < EPSILON);
    }
  }
}