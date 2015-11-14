extern crate audio_dsp;

#[cfg(test)]
mod api {
  #[cfg(test)]
  mod filter {
    use audio_dsp::filter::{
      OnePole,
      OneZero,
      TwoPole,
      TwoZero,
      Delay,
      LinearDelay
    };

    #[test]
    fn delay() {
      // Single sample delay
      let mut delay = Delay::new(1, 4);
      assert_eq!(0f32, delay.tick(1f32));
      assert_eq!(1f32, delay.tick(0f32));
    }

    #[test]
    fn linear_delay() {
      // Single sample delay
      let mut delay = LinearDelay::new(1f32, 4);
      assert_eq!(0f32, delay.tick(1f32));
      assert_eq!(1f32, delay.tick(0f32));
    }

    #[test]
    fn one_pole() {
      // Filter should not alter input signal
      let mut one_pole = OnePole::new();
      assert_eq!(1f32, one_pole.tick(1f32));
    }

    #[test]
    fn one_zero() {
      // Filter should not alter input signal
      let mut one_zero = OneZero::new();
      assert_eq!(1f32, one_zero.tick(1f32));
    }

    #[test]
    fn two_pole() {
      // Filter should not alter input signal
      let mut two_pole = TwoPole::new();
      assert_eq!(1f32, two_pole.tick(1f32));
    }

    #[test]
    fn two_zero() {
      // Filter should not alter input signal
      let mut two_zero = TwoZero::new();
      assert_eq!(1f32, two_zero.tick(1f32));
    }

    #[cfg(test)]
    mod biquad {
      use audio_dsp::filter::Biquad;
      use audio_dsp::filter::biquad::{
        Lowpass,
        Highpass
      };

      #[test]
      fn biquad() {
        let mut biquad  = Biquad::new();
        assert_eq!(0f32, biquad.tick(0f32));
      }

      #[test]
      fn lowpass() {
        let mut lowpass = Lowpass::new(44_100f32, 8_000f32, 0.71f32);
        assert!(lowpass.tick(0.1f32) != 0.1f32);
      }

      #[test]
      fn highpass() {
        let mut highpass = Highpass::new(44_100f32, 12_000f32, 0.71f32);
        assert!(highpass.tick(0.1f32) != 0.1f32);
      }
    }
  }
}

#[test]
#[allow(unused_imports)]
fn exports() {
  // Test all top-level exports for users
  use audio_dsp::filter::Delay;
  use audio_dsp::filter::LinearDelay;

  use audio_dsp::filter::OnePole;
  use audio_dsp::filter::OneZero;
  use audio_dsp::filter::TwoPole;
  use audio_dsp::filter::TwoZero;

  use audio_dsp::filter::Biquad;
  use audio_dsp::filter::biquad::Lowpass;
  use audio_dsp::filter::biquad::Highpass;

  assert!(true);
}

#[test]
#[allow(unused_imports)]
fn explicit_exports() {
  // Test all explicity exports to components
  // (biquad::Biquad is currently the only one)
  use audio_dsp::filter::biquad::Biquad;

  assert!(true);
}
