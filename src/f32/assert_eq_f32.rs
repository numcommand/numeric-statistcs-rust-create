/// assert_eq_f32: assert that two floating point numbers are within 2.0 * f32::EPSILON.
///
/// # Example
///
/// ```rust
/// #[macro_use]
/// use numeric_statistics::assert_eq_f32;
/// let a: f32 = 0.333333333;
/// let b: f32 = 1.0 / 3.0;
/// assert_eq_f32!(a, b);
/// ```
///
#[macro_export]
macro_rules! assert_eq_f32 {
    ($a:expr, $b:expr $(,)?) => {{
        match (&$a, &$b) {
            (a, b) => {
                if (a == b) { return; }
                let epsilon = f32::EPSILON * 2.0;
                let abs_diff = if (a >= b) { a - b } else { b - a };
                assert!(
                    abs_diff <= epsilon,
                    concat!(
                        "assertion failed: `assert_eq_f32`\n",
                        " left: {},\n",
                        " right: {},\n",
                        " abs_diff: {},\n",
                        " ε: {}",
                    ),
                    a,
                    b,
                    abs_diff,
                    epsilon
                )
            }
        }
    }};
}
