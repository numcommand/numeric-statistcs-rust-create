/// # Example
///
/// ```rust
/// #[macro_use]
/// use numeric_statistics::assert_eq_f64;
/// let a: f64 = 0.33333333333333333;
/// let b: f64 = 1.0 / 3.0;
/// assert_eq_f64!(a, b);
/// ```
/// 
#[macro_export]
macro_rules! assert_eq_f64 {
    ($a:expr, $b:expr $(,)?) => {{
        match (&$a, &$b) {
            (a, b) => {
                if (a == b) { return; }
                let epsilon = f64::EPSILON * 2.0;
                let abs_diff = if (a >= b) { a - b } else { b - a };
                assert!(
                    abs_diff <= epsilon,
                    concat!(
                        "assertion failed: `assert_eq_f64`\n",
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
