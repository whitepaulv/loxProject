use crate::error;
use crate::expression::Expr;
use crate::token::Literal;
use crate::token::Token;
use crate::token_type::TokenType;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    /// Like Java's `panicMode`: suppress extra errors until recovery.
    panic_mode: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token> ) -> Self {
        Parser {
            tokens: tokens,  
            current: 0,
            panic_mode: false,
        }
    }

    pub fn parse(&mut self, had_error: &mut bool) -> Expr {
        self.expression(had_error)
    }

    fn expression(&mut self, had_error: &mut bool) -> Expr {
        self.equality(had_error)
    }

    fn equality(&mut self, had_error: &mut bool) -> Expr {
        let mut left: Expr;
        left = self.comparison(had_error);

        while self.match_token_types(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.comparison(had_error);
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
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current -1]
    }

    fn comparison(&mut self, had_error: &mut bool) -> Expr {
        let mut expr: Expr; //HAS TO BE MUT !!!  SAME WITH TERM !! Please remember to avoid more debugging forever
        expr = self.term(had_error);

        while self.match_token_types(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.term(had_error);
            expr = Expr::Binary { left: Box::new(expr), operator: operator, right: Box::new(right) } ;
        }

        expr
    }

    fn term(&mut self, had_error: &mut bool) -> Expr {
        let mut expr: Expr;
        expr = self.factor(had_error);

        while self.match_token_types(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor(had_error);
            expr = Expr::Binary { left: Box::new(expr), operator: operator, right: Box::new(right),
            };
        }
        expr
    }

    fn factor(&mut self, had_error: &mut bool) -> Expr {
        let mut expr: Expr = self.unary(had_error); //  This one too!!

        while self.match_token_types(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary(had_error);
            expr = Expr::Binary { left: Box::new(expr), operator: operator, right: Box::new(right),
            };
        }

        expr
    }

    fn unary(&mut self, had_error: &mut bool) -> Expr {
        if self.match_token_types(&[TokenType::Bang, TokenType::Minus]) {
            let prefix: Token = self.previous().clone();
            let expression: Expr = self.unary(had_error);
            return Expr::Unary { prefix: prefix, expression: Box::new(expression) };
        }

        self.primary(had_error)
    }

    fn primary(&mut self, had_error: &mut bool) -> Expr {
        if self.match_token_types(&[TokenType::False]) {
            return Expr::Literal { value_type: Some(Literal::Bool(false)) };
        }
        if self.match_token_types(&[TokenType::True]) {
            return Expr::Literal { value_type: Some(Literal::Bool(true)) };
        }
        if self.match_token_types(&[TokenType::Nil]) {
            return Expr::Literal { value_type: None };
        }
        if self.match_token_types(&[TokenType::Number, TokenType::String]) {
            return Expr::Literal { value_type: self.previous().literal.clone() };
        }
        if self.match_token_types(&[TokenType::LeftParen]) {
            let expr: Expr = self.expression(had_error);
            self.consume(
                had_error,
                TokenType::RightParen,
                "Expect ')' after expression.",
            );
            return Expr::Grouping { expression: Box::new(expr) };
        }

        self.error_at_current(had_error, "Expect expression.");
        self.synchronize();
        Expr::Literal { value_type: None }
    }

    fn consume(&mut self, had_error: &mut bool, token_type: TokenType, message: &str) -> Token {
        if self.check(token_type) {
            return self.advance().clone();
        }

        self.error_at_current(had_error, message);
        self.synchronize();
        self.peek().clone()
    }

    fn parse_error(&mut self, had_error: &mut bool, token: &Token, message: &str) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;
        error(had_error, token.line, message);
    }

    fn error_at_current(&mut self, had_error: &mut bool, message: &str) {
        let token = self.peek().clone();
        self.parse_error(had_error, &token, message);
    }

    fn synchronize(&mut self) {
        self.panic_mode = false;

        while !self.is_at_end() {
            if self.current > 0 && self.previous().token_type == TokenType::Semicolon { // Used to not have self.current > 0,
                return;                                                                 // but running "+" as my only input threw
            }                                                                           // an error, as there is no previous token 
                                                                                        // before the first token
            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }

            self.advance();
        }
    }
}



#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::ast::ast_printer;
    use crate::scanner::Scanner;

    /// Scan + parse; returns printed AST and whether any error was reported.
    fn parse(source: &str) -> (String, bool) {
        let mut had_error = false;
        let tokens = Scanner::new(source.to_string()).scan_tokens(&mut had_error);
        let mut parser = Parser::new(tokens);
        let expr = parser.parse(&mut had_error);
        (ast_printer(&expr), had_error)
    }

    #[test]
    fn parses_addition() {
        let (ast, err) = parse("1 + 2");
        assert!(!err, "unexpected parse/scan error");
        assert_eq!(ast, "(+ 1 2)");
    }

    #[test]
    fn multiplication_binds_tighter_than_addition() {
        let (ast, err) = parse("1 + 2 * 3");
        assert!(!err);
        assert_eq!(ast, "(+ 1 (* 2 3))");
    }

    #[test]
    fn grouping_overrides_precedence() { // INITIALLY FAILED- Had to change the assert_eq to have group(), I initially forgot this.
        let (ast, err) = parse("(1 + 2) * 3");
        assert!(!err);
        assert_eq!(ast, "(* (group (+ 1 2)) 3)");
    }

    #[test]
    fn parses_unary_minus() {
        let (ast, err) = parse("-123");
        assert!(!err);
        assert_eq!(ast, "(- 123)");
    }

    #[test]
    fn parses_equality() {
        let (ast, err) = parse("1 == 2");
        assert!(!err);
        assert_eq!(ast, "(== 1 2)");
    }

    #[test]
    fn parses_comparison() {
        let (ast, err) = parse("3 > 2");
        assert!(!err);
        assert_eq!(ast, "(> 3 2)");
    }

    #[test]
    fn parses_nil_and_booleans() {
        let (nil, err_nil) = parse("nil");
        assert!(!err_nil);
        assert_eq!(nil, "nil");

        let (t, err_t) = parse("true");
        assert!(!err_t);
        assert_eq!(t, "true");

        let (f, err_f) = parse("false");
        assert!(!err_f);
        assert_eq!(f, "false");
    }

    #[test]
    fn parses_string_literal() {
        let (ast, err) = parse("\"lox\"");
        assert!(!err);
        assert_eq!(ast, "lox");
    }

    #[test]
    fn missing_right_paren_sets_error() {
        let (_, err) = parse("(1 + 2");
        assert!(err, "expected parse error for unclosed paren");
    }

    #[test]
    fn bare_operator_sets_error() { // INITIALLY FAILED: had to add "self.current > 0" to synchronize. 
        let (_, err) = parse("+");  // The issue was that + was the first token, and synchronize tried to check the previous
        assert!(err, "expected parse error for leading +"); // token. This broke everything.
    }

    #[test]
    fn empty_input_sets_error() {
        let (_, err) = parse("");
        assert!(err, "expected parse error for empty expression");
    }
}

