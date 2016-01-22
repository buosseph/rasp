use std::f32::consts::PI;

const BLACKMAN_HARRIS_COEFFS: [f32; 4] = [0.35875f32, 0.48829f32, 0.14128f32, 0.01168f32];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Window {
    Rectangular,
    Triangular,
    /// A special case of a Triangular window there ends are at zero
    Bartlett,
    Hann,
    Hamming,
    BlackmanHarris
}

fn gen_triangular_window(index: usize, block_size: usize) -> f32 {
  let size_minus_one = block_size - 1;
  let alpha = (size_minus_one as f32) / 2f32;
  let numerator = (index as f32) - alpha;
  let denominator = (block_size as f32) / 2f32;

  1f32 - (numerator / denominator).abs()
}

fn gen_bartlett_window(index: usize, block_size: usize) -> f32 {
  let size_minus_one = block_size - 1;
  let alpha = (size_minus_one as f32) / 2f32;
  let numerator = (index as f32) - alpha;

  1f32 - (numerator / alpha).abs()
}

fn gen_hann_window(index: usize, block_size: usize) -> f32 {
  let numerator = 2f32 * PI * (index as f32);
  let denominator = (block_size - 1) as f32;

  0.5f32 * (1f32 - (numerator / denominator).cos())
}

fn gen_hamming_window(index: usize, block_size: usize) -> f32 {
  let alpha = 0.54f32;
  let beta = 1f32 - alpha;
  let numerator = 2f32 * PI * (index as f32);
  let denominator = (block_size - 1) as f32;

  alpha - beta * (numerator / denominator).cos()
}

fn gen_blackman_hariss_window(index: usize, block_size: usize) -> f32 {
  let theta = 2f32 * PI * (index as f32) / ((block_size - 1) as f32);
  let a0 = BLACKMAN_HARRIS_COEFFS[0];
  let a1 = BLACKMAN_HARRIS_COEFFS[1];
  let a2 = BLACKMAN_HARRIS_COEFFS[2];
  let a3 = BLACKMAN_HARRIS_COEFFS[3];

  a0 - a1 * (theta).cos() + a2 * (2f32 * theta).cos() - a3 * (3f32 * theta).cos()
}

