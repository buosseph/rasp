# rasp
[![Build Status](https://travis-ci.org/brianuosseph/rasp.svg?branch=master)](https://travis-ci.org/brianuosseph/rasp)
[![Coverage Status](https://coveralls.io/repos/brianuosseph/rasp/badge.svg?branch=master&service=github)](https://coveralls.io/github/brianuosseph/rasp?branch=master)

An audio signal processing library in Rust.

This mainly is a project for me to learn, and implement, various DSP concepts.

## Design
The design, and general usage, of this library is greatly influenced by [The Synthesis Toolkit](https://ccrma.stanford.edu/software/stk/index.html) which is hosted by CCRMA and written in C++. All components implement a `tick` function, which passes an audio sample into the component to process and returns some output.

## TODO
- Main Features
  - [ ] Add examples
    - [ ] One per component
    - [ ] Multi-channel data examples
  - [ ] Benchmarks and profiling against STK C++ equivalent

### Future Additions
Features and components I'd like to add in the future, none of which are guaranteed.

- Allpass interpolating delay? (under `mod filter`, see `stk::DelayA`)
- Common Effects? (All optional, or just as examples)
  - CombFilter
  - Chorus
  - Flanger
  - Phaser
  - PitchShifter
  - Echo
  - Simple reverb?
- More filters (Based on DSPFilters by vinniefalco, all optional)
  - `filter::rdj`
  - `filter::butterworth`
  - `filter::chebyshev1`
  - `filter::chebyshev2`
  - `filter::elliptic`
  - `filter::bessel`
  - `filter::legendre`
  - `mod cascade`
    - All optional filters under `mod filter` are direct implementations
    - Use of cascading filters allows for large-order, user-defined filters
- Analysis components? (e.g. `LeakyIntegrator`, `PeakDetector`)
- Formant filter under `mod filter`
- ADSR evenlope under `mod envelope`
  - Other evenlope types
- Vector-based processing? (`tick(&[f32])`)
- Generators (see `stk::Generator`)
- Pluck-string model (see `stk::Twang`)
- Voice management? (see `stk::Voicer`)
- Generic support for `f32` and `f64` samples?
  - NOTE: This limits the usage of traits for common functions, can't use generics in traits
