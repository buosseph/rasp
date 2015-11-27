# rasp
[![Build Status](https://travis-ci.org/brianuosseph/audio-dsp.svg)](https://travis-ci.org/brianuosseph/audio-dsp)
[![Coverage Status](https://coveralls.io/repos/brianuosseph/audio-dsp/badge.svg?branch=master&service=github)](https://coveralls.io/github/brianuosseph/audio-dsp?branch=master)

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

- Add an allpass interpolating delay? (under `mod filter`, see `stk::DelayA`)
- Analysis components? (`LeakyIntegrator`, `PeakDetector`)
- Formant filter under `mod filter`
- ADSR evenlope under `mod envelope`
  - Other evenlope types
- Vector-based processing? (`tick(&[f32])`)
- Generators (see `stk::Generator`)
- Pluck-string model (see `stk::Twang`)
- Voice management? (see `stk::Voicer`)
- Generic support for `f32` and `f64` samples?
  - NOTE: This limits the usage of traits for common functions, can't use generics in traits
- Common Effects? (Would be an optional feature)
  - Chorus
  - Flanger
  - Phaser
  - PitchShifter
  - Echo
  - Reverb?
