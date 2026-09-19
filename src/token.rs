use crate::tokenType::TokenType;

#[derive(Debug, Clone)] 
pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
}
#[derive(Debug, Clone)]
pub struct Token { //  I'm making all these public. I don't know it it's needed but my life is easier bc of it.
    pub token_type: TokenType, // Cannot call this 'type' like in book because it is a protected keyword
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
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