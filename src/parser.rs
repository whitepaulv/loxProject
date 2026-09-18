use crate::token::Token;
use crate::tokenType::TokenType;
use crate::expression::Expr;

#[derive(Debug)]
struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token> ) -> Self {
        Parser {
            tokens: tokens,  
            current: 0,
        }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr: Expr;
        expr = comparison();

        while (self.match_token_types(TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL)) {
            let operator = 
        }
    }

    fn match_token_types(&mut self, types: TokenType) -> bool { // Need to change to an array!
        for each_type in types {
            if check(each_type) {
                advance();
                return true; // Pretty sure I cant just write 'true' and need 'return true;' for inside functions
            }
        }

        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if is_at_end() {
            return false;
        }
        return peek().token_type = token_type;
    }

    fn advance(&mut self) -> Token {
        if !is_at_end() {

        }
    }
}