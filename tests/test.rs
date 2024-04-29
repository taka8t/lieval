use lieval::*;

#[test]
fn operator_test() {
    assert_eq!(eval_from_str("1 + 2"), Ok(vec![3.0]));
    assert_eq!(eval_from_str("1 + -2"), Ok(vec![-1.0]));
    assert_eq!(eval_from_str("-1 + 2"), Ok(vec![1.0]));
    assert_eq!(eval_from_str("1 - 2"), Ok(vec![-1.0]));
    assert_eq!(eval_from_str("2 * 3"), Ok(vec![6.0]));
    assert_eq!(eval_from_str("2 * -3"), Ok(vec![-6.0]));
    assert_eq!(eval_from_str("2 / 4"), Ok(vec![0.5]));
    assert_eq!(eval_from_str("-2 / 4"), Ok(vec![-0.5]));
    assert_eq!(eval_from_str("-1.0"), Ok(vec![-1.0]));
    assert_eq!(eval_from_str("7 % 3.0"), Ok(vec![1.0]));
    assert_eq!(eval_from_str("7.0 % -3.0"), Ok(vec![1.0]));
    assert_eq!(eval_from_str("-7 % 3"), Ok(vec![-1.0]));
}

#[test]
fn function_test() {
    assert_eq!(eval_from_str("sin(-1 + 2 * 3)"), Ok(vec![(5f64).sin()]));
    assert_eq!(eval_from_str("cos(-1 + 2 * 3)"), Ok(vec![(5f64).cos()]));
    assert_eq!(eval_from_str("tan(-1 + 2 * 3)"), Ok(vec![(5f64).tan()]));
    assert_eq!(eval_from_str("exp(-1 + 2 * 3)"), Ok(vec![(5f64).exp()]));
    assert_eq!(eval_from_str("sqrt(-1 + 2 * 3)"), Ok(vec![(5f64).sqrt()]));
    assert_eq!(eval_from_str("ln(-1 + 2 * 3)"), Ok(vec![(5f64).ln()]));
    assert_eq!(eval_from_str("powi(1 - 2 * 3, 2)"), Ok(vec![25.0]));
    assert_eq!(eval_from_str("pow(1 - 2 * 3, 2)"), Ok(vec![(-5f64).powf(2.0)]));
    assert_eq!(eval_from_str("-powf(1 - 2 * 3, 2)"), Ok(vec![-5f64.powf(2.0)]));
}

#[test]
fn assoc_test() {
    assert_eq!(eval_from_str("1 + 2 * 3"), Ok(vec![7.0]));
    assert_eq!(eval_from_str("(1 + 2) * 3"), Ok(vec![9.0]));
    assert_eq!(eval_from_str("((1 + 2)) * 3"), Ok(vec![9.0]));
    assert_eq!(eval_from_str("(1 - (2 + 3)) * 5"), Ok(vec![-20.0]));
    assert_eq!(eval_from_str("powf((3-2)*5, sin(5-(3-1)))"), Ok(vec![((3f64-2.0)*5.0).powf(3f64.sin())]));
}

#[test]
fn custom_func_test() {
    let mut context = Context::new();
    assert_eq!(
        eval_from_str_with_context("1 + func()", context.set_func("func", 0, |_| 2.0)),
        Ok(vec![3.0])
    );
    assert_eq!(
        eval_from_str_with_context("1 + func(2)", context.set_func("func", 1, |x| x[0])),
        Ok(vec![3.0])
    );
    assert_eq!(
        eval_from_str_with_context("1 + func(2,3)", context.set_func("func", 2, |x| x[0] + x[1])),
        Ok(vec![6.0])
    );
    assert_eq!(
        eval_from_str_with_context("1 + func(x)", context.set_func("func", 1, |x| x[0] * 2.0).set_value("x", 1.0)),
        Ok(vec![3.0])
    );
    assert_eq!(
        eval_from_str_with_context("1 + func(sin(x))", context.set_func("func", 1, |x| x[0] * 2.0).set_value("x", 1.0)),
        Ok(vec![1.0 + (1.0f64.sin()) * 2.0])
    );
    assert_eq!(
        eval_from_str_with_context("1 + func(func(x))", context.set_func("func", 1, |x| x[0] * 2.0).set_value("x", 1.0)),
        Ok(vec![5.0])
    );

    assert_eq!(
        eval_from_str_with_context("1 + func(1,x,y,4)",
            context.set_func("func", 4, |x| x[0] + x[1] * x[2] + x[3])
            .set_value("x", 2.0)
            .set_value("y", 3.0)
        ),
        Ok(vec![12.0])
    );

    let mut expr_obj = Expr::new("1 + func(x)").unwrap();
    assert_eq!(expr_obj.set_var("x", 3.0).set_func("func", 1, |x| x[0] * 2.0).eval(), Ok(vec![7.0]));

    let mut expr_obj = Expr::new("1 + func1(x) + func2()").unwrap();
    assert_eq!(
        expr_obj.set_var("x", 3.0)
        .set_func("func1", 1, |x| x[0] * 2.0)
        .partial_eval().unwrap()
        .set_func("func2", 0, |_| 1.5).eval(), 
        Ok(vec![8.5])
    );

    let mut expr_obj = Expr::new("1 + func1(1, x, y, 4) + z").unwrap();
    assert_eq!(
        expr_obj.set_func("func1", 4, |x| x[0] + x[1] * x[2] + x[3])
        .set_var("x", 2.0)
        .set_var("y", 3.0)
        .partial_eval().unwrap()
        .set_var("z", -12.0).eval(), 
        Ok(vec![0.0])
    );
}

