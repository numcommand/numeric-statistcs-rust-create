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
pub fn average<T: AsRef<[f64]>>(values: T) -> f64 {
    let values = values.as_ref();
    if values.is_empty() { return f64::NAN; }
    let mut sum: f64 = 0.0;
    let mut len: usize = 0;
    values.iter().for_each(|x|
        if !x.is_nan() {
            sum += *x;
            len += 1;
        }
    );
    sum / len as f64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty() {
        let x: &[f64] = &[];
        assert!(average(x).is_nan());
    }

    #[test]
    fn test_nan() {
        let x: &[f64] = &[f64::NAN];
        assert!(average(x).is_nan());
    }

    #[test]
    fn test_value() {
        let x: &[f64] = &[1.0];
        assert_eq!(average(x), 1.0);
    }

    #[test]
    fn test_values_ascending() {
        let x = &[1.0, 2.0, 4.0];
        assert_eq!(average(x), 2.3333333333333333);
    }

    #[test]
    fn test_values_ascending_and_nans() {
        let x = &[1.0, f64::NAN, 2.0, f64::NAN, 4.0];
        assert_eq!(average(x), 2.3333333333333333);
    }

    #[test]
    fn test_values_descending() {
        let x = &[4.0, 2.0, 1.0];
        assert_eq!(average(x), 2.3333333333333333);
    }

    #[test]
    fn test_values_descending_and_nans() {
        let x = &[4.0, f64::NAN, 2.0, f64::NAN, 1.0];
        assert_eq!(average(x), 2.3333333333333333);
    }

}

