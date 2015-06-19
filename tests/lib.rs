extern crate audio_dsp;

#[test]
#[allow(unused_variables)]
fn api() {
  use audio_dsp::filter::Biquad;

  let biquad = Biquad::new();
}
