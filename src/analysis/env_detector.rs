/* The issue with a general EnvDetector:
  - An envelope detector, by definition, requires absolute input (non-negative or non-positive)
  - In the peak and rms impls this is taken care of using a preprocessor before integration
  - However in a general case, by this design, there is no set preprocessor
    - This means the input can break the envelope detector if not

  - One example of when this is used is for gain changing in compressors
    - This is where the attack and release of the gain comes from
    - However, to my understanding, this is a hacky way to implement an envelope
*/

/*
use ::envelope::LeakyIntegrator;

/// A general, single channel, envelope detector
///
/// It's important to note that envelope detectors are intended to work on
/// absolute inputs. Thus, input samples must be either non-negative or
/// non-positive. If this condition isn't satisfied, the processor can
/// result in unexpected behavior.
pub struct EnvDetector {
  integrator: LeakyIntegrator,
  sample_rate: f32,
  attack_gain: f32,
  release_gain: f32
}

impl EnvDetector {
  // Attack and release time are in seconds
  pub fn new(sample_rate: f32,
             attack_time: f32,
             release_time: f32) -> Self {
    let mut detector =
      EnvDetector {
        integrator: LeakyIntegrator::new(),
        sample_rate: sample_rate,
        attack_gain: 0f32,
        release_gain: 0f32,
      };

    detector.set_attack(attack_time);
    detector.set_release(release_time);
    detector
  }

  /// Attack_time is in seconds
  pub fn set_attack(&mut self, attack_time: f32) {
    self.attack_gain = /*1f32 - */ (-1f32 / (attack_time * self.sample_rate)).exp();
  }

  pub fn get_attack_gain(&self) -> f32 {
    self.attack_gain
  }

  /// Release_time is in seconds
  pub fn set_release(&mut self, release_time: f32) {
    self.release_gain = /*1f32 - */ (-1f32 / (release_time * self.sample_rate)).exp();
  }

  pub fn get_release_gain(&self) -> f32 {
    self.release_gain
  }

  /* This needs testing and debugging the most,
     composition can be very finicky here if not careful,
     also composition approach could use optimization to reduce memory usage
  */
  pub fn tick(&mut self, sample: f32) -> f32 {
    let input_envelope = sample;

    // The output_envelope is the integrator's delayed sample
    if self.integrator.last_out() < input_envelope {
      self.integrator.set_feedback_gain(self.attack_gain);
    }
    else {
      self.integrator.set_feedback_gain(self.release_gain);
    }

    let envelope = self.integrator.tick(input_envelope);
    envelope
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::f32::EPSILON;

  #[test]
  fn new() {
    let env_detector = EnvDetector::new(44100f32, 0f32, 0f32);

    // assert!(().abs() < EPSILON);
    panic!();
  }
}
*/

/* General EnvDetector prototype, with pre and post processor functions */

// < geofft> one option is to make the struct generic on <F: FnOnce(f32) -> f32, G: FnOnce(f32) -> f32>,
//           and take parameters in new(..., f: F, g: G)
// < geofft> if these are optional, I might consider making a trait for a pre- and post-processor 
// < geofft> trait Processor {fn preprocess(&mut self, f32) -> f32; fn postprocess(&mut self, f32) -> f32;} and having a function that 
//           accepts an Option<&Processor> 

/*trait Processor {
  fn preprocess(&mut self, f32) -> f32;
  fn postprocess(&mut self, f32) -> f32;
}*/

/*
/// A general, single channel, envelope detector
// TODO: Add functions pre and post integrator
pub struct EnvDetector<F: FnMut(f32) -> f32, G: FnMut(f32) ->f32> {
  integrator: LeakyIntegrator,
  pre_integrator: Option<F>,
  post_integrator: Option<G>,
  sample_rate: f32,
  attack_gain: f32,
  release_gain: f32
}

impl<F, G> EnvDetector<F, G>
  where F: FnMut(f32) -> f32, G: FnMut(f32) -> f32 {
  // Attack and release time are in seconds
  pub fn new(sample_rate: f32,
             attack_time: f32,
             release_time: f32) -> Self {
    let mut detector =
      EnvDetector {
        integrator: LeakyIntegrator::new(),
        pre_integrator: None,
        post_integrator: None,
        sample_rate: sample_rate,
        attack_gain: 0f32,
        release_gain: 0f32,
      };

    detector.set_attack(attack_time);
    detector.set_release(release_time);
    detector
  }

  // pub fn set_pre_processor<F>(function: F) where F: FnMut(f32) -> f32 {
  // }

  // pub fn set_post_processor<F>(function: F) where F: FnMut(f32) -> f32 {
  // }

  /// Attack_time is in seconds
  pub fn set_attack(&mut self, attack_time: f32) {
    self.attack_gain = 1f32 - (-1f32 / (attack_time * self.sample_rate)).exp();
  }

  /// Release_time is in seconds
  pub fn set_release(&mut self, release_time: f32) {
    self.attack_gain = 1f32 - (-1f32 / (release_time * self.sample_rate)).exp();
  }

  /* This needs testing and debugging the most,
     composition can be very finicky here if not careful,
     also composition approach could use optimization to reduce memory usage
  */
  pub fn tick(&mut self, sample: f32) -> f32 {
    // let input_envelope = self.pre_integrator(sample);
    let input_envelope = sample;

    // The output_envelope is the integrator's delayed sample
    if self.integrator.last_out() < input_envelope {
      self.integrator.set_feedback_gain(self.attack_gain);
    }
    else {
      self.integrator.set_feedback_gain(self.release_gain);
    }

    let envelope = self.integrator.tick(input_envelope);
    // self.post_integrator(envelope)
    envelope
  }
}
*/
