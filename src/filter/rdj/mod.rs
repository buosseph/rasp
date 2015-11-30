//! A set of biquad filters implementing the "RDJ Biquad" Cookbook formulas
//!
//! These filters are derived from the
//! [Biquad Cookbook by Robert Bristow-Johnson](http://www.musicdsp.org/files/Audio-EQ-Cookbook.txt).

mod lowpass;
mod highpass;

pub use self::lowpass::LowPass as LowPass;
pub use self::highpass::HighPass as HighPass;

use std::f32::MIN_POSITIVE;

// Cutoff frequency must be non-negative
const MIN_FREQUENCY: f32 = 0f32;

// Sample rate and Q must be non-zero
const MIN_Q: f32 = MIN_POSITIVE;
const MIN_SAMPLE_RATE: f32 = MIN_POSITIVE;
