use crate::token::{Token, Value};
use crate::parse::{parse_str_to_rpn};
use crate::context::Context;
use crate::error::EvalError;

#[derive(Debug, Clone)]
pub struct Expr {
    expr: Vec<Token>,
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

    // pub fn set_func(&mut self) -> &mut Self {}

    pub fn eval(&mut self) -> Result<Vec<Value>, EvalError> {
        eval_with_context(&self.expr, &self.context)
    }

    pub fn partial_eval(&mut self) -> Result<&mut Self, EvalError> {
        self.expr = partial_eval_with_context(&self.expr, &self.context)?;
        Ok(self)
    }
    
    // add.sub,mul... expr op expr
}

pub fn eval_from_str(expr: &str) -> Result<Vec<Value>, EvalError> {
    eval(&parse_str_to_rpn(expr)?)
}

pub fn eval_from_str_with_context(expr: &str, context: &Context) -> Result<Vec<Value>, EvalError> {
    eval_with_context(&parse_str_to_rpn(expr)?, context)
}

pub(crate) fn eval(tokens: &[Token]) -> Result<Vec<Value>, EvalError> {
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

                // todo: match custom func
            },
            _ => {
                return Err(EvalError::WrongExpression);
            }
        }
    }
    Ok(output)
}

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

                // todo: match custom func
            },
            _ => {
                return Err(EvalError::WrongExpression);
            }
        }
    }
    Ok(output)
}

pub(crate) fn eval_with_context(tokens: &[Token], context: &Context) -> Result<Vec<Value>, EvalError> {
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

                // todo: match custom func
            },
            _ => {
                return Err(EvalError::WrongExpression);
            }
        }
    }
    Ok(output)
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

                // todo: match custom func
            },
            _ => {
                return Err(EvalError::WrongExpression);
            }
        }
    }
    Ok(output)
}