use std::default;

use crate::token::{Literal, Token};
use crate::tokenType::TokenType;
use crate::error; // crate is the branch to use, not lox. Check cargo.toml for why

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self { // Very different from book because Rust cannot handle null.
        Scanner { 
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1, 
        }
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
                    // comment until end of line
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH);
                }
            }
            ' ' | '\r' | '\t' => {} // These are the 'do nothing' chars
            '\n' => self.line += 1, // Also a 'do nothing' char, but must update line count
             _ => error(had_error, self.line, "Unexpected Character"),
        }
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

    fn advance(& mut self) -> char {
        let c = self.source.as_bytes()[self.current] as char; // Needs 'as_bytes' because I cannot slice strings without it
        self.current += 1; // after reading char, advance counter. Similar to sexpression project strucutre.
        c // without 'as char' 2 lines above, this attempts to return a u8
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = self.source[self.start..self.current].to_string(); // have to use string slice to make string with .to-string()
        self.tokens.push(Token::new(token_type, text, None, self.line));
    }
}