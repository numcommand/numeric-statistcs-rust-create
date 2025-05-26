use super::variance;

/// Calculate statistical standard deviation for values.
///
/// Return NaN if the values are empty.
///
/// Filter NaN values in the stream.
/// 
/// # Example
///
/// ```rust
/// #[macro_use]
/// use numeric_statistics::assert_eq_f32;
/// use numeric_statistics::f32::standard_deviation::*;
/// let values = &[1.0, 2.0, 4.0];
/// let standard_deviation = standard_deviation(values);
/// assert_eq_f32!(standard_deviation, 1.5275253 as f32);
/// ```
/// 
pub fn standard_deviation<T: AsRef<[f32]>>(values: T) -> f32 {
    standard_deviation_with_variance(variance(&values))
}

/// Calculate statistical standard deviation for values, 
/// given a pre-calculated variance value.
///
/// Return NaN if the values are empty.
///
/// Filter NaN values in the stream.
/// 
/// # Example
///
/// ```rust
/// #[macro_use]
/// use numeric_statistics::assert_eq_f32;
/// use numeric_statistics::f32::{variance::*, standard_deviation::*};
/// let values = &[1.0, 2.0, 4.0];
/// let variance = variance(values);
/// let standard_deviation = standard_deviation_with_variance(variance);
/// assert_eq_f32!(standard_deviation, 1.5275253 as f32);
/// ```
/// 
pub fn standard_deviation_with_variance(variance: f32) -> f32 {
    variance.sqrt()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_eq_f32;

    #[test]
    fn test_empty() {
        let x: &[f32] = &[];
        assert!(standard_deviation(x).is_nan());
    }

    #[test]
    fn test_nan() {
        let x: &[f32] = &[f32::NAN];
        assert!(standard_deviation(x).is_nan());
    }

    #[test]
    fn test_value() {
        let x: &[f32] = &[1.0];
        assert_eq_f32!(standard_deviation(x), 0.0);
    }

    #[test]
    fn test_values_ascending() {
        let x = &[1.0, 2.0, 4.0];
        assert_eq_f32!(standard_deviation(x), 1.5275253);
    }

    #[test]
    fn test_values_ascending_and_nans() {
        let x = &[1.0, f32::NAN, 2.0, f32::NAN, 4.0];
        assert_eq_f32!(standard_deviation(x), 1.5275253);
    }

}
