use crate::LinkedHashMap;

#[derive(Debug, PartialEq)]
pub enum Token {
    VAR(char),
    LParen,
    RParen,
    OP(Operator)
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    AND,
    OR,
    XOR,
    NOT
}

pub struct Expr {
    pub rpn: Vec<Token>,
    pub ast_order: LinkedHashMap<char, usize>,
    pub bdd_order: LinkedHashMap<char, usize>
}


fn tokenize(input: &String) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::with_capacity(input.len());

    for c in input.chars() {
        match c {
            ' ' =>          continue,
            '&' =>          tokens.push(Token::OP(Operator::AND)),
            '|' =>          tokens.push(Token::OP(Operator::OR)),
            '^' =>          tokens.push(Token::OP(Operator::XOR)),
            '!' | '~' =>    tokens.push(Token::OP(Operator::NOT)),
            '(' =>          tokens.push(Token::LParen),
            ')' =>          tokens.push(Token::RParen),
            c => {
                if c.is_ascii_alphabetic() {
                    tokens.push(Token::VAR(c));
                } else {
                    return Err(format!("Non-alphabetical variable name {c} encountered in input string {input}"))
                }
            }
        }
    }
    Ok(tokens)
}

fn convert_rpn(tokens: Vec<Token>) -> Result<Vec<Token>, String> {
    let mut rpn: Vec<Token> = Vec::with_capacity(tokens.len());
    let mut op_stack: Vec<Token> = Vec::with_capacity(tokens.len());

    for token in tokens.into_iter() {
        match token {
            Token::LParen | Token::OP(Operator::NOT) => op_stack.push(token),
            Token::VAR(_) => rpn.push(token),
            Token::RParen => {
                loop {
                    let top = op_stack.pop();
                    let top_res = top.ok_or_else(|| return String::from("Unclosed right paren"));
                    if let Ok(t) = top_res {
                        if t == Token::LParen { break }
                        else { rpn.push(t) }
                    } else {
                        return Err(String::from("Unclosed right paren"));
                    }
                }
            },
            t => {
                while let Some(Token::OP(Operator::NOT)) = op_stack.last() {
                    rpn.push(Token::OP(Operator::NOT));
                    op_stack.pop();
                }
                op_stack.push(t);
            }
        }
    }

    if !op_stack.is_empty() {
        for op in op_stack.into_iter() {
            if op == Token::LParen { 
                return Err(String::from("Unclosed left paren"))
            } else {
                rpn.push(op);
            }
        }
    }

    Ok(rpn)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn token_simple() {
        let expected: Result<Vec<Token>, String> = Ok(vec![Token::VAR('a'), Token::OP(Operator::AND), Token::VAR('b')]);
        assert_eq!(expected, tokenize(&String::from("a & b")))

    }

    #[test]
    fn token_fail() {
       assert!(tokenize(&String::from("@ ^ $")).is_err())
    }

    #[test]
    fn rpn_not() {
        let expected: Vec<Token> = vec![Token::VAR('a'), Token::OP(Operator::NOT), Token::VAR('b'), Token::OP(Operator::AND)];
        let input = tokenize(&String::from("~a & b")).expect("tokenize step failed");
        assert_eq!(Ok(expected), convert_rpn(input))
    }

    #[test]
    fn rpn_left_paren_unclosed() {
        let input = tokenize(&String::from("(a & b")).expect("tokenize step failed");
        assert!(convert_rpn(input).is_err())
    }

    #[test]
    fn rpn_right_paren_unclosed() {
        let input = tokenize(&String::from("a & b)")).expect("tokenize step failed");
        assert!(convert_rpn(input).is_err())
    }
}