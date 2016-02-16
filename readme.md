# rasp
[![Build Status](https://travis-ci.org/brianuosseph/rasp.svg?branch=master)](https://travis-ci.org/brianuosseph/rasp)
[![Coverage Status](https://coveralls.io/repos/brianuosseph/rasp/badge.svg?branch=master&service=github)](https://coveralls.io/github/brianuosseph/rasp?branch=master)

An audio signal processing library in Rust.

The design, and general usage, of this library is based on the [Synthesis Toolkit](https://ccrma.stanford.edu/software/stk/index.html) which is hosted by CCRMA and written in C++.


## Usage

All objects intended to process samples of an audio signal implement the `Processor` trait. Samples are passed into the object using `process()` and returns an output sample.


## Features
- [x] Generic support for `f32` and `f64` samples
- [x] Basic digital filters
  - [x] RBJ filters
- [x] Integer and linear-interpolating delays
- [x] Peak and RMS envelope detectors
- [x] Window function iterators

## Future Work

### Filters
- [ ] Analog filters, based on [DSPFilters by vinniefalco](https://github.com/vinniefalco/DSPFilters)
  - [ ] `filter::butterworth`
  - [ ] `filter::chebyshev1`
  - [ ] `filter::chebyshev2`
  - [ ] `filter::elliptic`
  - [ ] `filter::bessel`
  - [ ] `filter::legendre`

### Delays
- [ ] `delay::AllpassDelay`, an all-pass interpolating delay-line (see `stk::DelayA`)
- [ ] Improved unit tests

### Documentation
- [ ] More inline-examples
- [ ] `examples/effects`
  - [ ] Ping-Pong Delay
  - [ ] Compressor
  - [ ] CombFilter?
  - [ ] Chorus
  - [ ] Flanger?
  - [ ] Phaser?
  - [ ] PitchShifter using a variable delay line
  - [ ] Echo?
  - [ ] Simple reverb?

### Generators
- [ ] `mod generators`
  - [ ] `mod oscillator`
  - [ ] `mod envelope`

#### Oscillators
- [ ] `mod oscillators`

Support of basic waveforms: `Sine`, `Triangle`, `Square`, and `Saw`. There are multiple implementations for oscillators, depending on the waveform. This list is not comprehensive.

#### Envelope Generators
- [ ] `mod envelope`
  - [ ] `Adsr`
  - [ ] `Ahdsr`
  - [ ] `Ar`

### Utilities
- [ ] `util::time`, a time conversion utility?
- [ ] `mod interpolate`?
- [ ] FFI?

#### `util::time`, a time conversion utility
- A Ruby-inspired syntax?
  - `4.milliseconds.to_samples::<f32>::(44100f32) -> 44.1f32`
  - `4.milliseconds.to_samples::<usize>::(44100f32) -> 44usize`

Justification, or "Why I don't want to use `time_calc`"
  - `time_calc` is designed for general time calculations, which is useful when creating a daw for example
    - So `Samples` in `time_calc` is an alias for `i64`, whereas time-based components in this crate rely on `usize` or `f32`
    - `time_calc` also includes converions that wouldn't be used in this crate (`SampleHz`, `Ticks`, `Ppqn`)

_TL;DR_: `time_calc` represents samples for a different use case that's uncessary for this crate

#### `mod interpolate`
Mainly for parameter smoothing functions
  - Linear, lagrange, and other forms of interpolation?

Whether this is implemented or not, docs must be updated to note lack of parameter interpolation
  - So discontinuities when changing parameters with an live signal are possible

#### FFI
If added...
  - Export to C
  - Header files for C++?
  - Package as a JUCE module?

Argument against...
  - DSP could just be done entirely in rust and abstracted as a single struct
  - User can create C bindings themselves

### Miscellaneous
- [ ] `mod formant`
- [ ] `mod pluck` Pluck-string model (see `stk::Twang`)
- [ ] `FFT/IFFT` and `STFT/ISTFT` implementations
