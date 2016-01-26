mod bartlett;
mod blackman;
mod blackman_harris;
mod hamming;
mod hann;
mod triangular;

pub use self::bartlett::BartlettIter               as BartlettIter;
pub use self::blackman::BlackmanIter               as BlackmanIter;
pub use self::blackman_harris::BlackmanHarrisIter  as BlackmanHarrisIter;
pub use self::hamming::HammingIter                 as HammingIter;
pub use self::hann::HannIter                       as HannIter;
pub use self::triangular::TriangularIter           as TriangularIter;

/** Notes on windows
  - The Bartlett/Triangular, Hann, and Hamming windows share a property:
    - when overlapped 50%, the sum of the windows is uniform (window(x) + window(y) = 1)
 */

use num::traits::Float;

use traits::FloatConst;

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
  /// A Blackman window, where `alpha = 0.16`
  Blackman,
  /// A Blakcman-Harris window
  BlackmanHarris
}

/// Applies a window, of the same size, to the given slice of samples.
///
/// This uses the available iterators to generate the window. If you need to
/// apply a window that is not of the same slice, then use the corresponding
/// window iterator and apply the window manually.
pub fn apply_window<T: Float + FloatConst>(samples: &mut [T], window: Window) {
  match window {
    Window::Rectangular => {},
    Window::Triangular => {
      let window = TriangularIter::<T>::new(samples.len());
      for (sample, window_gain) in samples.iter_mut().zip(window) {
        *sample = window_gain * *sample;
      }
    },
    Window::Bartlett => {
      let window = BartlettIter::<T>::new(samples.len());
      for (sample, window_gain) in samples.iter_mut().zip(window) {
        *sample = window_gain * *sample;
      }
    }
    Window::Hann => {
      let window = HannIter::<T>::new(samples.len());
      for (sample, window_gain) in samples.iter_mut().zip(window) {
        *sample = window_gain * *sample;
      }
    },
    Window::Hamming => {
      let window = HammingIter::<T>::new(samples.len());
      for (sample, window_gain) in samples.iter_mut().zip(window) {
        *sample = window_gain * *sample;
      }
    },
    Window::Blackman => {
      let window = BlackmanIter::<T>::new(samples.len());
      for (sample, window_gain) in samples.iter_mut().zip(window) {
        *sample = window_gain * *sample;
      }
    },
    Window::BlackmanHarris => {
      let window = BlackmanHarrisIter::<T>::new(samples.len());
      for (sample, window_gain) in samples.iter_mut().zip(window) {
        *sample = window_gain * *sample;
      }
    }
  }
}

#[cfg(test)]
mod apply_window {
  use super::*;

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
  }
}
