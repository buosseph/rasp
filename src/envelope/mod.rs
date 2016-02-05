pub mod ar;

pub use self::ar::Ar as Ar;

/*  Notes on envelopes
  - Names derive from states and their behaviors
    - Attack
      - Occurs when the trigger gate is on
      - Amount of time the envelope increases to 1.0
    - Hold
      - Occurs after attack while gate
      - Amount of time the envelope stays at 1.0 after attack, then enters decay
    - Decay
      - Occurs after attack or hold
      - Amount of time the envelope attenuates to sustain level
    - Sustain
      - Occurs after attack or decay
      - The level at which the envelope stays at while the trigger gate is still on
    - Release
      - Occurs when the trigger gate is off and after attack or sustain
      - Amount of time the envelope attenuates to 0.0
  - Some envelopes that may be implemented here
    - AR
    - AD
    - ASR
    - ADSR
    - AHDSR

  - Ideally, it would be best to have a general implementation to let users
    create their own envelopes.
  */