pub fn apply_window(samples: &mut [f32], window: Window) {
  let block_size = samples.len();
  match window {
    Window::Rectangular => {},
    Window::Triangular => {
      for (i, sample) in samples.iter_mut().enumerate() {
        *sample = gen_triangular_window(i, block_size) * *sample;
      }
    },
    Window::Bartlett => {
      for (i, sample) in samples.iter_mut().enumerate() {
        *sample = gen_bartlett_window(i, block_size) * *sample;
      }
    }
    Window::Hann => {
      for (i, sample) in samples.iter_mut().enumerate() {
        *sample = gen_hann_window(i, block_size) * *sample;
      }
    },
    Window::Hamming => {
      for (i, sample) in samples.iter_mut().enumerate() {
        *sample = gen_hamming_window(i, block_size) * *sample;
      }
    },
    Window::BlackmanHarris => {
      for (i, sample) in samples.iter_mut().enumerate() {
        *sample = gen_blackman_hariss_window(i, block_size) * *sample;
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::f32::EPSILON;

  #[test]
  fn triangular() {
    unimplemented!();

    let mut cases = vec![
      vec![1f32; 3]
    ];
    
    let results = vec![
      vec![0.5f32, 1f32, 0.5f32]
    ];

    for (signal, expected) in cases.iter_mut().zip(results.iter()) {
      apply_window(&mut *signal, Window::Triangular);
      for (actual, expected) in signal.iter().zip(expected.iter()) {
        println!("{:.6} - {:.6} = {:.6}", expected, actual, expected - actual);
        assert!((expected - actual).abs() < 1e-6f32);
      }
    }
  }

  #[test]
  fn bartlett() {
    let mut cases = vec![
      vec![1f32; 3],
      vec![1f32; 5],
      vec![1f32; 6],
      vec![1f32; 7]
    ];

    let results = vec![
      vec![0f32, 1f32, 0f32],
      vec![0f32, 0.5f32, 1f32, 0.5f32, 0f32],
      vec![0f32, 0.4f32, 0.8f32, 0.8f32, 0.4f32, 0f32],
      vec![0f32, 0.333333f32, 0.666666f32, 1f32, 0.6666666f32, 0.333333f32, 0f32],
    ];

    for (signal, expected) in cases.iter_mut().zip(results.iter()) {
      apply_window(&mut *signal, Window::Bartlett);
      for (actual, expected) in signal.iter().zip(expected.iter()) {
        println!("{:.6} - {:.6} = {:.6}", expected, actual, expected - actual);
        assert!((expected - actual).abs() < 1e-6f32);
      }
    }
  }

  #[test]
  fn hanning() {
    let mut cases = vec![
      vec![1f32; 3],
      vec![1f32; 5],
      vec![1f32; 6],
      vec![1f32; 7]
    ];

    let results = vec![
      vec![0f32, 1f32, 0f32],
      vec![0f32, 0.5f32, 1f32, 0.5f32, 0f32],
      vec![0f32, 0.345492f32, 0.904508f32, 0.904508f32, 0.345492f32, 0f32],
      vec![0f32, 0.25f32, 0.75f32, 1f32, 0.75f32, 0.25f32, 0f32]
    ];

    for (signal, expected) in cases.iter_mut().zip(results.iter()) {
      apply_window(&mut *signal, Window::Hann);
      for (actual, expected) in signal.iter().zip(expected.iter()) {
        println!("{:.6} - {:.6} = {:.6}", expected, actual, expected - actual);
        assert!((expected - actual).abs() < 1e-6f32);
      }
    }
  }

  #[test]
  fn hamming() {
    let mut cases = vec![
      vec![1f32; 3],
      vec![1f32; 5],
      vec![1f32; 6],
      vec![1f32; 7]
    ];

    let results = vec![
      vec![0.08f32, 1f32, 0.08f32],
      vec![0.08f32, 0.54f32, 1f32, 0.54f32, 0.08f32],
      vec![0.08f32, 0.397852f32, 0.912148f32, 0.912148f32, 0.397852f32, 0.08f32],
      vec![0.08f32, 0.31f32, 0.77f32, 1f32, 0.77f32, 0.31f32, 0.08f32]
    ];

    for (signal, expected) in cases.iter_mut().zip(results.iter()) {
      apply_window(&mut *signal, Window::Hamming);
      for (actual, expected) in signal.iter().zip(expected.iter()) {
        println!("{:.6} - {:.6} = {:.6}", expected, actual, expected - actual);
        assert!((expected - actual).abs() < 1e-6f32);
      }
    }
  }

  #[test]
  fn blackman_harris() {
    unimplemented!();

    let mut cases = vec![
      vec![1f32; 3],
      // vec![1f32; 5],
      // vec![1f32; 6],
      // vec![1f32; 7]
    ];

    let results = vec![
      vec![0.08f32, 1f32, 0.08f32],
      // vec![0.08f32, 0.54f32, 1f32, 0.54f32, 0.08f32],
      // vec![0.08f32, 0.397852f32, 0.912148f32, 0.912148f32, 0.397852f32, 0.08f32],
      // vec![0.08f32, 0.31f32, 0.77f32, 1f32, 0.77f32, 0.31f32, 0.08f32]
    ];

    for (signal, expected) in cases.iter_mut().zip(results.iter()) {
      apply_window(&mut *signal, Window::Hamming);
      for (actual, expected) in signal.iter().zip(expected.iter()) {
        println!("{:.6} - {:.6} = {:.6}", expected, actual, expected - actual);
        assert!((expected - actual).abs() < 1e-6f32);
      }
    }
  }
}