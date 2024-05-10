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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Constant {
    PI,
    Tau,
    E
}

impl Constant {
    pub(crate) fn eval(&self) -> Value {
        match self {
            Self::PI => std::f64::consts::PI,
            Self::Tau => std::f64::consts::TAU,
            Self::E => std::f64::consts::E
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Function {
    Min,
    Max,
    Powi,
    Powf,
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Sinh,
    Cosh,
    Tanh,
    Sqrt,
    Cbrt,
    Hypot,
    Exp,
    Exp2,
    Log,
    Log10,
    Log2,
    Ln,
    Floor,
    Ceil,
    Round,
    Fract,
    Trunc,
    Abs,
    Signum,
    DivEuclid,
    RemEuclid,
    Custom(String)
}

impl Function {
    pub(crate) fn num_args(&self) -> usize {
        match self {
            Self::Powi | Self::Powf | Self::Log => 2,
            Self::DivEuclid | Self::RemEuclid => 2,
            Self::Hypot => 2,
            Self::Min | Self::Max => 2,
            Self::Sin | Self::Cos | Self::Tan | Self::Asin | Self::Acos | Self::Atan
            | Self::Sinh | Self::Cosh | Self::Tanh => 1,
            Self::Sqrt | Self::Cbrt | Self::Ln | Self::Exp | Self::Exp2 | Self::Log10 | Self::Log2 => 1,
            Self::Floor | Self::Ceil | Self::Round | Self::Fract | Self::Trunc => 1,
            Self::Abs | Self::Signum => 1,
            _ => {unreachable!()},
        }
    }
    pub(crate) fn eval1(&self, arg: Value) -> Value {
        match self {
            Self::Sin => arg.sin(),
            Self::Cos => arg.cos(),
            Self::Tan => arg.tan(),
            Self::Asin => arg.asin(),
            Self::Acos => arg.acos(),
            Self::Atan => arg.atan(),
            Self::Sinh => arg.sinh(),
            Self::Cosh => arg.cosh(),
            Self::Tanh => arg.tanh(),
            Self::Sqrt => arg.sqrt(),
            Self::Cbrt => arg.cbrt(),
            Self::Ln => arg.ln(),
            Self::Log10 => arg.log10(),
            Self::Log2 => arg.log2(),
            Self::Exp => arg.exp(),
            Self::Exp2 => arg.exp2(),
            Self::Floor => arg.floor(),
            Self::Ceil => arg.ceil(),
            Self::Round => arg.round(),
            Self::Fract => arg.fract(),
            Self::Trunc => arg.trunc(),
            Self::Abs => arg.abs(),
            Self::Signum => arg.signum(),
            _ => {unreachable!()}
        }
    }
    pub(crate) fn eval2(&self, args: (Value, Value)) -> Value {
        match self {
            Self::Min => args.0.min(args.1),
            Self::Max => args.0.max(args.1),
            Self::Log => args.0.log(args.1),
            Self::Powi => args.0.powi(args.1 as i32), // unchecked
            Self::Powf => args.0.powf(args.1),
            Self::DivEuclid => args.0.div_euclid(args.1),
            Self::RemEuclid => args.0.rem_euclid(args.1),
            Self::Hypot => args.0.hypot(args.1),
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
    SemiColon,
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
            ";" => Ok(PreToken::SemiColon),
            "," => Ok(PreToken::Comma),
            c if c.chars().all(is_literalchar) => {
                Ok(PreToken::Literal(c.to_owned()))
            },
            _ => {Err(EvalError::InvalidString(s.to_owned()))}
        }
    }
}

impl FromStr for Constant {
    type Err = EvalError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PI" => Ok(Constant::PI),
            "TAU" => Ok(Constant::Tau),
            "E" => Ok(Constant::E),
            _ => Err(EvalError::ConstantNotFound),
        }
    }
}

impl FromStr for Function {
    type Err = EvalError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "min" => Ok(Function::Min),
            "max" => Ok(Function::Max),
            "sin" => Ok(Function::Sin),
            "cos" => Ok(Function::Cos),
            "tan" => Ok(Function::Tan),
            "asin" => Ok(Function::Asin),
            "acos" => Ok(Function::Acos),
            "atan" => Ok(Function::Atan),
            "sinh" => Ok(Function::Sinh),
            "cosh" => Ok(Function::Cosh),
            "tanh" => Ok(Function::Tanh),
            "powi" => Ok(Function::Powi),
            "powf" => Ok(Function::Powf),
            "pow" => Ok(Function::Powf),
            "sqrt" => Ok(Function::Sqrt),
            "cbrt" => Ok(Function::Cbrt),
            "hypot" => Ok(Function::Hypot),
            "ln" => Ok(Function::Ln),
            "log" => Ok(Function::Log),
            "log10" => Ok(Function::Log10),
            "log2" => Ok(Function::Log2),
            "exp" => Ok(Function::Exp),
            "exp2" => Ok(Function::Exp2),
            "floor" => Ok(Function::Floor),
            "ceil" => Ok(Function::Ceil),
            "round" => Ok(Function::Round),
            "fract" => Ok(Function::Fract),
            "trunc" => Ok(Function::Trunc),
            "abs" => Ok(Function::Abs),
            "signum" => Ok(Function::Signum),
            "div_euclid" => Ok(Function::DivEuclid),
            "rem_euclid" => Ok(Function::RemEuclid),
            _ => Ok(Function::Custom(s.to_owned()))
        }
    }
}