//! A set of biquad filters implementing the "RBJ Biquad" Cookbook formulas
//!
//! These filters are derived from the
//! [Audio EQ Cookbook by Robert Bristow-Johnson](http://www.musicdsp.org/files/Audio-EQ-Cookbook.txt).
//! Generally, the Q factor should be set to `0.7071f32` to have no additional
//! effect on the filter.

/* Q factor vs Bandwdith as filter parameters
  
  Q factor and bandwidth are two different ways to represent the same type of
  changes to the filter transfer functions, and the resulting output. Using
  the relationships below, you can convert between Q factor and bandwidth.

// In digital filter, 1/Q = 2*sinh(ln(2)/2*BW*w0/sin(w0))
// In analog prototype (transfer function), 1/Q = 2*sinh(ln(2)/2*BW)

*/

mod lowpass;
mod highpass;
mod bandpass;
mod allpass;
mod lowshelf;
mod highshelf;
mod bandstop;
mod peak;

pub use self::lowpass::LowPass as LowPass;
pub use self::highpass::HighPass as HighPass;
pub use self::bandpass::BandPass1 as BandPass1;
pub use self::bandpass::BandPass2 as BandPass2;
pub use self::allpass::AllPass as AllPass;
pub use self::lowshelf::LowShelf as LowShelf;
pub use self::highshelf::HighShelf as HighShelf;
pub use self::bandstop::BandStop as BandStop;
pub use self::peak::Peak as Peak;
