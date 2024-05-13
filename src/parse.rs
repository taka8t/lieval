use crate::token::{PreToken, Token, Value, UnaryOp, BinaryOp, Function, Constant};
use crate::error::EvalError;
use crate::util::{is_literalchar, is_identstr};
use std::str::FromStr;

pub fn parse_str_to_rpn(expr: &str) -> Result<Vec<Vec<Token>>, EvalError> {
    let tokens_vec = pretoken_to_tokens(&parse_str_to_pretokens(expr)?)?;
    let mut ret_tokens = vec![];
    for tokens in tokens_vec {
        ret_tokens.push(to_rpn(&tokens)?);
    }
    Ok(ret_tokens)
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

fn pretoken_to_tokens(pretokens: &[PreToken]) -> Result<Vec<Vec<Token>>, EvalError> {
    let mut tokens_vec = vec![];
    let mut tokens = vec![];
    let mut ptiter = pretokens.iter().peekable();
    let mut paren_count = 0;
    while let Some(pretoken) = ptiter.next() {
        match pretoken {
            PreToken::Literal(s) => {
                if let Ok(v) = s.parse::<Value>() {
                    tokens.push(Token::Value(v));
                }
                else if let Ok(c) = s.parse::<Constant>() {
                    tokens.push(Token::Value(c.eval()));
                }
                else if is_identstr(s) {
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
                paren_count += 1;
            },
            PreToken::RightParen => {
                tokens.push(Token::RightParen);
                paren_count -= 1;
            },
            PreToken::SemiColon => {
                tokens_vec.push(std::mem::take(&mut tokens));
            }
            PreToken::Comma => {
                if paren_count > 0 {
                    tokens.push(Token::Comma);
                }
                else {
                    tokens_vec.push(std::mem::take(&mut tokens));
                }
            },
        }
        if paren_count < 0 {
            return Err(EvalError::UnexpectedParenthesis);
        }
    }
    if paren_count == 0 {
        if !tokens.is_empty() {
            tokens_vec.push(std::mem::take(&mut tokens));
        }
        Ok(tokens_vec)
    }
    else {
        Err(EvalError::UnexpectedParenthesis)
    }
    
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