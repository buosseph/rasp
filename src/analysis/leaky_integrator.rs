use num;
use num::traits::Float;

use traits::Processor;

/// An integrator used to average a signal.
/// 
/// A `LeakyIntegrator` is a specific type of `OnePole` filter, where the
/// input signal gain, `b0`, and the feedback gain, `a1`, are complements such
/// that `a1 = 1 - b0`, as long as `0 <= a1 < 1`. Because of this relationship
/// the filter equation can be changed to `y[n] = x[n] + a1 * (y[n-1] - x[n])`
/// and integrator only uses one gain `a1`, or `alpha`.
pub struct LeakyIntegrator<T> {
  /// The feedback gain in the integrator (a1)
  alpha: T,
  /// The integrator delayed sample memory
  y_z1: T
}

impl<T> LeakyIntegrator<T> where T: Float {
  /// Creates a new `LeakyIntegrator`.
  ///
  /// The integrator will be initalized in a state that does not alter the
  /// input signal, with `alpha` set to zero.
  ///
  /// # Examples
  ///
  /// ```
  /// # #![allow(unused_mut)]
  /// use rasp::analysis::LeakyIntegrator;
  ///
  /// let mut integrator1: LeakyIntegrator<f32> = LeakyIntegrator::new();
  /// let mut integrator2: LeakyIntegrator<f64> = LeakyIntegrator::new();
  /// let mut integrator3 = LeakyIntegrator::<f32>::new();
  /// let mut integrator4 = LeakyIntegrator::<f64>::new();
  /// ```
  pub fn new() -> Self {
    LeakyIntegrator {
      alpha: num::zero(),
      y_z1: num::zero()
    }
  }

  /// Returns the `alpha` gain of the integrator.
  ///
  /// The internal gain is called `alpha` because of the relationship between
  /// the input and feedback gains of the integrator where `a1 = 1 - b0`.
  pub fn get_alpha(&self) -> T {
    self.alpha
  }

  /// Sets the `alpha` gain of the integrator, where `0 <= alpha < 1`.
  ///
  /// If the new `gain` does not satisfy the constraint for `alpha`, then
  /// the current `alpha` remains unchanged.
  ///
  /// The internal gain is called `alpha` because of the relationship between
  /// the input and feedback gains of the integrator where `a1 = 1 - b0`. If
  /// this property is not held, then the integrator will fails.
  ///
  /// # Examples
  ///
  /// ```
  /// use std::f32::EPSILON;
  /// use rasp::analysis::LeakyIntegrator;
  ///
  /// let mut integrator = LeakyIntegrator::new();
  /// integrator.set_alpha(0.99f32);
  /// assert!((integrator.get_alpha() - 0.99f32).abs() < EPSILON);
  ///
  /// // Alpha doesn't update if it's not a ratio less than 1
  /// integrator.set_alpha(1f32);
  /// assert!((integrator.get_alpha() - 0.99f32).abs() < EPSILON);
  /// integrator.set_alpha(-0.01f32);
  /// assert!((integrator.get_alpha() - 0.99f32).abs() < EPSILON);
  /// ```
  pub fn set_alpha(&mut self, gain: T) {
    if gain >= num::zero() && gain < num::one() {
      self.alpha = gain;
    }
  }
}

impl<T> Processor<T> for LeakyIntegrator<T> where T: Float {
  fn process(&mut self, value: T) -> T {
    self.y_z1 = value + self.alpha * (self.y_z1 - value);
    self.y_z1
  }

  fn clear(&mut self) {
    self.y_z1 = num::zero();
  }

  fn last_out(&self) -> T {
    self.y_z1
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::f32::*;
  use ::traits::Processor;

  #[test]
  fn new() {
    let integrator = LeakyIntegrator::<f32>::new();

    assert!((integrator.last_out() - 0f32).abs() < EPSILON);
    assert!((integrator.get_alpha() - 0f32).abs() < EPSILON);
  }

  #[test]
  fn gain() {
    let mut integrator = LeakyIntegrator::new();

    integrator.set_alpha(0.5f32);
    assert!((integrator.get_alpha() - 0.5f32).abs() < EPSILON);
  }

  #[test]
  fn memory() {
    let mut integrator = LeakyIntegrator::<f32>::new();
    assert!((integrator.last_out() - 0f32).abs() < EPSILON);

    integrator.set_alpha(0.5f32);
    let mut output = integrator.process(1f32);

    assert!((output - 0.5f32).abs() < EPSILON);
    assert!((integrator.last_out() - 0.5f32).abs() < EPSILON);

    integrator.clear();
    assert!((integrator.last_out() - 0f32).abs() < EPSILON);

    output = integrator.process(1f32);
    assert!((output - 0.5f32).abs() < EPSILON);
    assert!((integrator.last_out() - 0.5f32).abs() < EPSILON);
  }

  #[test]
  fn process() {
    let mut integrator = LeakyIntegrator::new();
    let expected = vec![0.5f32, 0.75f32, 0.875f32, 0.9375f32, 0.96875f32];

    integrator.set_alpha(0.5f32);

    for case in expected.iter() {
      let output = integrator.process(1f32);
      assert!((output - case).abs() < EPSILON);
    }
  }
}
