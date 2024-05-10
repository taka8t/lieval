use crate::token::{Function, Token, Value, BinaryOp};
use crate::parse::{parse_str_to_rpn};
use crate::context::Context;
use crate::error::EvalError;

use std::ops;

#[derive(Debug, Clone)]
pub struct Expr {
    expr: Vec<Vec<Token>>,
    context: Context
}

impl Expr {
    pub fn new(expr: &str) -> Result<Self, EvalError> {
        Ok(
            Self {
                expr: parse_str_to_rpn(expr)?,
                context: Context::new()
            }
        )
    }
    pub fn set_var(&mut self, var: &str, val: Value) -> &mut Self {
        self.context.set_value(var, val);
        self
    }

    pub fn set_func(&mut self, name: &str, n: usize, f: fn(&[Value]) -> Value) -> &mut Self {
        self.context.set_func(name, n, f);
        self
    }

    pub fn eval(&self) -> Result<Value, EvalError> {
        self.eval_index(0)
    }

    pub fn evals(&self) -> Result<Vec<Value>, EvalError> {
        let mut values = vec![];
        for expr in self.expr.iter() {
            values.push(eval_with_context(expr, &self.context)?);
        }
        Ok(values)
    }

    pub fn eval_index(&self, id: usize) -> Result<Value, EvalError> {
        if self.expr.len() <= id {
            Err(EvalError::WrongExprIndex(id))
        }
        else {
            eval_with_context(&self.expr[id], &self.context)
        }
    }

    pub fn partial_eval(&mut self) -> Result<&mut Self, EvalError> {
        self.partial_eval_index(0)
    }

    pub fn partial_evals(&mut self) -> Result<&mut Self, EvalError> {
        for i in 0..self.expr.len() {
            self.expr[i] = partial_eval_with_context(&self.expr[i], &self.context)?;
        }
        Ok(self)
    }

    pub fn partial_eval_index(&mut self, id: usize) -> Result<&mut Self, EvalError> {
        if self.expr.len() <= id {
            Err(EvalError::WrongExprIndex(id))
        }
        else {
            self.expr[id] = partial_eval_with_context(&self.expr[id], &self.context)?;
            Ok(self)
        }
    }

    fn apply_operator(&mut self, other: Expr, op: Token) {
        if self.expr.len() == other.expr.len() {
            for (l, r) in self.expr.iter_mut().zip(other.expr.into_iter()) {
                l.extend(r);
                l.push(op.clone());
            }
        }
        else if self.expr.len() == 1 {
            self.expr.resize(other.expr.len(), self.expr[0].clone());
            for (l, r) in self.expr.iter_mut().zip(other.expr.into_iter()) {
                l.extend(r);
                l.push(op.clone());
            }
        }
        else if other.expr.len() == 1 {
            for (l, r) in self.expr.iter_mut().zip(other.expr.iter().cycle().cloned()) {
                l.extend(r);
                l.push(op.clone());
            }
        }
        else {
            panic!("The number of exprs must be the same, or either one must be equal to 1");
        }
    }
}

macro_rules! expr_op {
    ($op:path, $name:ident, $token:expr) => {
        impl $op for Expr {
            type Output = Self;
            fn $name(mut self, other: Self) -> Self {
                self.context = Context::ctx_merge(&self.context, &other.context);
                self.apply_operator(other, $token);
                self
            }
        }
    };
    (assign $op:path, $name:ident, $token:expr) => {
        impl $op for Expr {
            fn $name(&mut self, other: Self) {
                self.context = Context::ctx_merge(&self.context, &other.context);
                self.apply_operator(other, $token);
            }
        }
    };
}

expr_op!(ops::Add, add, Token::Binary(BinaryOp::Add));
expr_op!(ops::Sub, sub, Token::Binary(BinaryOp::Sub));
expr_op!(ops::Mul, mul, Token::Binary(BinaryOp::Mul));
expr_op!(ops::Div, div, Token::Binary(BinaryOp::Div));

