pub enum Token {
    VAR((char, usize)),
    LParen,
    RParen,
    OP(Operator)
}

pub enum Operator {
    AND,
    OR,
    XOR,
    NOT
}

pub struct Expr {
    tokens: Vec<Token>,
    rpn: Vec<Token>
}

pub fn tokenize(input: &String) -> Result<Vec<Token>, String> {

}