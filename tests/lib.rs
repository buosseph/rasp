extern crate audio_dsp;

#[test]
#[allow(unused_variables, unused_assignments)]
fn api() {
  use audio_dsp::Filter;

  use audio_dsp::filter::Biquad;
  use audio_dsp::filter::Lowpass;
  use audio_dsp::filter::Highpass;

  use audio_dsp::filter::OneZero;
  use audio_dsp::filter::OnePole;
  use audio_dsp::filter::TwoZero;
  use audio_dsp::filter::TwoPole;

  let input = vec![0.1f64, 0.1f64, 0.1f64, 0.1f64];
  let mut output: f64;

  let mut biquad  = Biquad::new();
  output = biquad.tick(input[0]);
  assert_eq!(0.1f64, output);

  let mut lowpass = Lowpass::new(44_100f64, 8_000f64, 0.71f64);
  output = lowpass.tick(input[0]);  
  assert!(output != 0.1f64);

  let mut highpass = Highpass::new(44_100f64, 12_000f64, 0.71f64);
  output = highpass.tick(input[0]);
  assert!(output != 0.1f64);

  let mut one_zero = OneZero::new();
  output = one_zero.tick(input[0]);
  assert_eq!(0.1f64, output);

  let mut one_pole = OnePole::new();
  output = one_pole.tick(input[0]);
  assert_eq!(0.1f64, output);

  let mut two_zero = TwoZero::new();
  output = two_zero.tick(input[0]);
  assert_eq!(0.1f64, output);

  let mut two_pole = TwoPole::new();
  output = two_pole.tick(input[0]);
  assert_eq!(0.1f64, output);
}
