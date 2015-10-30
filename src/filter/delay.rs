use DspComponent;
use Filter;

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
  /// Create a delay line.
  ///
  /// Both `delay` and `max_delay` are represented in samples.
  pub fn new(delay: usize, max_delay: usize) -> Delay {
    if delay > max_delay {
      panic!("delay must be less than or equal to max_delay");
    }

    Delay {
      memory: vec![0f32; max_delay + 1],
      read_ptr: 0,
      write_ptr: 0,
      delay: delay
    }
  }

  /// Set the maximum delay-line length, in samples.
  pub fn set_max_delay(&mut self, delay: usize) {
    // if delay < self.memory.len() { return; }
    self.memory.resize(delay + 1, 0f32);
  }

  /// Returns the maximum delay-line lenght, in samples.
  pub fn get_max_delay(&self) -> usize {
    self.memory.len() - 1
  }

  /// Set the current delay-line length, in samples.
  pub fn set_delay(&mut self, delay: usize) {
    if delay > self.memory.len() - 1 {
      panic!("delay must be less than or equal to max_delay");
    }

    if self.write_ptr >= delay {
      self.read_ptr = self.write_ptr - delay;
    }
    else {
      self.read_ptr = self.memory.len() + self.write_ptr - delay;
    }

    self.delay = delay;
  }

  /// Returns the current delay-line length, in samples.
  pub fn get_delay(&self) -> usize {
    self.delay
  }

  /// Returns the value that will be output by the next call to `tick()`.
  pub fn next_out(&self) -> f32 {
    self.memory[self.read_ptr]
  }
}

impl DspComponent for Delay {
  /// Creates a new `Delay` with a delay-time of zero and a maximum delay of
  /// 4095 samples.
  fn new() -> Delay {
    Delay::new(0, 4095)
  }

  fn tick(&mut self, sample: f32) -> f32 {
    // write input sample into memory
    self.memory[self.write_ptr] = sample;
    self.write_ptr += 1;
    self.write_ptr %= self.memory.len(); // Modulo or if write_ptr == self. memory.len() { write_ptr = 0; } ?

    // read and return next sample in delay line
    let output = self.memory[self.read_ptr];
    self.read_ptr += 1;
    self.read_ptr %= self.memory.len(); // Modulo or if read_ptr == self. memory.len() { read_ptr = 0; } ?
    output
  }
}

#[cfg(test)]
mod tests {
  use DspComponent;
  use std::f32::EPSILON;
  use super::*;

  #[test]
  fn tick() {
    let mut input     = vec![0f32; 5];    input[0] = 1f32;
    let mut expected  = vec![0f32; 5]; expected[4] = 1f32;
    let mut delay     = Delay::new(4, 4095);

    for (i, sample) in input.iter().enumerate() {
      assert!((expected[i] - delay.tick(*sample)).abs() < EPSILON);
    }
  }
}








