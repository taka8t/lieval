#[derive(Debug, Clone, PartialEq)]
pub enum EvalError {
    FunctionNotFound(String),
    ConstantNotFound,
    InvalidString(String),
    UnexpectedParenthesis,
    UndefinedFunction(String),
    UndefinedVariable(String),
    WrongExpression,
    WrongArguments(usize),
    WrongExprIndex(usize),
}

impl std::error::Error for EvalError {}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidString(s) => {write!(f, "string {} is Invalid", s)},
            Self::ConstantNotFound => {write!(f, "Consts is not found", )},
            Self::FunctionNotFound(s) => {write!(f, "function {} is unimplemented", s)},
            Self::UnexpectedParenthesis => {write!(f, "unexpected or unbalanced parehthesis",)},
            Self::UndefinedFunction(s) => {write!(f, "function {} is undefined", s)},
            Self::UndefinedVariable(s) => {write!(f, "variable {} is undefined", s)},
            Self::WrongExpression => {write!(f, "Wrong expression.")},
            Self::WrongArguments(n) => {write!(f, "Expected number of arguments is {}", n)},
            Self::WrongExprIndex(n) => {write!(f, "Wrong Expr Vec index: {}", n)},
        }   
    }
}
