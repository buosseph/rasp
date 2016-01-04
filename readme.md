# rasp
[![Build Status](https://travis-ci.org/brianuosseph/rasp.svg?branch=master)](https://travis-ci.org/brianuosseph/rasp)
[![Coverage Status](https://coveralls.io/repos/brianuosseph/rasp/badge.svg?branch=master&service=github)](https://coveralls.io/github/brianuosseph/rasp?branch=master)

An audio signal processing library in Rust.

This is a side project for me to learn, and implement, various DSP concepts.

## Design
The design, and general usage, of this library is greatly influenced by [The Synthesis Toolkit](https://ccrma.stanford.edu/software/stk/index.html) which is hosted by CCRMA and written in C++. All components implement a `tick` function, which take an audio sample to process and returns some output.

### Future Additions
Features and components I'd like to add in the future.

- Update `rbj` filters to use transposed direct-form 2 biquads
- `AllpassDelay`, an all-pass interpolating delay-line (see `stk::DelayA`)
- Slice-based processing (`tick(&[f32])`)
- FFI
- More filters (Based on DSPFilters by vinniefalco, all optional)
  - `filter::butterworth`
  - `filter::chebyshev1`
  - `filter::chebyshev2`
  - `filter::elliptic`
  - `filter::bessel`
  - `filter::legendre`
- Common effects? (All optional, or just as examples)
  - CombFilter
  - Chorus
  - Flanger
  - Phaser
  - PitchShifter
  - Echo
  - Simple reverb?
- `mod envelope`
  - `Adsr`
  - `Ahdsr`
  - Others?
- Formant filter
- `mod generator` (see `stk::Generator`)
- Pluck-string model (see `stk::Twang`)
