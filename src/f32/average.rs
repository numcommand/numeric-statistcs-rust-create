//! Statistics module for calculating various statistical measures.
//! 
//! These are simple naive implementations, just enough to work for
//! this crate's purpose of network monitoring, such as for latency.

/// Calculate statistical average for values.
///
/// Return NaN if the values are empty.
/// 
/// Filter NaN values in the stream.
/// 
pub fn average<T: AsRef<[f32]>>(values: T) -> f32 {
    let values = values.as_ref();
    if values.is_empty() { return f32::NAN; }
    let mut sum: f32 = 0.0;
    let mut len: usize = 0;
    values.iter().for_each(|x|
        if !x.is_nan() {
            sum += *x;
            len += 1;
        }
    );
    sum / len as f32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty() {
        let x: &[f32] = &[];
        assert!(average(x).is_nan());
    }

    #[test]
    fn test_nan() {
        let x: &[f32] = &[f32::NAN];
        assert!(average(x).is_nan());
    }

    #[test]
    fn test_value() {
        let x: &[f32] = &[1.0];
        assert_eq_float!(average(x), 1.0);
    }

    #[test]
    fn test_values_ascending() {
        let x = &[1.0, 2.0, 4.0];
        assert_eq_float!(average(x), 2.3333333);
    }

    #[test]
    fn test_values_ascending_and_nans() {
        let x = &[1.0, f32::NAN, 2.0, f32::NAN, 4.0];
        assert_eq_float!(average(x), 2.3333333);
    }

    #[test]
    fn test_values_descending() {
        let x = &[4.0, 2.0, 1.0];
        assert_eq_float!(average(x), 2.3333333);
    }

    #[test]
    fn test_values_descending_and_nans() {
        let x = &[4.0, f32::NAN, 2.0, f32::NAN, 1.0];
        assert_eq_float!(average(x), 2.3333333);
    }

}

