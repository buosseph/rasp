extern crate rasp;

#[cfg(test)]
mod api {
  #[cfg(test)]
  mod filter {
    use rasp::filter::{
      OnePole,
      OneZero,
      TwoPole,
      TwoZero,
      Biquad,
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

    #[test]
    fn biquad() {
      let mut biquad  = Biquad::new();
      assert_eq!(0f32, biquad.tick(0f32));
    }

    #[cfg(test)]
    mod rdj {
      use rasp::filter::rdj::{
        LowPass,
        HighPass
      };

      #[test]
      fn lowpass() {
        let mut filter = LowPass::new(44_100f32, 8_000f32, 0.71f32);
        assert!(filter.tick(0.1f32) != 0.1f32);
      }

      #[test]
      fn highpass() {
        let mut filter = HighPass::new(44_100f32, 12_000f32, 0.71f32);
        assert!(filter.tick(0.1f32) != 0.1f32);
      }
    }
  }
}

#[test]
#[allow(unused_imports)]
fn exports() {
  // Test all top-level exports for users
  use rasp::filter::Delay;
  use rasp::filter::LinearDelay;

  use rasp::filter::OnePole;
  use rasp::filter::OneZero;
  use rasp::filter::TwoPole;
  use rasp::filter::TwoZero;

  use rasp::filter::Biquad;
  use rasp::filter::rdj::LowPass;
  use rasp::filter::rdj::HighPass;

  assert!(true);
}
