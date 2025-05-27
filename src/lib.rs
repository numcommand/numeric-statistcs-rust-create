//! # Numeric Statistics Rust crate
//! 
//! Numeric Statistics provides functions for min, max, average, variance, standard deviation, and more to come.
//! 
//! ## Example
//! 
//! ```rust
//! #[macro_use]
//! use numeric_statistics::assert_eq_f64;
//! use numeric_statistics::f64::{
//!     min::*,
//!     max::*,
//!     average::*,
//!     variance::*, 
//!     standard_deviation::*
//! };
//! 
//! let values = &[1.0, 2.0, 4.0];
//! let min = min(values);
//! let max = max(values);
//! let average = average(values);
//! let variance = variance(values);
//! let standard_deviation = standard_deviation(values);
//! 
//! assert_eq!(min, 1.0);
//! assert_eq!(max, 4.0);
//! assert_eq_f64!(average, 2.3333333333333333 as f64);
//! assert_eq_f64!(variance, 2.3333333333333333 as f64);
//! assert_eq_f64!(standard_deviation, 1.5275252316519465 as f64);
//! ```
//! 
//! ## All
//! 
//! You can create all the numeric statistics all within one struct:
//! 
//! ```rust
//! #[macro_use]
//! use numeric_statistics::assert_eq_f64;
//! use numeric_statistics::f64::all::All;
//! 
//! let values = &[1.0, 2.0, 4.0];
//! let all = All::new(&values);
//! println!("{}", all);
//! 
//! assert_eq!(all.min, 1.0);
//! assert_eq!(all.max, 4.0);
//! assert_eq_f64!(all.average, 2.3333333333333333 as f64);
//! assert_eq_f64!(all.variance, 2.3333333333333333 as f64);
//! assert_eq_f64!(all.standard_deviation, 1.5275252316519465 as f64);
//! ```
//! 
//! Output:
//! 
//! ```stdout
//! min: 1.0
//! max: 4.0
//! average: 2.3333333333333335
//! variance: 2.333333333333333
//! standard deviation: 1.527525231651946
//! ```
//! 
//! ## Num Command
//! 
//! This is a work-in-progress to translate the Num Command software from POSIX into Rust.
//! 
//! <https://github.com/numcommand/num>
//! 

pub mod f32;
pub mod f64;