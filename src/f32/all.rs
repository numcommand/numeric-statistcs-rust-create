#[derive(Debug, Clone)]
pub struct All {
    pub min: f32,
    pub max: f32,
    pub average: f32,
    pub variance: f32,
    pub standard_deviation: f32,
}

use crate::f32::{
    min::*,
    max::*,
    average::*,
    variance::*,
    standard_deviation::*,
};

impl All {
    pub fn new<T: AsRef<[f32]>>(values: T) -> All {
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
    use crate::assert_eq_f32;

    #[test]
    fn test_new() {
        let x = &[1.0, 2.0, 4.0];
        let all = All::new(x);
        assert_eq!(all.min, 1.0);
        assert_eq!(all.max, 4.0);
        assert_eq_f32!(all.average, 2.3333333);
        assert_eq_f32!(all.variance, 2.3333333);
        assert_eq_f32!(all.standard_deviation, 1.5275253);
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
                "average: 2.3333333\n",
                "variance: 2.3333335\n",
                "standard deviation: 1.5275253\n"
            )
        );
    }

}
