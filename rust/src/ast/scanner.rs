use std::usize;

use super::{
    lang::Lang,
    tokens::{Token, TokenType},
};

pub struct Scanner
{
    source: String,
    tokens: Vec<Token>,
    start: i32,
    current: i32,
    line: i32,
    lang: Lang,
}

impl Scanner
{
    pub fn new() -> Scanner
    {
        Scanner
        {
            source: String::new(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            lang: Lang::new(),
        }
    }

    pub fn with_source(source: String) -> Scanner
    {
        Scanner
        {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            lang: Lang::new(),
        }
    }

    pub fn set_source(&mut self, source: String)
    {
        self.source = source;
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, String>
    {
        while !self.is_at_end()
        {
            self.start = self.current;
            match self.scan_token()
            {
                Ok(_) => {},
                Err(_) => return Err(String::from("Failed to parse tokens")),
            }
        }

        // Push an EOF and return a ref to tokens generated
        self.tokens.push(Token::new(TokenType::EOF, String::new(), String::new(), self.line));
        Ok(&self.tokens)
    }

    fn substr(&self, start: usize, end: usize) -> &str
    {
        &self.source[start as usize..end as usize]
    }

    fn char_at(&self, pos: usize) -> Option<char>
    {
        self.source.chars().nth(pos)
    }

    fn add_token_lit(&mut self, type_of: TokenType, literal: String)
    {
        let text = self.substr(self.start as usize, self.current as usize).to_string();
        self.tokens.push(Token::new(type_of, text, literal, self.line));
    }

    fn add_token(&mut self, type_of: TokenType)
    {
        self.add_token_lit(type_of, String::new());
    }

    fn advance(&mut self) -> char
    {
        self.current += 1;
        //println!("Hitting char: {} of {}", self.current, self.source.len());
        self.char_at(self.current as usize - 1).unwrap()
    }

    fn peek(&self) -> char
    {
        if self.is_at_end() { return '\0' };
        self.char_at(self.current as usize).unwrap()
    }

    fn peek_next(&self) -> char
    {
        if self.current as usize + 1 >= self.source.len() { return '\0' }
        self.char_at(self.current as usize + 1).unwrap()
    }

    fn match_char(&mut self, expected: char) -> bool
    {
        if  self.is_at_end() ||
            self.source.chars().nth(self.current as usize).unwrap() != expected
        {
            return false; 
        }

        self.current += 1;
        true
    }

    fn is_digit(character: char) -> bool
    {
        character >= '0' && character <= '9'
    }

    fn is_alpha(character: char) -> bool
    {
        (character >= 'a' && character <= 'z') ||
        (character >= 'A' && character <= 'Z') ||
        character == '_'
    }

    fn is_alphanumeric(character: char) -> bool
    {
        Scanner::is_alpha(character) || Scanner::is_digit(character)
    } 

    fn number(&mut self)
    {
        while Scanner::is_digit(self.peek()) { self.advance(); }

        if self.peek() == '.' && Scanner::is_digit(self.peek_next())
        {
            self.advance();

            while Scanner::is_digit(self.peek()) { self.advance(); }
        }

        self.add_token_lit(TokenType::Number, self.substr(self.start as usize, self.current as usize).to_string());
    }

    fn string(&mut self) -> Result<(), String>
    {
        while self.peek() != '"' && !self.is_at_end()
        {
            if self.peek() == '\n' { self.line += 1; }
            self.advance();
        }

        if self.is_at_end()
        {
            return Err(format!("Undetermined string on {}", self.line));
        }

        self.advance();

        let value = self.substr(self.start as usize + 1, self.current as usize - 1).to_string();
        self.add_token_lit(TokenType::String, value);

        Ok(())
    }

    fn identifier(&mut self)
    {
        while Scanner::is_alphanumeric(self.peek()) { self.advance(); }

        let text = self.substr(self.start as usize, self.current as usize).to_string();

        if let Some(entry) = self.lang.keywords.get_key_value(&text)
        {
            // Cant do one-liner since borrow checker complains 
            let token_type = *entry.1;
            self.add_token(token_type);
        }
        else
        {
            self.add_token(TokenType::Identifier);
        }
    }

    fn scan_token(&mut self) -> Result<(), String>
    {
        use TokenType::*;

        let current_char: char = self.advance();

        match current_char
        {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            ';' => self.add_token(Semicolon),
            '*' => self.add_token(Star),
            '!' => {
                let tok = if self.match_char('=') { BangEqual } else { Bang };
                self.add_token(tok);
            }
            '=' => {
                let tok = if self.match_char('=') { EqualEqual } else { Equal };
                self.add_token(tok);
            }
            '<' => {
                let tok = if self.match_char('=') { LessEqual } else { Less };
                self.add_token(tok);
            }
            '>' => {
                let tok = if self.match_char('=') { GreaterEqual } else { Greater };
                self.add_token(tok);
            }
            '/' => {
                if self.match_char('=') 
                { 
                    while self.peek() != '\n' && !self.is_at_end()
                    {
                        self.advance();
                    }
                } else {  
                    self.add_token(Slash);
                };
            }

            '"' => {
                match self.string()
                {
                    Err(e) => return Err(e),
                    _ => {}
                }
            }

            // Whitespace
            ' ' | '\r' | '\t' => {},
            
            // Newline
            '\n' => self.line += 1,

            _ => 
            {
                if Scanner::is_digit(current_char)
                {
                    self.number();
                }
                else if Scanner::is_alpha(current_char)
                {
                    self.identifier();
                }
                else
                {
                    return Err(format!("Unexpected character {}", current_char)); // Should not hit this :^)
                }
            }
        }

        Ok(())
    }

    fn is_at_end(&self) -> bool
    {
        self.current >= self.source.len() as i32
    }
}