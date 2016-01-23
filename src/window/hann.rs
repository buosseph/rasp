use num;
use num::traits::Float;

use std::marker::PhantomData;
use traits::FloatConst;

/// An iterator that generates a Hann, or Hanning, window.
pub struct HannIter<T: Float + FloatConst> {
  index: usize,
  size: usize,
  phantom: PhantomData<T>
}

impl<T> HannIter<T> where T: Float + FloatConst {
  pub fn new(size: usize) -> Self {
    HannIter {
      index: 0,
      size: size,
      phantom: PhantomData
    }
  }

  fn generate_window(&self) -> T {
    let one : T = T::one();
    let two : T = T::two();
    let half: T = num::cast(0.5f64).unwrap();
    let index_float    : T = num::cast(self.index).unwrap();
    let size_minus_one : T = num::cast(self.size - 1).unwrap();

    let numerator = two * T::pi() * index_float;

    half * (one - (numerator / size_minus_one).cos())
  }
}

impl<T> Iterator for HannIter<T> where T: Float + FloatConst {
  type Item = T;

  fn next(&mut self) -> Option<T> {
    if self.index < self.size {
      let window_sample = self.generate_window();
      self.index += 1;
      Some(window_sample)
    }
    else {
      None
    }
  }
}

impl<T> ExactSizeIterator for HannIter<T> where T: Float + FloatConst {
  fn len(&self) -> usize {
    self.size
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn next() {
    let results = vec![
      vec![0f32, 1f32, 0f32],
      vec![0f32, 0.5f32, 1f32, 0.5f32, 0f32],
      vec![0f32, 0.345492f32, 0.904508f32, 0.904508f32, 0.345492f32, 0f32],
      vec![0f32, 0.25f32, 0.75f32, 1f32, 0.75f32, 0.25f32, 0f32]
    ];

    for signal in results.iter() {
      let mut window_iter = HannIter::<f32>::new(signal.len());

      for expected in signal.iter() {
        let actual = window_iter.next().unwrap();
        println!("{:.6} - {:.6} = {:.6}", expected, actual, expected - actual);
        assert!((expected - actual).abs() < 1e-6f32);
      }
    }
  }

  #[test]
  fn collect() {
    let results = vec![
      vec![0f32, 1f32, 0f32],
      vec![0f32, 0.5f32, 1f32, 0.5f32, 0f32],
      vec![0f32, 0.345492f32, 0.904508f32, 0.904508f32, 0.345492f32, 0f32],
      vec![0f32, 0.25f32, 0.75f32, 1f32, 0.75f32, 0.25f32, 0f32]
    ];

    for signal in results.iter() {
      let window_iter = HannIter::<f32>::new(signal.len());
      let samples: Vec<f32> = window_iter.collect();
      for (actual, expected) in signal.iter().zip(samples.iter()) {
        println!("{:.6} - {:.6} = {:.6}", expected, actual, expected - actual);
        assert!((expected - actual).abs() < 1e-6f32);
      }
    }
  }

  #[test]
  fn len() {
    let cases = vec![
      vec![1f32; 3],
      vec![1f32; 5],
      vec![1f32; 6],
      vec![1f32; 7]
    ];

    for signal in cases.iter() {
      let window_iter: HannIter<f32> = HannIter::new(signal.len());
      assert_eq!(signal.len(), window_iter.len());
    }
  }
}
