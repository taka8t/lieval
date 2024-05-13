//! # lieval
//! 
//! `lieval` is a lightweight Rust crate for parsing and evaluating mathematical expressions from strings.
//! 
//! ## Features
//! 
//! - Parse and evaluate simple mathematical expressions.
//!     - Basic arithmetic operations: `+`, `-`, `*`, `/`, `%`
//!     - Parentheses for expression grouping
//!     - Common mathematical functions: `sin`, `cos`, `atan`, `cosh`, `pow`, `sqrt`, `hypot`, `exp`, `ln`, `div_euclid`, `floor` etc...
//!     - mathematical constants such as `PI`, `TAU`, and `E`.
//! - Support for variables, operators, and functions.
//! - Minimal dependencies.
//! - Provides a simple and easy-to-use API.
//! 
//! ## Usage
//! 
//! Add the `lieval` crate to your `Cargo.toml` file:
//! 
//! ```toml
//! [dependencies]
//! lieval = "<version>"
//! ```
//! 
//! Then, in your Rust code:
//! 
//! ```rust
//! use lieval::*;
//! 
//! assert_eq!(eval_from_str("1.0 + 2 * (3 - 1)"), Ok(vec![5.0]));
//! assert_eq!(
//!     eval_from_str("1.0 - sin(3.14 / 2) * powf(1.5, 2.5)"),
//!     Ok(vec![1.0 - (3.14f64 / 2.0).sin() * 1.5f64.powf(2.5)])
//! );
//! 
//! let mut expr = Expr::new("sqrt(4)").unwrap();
//! assert_eq!(expr.eval(), Ok(2.0));
//! 
//! // using macro `ex!`
//! assert_eq!(ex!("sqrt(4)").eval(), Ok(2.0));
//! ```
//! 
//! You can assign numerical values to variables and evaluate them using `Context`.
//! 
//! ```rust
//! # use lieval::*;
//! #
//! let mut context = Context::new();
//! 
//! assert_eq!(
//!     eval_from_str_with_context("1 / x", context.set_value("x", 2.0)),
//!     Ok(vec![0.5])
//! );
//! 
//! assert_eq!(ex!("sqrt(2+x)").set_var("x", 2.0).eval(), Ok(2.0));
//! assert_eq!(ex!("sqrt(2+x+y)").set_var("x", 2.0).set_var("y", 5.0).eval(), Ok(3.0));
//! ```
//! 
//! You can use custom functions.
//! 
//! ```rust
//! # use lieval::*;
//! # 
//! let mut context = Context::new();
//! 
//! assert_eq!(
//!     eval_from_str_with_context("1 + func(2,3)", context.set_func("func", 2, |x| x[0] + x[1])),
//!     Ok(vec![6.0])
//! );
//! assert_eq!(
//!     ex!("1 + func(x)").set_func("func", 1, |x| x[0] * 2.0).set_var("x", 1.0).eval(),
//!     Ok(3.0)
//! );
//! ```
//! 
//! You can evaluate multiple expressions separated by commas or semicolons.
//! 
//! ```rust
//! # use lieval::*;
//! # 
//! assert_eq!(
//!     eval_from_str("1 + 2, sin(3 + 0.14); 7 % 3"), 
//!     Ok(vec![3.0, (3.14f64).sin(), 7.0 % 3.0])
//! );
//! 
//! let mut expr = ex!("sqrt(1+x); 1+3, hypot(x,4)");
//! assert_eq!(expr.set_var("x", 3.0).evals(), Ok(vec![2.0, 4.0, 5.0]));
//! assert_eq!(expr.eval(), Ok(2.0));
//! assert_eq!(expr.eval_index(2), Ok(5.0));
//! ```
//! 
//! You can efficiently evaluate by precomputing using `partial_eval`.
//! 
//! ```rust
//! # use lieval::*;
//! # 
//! let mut expr = ex!("a1 + a2 * sin(x)");
//! expr.set_var("a1", 1.0)
//!     .set_var("a2", 0.5)
//!     .partial_eval()
//!     .unwrap();
//! let mut x = 1.0;
//! for _ in 0..10 {
//!     x = expr.set_var("x", x).eval().unwrap();
//!     assert_eq!(expr.set_var("x", x).eval(), Ok(1.0 + 0.5 * x.sin()));
//! }
//! ```
//! 
//! You can perform arithmetic operations between Expr objects.
//! 
//! ```rust
//! # use lieval::*;
//! # 
//! let expr1 = Expr::new("1+x").unwrap();
//! assert_eq!((expr1 + ex!("2*x")).set_var("x", 2.0).eval(), Ok(7.0));
//! 
//! assert_eq!((ex!("1+x, 2+x, 3+x") + ex!("2*x, 3*x, 4*x")).set_var("x", 2.0).evals(), Ok(vec![7.0, 10.0, 13.0]));
//! 
//! // broadcasting
//! let expr1 = Expr::new("1+x").unwrap();
//! let expr2 = Expr::new("2*x, 3*x, 4*x").unwrap();
//! assert_eq!((ex!("1+x") * ex!("2*x, 3*x, 4*x")).set_var("x", 2.0).evals(), Ok(vec![12.0, 18.0, 24.0]));
//! assert_eq!((ex!("1+x") * 2.0 * ex!("x") + ex!("2*x, 3*x, 4*x") + 1.0).set_var("x", 2.0).evals(), Ok(vec![17.0, 19.0, 21.0]));
//! 
//! // If variables conflict, the variable in the left expression takes precedence,
//! // so use partial_eval beforehand.
//! let mut expr1 = Expr::new("2*x").unwrap();
//! expr1.set_var("x", 3.0).partial_eval().unwrap();
//! assert_eq!((-ex!("1+x") * expr1).set_var("x", 2.0).eval(), Ok(-18.0));
//! ```
//! 
//! ## API Documentation
//! 
//! Detailed API documentation can be found [here](https://docs.rs/lieval).
//! 
//! ## License
//! 
//! This project is licensed under the MIT license.


mod eval;
mod parse;
mod token;
mod context;
mod error;
mod util;

pub use crate::{
    eval::{Expr, eval_from_str, eval_from_str_with_context},
    context::Context,
    error::EvalError,
};