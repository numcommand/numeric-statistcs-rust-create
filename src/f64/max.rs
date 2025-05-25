/// Calculate statistical max for values.
///
/// # Nan
/// 
/// Return NaN if the values are empty.
///
/// From <https://doc.rust-lang.org/std/primitive.f64.html#method.max>:
///
/// If one of the arguments is NaN, then the other argument is returned. This
/// follows the IEEE 754-2008 semantics for maxNum, except for handling of
/// signaling NaNs; this function handles all NaNs the same way and avoids
/// maxNum’s problems with associativity. This also matches the behavior of
/// libm’s fmax. In particular, if the inputs compare equal (such as for the case
/// of +0.0 and -0.0), either input may be returned non-deterministically.
///
pub fn max<T: AsRef<[f64]>>(values: T) -> f64 {
    let values = values.as_ref();
    if values.is_empty() { return f64::NAN; }
    values.iter().fold(f64::NAN, |a, x| f64::max(a, *x))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty() {
        let x: &[f64] = &[];
        assert!(max(x).is_nan());
    }

    #[test]
    fn test_nan() {
        let x: &[f64] = &[f64::NAN];
        assert!(max(x).is_nan());
    }

    #[test]
    fn test_value() {
        let x: &[f64] = &[1.0];
        assert_eq!(max(x), 1.0);
    }

    #[test]
    fn test_values_ascending() {
        let x = &[1.0, 2.0, 3.0];
        assert_eq!(max(x), 3.0);
    }

    #[test]
    fn test_values_ascending_and_nans() {
        let x = &[1.0, f64::NAN, 2.0, f64::NAN, 3.0];
        assert_eq!(max(x), 3.0);
    }

    #[test]
    fn test_values_descending() {
        let x = &[3.0, 2.0, 1.0];
        assert_eq!(max(x), 3.0);
    }

    #[test]
    fn test_values_descending_and_nans() {
        let x = &[3.0, f64::NAN, 2.0, f64::NAN, 1.0];
        assert_eq!(max(x), 3.0);
    }

}
