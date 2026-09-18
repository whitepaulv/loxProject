use crate::tokenType::TokenType;

#[derive(Debug)]
pub enum Literal {
    Number(f64),
    String(String),
}
#[derive(Debug)]
pub struct Token {
    token_type: TokenType, // Cannot call this 'type' like in book because it is a protected keyword
    pub lexeme: String,
    literal: Option<Literal>,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<Literal>, line: usize,) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
    
    pub fn to_string(&self) -> String {
        format!("{:?} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}