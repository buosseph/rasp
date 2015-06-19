pub mod filter;

pub use filter::Biquad;

pub trait Filter {
  /// Processes sample and stores input and output to memory
  fn tick(&mut self, sample: f64) -> f64;

  /// Resets memory of all previous input and output to zero
  fn clear(&mut self);
}

#[test]
fn it_works() {
}
