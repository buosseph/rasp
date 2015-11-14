/// A time-varying delay line.
pub struct Delay {
  memory: Vec<f32>,
  read_ptr: usize,
  write_ptr: usize,
  /// Delay time as a number of samples, which must be less than or equal to
  /// the size of the delay internal memory.
  delay: usize
}

impl Delay {
  /// Creates a delay line.
  ///
  /// Both `delay` and `max_delay` are represented in samples. The `delay`
  /// value will be clipped if it is greater than `max_delay`.
  pub fn new(delay: usize, max_delay: usize) -> Delay {
    let mut num_samples = delay;
    if num_samples > max_delay {
      num_samples = max_delay;
    }

    let mut delay_line =
      Delay {
        memory: vec![0f32; max_delay + 1],
        read_ptr: 0,
        write_ptr: 0,
        delay: 0
      };

    delay_line.set_delay(num_samples);
    delay_line
  }

  /// Set the maximum delay-line length, in samples.
  pub fn set_max_delay(&mut self, delay: usize) {
    if delay < self.memory.len() { return; }
    else {
      self.memory.resize(delay + 1, 0f32);
    }
  }

  /// Returns the maximum delay-line lenght, in samples.
  pub fn get_max_delay(&self) -> usize {
    self.memory.len() - 1
  }

  /// Set the current delay-line length, in samples.
  ///
  /// The `delay` value will be clipped if it is greater than `max_delay`.
  pub fn set_delay(&mut self, delay: usize) {
    let mut num_samples = delay;
    if num_samples > self.memory.len() - 1 {
      num_samples = self.memory.len() - 1;
    }

    if self.write_ptr >= num_samples {
      self.read_ptr = self.write_ptr - num_samples;
    }
    else {
      self.read_ptr = self.memory.len() + self.write_ptr - num_samples;
    }

    self.delay = num_samples;
  }

  /// Returns the current delay-line length, in samples.
  pub fn get_delay(&self) -> usize {
    self.delay
  }

  /// Returns the value that will be output by the next call to `tick()`.
  pub fn next_out(&self) -> f32 {
    self.memory[self.read_ptr]
  }

  /// Processes and stores input sample into memory and outputs calculated
  /// sample.
  pub fn tick(&mut self, sample: f32) -> f32 {
    // write input sample into memory
    self.memory[self.write_ptr] = sample;
    self.write_ptr += 1;
    self.write_ptr %= self.memory.len();

    // read and return next sample in delay line
    let output = self.memory[self.read_ptr];
    self.read_ptr += 1;
    self.read_ptr %= self.memory.len();
    output
  }

  /// Returns the value at `tap_delay` samples from the current delay-line
  /// input.
  pub fn tap_out(&self, tap_delay: usize) -> f32 {
    let mut tap: isize = self.write_ptr as isize - tap_delay as isize - 1;
    if tap < 0 {
      tap += self.memory.len() as isize;
    }
    self.memory[tap as usize]
  }

  /// Sets the value at `tap_delay` samples from the current delay-line
  /// input.
  pub fn tap_in(&mut self, value: f32, tap_delay: usize) {
    let mut tap: isize = self.write_ptr as isize - tap_delay as isize - 1;
    if tap < 0 {
      tap += self.memory.len() as isize;
    }
    self.memory[tap as usize] = value;
  }

  /// Adds to the value at `tap_delay` samples from the current delay-line
  /// input.
  pub fn add_to(&mut self, value: f32, tap_delay: usize) -> f32 {
    let mut tap: isize = self.write_ptr as isize - tap_delay as isize - 1;
    if tap < 0 {
      tap += self.memory.len() as isize;
    }
    self.memory[tap as usize] += value;
    self.memory[tap as usize]
  }
}

#[cfg(test)]
mod tests {
  use std::f32::EPSILON;
  use super::*;

  #[test]
  fn new() {
    let mut delay1 = Delay::new(0, 4095);
    let delay2 = Delay::new(4, 4095);

    assert_eq!(delay1.next_out(), 0f32);
    assert_eq!(delay2.next_out(), 0f32);

    assert!(delay1.get_delay() != delay2.get_delay());
    assert_eq!(delay1.get_max_delay(), delay2.get_max_delay());

    delay1.set_delay(4);
    assert_eq!(delay1.get_delay(), delay2.get_delay());
  }

  #[test]
  fn tick() {
    let mut input     = vec![0f32; 5];    input[0] = 1f32;
    let mut expected  = vec![0f32; 5]; expected[4] = 1f32;
    let mut delay     = Delay::new(4, 4095);

    for (i, sample) in input.iter().enumerate() {
      assert!((expected[i] - delay.tick(*sample)).abs() < EPSILON);
    }
  }

  #[test]
  fn tap_out() {
    // NOTE: More test cases should be added
    let input     = vec![0f32, 0.25f32, 0.5f32, 0.75f32];
    let expected  = vec![0.75f32, 0.5f32, 0.25f32, 0f32];
    let mut delay = Delay::new(4, 4095);

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
    let mut delay = Delay::new(4, 4095);

    for (i, sample) in input.iter().enumerate() {
      delay.tap_in(*sample, i);
    }

    for sample in expected.iter() {
      assert_eq!(*sample, delay.tick(0f32));
    }
  }

  #[test]
  fn add_to() {
    // NOTE: More test cases should be added
    let input     = vec![0f32, 0.25f32, 0.5f32, 0.75f32];
    let expected  = vec![0.75f32, 0.5f32, 0.25f32, 0f32];
    let mut delay = Delay::new(4, 4095);

    for (i, sample) in input.iter().enumerate() {
      delay.add_to(*sample, i);
    }

    for sample in expected.iter() {
      assert_eq!(*sample, delay.tick(0f32));
    }
  }
}
