// Integration tests
extern crate rasp;

#[cfg(test)]
mod api {
  mod analysis {
    use std::f32::EPSILON;
    use rasp::traits::Filter;
    use rasp::analysis::{
      LeakyIntegrator,
      PeakEnvDetector,
      RmsEnvDetector
    };

    // No component here should alter the input until attack and relase are set

    #[test]
    fn leaky_integrator() {
      let mut integrator = LeakyIntegrator::new();
      assert!((integrator.tick(1f32) - 1f32).abs() < EPSILON);
    }

    #[test]
    fn peak_detector() {
      let mut detector = PeakEnvDetector::new();
      assert!((detector.tick(1f32) - 1f32).abs() < EPSILON);
    }

    #[test]
    fn rms_detector() {
      let mut detector = RmsEnvDetector::new();
      assert!((detector.tick(1f32) - 1f32).abs() < EPSILON);
    }
  }

  mod filter {
    use std::f32::EPSILON;
    use rasp::traits::Filter;
    use rasp::filter::{
      OnePole,
      OneZero,
      TwoPole,
      TwoZero,
      Biquad
    };

    // No component here should alter the input until coefficients are set

    #[test]
    fn one_pole() {
      let mut one_pole = OnePole::new();
      assert!((one_pole.tick(1f32) - 1f32).abs() < EPSILON);
    }

    #[test]
    fn one_zero() {
      let mut one_zero = OneZero::new();
      assert!((one_zero.tick(1f32) - 1f32).abs() < EPSILON);
    }

    #[test]
    fn two_pole() {
      let mut two_pole = TwoPole::new();
      assert!((two_pole.tick(1f32) - 1f32).abs() < EPSILON);
    }

    #[test]
    fn two_zero() {
      let mut two_zero = TwoZero::new();
      assert!((two_zero.tick(1f32) - 1f32).abs() < EPSILON);
    }

    #[test]
    fn biquad() {
      let mut biquad  = Biquad::new();
      assert!((biquad.tick(1f32) - 1f32).abs() < EPSILON);
    }

    #[cfg(test)]
    mod rbj {
      use rasp::traits::Filter;
      use rasp::filter::rbj::{
        LowPass,
        HighPass,
        BandPass1,
        BandPass2,
        BandStop,
        LowShelf,
        HighShelf,
        AllPass,
        Peak
      };

      #[test]
      fn lowpass() {
        let mut filter = LowPass::new();
        filter.set_coefficients(44_100f32, 12_000f32, 0.71f32);
        assert!(filter.tick(0.1f32) != 0.1f32);
      }

      #[test]
      fn highpass() {
        let mut filter = HighPass::new();
        filter.set_coefficients(44_100f32, 12_000f32, 0.71f32);
        assert!(filter.tick(0.1f32) != 0.1f32);
      }

      #[test]
      fn bandpass1() {
        let mut filter = BandPass1::new();
        filter.set_coefficients(44_100f32, 12_000f32, 0.71f32);
        assert!(filter.tick(0.1f32) != 0.1f32);
      }

      #[test]
      fn bandpass2() {
        let mut filter = BandPass2::new();
        filter.set_coefficients(44_100f32, 12_000f32, 0.71f32);
        assert!(filter.tick(0.1f32) != 0.1f32);
      }

      #[test]
      fn bandstop() {
        let mut filter = BandStop::new();
        filter.set_coefficients(44_100f32, 12_000f32, 0.71f32);
        assert!(filter.tick(0.1f32) != 0.1f32);
      }

      #[test]
      fn lowshelf() {
        let mut filter = LowShelf::new();
        filter.set_coefficients(44_100f32, 12_000f32, 3f32, 0.71f32);
        assert!(filter.tick(0.1f32) != 0.1f32);
      }

      #[test]
      fn highshelf() {
        let mut filter = HighShelf::new();
        filter.set_coefficients(44_100f32, 12_000f32, 3f32, 0.71f32);
        assert!(filter.tick(0.1f32) != 0.1f32);
      }

      #[test]
      fn allpass() {
        let mut filter = AllPass::new();
        filter.set_coefficients(44_100f32, 12_000f32, 0.71f32);
        assert!(filter.tick(0.1f32) != 0.1f32);
      }

      #[test]
      fn peak() {
        let mut filter = Peak::new();
        filter.set_coefficients(44_100f32, 12_000f32, 3f32, 0.71f32);
        assert!(filter.tick(0.1f32) != 0.1f32);
      }
    }
  }

  mod delay {
    use std::f32::EPSILON;
    use rasp::traits::Filter;
    use rasp::delay::{
      Delay,
      LinearDelay
    };

    #[test]
    fn delay() {
      // Single sample delay
      let mut delay = Delay::new(1, 4);
      assert!((delay.tick(1f32) - 0f32).abs() < EPSILON );
      assert!((delay.tick(0f32) - 1f32).abs() < EPSILON );
    }

    #[test]
    fn linear_delay() {
      // Single sample delay
      let mut delay = LinearDelay::new(1f32, 4);
      assert!((delay.tick(1f32) - 0f32).abs() < EPSILON );
      assert!((delay.tick(0f32) - 1f32).abs() < EPSILON );
    }
  }

  mod util {
    use rasp::util;
    use std::f32::EPSILON;

    #[test]
    fn conversions() {
      assert!((util::to_db(0f32) - -120f32).abs() < EPSILON);
      assert!((util::to_sample(-120f32) - 0f32).abs() < EPSILON);
    }
  }
}
