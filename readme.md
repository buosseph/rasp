# audio-dsp
[![Build Status](https://travis-ci.org/brianuosseph/audio-dsp.svg)](https://travis-ci.org/brianuosseph/audio-dsp)

This library contains a number of common components used in digital signal processing applications.

## Design
The design, and general usage, of this library is greatly influenced by [The Synthesis Toolkit](https://ccrma.stanford.edu/software/stk/index.html) which is hosted by CCRMA and written in C++. All components implement the `DspComponent` trait which provides the fuctions common in all components. The most important being `tick` which passes input to be processed by the component and returns a value.

## TODO
- Main Features
  - [ ] Refactor traits and abstractions
    - [x] Add `DspComponent` to all components
  - [x] Change sample type to `f32`
  - [ ] Update documentation
  - [x] Update tests
    - Most tests will fail now that sample type has been changed to `f32`
  - [ ] Add examples
    - [ ] One per `DspComponent`
    - [ ] Multi-channel data using one `DspComponent` per channel
  - [ ] Benchmarks and profiling against STK C++ equivalent
  - [ ] Add delay implementations under `mod filter`
  - [ ] A better name!

### Future Additions
Features and components I'd like to add in the future, none of which are guaranteed.

- Analysis components? (`LeakyIntegrator`, `PeakDetector`)
- Vector-based computations? (`tick(&[f32])`)
- Generators (see `stk::Generator`)
- Pluck-string model (see `stk::Twang`)
- Generic support for `f32` and `f64` samples?
- Common Effects? (Chorus, Flanger, Pitchshift, Echo, Reverb) Would be an optional feature