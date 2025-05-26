use super::average;

/// Calculate statistical variance for values.
///
/// Return NaN if the values are empty.
///
/// Filter NaN values in the stream.
///
/// # Example
///
/// ```rust
/// #[macro_use]
/// use numeric_statistics::assert_eq_f64;
/// use numeric_statistics::f64::variance::*;
/// let values = &[1.0, 2.0, 4.0];
/// let variance = variance(values);
/// assert_eq_f64!(variance,  2.3333333333333333 as f64);
/// ```
/// 
pub fn variance<T: AsRef<[f64]>>(values: T) -> f64 {
    let values = values.as_ref();
    if values.is_empty() { return f64::NAN; }
    let average = average(values);
    variance_with_average(values, average)
}

/// Calculate statistical variance for values, 
/// given a pre-calculated average value.
///
/// Return NaN if the values are empty.
///
/// Filter NaN values in the stream.
/// 
/// # Example
///
/// ```rust
/// #[macro_use]
/// use numeric_statistics::assert_eq_f64;
/// use numeric_statistics::f64::{average::*, variance::*};
/// let values = &[1.0, 2.0, 4.0];
/// let average = average(values);
/// let variance = variance_with_average(values, average);
/// assert_eq_f64!(variance, 2.3333333333333333 as f64);
/// ```
/// 
pub fn variance_with_average<T: AsRef<[f64]>>(values: T, average: f64) -> f64 {
    let values = values.as_ref();
    if values.is_empty() { return f64::NAN; }
    let mut delta_square_sum: f64 = 0.0;
    let mut len: usize = 0;
    values.iter().for_each(|x|
        if !x.is_nan() {
            let delta = *x - average;
            delta_square_sum += delta * delta;
            len += 1;
        }
    );
    match len {
        0 => return f64::NAN,
        1 => return 0.0,
        x => delta_square_sum / (x - 1) as f64
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_eq_f64;

    #[test]
    fn test_empty() {
        let x: &[f64] = &[];
        assert!(variance(x).is_nan());
    }

    #[test]
    fn test_nan() {
        let x: &[f64] = &[f64::NAN];
        assert!(variance(x).is_nan());
    }

    #[test]
    fn test_value() {
        let x: &[f64] = &[1.0];
        assert_eq_f64!(variance(x), 0.0);
    }

    #[test]
    fn test_values_ascending() {
        let x = &[1.0, 2.0, 4.0];
        assert_eq_f64!(variance(x), 2.3333333333333333);
    }

    #[test]
    fn test_values_ascending_and_nans() {
        let x = &[1.0, f64::NAN, 2.0, f64::NAN, 4.0];
        assert_eq_f64!(variance(x), 2.3333333333333333);
    }

}
