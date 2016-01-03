use num;
use num::traits::Float;

use traits::{
  Filter,
  TappableDelayLine
};

/// A time-varying, linear interpolating delay line.
pub struct LinearDelay<T> {
  memory: Vec<T>,
  output: T,
  read_ptr: usize,
  write_ptr: usize,
  /// Delay time as a number of samples, which must be less than or equal to
  /// the size of the delay internal memory.
  delay: f32,
  do_next_out: bool,
  next_out: T,
  // Interpolation multiplers
  alpha: T,
  om_alpha: T
}

impl<T> LinearDelay<T> where T: Float {
  /// Creates a delay line.
  ///
  /// Both `delay` and `max_delay` are represented in samples. The `delay`
  /// value will be clipped if it is greater than `max_delay`.
  ///
  /// # Examples
  ///
  /// ```
  /// # #![allow(unused_mut)]
  /// use rasp::delay::LinearDelay;
  ///
  /// let sample_rate: usize = 44100;
  /// let max_delay = 5; // seconds
  /// let delay = 1300.5f32; // samples
  ///
  /// let mut delay1: LinearDelay<f32> = LinearDelay::new(delay, max_delay * sample_rate);
  /// let mut delay2: LinearDelay<f64> = LinearDelay::new(delay, max_delay * sample_rate);
  /// let mut delay3 = LinearDelay::<f32>::new(delay, max_delay * sample_rate);
  /// let mut delay4 = LinearDelay::<f64>::new(delay, max_delay * sample_rate);
  /// ```
  pub fn new(delay: f32,
             max_delay: usize) -> Self {
    let mut delay_time = delay;
    if delay_time > max_delay as f32 {
      delay_time = max_delay as f32;
    }
    if delay_time < 0f32 {
      delay_time = 0f32;
    }

    let mut delay_line =
      LinearDelay {
        memory: vec![num::zero(); max_delay + 1],
        output: num::zero(),
        read_ptr: 0,
        write_ptr: 0,
        delay: 0f32,
        do_next_out: true,
        next_out: num::zero(),
        alpha: num::zero(),
        om_alpha: num::zero()
      };

    delay_line.set_delay(delay_time);
    delay_line
  }

  /// Set the maximum delay-line length, in samples.
  pub fn set_max_delay(&mut self, delay: usize) {
    if delay < self.memory.len() { return; }
    self.memory.resize(delay + 1, num::zero());
  }

  /// Returns the maximum delay-line lenght, in samples.
  pub fn get_max_delay(&self) -> usize {
    self.memory.len() - 1
  }

  /// Set the current delay-line length, in samples.
  ///
  /// The `delay` value will be clipped if it is greater than `max_delay`.
  pub fn set_delay(&mut self, delay: f32) {
    let mut delay_time = delay;
    let max_delay_samples = (self.memory.len() - 1) as f32;
    if delay_time > max_delay_samples {
      delay_time = max_delay_samples;
    }
    if delay_time < 0f32 {
      delay_time = 0f32;
    }

    let mut read_ptr_integer: f32 = self.write_ptr as f32 - delay_time;
    self.delay = delay_time;

    while read_ptr_integer < 0f32 {
      read_ptr_integer += self.memory.len() as f32;
    }

    // save integer part
    self.read_ptr = read_ptr_integer as usize;
    if self.read_ptr == self.memory.len() {
      self.read_ptr = 0;
    }

    // save fractional part
    self.alpha = num::cast(read_ptr_integer - self.read_ptr as f32).unwrap();
    self.om_alpha = T::one() - self.alpha;
  }

  /// Returns the current delay-line length, in samples.
  pub fn get_delay(&self) -> f32 {
    self.delay
  }

  /// Returns the value that will be output by the next call to `tick()`.
  pub fn next_out(&mut self) -> T {
    if self.do_next_out {
      // First half of interpolation
      self.next_out = self.memory[self.read_ptr] * self.om_alpha;
      // Second half
      if self.read_ptr < self.memory.len() - 1 {
        self.next_out = self.next_out
                      + (self.memory[self.read_ptr + 1] * self.alpha);
      }
      else {
        self.next_out = self.next_out + (self.memory[0] * self.alpha);
      }

      self.do_next_out = false
    }

    return self.next_out;
  }
}

impl<T> Filter<T> for LinearDelay<T> where T: Float {
  fn tick(&mut self, sample: T) -> T {
    // write input sample into memory
    self.memory[self.write_ptr] = sample;
    self.write_ptr += 1;
    self.write_ptr %= self.memory.len();

    // interpolate
    self.output = self.next_out();
    self.do_next_out = true;

    // increment read_ptr
    self.read_ptr += 1;
    self.read_ptr %= self.memory.len();

    self.output
  }

  fn clear(&mut self) {
    for sample in self.memory.iter_mut() {
      *sample = num::zero();
    }
    self.output = num::zero();
  }