expr_op!(assign ops::AddAssign, add_assign, Token::Binary(BinaryOp::Add));
expr_op!(assign ops::SubAssign, sub_assign, Token::Binary(BinaryOp::Sub));
expr_op!(assign ops::MulAssign, mul_assign, Token::Binary(BinaryOp::Mul));
expr_op!(assign ops::DivAssign, div_assign, Token::Binary(BinaryOp::Div));

pub fn eval_from_str(expr: &str) -> Result<Vec<Value>, EvalError> {
    let tokens_vec = parse_str_to_rpn(expr)?;
    let mut values = vec![];
    for tokens in tokens_vec {
        values.push(eval(&tokens)?);
    }
    Ok(values)
}

pub fn eval_from_str_with_context(expr: &str, context: &Context) -> Result<Vec<Value>, EvalError> {
    let tokens_vec = parse_str_to_rpn(expr)?;
    let mut values = vec![];
    for tokens in tokens_vec {
        values.push(eval_with_context(&tokens, context)?);
    }
    Ok(values)
}

pub(crate) fn eval(tokens: &[Token]) -> Result<Value, EvalError> {
    let mut output = Vec::with_capacity(8);
    for token in tokens.iter() {
        match token {
            Token::Value(v) => {output.push(*v);},
            Token::Var(s) => {
                return Err(EvalError::UndefinedVariable(s.clone()));
            },
            Token::Unary(op) => {
                if let Some(v) = output.pop() {
                    output.push(op.eval(v));
                }
                else {
                    return Err(EvalError::WrongExpression);
                }
            },
            Token::Binary(op) => {
                if let (Some(v2), Some(v1)) = (output.pop(), output.pop()) {
                    output.push(op.eval(v1, v2));
                }
                else {
                    return Err(EvalError::WrongExpression);
                }
            },
            Token::Function(Function::Custom(s)) => {
                return Err(EvalError::UndefinedFunction(s.clone()));
            }
            Token::Function(func) => {
                // builtin func
                let n_args = func.num_args();
                if n_args == 1 {
                    if let Some(v) = output.pop() {
                        output.push(func.eval1(v));
                    }
                    else {
                        return Err(EvalError::WrongExpression);
                    }
                }
                else if n_args == 2 {
                    if let (Some(v2), Some(v1)) = (output.pop(), output.pop()) {
                        output.push(func.eval2((v1, v2)));
                    }
                    else {
                        return Err(EvalError::WrongExpression);
                    }
                }
                else {
                    // unimplemented
                }
            },
            _ => {
                return Err(EvalError::WrongExpression);
            }
        }
    }
    if output.len() != 1 {
        Err(EvalError::WrongExpression)
    }
    else if let Some(v) = output.pop() {
        Ok(v)
    }
    else {
        Err(EvalError::WrongExpression)
    }
}

#[allow(dead_code)]
fn partial_eval(tokens: &[Token]) -> Result<Vec<Token>, EvalError> {
    let mut output = Vec::with_capacity(8);
    for token in tokens.iter() {
        match token {
            Token::Value(_) | Token::Var(_) => {output.push(token.clone());},
            Token::Unary(op) => {
                let top = output.pop();
                if let Some(Token::Value(v)) = top {
                    output.push(Token::Value(op.eval(v)));
                }
                else if let Some(t) = top {
                    output.push(t);
                    output.push(token.clone());
                }
                else {
                    return Err(EvalError::WrongExpression);
                }
            },
            Token::Binary(op) => {
                let top = (output.pop(), output.pop());
                if let (Some(Token::Value(v2)), Some(Token::Value(v1))) = top {
                    output.push(Token::Value(op.eval(v1, v2)));
                }
                else if let (Some(t1), Some(t2)) = top {
                    output.push(t2);
                    output.push(t1);
                    output.push(token.clone());
                }
                else {
                    return Err(EvalError::WrongExpression);
                }
            },
            Token::Function(Function::Custom(_)) => {
                output.push(token.clone());
            }
            Token::Function(func) => {
                let n_args = func.num_args();
                if n_args == 1 {
                    let top = output.pop();
                    if let Some(Token::Value(v)) = top {
                        output.push(Token::Value(func.eval1(v)));
                    }
                    else if let Some(t) = top {
                        output.push(t);
                        output.push(token.clone());
                    }
                    else {
                        return Err(EvalError::WrongExpression);
                    }
                }
                else if n_args == 2 {
                    let top = (output.pop(), output.pop());
                    if let (Some(Token::Value(v2)), Some(Token::Value(v1))) = top {
                        output.push(Token::Value(func.eval2((v1, v2))));
                    }
                    else if let (Some(t1), Some(t2)) = top {
                        output.push(t2);
                        output.push(t1);
                        output.push(token.clone());
                    }
                    else {
                        return Err(EvalError::WrongExpression);
                    }
                }
                else {
                    // unimplemented
                }
            },
            _ => {
                return Err(EvalError::WrongExpression);
            }
        }
    }
    Ok(output)
}

