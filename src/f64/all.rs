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

}
