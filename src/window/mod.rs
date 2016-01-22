use std::f32::consts::PI;

const BLACKMAN_COEFFS: [f32; 3] = [(1f32 - 0.16f32)/2f32, 0.5f32, 0.16f32/2f32];
const BLACKMAN_HARRIS_COEFFS: [f32; 4] = [0.35875f32, 0.48829f32, 0.14128f32, 0.01168f32];

/// A window function
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Window {
  /// A rectangular window
  Rectangular,
  /// A triangular window
  Triangular,
  /// A triangular window where the ends are zero
  Bartlett,
  /// A Hann, or Hanning, window
  Hann,
  /// A Hamming window
  Hamming,
  /// A Blackman window where `alpha = 0.16`
  Blackman,
  /// A Blakcman-Harris window
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

fn gen_blackman_window(index: usize, block_size: usize) -> f32 {
  let theta = 2f32 * PI * (index as f32) / ((block_size - 1) as f32);
  let a0 = BLACKMAN_COEFFS[0];
  let a1 = BLACKMAN_COEFFS[1];
  let a2 = BLACKMAN_COEFFS[2];

  a0 - a1 * (theta).cos() + a2 * (2f32 * theta).cos()
}

fn gen_blackman_hariss_window(index: usize, block_size: usize) -> f32 {
  let theta = 2f32 * PI * (index as f32) / ((block_size - 1) as f32);
  let a0 = BLACKMAN_HARRIS_COEFFS[0];
  let a1 = BLACKMAN_HARRIS_COEFFS[1];
  let a2 = BLACKMAN_HARRIS_COEFFS[2];
  let a3 = BLACKMAN_HARRIS_COEFFS[3];

  a0 - a1 * (theta).cos() + a2 * (2f32 * theta).cos() - a3 * (3f32 * theta).cos()
}

/// Returns a vector a values following the provided window function.
pub fn generate_window(window: Window, size: usize) -> Vec<f32> {
  let mut samples = vec![1f32; size];

  match window {
    Window::Rectangular => {},
    Window::Triangular => {
      for (i, sample) in samples.iter_mut().enumerate() {
        *sample = gen_triangular_window(i, size);
      }
    },
    Window::Bartlett => {
      for (i, sample) in samples.iter_mut().enumerate() {
        *sample = gen_bartlett_window(i, size);
      }
    }
    Window::Hann => {
      for (i, sample) in samples.iter_mut().enumerate() {
        *sample = gen_hann_window(i, size);
      }
    },
    Window::Hamming => {
      for (i, sample) in samples.iter_mut().enumerate() {
        *sample = gen_hamming_window(i, size);
      }
    },
    Window::Blackman => {
      for (i, sample) in samples.iter_mut().enumerate() {
        *sample = gen_blackman_window(i, size);
      }
    },
    Window::BlackmanHarris => {
      for (i, sample) in samples.iter_mut().enumerate() {
        *sample = gen_blackman_hariss_window(i, size);
      }
    }
  }

  samples
}

/// Applies a window function to the given slice of samples.
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
    Window::Blackman => {
      for (i, sample) in samples.iter_mut().enumerate() {
        *sample = gen_blackman_window(i, block_size) * *sample;
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
  fn rectangular() {
    let window = Window::Rectangular;

    let mut cases = vec![
      vec![1f32; 3],
      vec![1f32; 5],
      vec![1f32; 6],
      vec![1f32; 7]
    ];

    let results = vec![
      vec![1f32; 3],
      vec![1f32; 5],
      vec![1f32; 6],
      vec![1f32; 7]
    ];

    for (signal, expected) in cases.iter_mut().zip(results.iter()) {
      apply_window(&mut *signal, window);
      for (actual, expected) in signal.iter().zip(expected.iter()) {
        println!("{:.6} - {:.6} = {:.6}", expected, actual, expected - actual);
        assert!((expected - actual).abs() < 1e-6f32);
      }
    }

    for signal in results.iter() {
      let samples = generate_window(window, signal.len());
      for (actual, expected) in signal.iter().zip(samples.iter()) {
        println!("{:.6} - {:.6} = {:.6}", expected, actual, expected - actual);
        assert!((expected - actual).abs() < 1e-6f32);
      }
    }
  }

  // Values are from octave implementation
  /*
    function y = triang(n, N)
      y = 1 - abs( (n - ((N-1)/2)) / ((N)/2) );
    endfunction
  */
  #[test]
  fn triangular() {
    let window = Window::Triangular;

    let mut cases = vec![
      vec![1f32; 3],
      vec![1f32; 5],
      vec![1f32; 6],
      vec![1f32; 7]
    ];
    
    let results = vec![
      vec![0.3333333f32, 1f32, 0.3333333f32],
      vec![0.2f32, 0.6f32, 1f32, 0.6f32, 0.2f32],
      vec![0.166667f32, 0.5f32, 0.833333f32, 0.833333f32, 0.5f32, 0.166667f32],
      vec![0.142857f32, 0.428571f32, 0.714286f32, 1f32, 0.714286f32, 0.428571f32, 0.142857f32],
    ];

    for (signal, expected) in cases.iter_mut().zip(results.iter()) {
      apply_window(&mut *signal, window);
      for (actual, expected) in signal.iter().zip(expected.iter()) {
        println!("{:.6} - {:.6} = {:.6}", expected, actual, expected - actual);
        assert!((expected - actual).abs() < 1e-6f32);
      }
    }

    for signal in results.iter() {
      let samples = generate_window(window, signal.len());
      for (actual, expected) in signal.iter().zip(samples.iter()) {
        println!("{:.6} - {:.6} = {:.6}", expected, actual, expected - actual);
        assert!((expected - actual).abs() < 1e-6f32);
      }
    }
  }

  // Values are from octave's `bartlett` function
  #[test]
  fn bartlett() {
    let window = Window::Bartlett;

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
      apply_window(&mut *signal, window);
      for (actual, expected) in signal.iter().zip(expected.iter()) {
        println!("{:.6} - {:.6} = {:.6}", expected, actual, expected - actual);
        assert!((expected - actual).abs() < 1e-6f32);
      }
    }

    for signal in results.iter() {
      let samples = generate_window(window, signal.len());
      for (actual, expected) in signal.iter().zip(samples.iter()) {
        println!("{:.6} - {:.6} = {:.6}", expected, actual, expected - actual);
        assert!((expected - actual).abs() < 1e-6f32);
      }
    }
  }

  // Values are from octave's `hanning` function
  #[test]
  fn hann() {
    let window = Window::Hann;

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
      apply_window(&mut *signal, window);
      for (actual, expected) in signal.iter().zip(expected.iter()) {
        println!("{:.6} - {:.6} = {:.6}", expected, actual, expected - actual);
        assert!((expected - actual).abs() < 1e-6f32);
      }
    }

    for signal in results.iter() {
      let samples = generate_window(window, signal.len());
      for (actual, expected) in signal.iter().zip(samples.iter()) {
        println!("{:.6} - {:.6} = {:.6}", expected, actual, expected - actual);
        assert!((expected - actual).abs() < 1e-6f32);
      }
    }
  }

  // Values are from octave's `hamming` function
  #[test]
  fn hamming() {
    let window = Window::Hamming;

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
      apply_window(&mut *signal, window);
      for (actual, expected) in signal.iter().zip(expected.iter()) {
        println!("{:.6} - {:.6} = {:.6}", expected, actual, expected - actual);
        assert!((expected - actual).abs() < 1e-6f32);
      }
    }

    for signal in results.iter() {
      let samples = generate_window(window, signal.len());
      for (actual, expected) in signal.iter().zip(samples.iter()) {
        println!("{:.6} - {:.6} = {:.6}", expected, actual, expected - actual);
        assert!((expected - actual).abs() < 1e-6f32);
      }
    }
  }

  #[test]
  fn blackman() {
    let window = Window::Blackman;

    let mut cases = vec![
      vec![1f32; 3],
      vec![1f32; 5],
      vec![1f32; 6],
      vec![1f32; 7]
    ];

    let results = vec![
      vec![-1.3878e-17f32, 1f32, -1.3878e-17f32],
      vec![-1.3878e-17f32, 3.4e-1f32, 1f32, 3.4e-1f32, -1.3878e-17f32],
      vec![-1.3878e-17f32, 2.0077e-1f32, 8.4923e-1f32, 8.4923e-1f32, 2.0077e-1f32, -1.3878e-17f32],
      vec![-1.3878e-17f32, 1.3e-1f32, 6.3e-1f32, 1f32, 6.3e-1f32, 1.3e-1f32, -1.3878e-17f32]
    ];

    for (signal, expected) in cases.iter_mut().zip(results.iter()) {
      apply_window(&mut *signal, window);
      for (actual, expected) in signal.iter().zip(expected.iter()) {
        println!("{:.6} - {:.6} = {:.6}", expected, actual, expected - actual);
        assert!((expected - actual).abs() < 1e-6f32);
      }
    }

    for signal in results.iter() {
      let samples = generate_window(window, signal.len());
      for (actual, expected) in signal.iter().zip(samples.iter()) {
        println!("{:.6} - {:.6} = {:.6}", expected, actual, expected - actual);
        assert!((expected - actual).abs() < 1e-6f32);
      }
    }
  }

  // Values are from octave implementation
  /*
    function y = w(n, N)
      y = 0.35875 - 0.48829 * cos(2 * pi * n / (N-1)) + 0.14128 * cos(4 * pi * n / (N-1)) - 0.01168 * cos(6 * pi * n / (N-1));
    endfunction
  */
  #[test]
  fn blackman_harris() {
    let window = Window::BlackmanHarris;

    let mut cases = vec![
      vec![1f32; 3],
      vec![1f32; 5],
      vec![1f32; 6],
      vec![1f32; 7]
    ];

    let results = vec![
      vec![6e-5f32, 1f32, 6e-5f32],
      vec![6e-5f32, 0.21747f32, 1f32, 0.21747f32, 6e-5f32],
      vec![6e-5f32, 0.103011f32, 0.793834f32, 0.793834f32, 0.103011f32, 6e-5f32],
      vec![06e-5f32, 0.055645f32, 0.520575f32, 1f32, 0.520575f32, 0.055645f32, 6e-5f32]
    ];

    for (signal, expected) in cases.iter_mut().zip(results.iter()) {
      apply_window(&mut *signal, window);
      for (actual, expected) in signal.iter().zip(expected.iter()) {
        println!("{:.6} - {:.6} = {:.6}", expected, actual, expected - actual);
        assert!((expected - actual).abs() < 1e-6f32);
      }
    }

    for signal in results.iter() {
      let samples = generate_window(window, signal.len());
      for (actual, expected) in signal.iter().zip(samples.iter()) {
        println!("{:.6} - {:.6} = {:.6}", expected, actual, expected - actual);
        assert!((expected - actual).abs() < 1e-6f32);
      }
    }
  }
}