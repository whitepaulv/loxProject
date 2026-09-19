use crate::token::Token;
use crate::token::Literal;
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
        let mut left: Expr;
        left = self.comparison();

        while (self.match_token_types(&[TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL])) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.comparison();
            left = Expr::Binary { left: Box::new(left), operator: operator, right: Box::new(right) };
        } // I tend to forget to close these statements above with a ;. Need to remember this debugging

        left
    }

    fn match_token_types(&mut self, types: &[TokenType]) -> bool { // Need to change to an array!
        for each_type in types {
            if self.check(*each_type) {
                self.advance();
                return true; // Pretty sure I cant just write 'true' and need 'return true;' for inside functions
            }
        }

        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current -1]
    }

    fn comparison(&mut self) -> Expr {
        let mut expr: Expr; //HAS TO BE MUT !!!  SAME WITH TERM !! Please remember to avoid more debugging forever
        expr = self.term();

        while (self.match_token_types(&[TokenType::GREATER, TokenType::GREATER_EQUAL, TokenType::LESS, TokenType::LESS_EQUAL])) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.term();
            expr = Expr::Binary { left: Box::new(expr), operator: operator, right: Box::new(right) } ;
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr: Expr;
        expr = self.factor();

        while self.match_token_types(&[TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Expr::Binary { left: Box::new(expr), operator: operator, right: Box::new(right),
            };
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr: Expr = self.unary(); //  This one too!!

        while self.match_token_types(&[TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Expr::Binary { left: Box::new(expr), operator: operator, right: Box::new(right),
            };
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_token_types(&[TokenType::BANG, TokenType::MINUS]) {
            let prefix: Token = self.previous().clone();
            let expression: Expr = self.unary();
            return Expr::Unary { prefix: prefix, expression: Box::new(expression) };
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.match_token_types(&[TokenType::FALSE]) {
            return Expr::Literal { value_type: Some(Literal::Bool(false)) };
        }
        if self.match_token_types(&[TokenType::TRUE]) {
            return Expr::Literal { value_type: Some(Literal::Bool(true)) };
        }
        if self.match_token_types(&[TokenType::NIL]) {
            return Expr::Literal { value_type: None };
        }
        if self.match_token_types(&[TokenType::NUMBER, TokenType::STRING]) {
            return Expr::Literal { value_type: self.previous().literal.clone() };
        }
        if self.match_token_types(&[TokenType::LEFT_PAREN]) {
            let expr: Expr = self.expression();
            //consume
            return Expr::Grouping { expression: Box::new(expr) };
        }
        let expr: Expr;
        return expr; // This shouldn't happen ever

    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Token {
        if self.check(token_type) {
            return self.advance();
        }
        
    }

}