# rasp
[![Build Status](https://travis-ci.org/brianuosseph/rasp.svg?branch=master)](https://travis-ci.org/brianuosseph/rasp)
[![Coverage Status](https://coveralls.io/repos/brianuosseph/rasp/badge.svg?branch=master&service=github)](https://coveralls.io/github/brianuosseph/rasp?branch=master)

An audio signal processing library in Rust.

The design, and general usage, of this library is based on the [Synthesis Toolkit](https://ccrma.stanford.edu/software/stk/index.html) which is hosted by CCRMA and written in C++.


## Usage

All objects intended to process samples of an audio signal implement the `Processor` trait. Samples are passed into the object using `process()` and returns an output sample.


### Future Additions

Features and components I'd like to add in the future.

- Add general `EnvelopeDetector`?
  - Must warn in documentation that input must be absolute values
  - An example of when this is used is for gain changing in compressors
    - This is where the attack and release of the gain comes from
- `AllpassDelay`, an all-pass interpolating delay-line (see `stk::DelayA`)
- FFI
  - Export to C
  - Header files for C++?
  - Package as a JUCE module?
- `util::time` - Time conversion utility
  - A Ruby-inspired syntax?
    - `4.milliseconds.to_samples::<f32>::(44100f32) -> 44.1f32`
    - `4.milliseconds.to_samples::<usize>::(44100f32) -> 44usize`
  - Justification, or "Why I don't want to use `time_calc`"
    - `time_calc` is designed for general time calculations, which is useful when creating a daw for example
      - So `Samples` in `time_calc` is an alias for `i64`, whereas time-based components in this crate rely on `usize` or `f32`
      - `time_calc` also includes converions that wouldn't be used in this crate (`SampleHz`, `Ticks`, `Ppqn`)
    - TL;DR: `time_calc` represents samples for a different use case that's uncessary for this crate
- More filters (Based on DSPFilters by vinniefalco, all optional)
  - `filter::butterworth`
  - `filter::chebyshev1`
  - `filter::chebyshev2`
  - `filter::elliptic`
  - `filter::bessel`
  - `filter::legendre`
- Simple effects as examples
  - Ping-Pong Delay
  - Compressor
  - CombFilter?
  - Chorus
  - Flanger?
  - Phaser?
  - PitchShifter, from a variable delay line
  - Echo?
  - Simple reverb?
- `mod envelope`
  - `Adsr`
  - `Ahdsr`
  - `Ar`/`Asr` (user would have no control of sustain)
- Formant filter
- `mod generator` (see `stk::Generator`)
- Pluck-string model (see `stk::Twang`)
