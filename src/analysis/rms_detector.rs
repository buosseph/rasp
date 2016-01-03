/// A rms envelope detector.
///
/// [Based on code by Bram](http://musicdsp.org/showArchiveComment.php?ArchiveID=97)
pub struct RmsEnvDetector {
  // The alpha, or a1, coefficients used in the integrator
  attack_gain: f32,
  release_gain: f32,
  // The integrator memory
  envelope: f32,
}

impl RmsEnvDetector {
  /// Creates a new `RmsEnvDetector`.
  ///
  /// The envelope detector will be initialized in a state that does not alter
  /// the input signal. Both `set_attack()` and `set_release()` must be called,
  /// with valid arguments, to make the envelope detector functional.
  ///
  /// # Examples
  ///
  /// ```
  /// use std::f32::EPSILON;
  /// use rasp::analysis::RmsEnvDetector;
  ///
  /// let sample_rate = 44100f32;
  /// let mut detector = RmsEnvDetector::new();
  ///
  /// // Without calling either of these, the detector will not work
  /// detector.set_attack(0.02f32 * sample_rate); // 20 millisecond attack
  /// detector.set_release(0.2f32 * sample_rate); // 200 millisecond release
  ///
  /// ```
  pub fn new() -> Self {
    RmsEnvDetector {
      attack_gain: 0f32,
      release_gain: 0f32,
      envelope: 0f32
    }
  }

  /// Returns the internal attack gain.
  pub fn get_attack_gain(&self) -> f32 {
    self.attack_gain
  }

  /// Sets the internal attack gain based on the provided `attack_length`.
  ///
  /// `attack_length` is the number of samples it takes for the follower to
  /// reach its target value when the envelope is increasing. In other words,
  /// it's the attack time in samples. `attack_length` must be greater than
  /// zero, else the attack gain is not updated.
  ///
  /// # Examples
  ///
  /// ```
  /// use std::f32::EPSILON;
  /// use rasp::analysis::PeakEnvDetector;
  ///
  /// let sample_rate = 44100f32;
  /// let attack_time = 0.020f32; // 20 milliseconds
  /// let release_time = 0.20f32; // 200 milliseconds
  /// let mut detector = PeakEnvDetector::new();
  ///
  /// detector.set_attack(attack_time * sample_rate);
  /// detector.set_release(release_time * sample_rate);
  ///
  /// ```
  pub fn set_attack(&mut self, attack_length: f32) {
    if attack_length > 0f32 && attack_length.is_finite() {
      self.attack_gain = (-1f32 / attack_length).exp();
    }
  }

  /// Returns the internal release gain.
  pub fn get_release_gain(&self) -> f32 {
    self.release_gain
  }

  /// Sets the internal release gain based on the provided `release_length`.
  ///
  /// `release_length` is the number of samples it takes for the follower to
  /// reach its target value when the envelope is decreasing. In other words,
  /// it's the release time in samples. `release_length` must be greater than
  /// zero, else the release gain is not updated.
  ///
  /// # Examples
  ///
  /// ```
  /// use std::f32::EPSILON;
  /// use rasp::analysis::PeakEnvDetector;
  ///
  /// let sample_rate = 44100f32;
  /// let attack_time = 0.020f32; // 20 milliseconds
  /// let release_time = 0.20f32; // 200 milliseconds
  /// let mut detector = PeakEnvDetector::new();
  ///
  /// detector.set_attack(attack_time * sample_rate);
  /// detector.set_release(release_time * sample_rate);
  ///
  /// ```
  pub fn set_release(&mut self, release_length: f32) {
    if release_length > 0f32 && release_length.is_finite() {
      self.release_gain = (-1f32 / release_length).exp();
    }
  }

  pub fn tick(&mut self, sample: f32) -> f32 {
    let input_envelope = sample * sample;

    // The amount to feedback into input_envelope
    let a1 =
      if self.envelope < input_envelope {
        self.attack_gain
      }
      else {
        self.release_gain
      };

    self.envelope = input_envelope + a1 * (self.envelope - input_envelope);
    self.envelope.sqrt()
  }

  pub fn clear(&mut self) {
    self.envelope = 0f32;
  }

  pub fn last_out(&self) -> f32 {
    self.envelope.sqrt()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::f32::*;

  #[test]
  fn new() {
    let detector = RmsEnvDetector::new();

    assert!((detector.last_out() - 0f32).abs() < EPSILON);
    assert!((detector.get_attack_gain() - 0f32).abs() < EPSILON);
    assert!((detector.get_release_gain() - 0f32).abs() < EPSILON);
  }

  #[test]
  fn attack_and_release() {
    let sample_rate = 44100f32;
    let attack = 0.02f32 * sample_rate;
    let release = 0.2f32 * sample_rate;
    let expected_attack_gain = (-1f32 / attack).exp();
    let expected_release_gain = (-1f32 / release).exp();
    let invalid = vec![0f32, INFINITY, NAN];

    let mut detector = RmsEnvDetector::new();
    detector.set_attack(attack);
    detector.set_release(release);
    assert!((detector.get_attack_gain() - expected_attack_gain).abs() < EPSILON);
    assert!((detector.get_release_gain() - expected_release_gain).abs() < EPSILON);

    // Invalid values
    for value in invalid.iter() {
      detector.set_attack(*value);
      detector.set_release(*value);
      assert!((detector.get_attack_gain() - expected_attack_gain).abs() < EPSILON);
      assert!((detector.get_release_gain() - expected_release_gain).abs() < EPSILON);
    }
  }

  #[test]
  fn memory() {
    let mut detector = RmsEnvDetector::new();
    detector.set_attack(0.02f32 * 44100f32);
    detector.set_release(0.2f32 * 44100f32);

    let output = detector.tick(1f32);
    assert!((detector.last_out() - output).abs() < EPSILON);

    detector.clear();
    assert!((detector.last_out() - 0f32).abs() < EPSILON);
  }
}
