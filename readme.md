# rasp
[![Build Status](https://travis-ci.org/brianuosseph/rasp.svg?branch=master)](https://travis-ci.org/brianuosseph/rasp)
[![Coverage Status](https://coveralls.io/repos/brianuosseph/rasp/badge.svg?branch=master&service=github)](https://coveralls.io/github/brianuosseph/rasp?branch=master)

An audio signal processing library in Rust.

This is a side project for me to learn, and implement, various DSP concepts.

## Design
The design, and general usage, of this library is greatly influenced by [The Synthesis Toolkit](https://ccrma.stanford.edu/software/stk/index.html) which is hosted by CCRMA and written in C++. All components implement a `tick` function, which take an audio sample to process and returns some output.

### Future Additions
Features and components I'd like to add in the future.

- Add other biquad transposed direct-form 2
  - Current `Biquad` is of direct-from 1
    - Which is better for fixed point impls due to no possibility of overflow
  - Transposed direct-form 2
    - Better for floating point impls due to less memory
    - Does allow overflow, but that's mitigated by the use of floating point types
    - Transposed is more suited for floating point because it reduces the difference
      intermediate sums, which results in more accurate calculations
      - `out = in * b0 + z1; z1 = in * b1 + z2 - a1 * out; z2 = in * b2 - a2 * out;`
  - Because this crate uses floating point input, should both forms
    be included or just the more appropriate form?
  - Either way, update `rbj` filters to use transposed direct-form 2
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