#[test]
fn expr_test() {
    assert_eq!(
        eval_from_str("0.5 + 3.0 * -cos(sin(1.0 - 2.0) + 1.5) + 5.5"),
        Ok(vec![0.5 + 3.0 * -((1f64 - 2.0).sin() + 1.5).cos() + 5.5])
    );
    assert_eq!(eval_from_str("1.0 + 2 * (3 - 1)"), Ok(vec![5.0]));
    assert_eq!(
        eval_from_str("1.0 - sin(3.14 / 2) * powf(1.5, 2.5)"),
        Ok(vec![1.0 - (3.14f64 / 2.0).sin() * 1.5f64.powf(2.5)])
    );

    assert_eq!(eval_from_str("1 + 2, sin(3 + 0.14), 7 % 3"), Ok(vec![3.0, (3.14f64).sin(), 7.0 % 3.0]));
}

#[test]
fn expr_with_context_test() {
    let mut context = Context::new();
    assert_eq!(
        eval_from_str_with_context("1 / x", context.set_value("x", 2.0)),
        Ok(vec![0.5])
    );
    assert_eq!(
        eval_from_str_with_context("0.5 + x * -cos(sin(1.0 - 2.0) + 1.5) + 5.5", context.set_value("x", 3.0)),
        Ok(vec![0.5 + 3.0 * -((1f64 - 2.0).sin() + 1.5).cos() + 5.5])
    );
    assert_eq!(
        eval_from_str_with_context("0.5 + x * -cos(sin(1.0 - 2.0) + 1.5) + 5.5", context.set_value("x", -1.5)),
        Ok(vec![0.5 + -1.5 * -((1f64 - 2.0).sin() + 1.5).cos() + 5.5])
    );
}

#[test]
fn expr_object_test() {
    let mut expr_obj = Expr::new("sqrt(4)").unwrap();
    assert_eq!(expr_obj.eval(), Ok(vec![2.0]));

    let mut expr_obj = Expr::new("sqrt(2+x)").unwrap();
    assert_eq!(expr_obj.set_var("x", 2.0).eval(), Ok(vec![2.0]));

    let mut expr_obj = Expr::new("sqrt(2+x+y)").unwrap();
    assert_eq!(expr_obj.set_var("x", 2.0).set_var("y", 5.0).eval(), Ok(vec![3.0]));

    let expr1 = "0.5 + 3.0 * -cos(sin(1.0 - 2.0) + 1.5) + 5.5";
    let expr2 = "0.5 + x * -cos(sin(1.0 - 2.0) + 1.5) + 5.5";
    let result1 = Ok(vec![0.5 + 3.0 * -((1f64 - 2.0).sin() + 1.5).cos() + 5.5]);
    let result2 = Ok(vec![0.5 + -1.5 * -((1f64 - 2.0).sin() + 1.5).cos() + 5.5]);

    let mut expr_obj = Expr::new(&expr1).unwrap();
    assert_eq!(expr_obj.eval(), result1);

    let mut expr_obj = Expr::new(&expr2).unwrap();
    assert_eq!(expr_obj.set_var("x", 3.0).eval(), result1);
    assert_eq!(expr_obj.set_var("x", -1.5).eval(), result2);

    let mut expr_obj = Expr::new(&expr2).unwrap();
    expr_obj.partial_eval().unwrap();
    assert_eq!(expr_obj.set_var("x", 3.0).eval(), result1);
    assert_eq!(expr_obj.set_var("x", -1.5).eval(), result2);
}

#[test]
fn partial_eval_test() {
    let mut expr_obj = Expr::new("a1 + a2 * sin(x)").unwrap();
    expr_obj.set_var("a1", 1.0)
        .set_var("a2", 0.5)
        .partial_eval()
        .unwrap();
    let mut x = 1.0;
    for _ in 0..10 {
        x = expr_obj.set_var("x", x).eval().unwrap()[0];
        assert_eq!(expr_obj.set_var("x", x).eval(), Ok(vec![1.0 + 0.5 * x.sin()]));
    }
}