  fn last_out(&self) -> T {
    self.output
  }
}

impl<T> TappableDelayLine<T> for LinearDelay<T> where T: Float {
  fn tap_out(&self, tap_delay: usize) -> T {
    let mut tap: isize = self.write_ptr as isize - tap_delay as isize - 1;
    if tap < 0 {
      tap += self.memory.len() as isize;
    }
    self.memory[tap as usize]
  }

  fn tap_in(&mut self, value: T, tap_delay: usize) {
    let mut tap: isize = self.write_ptr as isize - tap_delay as isize - 1;
    if tap < 0 {
      tap += self.memory.len() as isize;
    }
    self.memory[tap as usize] = value;
  }

  fn add_to(&mut self, value: T, tap_delay: usize) -> T {
    let mut tap: isize = self.write_ptr as isize - tap_delay as isize - 1;
    if tap < 0 {
      tap += self.memory.len() as isize;
    }
    self.memory[tap as usize] = self.memory[tap as usize] + value;
    self.memory[tap as usize]
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::f32::EPSILON;
  use ::traits::{Filter, TappableDelayLine};

  #[test]
  fn new() {
    let mut delay1 = LinearDelay::<f32>::new(0f32, 4095);
    let mut delay2 = LinearDelay::<f32>::new(0.5f32, 4095);

    assert_eq!(delay1.next_out(), 0f32);
    assert_eq!(delay2.next_out(), 0f32);

    assert!((delay1.get_delay() - 0f32).abs()   < EPSILON);
    assert!((delay2.get_delay() - 0.5f32).abs() < EPSILON);
    assert_eq!(delay1.get_max_delay(), delay2.get_max_delay());

    delay1.set_delay(2.65f32);
    assert!((delay1.get_delay() - 2.65f32).abs()   < EPSILON);
  }

  #[test]
  fn new_beyond_bounds() {
    let delay1 = LinearDelay::<f32>::new(2000f32, 1000);
    assert!((delay1.get_delay() - delay1.get_max_delay() as f32).abs() < EPSILON);

    let delay2 = LinearDelay::<f32>::new(-2000f32, 1000);
    assert!((delay2.get_delay() - 0f32).abs() < EPSILON);
  }

  #[test]
  fn set_delay() {
    let max_delay = 1000;
    let mut delay = LinearDelay::<f32>::new(500f32, max_delay);

    delay.set_delay(2000f32);
    assert!((delay.get_delay() - max_delay as f32).abs() < EPSILON);

    delay.set_delay(-2000f32);
    assert!((delay.get_delay() - 0f32).abs() < EPSILON);
  }

  #[test]
  fn tick() {
    // No interpolation case
    let mut input     = vec![0f32; 5];    input[0] = 1f32;
    let mut expected  = vec![0f32; 5]; expected[4] = 1f32;
    let mut delay1    = LinearDelay::<f32>::new(4f32, 4095);

    for (i, sample) in input.iter().enumerate() {
      assert!((expected[i] - delay1.tick(*sample)).abs() < EPSILON);
    }

    // Interpolation case
    let mut delay2 = LinearDelay::<f32>::new(2.5f32, 4095);
    expected = vec![0f32; 5];
    expected[2] = 0.5f32;
    expected[3] = 0.5f32;

    for (i, sample) in input.iter().enumerate() {
      assert!((expected[i] - delay2.tick(*sample)).abs() < EPSILON);
    }
  }

  #[test]
  fn tap_out() {
    // NOTE: More test cases should be added
    let input     = vec![0f32, 0.25f32, 0.5f32, 0.75f32];
    let expected  = vec![0.75f32, 0.5f32, 0.25f32, 0f32];
    let mut delay = LinearDelay::<f32>::new(4f32, 4095);

    for sample in input.iter() {
      delay.tick(*sample);
      assert_eq!(*sample, delay.tap_out(0));
    }

    for (i, sample) in expected.iter().enumerate() {
      assert_eq!(*sample, delay.tap_out(i));
    }
  }

  #[test]
  fn tap_in() {
    // NOTE: More test cases should be added
    let input     = vec![0f32, 0.25f32, 0.5f32, 0.75f32];
    let expected  = vec![0.75f32, 0.5f32, 0.25f32, 0f32];
    let mut delay = LinearDelay::<f32>::new(4f32, 4095);

    for (i, sample) in input.iter().enumerate() {
      delay.tap_in(*sample, i);
    }

    for sample in expected.iter() {
      assert_eq!(*sample, delay.tick(0f32));
    }
  }

  // Edge cases:
  // - infinity
  // - negative infinity
  // - negative zero
  // - negative numbers
  // let cases: Vec<f32> = vec![(1f32/0f32), (-1f32/0f32), -0f32, -25.5f32];
}
