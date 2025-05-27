#[derive(Debug, Clone)]
pub struct All {
    pub min: f64,
    pub max: f64,
    pub average: f64,
    pub variance: f64,
    pub standard_deviation: f64,
}

use crate::f64::{
    min::*,
    max::*,
    average::*,
    variance::*,
    standard_deviation::*,
};

impl All {
    pub fn new<T: AsRef<[f64]>>(values: T) -> All {
        let min = min(&values);
        let max = max(&values);
        let average = average(&values);
        let variance = variance_with_average(&values, average);
        let standard_deviation = standard_deviation_with_variance(variance);
        All { 
            min, 
            max, 
            average, 
            variance,
            standard_deviation  
        }
    }
}

use std::fmt;
impl fmt::Display for All {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            concat!(
                "min: {:?}\n",
                "max: {:?}\n",
                "average: {:?}\n",
                "variance: {:?}\n",
                "standard deviation: {:?}\n",
            ),
            self.min,
            self.max,
            self.average,
            self.variance,
            self.standard_deviation,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_eq_f64;

    #[test]
    fn test_new() {
        let x = &[1.0, 2.0, 4.0];
        let all: All = All::new(x);
        assert_eq!(all.min, 1.0);
        assert_eq!(all.max, 4.0);
        assert_eq_f64!(all.average, 2.3333333333333335);
        assert_eq_f64!(all.variance, 2.3333333333333335);
        assert_eq_f64!(all.standard_deviation, 1.5275252316519465);
    }

    #[test]
    fn test_fmt() {
        let x = &[1.0, 2.0, 4.0];
        let all = All::new(x);
        assert_eq!(
            all.to_string(),
            concat!(
                "min: 1.0\n",
                "max: 4.0\n",
                "average: 2.3333333333333335\n",
                "variance: 2.333333333333333\n",
                "standard deviation: 1.5275252316519465\n"
            )
        );
    }

}
