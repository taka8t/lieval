#![feature(test)]

extern crate test;

use lieval::*;
use test::Bencher;

const N: usize = 10000;

const EXPR1: &str = "2 * 3 + cos(PI)";
const EXPR2: &str = "a1 * a2 + sin(x+2)";
const EXPR3: &str = "cos(x) + sin(y)";

#[bench]
fn eval(b: &mut Bencher) {
    b.iter(|| {
        eval_from_str(EXPR1).unwrap();
    });
}

#[bench]
fn eval_obj(b: &mut Bencher) {
    let expr_obj = Expr::new(EXPR1).unwrap();
    b.iter(|| {
        expr_obj.eval().unwrap();
    });
}


#[bench]
fn eval_with_context(b: &mut Bencher) {
    let mut expr_obj = Expr::new(EXPR2).unwrap();
    let mut xn = 1.0;
    b.iter(|| {
        for _ in 0..N {
            let nx = expr_obj
            .set_var("a1", 2.0)
            .set_var("a2", 3.0)
            .set_var("x", xn)
            .eval().unwrap();
            xn = nx;
        }
    });
}

#[bench]
fn eval_with_context2(b: &mut Bencher) {
    let mut expr_obj = Expr::new(EXPR3).unwrap();
    let mut xn = 1.0;
    b.iter(|| {
        for _ in 0..N {
            let nx = expr_obj
            .set_var("y", 1.0)
            .set_var("x", xn)
            .eval().unwrap();
            xn = nx;
        }
    });
}

#[bench]
fn partial_eval_with_context(b: &mut Bencher) {
    let mut expr_obj = Expr::new(EXPR2).unwrap();
    expr_obj.set_var("a1", 2.0).set_var("a2", 3.0).partial_eval().unwrap();

    let mut xn = 1.0;
    b.iter(|| {
        for _ in 0..N {
            let nx = expr_obj
                .set_var("x", xn)
                .eval().unwrap();
            xn = nx;
        }
    });
}

#[bench]
fn partial_eval_with_context2(b: &mut Bencher) {
    let mut expr_obj = Expr::new(EXPR3).unwrap();
    expr_obj.set_var("y", 1.0).partial_eval().unwrap();

    let mut xn = 1.0;
    b.iter(|| {
        for _ in 0..N {
            let nx = expr_obj
                .set_var("x", xn)
                .eval().unwrap();
            xn = nx;
        }
    });
}