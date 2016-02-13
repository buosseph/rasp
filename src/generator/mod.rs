pub mod trivial;

/* Notes on generators
  - Oscillators
    - There are a few ways to generate a signal

    - Trivial
      - Math based approach, creates major aliasing

    - The following implementations only work on waveforms with straight lines
      - BLIT (saw, square, triangle)
      - BLEP (saw, square, triangle?)
        - This would be the best for creating a basic synthesizer
        - PolyBLEP is a variation of this approach

    - Wavetable
    - Complex? (sine)
      - Uses complex numbers internally

    - In order to potentially support FM, negative frequencies must be valid
    - Generators can potentially be iterators as well (consider which is best)
 */

// TODO: Move traits to mod traits
pub trait Generator {
  fn tick(&mut self) -> f32;
  fn last_out(&self) -> f32;
  fn reset(&mut self);
}

pub trait Oscillator : Generator {
  fn get_frequency(&self) -> f32;
  fn get_phase(&self) -> f32;
  fn set_frequency(&mut self);
  fn set_phase(&mut self);
}

// // Note: keep track of phase as an accumulator, do not directly calculate
// pub struct Sine {
//   sample_rate: f32,
//   freuency: f32,
//   phase_index: f32,
//   phase_delta: f32 // = N * f / Fs
// }

