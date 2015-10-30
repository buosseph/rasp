#![feature(vec_resize)]
pub mod filter;

pub use filter::Biquad;
pub use filter::Lowpass;
pub use filter::Highpass;

pub use filter::OnePole;
pub use filter::OneZero;
pub use filter::TwoPole;
pub use filter::TwoZero;

/// A DSP component.
///
/// This contains the base functions common in all components implemented in
/// this library.
pub trait DspComponent {
  /// Creates a new `DspComponent`.
  ///
  /// The component will be initalized in a state that does not alter the
  /// input signal.
  fn new() -> Self;

  /// Processes and stores input sample into memory and outputs calculated
  /// sample.
  fn tick(&mut self, sample: f32) -> f32;
}

/// A digital filter.
///
/// A filter is defined as a linear, time-invariant system that processes, in
/// this case, digital audio samples. This includes traditional filter, such
/// as lowpass and highpass filters, and delays.
pub trait Filter {
  /// Resets memory of all previous input and output to zero.
  fn clear(&mut self);

  /// Returns the last computed output sample.
  fn last_out(&self) -> f32;
}

/// A digital delay-line.
///
/// A delay-line stalls an input signal for some number of samples. Every
/// delay-line has a maximum number of samples which it can buffer and may, or
/// may not, interpolate samples depending on the implementation of the delay
/// component.
pub trait DelayLine {
  fn set_delay_time();
  fn get_delay_time();
  fn set_max_delay_time();
  fn get_max_delay_time();
}
