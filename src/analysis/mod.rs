mod leaky_integrator;
mod peak_detector;
mod rms_detector;

pub use self::leaky_integrator::LeakyIntegrator as LeakyIntegrator;
pub use self::peak_detector::PeakEnvDetector    as PeakEnvDetector;
pub use self::rms_detector::RmsEnvDetector      as RmsEnvDetector;


/* Notes on envelope detection, also known as envelope following

  - The key component to envelope detection is integration, or averaging
    - You can't just average a signal by traditional means (divide the sum by
      the number of elements) because the number of samples is undefined.
    - So a leaky integrator is used to instead
      - A leaky integrator is a specific type of one pole filter with a
        special property that it's gains are complements

  - Envelope detctors depend on modifying a leaky integrator based on the input
    - The leaky integrator switches between an attack and release gain based on
      whether the envelope is rising or falling.
      - The gain amounts determine how long it takes for the integrator
        to reach the desired value, this is known as attack and relase time
      - When the input is rising above the stored envelope value, then the
        attack gain is used and will reach the desired value in the
        corresponding attack time
      - When the input is falling below the stored envelope value, then the
        release gain is used and will reach the desired value in the
        corresponding release time
    - Working with this changing integrator, it's easy to different envelope
      detectors
      - Peak envelope detector
        - x -> abs(x) -> leaky integrator -> y
      - RMS envelope detector
        - x -> x^2 -> leaky integrator -> sqrt(x) -> y

  - You may think, "so an envelope detector is just a leaky integrator with some
    additional processing before and/or after." Well, that's not true; envelope
    detectors have an additional property that separates it from a leaky integrator.
    - An envelope detector requires an absolute input signal (where the samples
      are always non-negative or non-positive, they always use the same sign),
      whereas a leaky integrator doesn't care what type of input it gets (because
      its job is to just integrate the signal).
    - So you can't just use a leaky integrator, and change its gains based on
      the stored value (what you would assume is the envelope), and call it an
      envelope detector. Additionally, you can't just use any pre or post
      processing on the integrator, unless it guarantees an absolute value.
*/
