//! # lieval
//! 
//! `lieval` is a lightweight Rust crate for parsing and evaluating mathematical expressions from strings.
//! 
//! ## Features
//! 
//! - Parse and evaluate simple mathematical expressions.
//!     - Basic arithmetic operations: `+`, `-`, `*`, `/`, `%`
//!     - Parentheses for expression grouping
//!     - Common mathematical functions: `sin`, `cos`, `tan`, `pow`, `sqrt`, `exp`, `ln`, etc.
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
//! assert_eq!(expr.eval(), Ok(vec![2.0]));
//! ```
//! 
//! You can assign numerical values to variables and evaluate them using `Context`.
//! 
//! ```rust
//! use lieval::*;
//! 
//! let mut context = Context::new();
//! assert_eq!(
//!     eval_from_str_with_context("1 / x", context.set_value("x", 2.0)),
//!     Ok(vec![0.5])
//! );
//! 
//! let mut expr = Expr::new("sqrt(2+x)").unwrap();
//! assert_eq!(expr.set_var("x", 2.0).eval(), Ok(vec![2.0]));
//! 
//! let mut expr = Expr::new("sqrt(2+x+y)").unwrap();
//! assert_eq!(expr.set_var("x", 2.0).set_var("y", 5.0).eval(), Ok(vec![3.0]));
//! ```
//! 
//! You can evaluate multiple expressions separated by commas.
//! 
//! ```rust
//! use lieval::*;
//! 
//! assert_eq!(
//!     eval_from_str("1 + 2, sin(3 + 0.14), 7 % 3"), 
//!     Ok(vec![3.0, (3.14f64).sin(), 7.0 % 3.0])
//! );
//! ```
//! 
//! You can efficiently evaluate by precomputing using `partial_eval`.
//! 
//! ```rust
//! use lieval::*;
//! 
//! let mut expr = Expr::new("a1 + a2 * sin(x)").unwrap();
//! expr.set_var("a1", 1.0)
//!     .set_var("a2", 0.5)
//!     .partial_eval()
//!     .unwrap();
//! let mut x = 1.0;
//! for _ in 0..10 {
//!     x = expr.set_var("x", x).eval().unwrap()[0];
//!     assert_eq!(expr.set_var("x", x).eval(), Ok(vec![1.0 + 0.5 * x.sin()]));
//! }
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