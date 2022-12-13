pub struct Expr {
    num_vars: u32,
    pub tokens: Vec<Token>,
//    pub AST:
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
    VAR(char),
    AND,
    OR,
    XOR,
    NOT,
    L_PAREN,
    R_PAREN
}

impl Expr {
    pub fn build_expr(input: String) -> Result<Expr, &'static str> {
        if input.len() == 0 { return Err("Input string was empty."); }
        let mut parsed: Vec<Token> = Vec::new();
        let mut num_vars: u32 = 0;

        for c in input.chars() {
            match c {
                ' ' => continue,
                '&' => parsed.push(Token::AND),
                '|' => parsed.push(Token::OR),
                '^' => parsed.push(Token::XOR),
                '!' | '~' => parsed.push(Token::NOT),
                '(' => parsed.push(Token::L_PAREN),
                ')' => parsed.push(Token::R_PAREN),
                _   => {
                    parsed.push(Token::VAR(c));
                    num_vars += 1;
                }
            }
        }

        Ok(Expr {num_vars, tokens: parsed})
    }

    fn convert_rpn(mut tokens: Vec<Token>) -> Vec<Token> {
        tokens
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
    fn simple_test() {
        let expected: Vec<Token>  = vec![Token::VAR('a'), Token::AND, Token::VAR('b')];

        assert_eq!(Expr::build_expr(String::from("a & b")).unwrap().parsed, expected);
    }
}