pub(crate) fn eval_with_context(tokens: &[Token], context: &Context) -> Result<Value, EvalError> {
    let mut output = Vec::with_capacity(8);
    for token in tokens.iter() {
        match token {
            Token::Value(v) => {output.push(*v);},
            Token::Var(s) => {
                if let Some(&v) = context.get_value(s) {
                    output.push(v);
                }
                else {
                    return Err(EvalError::UndefinedVariable(s.clone()));
                }
            }
            Token::Unary(op) => {
                if let Some(v) = output.pop() {
                    output.push(op.eval(v));
                }
                else {
                    return Err(EvalError::WrongExpression);
                }
            },
            Token::Binary(op) => {
                if let (Some(v2), Some(v1)) = (output.pop(), output.pop()) {
                    output.push(op.eval(v1, v2));
                }
                else {
                    return Err(EvalError::WrongExpression);
                }
            },
            Token::Function(Function::Custom(s)) => {
                if let Some(fc) = context.get_func(s) {
                    match fc.get_arg_len() {
                        0 => {
                            output.push(fc.call(&[]));
                        },
                        1 => {
                            if let Some(v) = output.pop() {
                                output.push(fc.call(&[v]));
                            }
                            else {
                                return Err(EvalError::WrongExpression);
                            }
                        },
                        2 => {
                            if let (Some(v2), Some(v1)) = (output.pop(), output.pop()) {
                                output.push(fc.call(&[v1, v2]));
                            }
                            else {
                                return Err(EvalError::WrongExpression);
                            }
                        },
                        n => {
                            let args = output.iter().rev().take(n).copied().rev().collect::<Vec<Value>>();
                            if args.len() == n {
                                output.truncate(output.len() - n);
                                output.push(fc.call(&args));
                            }
                            else {
                                return Err(EvalError::WrongExpression);
                            }
                        }
                    }
                }
                else {
                    return Err(EvalError::UndefinedFunction(s.clone()));
                }
            }
            Token::Function(func) => {
                let n_args = func.num_args();
                if n_args == 1 {
                    if let Some(v) = output.pop() {
                        output.push(func.eval1(v));
                    }
                    else {
                        return Err(EvalError::WrongExpression);
                    }
                }
                else if n_args == 2 {
                    if let (Some(v2), Some(v1)) = (output.pop(), output.pop()) {
                        output.push(func.eval2((v1, v2)));
                    }
                    else {
                        return Err(EvalError::WrongExpression);
                    }
                }
                else {
                    // unimplemented
                }
            },
            _ => {
                return Err(EvalError::WrongExpression);
            }
        }
    }
    if output.len() != 1 {
        Err(EvalError::WrongExpression)
    }
    else if let Some(v) = output.pop() {
        Ok(v)
    }
    else {
        Err(EvalError::WrongExpression)
    }
}

