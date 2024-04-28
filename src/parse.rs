use crate::token::{PreToken, Token, Value, UnaryOp, BinaryOp, Function};
use crate::error::EvalError;
use crate::util::is_literalchar;
use std::str::FromStr;

pub fn parse_str_to_rpn(expr: &str) -> Result<Vec<Token>, EvalError> {
    let tokens = pretoken_to_tokens(&parse_str_to_pretokens(expr)?)?;
    to_rpn(&tokens)
}

fn parse_str_to_pretokens(expr: &str) -> Result<Vec<PreToken>, EvalError> {
    let n = expr.len();
    let mut l = 0;
    let mut pretokens = vec![];
    for (r, c) in expr.chars().enumerate() {
        if !is_literalchar(c) {
            if l < r {pretokens.push(PreToken::from_str(&expr[l..r])?);}
            if !c.is_whitespace() {
                pretokens.push(PreToken::from_str(&c.to_string())?);
            }
            l = r+1;
        }
    }
    if l < n {
        pretokens.push(PreToken::from_str(&expr[l..n])?);
    }
    Ok(pretokens)
}

fn pretoken_to_tokens(pretokens: &[PreToken]) -> Result<Vec<Token>, EvalError> {
    let mut tokens = vec![];
    let mut ptiter = pretokens.iter().peekable();
    while let Some(pretoken) = ptiter.next() {
        match pretoken {
            PreToken::Literal(s) => {
                if let Ok(v) = s.parse::<Value>() {
                    tokens.push(Token::Value(v));
                }
                else if let Some(hc) = s.chars().next() {
                    if (hc.is_alphabetic() || hc == '_')
                    && s.chars().all(|c| c.is_alphanumeric() || c == '_') {
                        if let Some(PreToken::LeftParen) = ptiter.peek() {
                            tokens.push(Token::Function(s.parse::<Function>()?));
                        }
                        else {
                            tokens.push(Token::Var(s.to_string()));
                        }
                    }
                    else {
                        return Err(EvalError::InvalidString(s.clone()));
                    }
                }
                else {
                    return Err(EvalError::InvalidString(s.clone()));
                }
            },
            PreToken::Plus => {
                tokens.push(Token::Binary(BinaryOp::Add));
            },
            PreToken::Minus => {
                if let Some(token) = tokens.last() {
                    match token {
                        Token::RightParen | Token::Value(_) | Token::Var(_) => {
                            tokens.push(Token::Binary(BinaryOp::Sub));
                        },
                        _ => {tokens.push(Token::Unary(UnaryOp::Neg));}
                    }
                }
                else {
                    tokens.push(Token::Unary(UnaryOp::Neg));
                }
            },
            PreToken::Asterisk => {
                tokens.push(Token::Binary(BinaryOp::Mul));
            },
            PreToken::Slash => {
                tokens.push(Token::Binary(BinaryOp::Div));
            },
            PreToken::Percent => {
                tokens.push(Token::Binary(BinaryOp::Rem));
            },
            PreToken::LeftParen => {
                tokens.push(Token::LeftParen);
            },
            PreToken::RightParen => {
                tokens.push(Token::RightParen);
            },
            PreToken::Comma => {
                tokens.push(Token::Comma);
            },
        }
    }
    Ok(tokens)
}

fn to_rpn(tokens: &[Token]) -> Result<Vec<Token>, EvalError> {
    // Shunting yard
    let mut rpn_stack: Vec<Token> = vec![];
    let mut op_stack: Vec<Token> = vec![];

    for token in tokens.iter() {
        match token {
            Token::Value(_) | Token::Var(_) => {
                rpn_stack.push(token.clone());
            },
            _ => {
                let (l_asc, _) = token.precedence();
                while let Some(top_token) = op_stack.last() {
                    let (_, r_asc) = top_token.precedence();
                    if l_asc > r_asc {
                        break
                    }
                    rpn_stack.push(op_stack.pop().unwrap());
                }
                if *token == Token::RightParen {
                    match op_stack.last() {
                        Some(Token::LeftParen) => {
                            op_stack.pop().unwrap();
                            if let Some(Token::Function(_)) = op_stack.last() {
                                rpn_stack.push(op_stack.pop().unwrap());
                                // todo: if custom function arguments cnt
                            }
                            continue;
                        },
                        _ => {
                            return Err(EvalError::UnexpectedParenthesis);
                        }
                    }
                }
                if *token != Token::Comma {
                    op_stack.push(token.clone());
                }
            } 
        }
    }
    while let Some(top_token) = op_stack.pop() {
        if top_token == Token::LeftParen || top_token == Token::RightParen {
            return Err(EvalError::UnexpectedParenthesis);
        }
        rpn_stack.push(top_token);
    }
    Ok(rpn_stack)

}