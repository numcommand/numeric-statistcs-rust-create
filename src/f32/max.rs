/// Calculate statistical max for values.
///
/// # Nan
/// 
/// Return NaN if the values are empty.
///
/// From <https://doc.rust-lang.org/std/primitive.f32.html#method.max>:
///
/// If one of the arguments is NaN, then the other argument is returned. This
/// follows the IEEE 754-2008 semantics for maxNum, except for handling of
/// signaling NaNs; this function handles all NaNs the same way and avoids
/// maxNum’s problems with associativity. This also matches the behavior of
/// libm’s fmax. In particular, if the inputs compare equal (such as for the case
/// of +0.0 and -0.0), either input may be returned non-deterministically.
///
pub fn max<T: AsRef<[f32]>>(values: T) -> f32 {
    let values = values.as_ref();
    if values.is_empty() { return f32::NAN; }
    values.iter().fold(f32::NAN, |a, x| f32::max(a, *x))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty() {
        let x: &[f32] = &[];
        assert!(max(x).is_nan());
    }

    #[test]
    fn test_nan() {
        let x: &[f32] = &[f32::NAN];
        assert!(max(x).is_nan());
    }

    #[test]
    fn test_value() {
        let x: &[f32] = &[1.0];
        assert_eq!(max(x), 1.0);
    }

    #[test]
    fn test_values_ascending() {
        let x = &[1.0, 2.0, 3.0];
        assert_eq!(max(x), 3.0);
    }

    #[test]
    fn test_values_ascending_and_nans() {
        let x = &[1.0, f32::NAN, 2.0, f32::NAN, 3.0];
        assert_eq!(max(x), 3.0);
    }

    #[test]
    fn test_values_descending() {
        let x = &[3.0, 2.0, 1.0];
        assert_eq!(max(x), 3.0);
    }

    #[test]
    fn test_values_descending_and_nans() {
        let x = &[3.0, f32::NAN, 2.0, f32::NAN, 1.0];
        assert_eq!(max(x), 3.0);
    }

}
