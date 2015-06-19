extern crate audio_dsp;

#[test]
#[allow(unused_variables)]
fn api() {
  use audio_dsp::filter::Biquad;
  use audio_dsp::filter::Lowpass;

  let biquad = Biquad::new();
  let lowpass = Lowpass::new(44_100f64, 8_000f64, 0.71f64);
}
