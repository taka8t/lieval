use crate::error::EvalError;
use crate::util::is_literalchar;
use std::str::FromStr;

pub type Value = f64;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnaryOp {
    Neg,
}

impl UnaryOp {
    pub(crate) fn eval(&self, arg: Value) -> Value {
        match self {
            Self::Neg => -arg,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

impl BinaryOp {
    pub(crate) fn eval(&self, lhs: Value, rhs: Value) -> Value {
        match self {
            Self::Add => lhs + rhs,
            Self::Sub => lhs - rhs,
            Self::Mul => lhs * rhs,
            Self::Div => lhs / rhs,
            Self::Rem => lhs % rhs,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Function {
    Powi,
    Powf,
    Sin,
    Cos,
    Tan,
    Sqrt,
    Exp,
    Ln,
    Custom(String)
}

impl Function {
    pub(crate) fn num_args(&self) -> usize {
        match self {
            Self::Powi
            | Self::Powf => 2,
            Self::Sin
            | Self::Cos
            | Self::Tan
            | Self::Sqrt
            | Self::Ln
            | Self::Exp => 1,
            _ => {unreachable!()},
        }
    }
    pub(crate) fn eval1(&self, arg: Value) -> Value {
        match self {
            Self::Sin => arg.sin(),
            Self::Cos => arg.cos(),
            Self::Tan => arg.tan(),
            Self::Sqrt => arg.sqrt(),
            Self::Ln => arg.ln(),
            Self::Exp => arg.exp(),
            _ => {unreachable!()}
        }
    }
    pub(crate) fn eval2(&self, args: (Value, Value)) -> Value {
        match self {
            Self::Powi => args.0.powi(args.1 as i32), // unchecked
            Self::Powf => args.0.powf(args.1),
            _ => {unreachable!()}
        }
    }
    // fn evaln(&self, args: &[Value]) -> Value {
    // }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Unary(UnaryOp),
    Binary(BinaryOp),
    Function(Function),
    Value(Value),
    Var(String),
    LeftParen,
    RightParen,
    Comma,
}

impl Token {
    pub(crate) fn precedence(&self) -> (u8, u8) {
        match self {
            Self::Binary(BinaryOp::Add) | Self::Binary(BinaryOp::Sub) => (50, 51),
            Self::Binary(BinaryOp::Mul) | Self::Binary(BinaryOp::Div) | Self::Binary(BinaryOp::Rem) => (55, 56),
            Self::Unary(UnaryOp::Neg) => (99, 65),
            Self::Function(_) => (97, 10),
            Self::LeftParen => (99, 2),
            Self::RightParen => (3, 100),
            Self::Comma => (5, 5),
            // Self::Eq => (8, 7)
            _ => (100, 100)
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum PreToken {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    LeftParen,
    RightParen,
    Comma,
    Literal(String)
}

impl FromStr for PreToken {
    type Err = EvalError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(PreToken::Plus),
            "-" => Ok(PreToken::Minus),
            "*" => Ok(PreToken::Asterisk),
            "/" => Ok(PreToken::Slash),
            "%" => Ok(PreToken::Percent),
            "(" => Ok(PreToken::LeftParen),
            ")" => Ok(PreToken::RightParen),
            "," => Ok(PreToken::Comma),
            c if c.chars().all(is_literalchar) => {
                Ok(PreToken::Literal(c.to_owned()))
            },
            _ => {Err(EvalError::InvalidString(s.to_owned()))}
        }
    }
}

impl FromStr for Function {
    type Err = EvalError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sin" => Ok(Function::Sin),
            "cos" => Ok(Function::Cos),
            "tan" => Ok(Function::Tan),
            "powi" => Ok(Function::Powi),
            "powf" => Ok(Function::Powf),
            "pow" => Ok(Function::Powf),
            "sqrt" => Ok(Function::Sqrt),
            "ln" => Ok(Function::Ln),
            "exp" => Ok(Function::Exp),
            _ => Ok(Function::Custom(s.to_owned()))
        }
    }
}