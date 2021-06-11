use super::tokens::{Token, TokenType};

pub struct Scanner
{
    source: String,
    tokens: Vec<Token>,
    start: i32,
    current: i32,
    line: i32,
}

impl Scanner
{
    pub fn new(source: String) -> Scanner
    {
        Scanner
        {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
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

    fn add_token_lit(&mut self, type_of: TokenType, literal: String)
    {
        let text = &self.source[self.start as usize..self.current as usize];
        self.tokens.push(Token::new(type_of, String::from(text), literal, self.line));
    }

    fn add_token(&mut self, type_of: TokenType)
    {
        self.add_token_lit(type_of, String::new());
    }

    fn advance(&mut self) -> char
    {
        self.current += 1;
        self.source.chars().nth(self.current as usize).unwrap()
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
            _ => return Err(format!("Unexpected character {}", current_char)), // Should not hit this :^)
        }

        Ok(())
    }

    fn is_at_end(&self) -> bool
    {
        self.current >= self.source.len() as i32
    }
}