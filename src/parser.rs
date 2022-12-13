use std::collections::VecDeque;

pub struct Expr {
    num_vars: u32,
    pub tokens: Vec<Token>,
    pub rpn: Vec<Token>
//    pub AST:
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(dead_code)]
pub enum Token {
    VAR(char),
    AND,
    OR,
    XOR,
    NOT,
    LParen,
    RParen
}

impl Expr {
    fn new() -> Expr {
        Expr {
            num_vars: 0,
            tokens: Vec::new(),
            rpn: Vec::new()
        }
    }

    pub fn build_expr(input: String) -> Result<Expr, &'static str> {
        let mut expression = Expr::new();

        expression.parse_input(input)?;
        expression.convert_rpn();

        Ok(expression)
    }

    fn parse_input(&mut self, input: String) -> Result<(), &'static str> {
        if input.len() == 0 { return Err("Input string was empty."); }

        for c in input.chars() {
            match c {
                ' ' => continue,
                '&' => self.tokens.push(Token::AND),
                '|' => self.tokens.push(Token::OR),
                '^' => self.tokens.push(Token::XOR),
                '!' | '~' => self.tokens.push(Token::NOT),
                '(' => self.tokens.push(Token::LParen),
                ')' => self.tokens.push(Token::RParen),
                _   => {
                    self.tokens.push(Token::VAR(c));
                    self.num_vars += 1;
                }
            }
        }

        Ok(()) 
    }

    fn convert_rpn(&mut self) {
        let mut ops: VecDeque<Token> = VecDeque::new();

        for token in self.tokens.iter() {
            match token {
                Token::VAR(_) => self.rpn.push(*token),
                Token::LParen => ops.push_back(*token),
                Token::RParen => {
                    loop {
                        let top = ops.pop_back().unwrap(); 
                        if top == Token::LParen { break; }
                        self.rpn.push(top);
                    }
                },
                _ => ops.push_back(*token)
            }
        }

        while ops.len() != 0 {
            self.rpn.push(ops.pop_front().unwrap());
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_string() {
        assert!(Expr::build_expr(String::from("")).is_err());
    }

    #[test]
    fn simple_parse_test() {
        let parse_expected: Vec<Token>  = vec![Token::VAR('a'), Token::AND, Token::VAR('b')];
        let rpn_expected: Vec<Token> = vec![Token::VAR('a'), Token::VAR('b'), Token::AND];
        let expression = Expr::build_expr(String::from("a & b")).unwrap();

        assert_eq!(expression.tokens, parse_expected);
        assert_eq!(expression.rpn, rpn_expected);
    }

    #[test]
    fn complex_build_test() {
        let input = String::from("((a ^ b) & ~(c & d)) | e");
        let parse_expected: Vec<Token> = vec![Token::LParen, Token::LParen, Token::VAR('a'), Token::XOR, Token::VAR('b'),
        Token::RParen, Token::AND, Token::NOT, Token::LParen, Token::VAR('c'), Token::AND, Token::VAR('d'), Token::RParen, Token::RParen, Token::OR, Token::VAR('e')];
        let rpn_expected: Vec<Token> = vec![Token::VAR('a'), Token::VAR('b'), Token::XOR, Token::VAR('c'), Token::VAR('d'), Token::AND, 
        Token::NOT, Token::AND, Token::VAR('e'), Token::OR];
        
        let expression = Expr::build_expr(input).unwrap();

        assert_eq!(expression.tokens, parse_expected);
        assert_eq!(expression.rpn, rpn_expected);
    }

}