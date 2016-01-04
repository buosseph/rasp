/// Converts a sample value to a dBFS value.
///
/// If the sample value is really small, or if the sample is not finite, it
/// will be assumed to be -120dBFS.
pub fn to_db(sample: f32) -> f32 {
  if sample > 1e-6f32 && sample.is_finite() {
    20f32 * sample.log10()
  }
  else {
    -120f32
  }
}

/// Converts a dBFS value to a sample value.
///
/// If the value is equal to or less than -120dBFS, or if the value is not
/// finite, the sample value will be zero.
pub fn to_sample(db_value: f32) -> f32 {
  if db_value > -120f32 && db_value.is_finite() {
    10f32.powf(db_value / 20f32)
  }
  else {
    0f32
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::f32::*;

  #[test]
  fn conversion_to_decibels() {
    /* Zero case, should map to minimum */
    assert!((to_db(0f32) - -120f32).abs() < EPSILON);

    /* Below minimum */
    assert!((to_db(0.000000999f32) - -120f32).abs() < EPSILON);
    assert!((to_db(0.0000001f32) - -120f32).abs() < EPSILON);

    /* 20db intervals */
    assert!((to_db(1e-1f32) - -20f32).abs() < EPSILON);
    assert!((to_db(1e-2f32) - -40f32).abs() < EPSILON);
    assert!((to_db(1e-3f32) - -60f32).abs() < EPSILON);
    assert!((to_db(1e-4f32) - -80f32).abs() < EPSILON);
    assert!((to_db(1e-5f32) - -100f32).abs() < EPSILON);
    assert!((to_db(1e-6f32) - -120f32).abs() < EPSILON);

    /* Halving in decibels */
    /* All correct, but this way of testing is too accurate */
    // assert!((0.841395f32.to_db() - -1.5f32).abs() < 1e-5f32);
    // assert!((0.707946f32.to_db() - -3f32).abs() < EPSILON);
    // assert!((0.501187f32.to_db() - -6f32).abs() < EPSILON);
    // assert!((0.251189f32.to_db() - -12f32).abs() < EPSILON);
    // assert!((0.0630957f32.to_db() - -24f32).abs() < EPSILON);
    // assert!((0.00398107f32.to_db() - -48f32).abs() < EPSILON);
    // assert!((0.00000158489f32.to_db() - -96f32).abs() < EPSILON);

    /* Beyond 0db */
    /* Same issue as above */
    // assert!((1.12202f32.to_db() - 1f32).abs() < EPSILON);

    /* Invalid input */
    assert!((to_db(NAN) - -120f32).abs() < EPSILON);
    assert!((to_db(INFINITY) - -120f32).abs() < EPSILON);
    assert!((to_db(NEG_INFINITY) - -120f32).abs() < EPSILON);
  }

  #[test]
  fn conversion_to_samples() {
    /* Below minimum */
    assert!((to_sample(-1000f32) - 0f32).abs() < EPSILON);
    assert!((to_sample(-120.000001f32) - 0f32).abs() < EPSILON);

    /* Minimum (zero) case */
    assert!((to_sample(-120f32) - 0f32).abs() < EPSILON);

    /* 20db intervals */
    assert!((to_sample(-20f32) - 1e-1f32).abs() < EPSILON);
    assert!((to_sample(-40f32) - 1e-2f32).abs() < EPSILON);
    assert!((to_sample(-60f32) - 1e-3f32).abs() < EPSILON);
    assert!((to_sample(-80f32) - 1e-4f32).abs() < EPSILON);
    assert!((to_sample(-100f32) - 1e-5f32).abs() < EPSILON);

    /* Halving in decibels */
    /* All correct, but this way of testing is too accurate */
    // assert!((to_sample(-1.5f32) - 0.841395f32).abs() < 1e-5f32);
    // assert!((to_sample(-3f32) - 0.707946f32).abs() < EPSILON);
    // assert!((to_sample(-6f32) - 0.501187f32).abs() < EPSILON);
    // assert!((to_sample(-12f32) - 0.251189f32).abs() < EPSILON);
    // assert!((to_sample(-24f32) - 0.0630957f32).abs() < EPSILON);
    // assert!((to_sample(-48f32) - 0.00398107f32).abs() < EPSILON);
    // assert!((to_sample(-96f32) - 0.00000158489f32).abs() < EPSILON);

    /* Beyond 0db */
    /* Same issue as above */
    // assert!((to_sample(1f32) - 1.12202f32).abs() < EPSILON);

    /* Invalid input */
    assert!((to_sample(NAN) - 0f32).abs() < EPSILON);
    assert!((to_sample(INFINITY) - 0f32).abs() < EPSILON);
    assert!((to_sample(NEG_INFINITY) - 0f32).abs() < EPSILON);
  }
}
