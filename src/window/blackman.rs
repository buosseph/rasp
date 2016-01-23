use num;
use num::traits::Float;

use std::marker::PhantomData;
use traits::FloatConst;

const COEFFICIENTS: [f64; 3] = [(1f64 - 0.16f64)/2f64, 0.5f64, 0.16f64/2f64];

/// An iterator that generates a Blackman window.
pub struct BlackmanIter<T: Float + FloatConst> {
  index: usize,
  size: usize,
  phantom: PhantomData<T>
}

impl<T> BlackmanIter<T> where T: Float + FloatConst {
  pub fn new(size: usize) -> Self {
    BlackmanIter {
      index: 0,
      size: size,
      phantom: PhantomData
    }
  }

  fn generate_window(&self) -> T {
    let two: T = T::two();
    let index_float    : T = num::cast(self.index).unwrap();
    let size_minus_one : T = num::cast(self.size - 1).unwrap();

    let theta = two * T::pi() * index_float / size_minus_one;
    let a0: T = num::cast(COEFFICIENTS[0]).unwrap();
    let a1: T = num::cast(COEFFICIENTS[1]).unwrap();
    let a2: T = num::cast(COEFFICIENTS[2]).unwrap();

    a0 - a1 * (theta).cos() + a2 * (two * theta).cos()
  }
}

impl<T> Iterator for BlackmanIter<T> where T: Float + FloatConst {
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

impl<T> ExactSizeIterator for BlackmanIter<T> where T: Float + FloatConst {
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
      vec![-1.3878e-17f32, 1f32, -1.3878e-17f32],
      vec![-1.3878e-17f32, 3.4e-1f32, 1f32, 3.4e-1f32, -1.3878e-17f32],
      vec![-1.3878e-17f32, 2.0077e-1f32, 8.4923e-1f32, 8.4923e-1f32, 2.0077e-1f32, -1.3878e-17f32],
      vec![-1.3878e-17f32, 1.3e-1f32, 6.3e-1f32, 1f32, 6.3e-1f32, 1.3e-1f32, -1.3878e-17f32]
    ];

    for signal in results.iter() {
      let mut window_iter = BlackmanIter::<f32>::new(signal.len());

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
      vec![-1.3878e-17f32, 1f32, -1.3878e-17f32],
      vec![-1.3878e-17f32, 3.4e-1f32, 1f32, 3.4e-1f32, -1.3878e-17f32],
      vec![-1.3878e-17f32, 2.0077e-1f32, 8.4923e-1f32, 8.4923e-1f32, 2.0077e-1f32, -1.3878e-17f32],
      vec![-1.3878e-17f32, 1.3e-1f32, 6.3e-1f32, 1f32, 6.3e-1f32, 1.3e-1f32, -1.3878e-17f32]
    ];

    for signal in results.iter() {
      let window_iter = BlackmanIter::<f32>::new(signal.len());
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
      let window_iter: BlackmanIter<f32> = BlackmanIter::new(signal.len());
      assert_eq!(signal.len(), window_iter.len());
    }
  }
}
