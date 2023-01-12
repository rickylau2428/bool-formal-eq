use std::{collections::{HashMap}};

#[derive(Debug)]
pub struct Expr {
    pub tokens: Vec<Token>,
    // pub rpn: Vec<Token>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Token {
    VAR(usize),
    OP(Operator),
    LParen,
    RParen
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Operator {
    AND,
    OR,
    XOR,
    NOT
}

impl Expr {
    pub fn new() -> Expr {
        Expr {
            tokens: Vec::new(),
            // rpn: Vec::new(),
        }
    }

    pub fn build_expr<F>(&mut self, input: String, bool_vars: &mut HashMap<char, usize>, mut handle_var: F) -> Result<&mut Self, &'static str> where
        F: FnMut(char, &mut HashMap<char, usize>) -> Result<(), &'static str> 
        {
            if input.len() == 0 { return Err("Input string was empty."); }

            for c in input.chars() {
                match c {
                    ' ' => continue,
                    '&' =>          self.tokens.push(Token::OP(Operator::AND)),
                    '|' =>          self.tokens.push(Token::OP(Operator::OR)),
                    '^' =>          self.tokens.push(Token::OP(Operator::XOR)),
                    '!' | '~' =>    self.tokens.push(Token::OP(Operator::NOT)),
                    '(' =>          self.tokens.push(Token::LParen),
                    ')' =>          self.tokens.push(Token::RParen),
                    _   => {
                        handle_var(c, bool_vars)?;
                        self.tokens.push(Token::VAR(bool_vars.get(&c).unwrap().clone()))
                    }
                }
            }

        // self.parse_input(input, bool_vars)?;
        // self.convert_rpn();

        Ok(self)
    }

    // fn parse_input<F>(&mut self, input: String, bool_vars: &mut HashMap<char, usize>, handle_var: F) -> Result<(), &'static str> where
    //     F: FnOnce() {
    //     if input.len() == 0 { return Err("Input string was empty."); }

    //     for c in input.chars() {
    //         match c {
    //             ' ' => continue,
    //             '&' =>          self.tokens.push(Token::OP(Operator::AND)),
    //             '|' =>          self.tokens.push(Token::OP(Operator::OR)),
    //             '^' =>          self.tokens.push(Token::OP(Operator::XOR)),
    //             '!' | '~' =>    self.tokens.push(Token::OP(Operator::NOT)),
    //             '(' =>          self.tokens.push(Token::LParen),
    //             ')' =>          self.tokens.push(Token::RParen),
    //             _   => {
    //                 if c.is_alphabetic() && bool_vars.contains_key(&c) {
    //                     bool_vars.insert(c, bool_vars.len());
    //                     self.num_vars += 1;
    //                 }
    //                 self.tokens.push(Token::VAR(bool_vars.get(&c).unwrap().clone()));
    //             }
    //         }
    //     }

    //     Ok(()) 
    // }

    // fn convert_rpn(&mut self) {
    //     let mut ops: VecDeque<Token> = VecDeque::new();

    //     for token in self.tokens.iter() {
    //         match token {
    //             Token::VAR(_) => self.rpn.push(*token),
    //             Token::LParen => ops.push_back(*token),
    //             Token::RParen => {
    //                 loop {
    //                     let top = ops.pop_back().unwrap(); 
    //                     if top == Token::LParen { break; }
    //                     self.rpn.push(top);
    //                 }
    //             },
    //             _ => ops.push_back(*token)
    //         }
    //     }

    //     while ops.len() != 0 {
    //         self.rpn.push(ops.pop_front().unwrap());
    //     }
    // }
}

// #[cfg(test)]
// mod test {
    // use super::*;
// 
    // #[test]
    // fn empty_string() {
        // assert!(Expr::build_expr(String::from("")).is_err());
    // }
// 
    // #[test]
    // fn simple_parse_test() {
        // let parse_expected: Vec<Token>  = vec![Token::VAR(0), Token::OP(Operator::AND), Token::VAR(1)];
        // let rpn_expected: Vec<Token> = vec![Token::VAR(0), Token::VAR(1), Token::OP(Operator::AND)];
        // let expression = Expr::build_expr(String::from("a & b")).unwrap();
// 
        // assert_eq!(expression.get_num_vars(), 2);
        // assert_eq!(expression.tokens, parse_expected);
        // assert_eq!(expression.rpn, rpn_expected);
    // }
// 
    // #[test]
    // fn complex_build_test() {
        // let input = String::from("((a ^ b) & ~(c & d)) | e");
        // let parse_expected: Vec<Token> = vec![Token::LParen, Token::LParen, Token::VAR(0), Token::OP(Operator::XOR), Token::VAR(1),
        // Token::RParen, Token::OP(Operator::AND), Token::OP(Operator::NOT), Token::LParen, Token::VAR(2), Token::OP(Operator::AND), Token::VAR(3), Token::RParen, Token::RParen, Token::OP(Operator::OR), Token::VAR(4)];
        // let rpn_expected: Vec<Token> = vec![Token::VAR(0), Token::VAR(1), Token::OP(Operator::XOR), Token::VAR(2), Token::VAR(3), Token::OP(Operator::AND), 
        // Token::OP(Operator::NOT), Token::OP(Operator::AND), Token::VAR(4), Token::OP(Operator::OR)];
        // 
        // let expression = Expr::build_expr(input).unwrap();
// 
        // assert_eq!(expression.tokens, parse_expected);
        // assert_eq!(expression.rpn, rpn_expected);
    // }
// 
// }