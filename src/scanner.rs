use std::default;
use std::collections::HashMap;

use crate::token::{Literal, Token};
use crate::tokenType::TokenType;
use crate::error; // crate is the branch to use, not lox. Check cargo.toml for why

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Self { // Very different from book because Rust cannot handle null.
        Scanner { 
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1, 
            keywords: Self::init_keywords(), // You cannot use self.(x) inside of a new funciton. (maybe you can but I couldn't figure it out)
        }
    }

    fn init_keywords() -> HashMap<String, TokenType> {
        let mut keywords = HashMap::new();

        keywords.insert("and".to_string(), TokenType::AND);
        keywords.insert("class".to_string(), TokenType::CLASS);
        keywords.insert("else".to_string(), TokenType::ELSE);
        keywords.insert("false".to_string(), TokenType::FALSE);
        keywords.insert("for".to_string(), TokenType::FOR);
        keywords.insert("fun".to_string(), TokenType::FUN);
        keywords.insert("if".to_string(), TokenType::IF);
        keywords.insert("nil".to_string(), TokenType::NIL);
        keywords.insert("or".to_string(), TokenType::OR);
        keywords.insert("print".to_string(), TokenType::PRINT);
        keywords.insert("return".to_string(), TokenType::RETURN);
        keywords.insert("super".to_string(), TokenType::SUPER);
        keywords.insert("this".to_string(), TokenType::THIS);
        keywords.insert("true".to_string(), TokenType::TRUE);
        keywords.insert("var".to_string(), TokenType::VAR);
        keywords.insert("while".to_string(), TokenType::WHILE);
        keywords
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_tokens(mut self, had_error: &mut bool) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token(had_error);
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            String::new(),
            None, // I can pass in 'None' here because its type is Option<Literal>
            self.line,
        ));

        self.tokens
    }  

    fn scan_token(&mut self, had_error: &mut bool) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            '!' => {
                let token_type = if self.match_char('=') {
                    TokenType::BANG_EQUAL
                } else {
                    TokenType::BANG
                };
                self.add_token(token_type);
            }
            '=' => {
                let token_type = if self.match_char('=') {
                    TokenType::EQUAL_EQUAL
                } else {
                    TokenType::EQUAL
                };
                self.add_token(token_type);
            }
            '<' => {
                let token_type = if self.match_char('=') {
                    TokenType::LESS_EQUAL
                } else {
                    TokenType::LESS
                };
                self.add_token(token_type);
            }
            '>' => {
                let token_type = if self.match_char('=') {
                    TokenType::GREATER_EQUAL
                } else {
                    TokenType::GREATER
                };
                self.add_token(token_type);
            }
            '/' => {
                if self.match_char('/') {
                    // Creates a comment. Comments stop at either a newline or EOF
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH);
                }
            }
            ' ' | '\r' | '\t' => {} // These are the 'do nothing' chars
            '\n' => self.line += 1, // Also a 'do nothing' char, but must update line count

            '"' => self.string(had_error),
            _ => {
                if self.is_digit(&c) {
                    self.number();
                } else if self.is_alpha(&c) {
                    self.identifier();
                } else {
                    error(had_error, self.line, "Unexpected character.");
                }
            }
        }
    }

    fn identifier(&mut self) {
        while(self.is_alpha_numeric(&(self.peek()))) {
            self.advance();
        }

        let text = self.source[self.start..self.current].to_string();
        let token_type = self.keywords.get(&text).copied().unwrap_or(TokenType::IDENTIFIER);
        self.add_token(token_type);
    }

    fn number(&mut self) {
        while(self.is_digit(&self.peek())) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(&(self.peek_next())) {
            self.advance();

            while(self.is_digit(&self.peek())) {
                self.advance();
            }
        }

        let text = self.source[self.start..self.current].to_string(); // Rust doesn't have the same 'source.substring() that Java does,
        let value: f64 = text.parse().expect("Invalid number literal");  // so this implementation is used instead of that in addToken.

        self.add_token_with_literal(TokenType::NUMBER, Some(Literal::Number(value)));
    }

    fn string(&mut self, had_error: &mut bool) {
        while (self.peek() != '"' && !(self.is_at_end())) {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if(self.is_at_end()) {
            error(had_error, self.line, "Undetermined string");
            return;
        }

        self.advance();

        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token_with_literal(TokenType::STRING, Some(Literal::String(value))); // Have to change how addToken is called from the book, just like I changed the function/method
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false; 
        }
        if self.source.as_bytes()[self.current] as char != expected { // Once again, have to use to_bytes() and as char
            return false;
        }

        self.current += 1;
        true // You can't do this inside if blocks, which is why I had to use return there
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source.as_bytes()[self.current] as char;
    }

    fn peek_next(&self) -> char {
        if (self.current + 1 >= self.source.len()) {
            return '\0';
        }
        self.source.as_bytes()[self.current + 1] as char // Have to do the weird Rust ".as_bytes() ... to char" thing again
    }

    fn is_alpha(&self, c: &char) -> bool {
        (*c >= 'a' && *c <= 'z') || (*c >= 'A' && *c <= 'Z') || *c == '_'
    }

    fn is_digit(&self, c: &char) -> bool {
        c.is_ascii_digit() // I have no idea why this funciton / method had to be created but the book made it.
    }

    fn is_alpha_numeric(&self, c: &char) -> bool {
        self.is_digit(c) || self.is_alpha(c)
    } 

    fn advance(& mut self) -> char {
        let c = self.source.as_bytes()[self.current] as char; // Needs 'as_bytes' because I cannot slice strings without it
        self.current += 1; // after reading char, advance counter. Similar to sexpression project strucutre.
        c // without 'as char' 2 lines above, this attempts to return a u8
    }

    fn add_token(&mut self, token_type: TokenType) { // Not entirely sure if overloading is supported in Rust or not, so I made 2 different
        self.add_token_with_literal(token_type, None); // functions, but one is essentially a helper for addTokenWithLiteral.
    }
    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<Literal>) { // Serves the purpose of addToken in the book
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(token_type, text, literal, self.line));
    }
}