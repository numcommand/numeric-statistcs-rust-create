#[allow(unused_macros)]
macro_rules! assert_eq_float {
    ($a:expr, $b:expr $(,)?) => {{
        match (&$a, &$b) {
            (a, b) => {
                let epsilon = 2.0 * f64::EPSILON;
                let abs_diff = if (a >= b) { a - b } else { b - a };
                assert!(
                    abs_diff <= epsilon,
                    concat!(
                        "assertion failed: `assert_eq_float`\n",
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