pub(crate) fn partial_eval_with_context(tokens: &[Token], context: &Context) -> Result<Vec<Token>, EvalError> {
    let mut output = Vec::with_capacity(8);
    for token in tokens.iter() {
        match token {
            Token::Value(_) => {output.push(token.clone());},
            Token::Var(s) => {
                if let Some(&v) = context.get_value(s) {
                    output.push(Token::Value(v));
                }
                else {
                    output.push(token.clone());
                }
            }
            Token::Unary(op) => {
                let top = output.pop();
                if let Some(Token::Value(v)) = top {
                    output.push(Token::Value(op.eval(v)));
                }
                else if let Some(t) = top {
                    output.push(t);
                    output.push(token.clone());
                }
                else {
                    return Err(EvalError::WrongExpression);
                }
            },
            Token::Binary(op) => {
                let top = (output.pop(), output.pop());
                if let (Some(Token::Value(v2)), Some(Token::Value(v1))) = top {
                    output.push(Token::Value(op.eval(v1, v2)));
                }
                else if let (Some(t1), Some(t2)) = top {
                    output.push(t2);
                    output.push(t1);
                    output.push(token.clone());
                }
                else {
                    return Err(EvalError::WrongExpression);
                }
            },
            Token::Function(Function::Custom(s)) => {
                if let Some(fc) = context.get_func(s) {
                    match fc.get_arg_len() {
                        0 => {
                            output.push(Token::Value(fc.call(&[])));
                        },
                        1 => {
                            let top = output.pop();
                            if let Some(Token::Value(v)) = top {
                                output.push(Token::Value(fc.call(&[v])));
                            }
                            else if let Some(t) = top {
                                output.push(t);
                                output.push(token.clone());
                            }
                            else {
                                return Err(EvalError::WrongExpression);
                            }
                        },
                        2 => {
                            let top = (output.pop(), output.pop());
                            if let (Some(Token::Value(v2)), Some(Token::Value(v1))) = top {
                                output.push(Token::Value(fc.call(&[v1, v2])));
                            }
                            else if let (Some(t1), Some(t2)) = top {
                                output.push(t2);
                                output.push(t1);
                                output.push(token.clone());
                            }
                            else {
                                return Err(EvalError::WrongExpression);
                            }
                        },
                        n => {
                            if output.len() >= n {
                                let args = output.iter()
                                    .rev()
                                    .take(n)
                                    .filter_map(|t| match t {
                                        Token::Value(v) => Some(*v),
                                        _ => None
                                    })
                                    .rev()
                                    .collect::<Vec<Value>>();
                                if args.len() == n {
                                    output.truncate(output.len() - n);
                                    output.push(Token::Value(fc.call(&args)));
                                }
                                else {
                                    output.push(token.clone());
                                }
                            }
                            else {
                                return Err(EvalError::WrongExpression);
                            }
                        }
                    }
                }
                else {
                    output.push(token.clone());
                }
            }
            Token::Function(func) => {
                // builtin func
                let n_args = func.num_args();
                if n_args == 1 {
                    let top = output.pop();
                    if let Some(Token::Value(v)) = top {
                        output.push(Token::Value(func.eval1(v)));
                    }
                    else if let Some(t) = top {
                        output.push(t);
                        output.push(token.clone());
                    }
                    else {
                        return Err(EvalError::WrongExpression);
                    }
                }
                else if n_args == 2 {
                    let top = (output.pop(), output.pop());
                    if let (Some(Token::Value(v2)), Some(Token::Value(v1))) = top {
                        output.push(Token::Value(func.eval2((v1, v2))));
                    }
                    else if let (Some(t1), Some(t2)) = top {
                        output.push(t2);
                        output.push(t1);
                        output.push(token.clone());
                    }
                    else {
                        return Err(EvalError::WrongExpression);
                    }
                }
                else {
                    // unimplemented
                }
            },
            _ => {
                return Err(EvalError::WrongExpression);
            }
        }
    }
    Ok(output)
}