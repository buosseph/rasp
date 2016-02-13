mod sine;

pub use self::sine::Sine as Sine;

/* Notes
  - The oscillators in this module work generally the same, being based on a
    numerically-controlled oscillator (aka a phase accumulator)
  - Each oscillator treats phase as an accumulator, adding an increment based
    on the oscillator frequency and sample rate
    - The phase accumulator is first wrapped (genearlly the phase accumulator is unipolar
      and wrapped between 0 and 1)
    - The phase accumulator is then passed through the oscillator function to determine
      the sample value
    - The phase accumulator is incremented and the sample value is output
 */