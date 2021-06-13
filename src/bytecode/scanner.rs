#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus,
    Semicolon, Slash, Star,

    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    
    // Literals.
    Identifier, String, Number,

    // Keywords.
    And, Class, Else, False,
    For, Func, If, Null, Or,
    Print, Return, Super, This,
    True, Var, While,
  
    Error, EOF
} 

#[derive(Copy, Clone)]
pub struct Token
{
    pub type_of: TokenType,
    pub start: usize,
    pub length: usize,
    pub line: usize,
}

pub struct Scanner
{
    source: String,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner
{
    pub fn new() -> Scanner
    {
        Scanner
        {
            source: String::new(),
            start: 0,
            current: 0,
            line: 0,
        }
    }

    pub fn init(&mut self, source: String)
    {
        self.source = source;
        self.line = 1;
    }

    pub fn scan_token(&mut self) -> Token
    {
        self.skip_whitespace();

        self.start = self.current;

        if self.is_at_end() { return self.make_token(TokenType::EOF); }

        let c: char = self.advance();

        if Scanner::is_alpha(c) { return self.identifier(); }
        if Scanner::is_digit(c) { return self.number(); }

        use TokenType::*;

        match c as u8
        {
            b'(' => return self.make_token(LeftParen),
            b')' => return self.make_token(RightParen),
            b'{' => return self.make_token(LeftBrace),
            b'}' => return self.make_token(RightBrace),
            b';' => return self.make_token(Semicolon),
            b',' => return self.make_token(Comma),
            b'.' => return self.make_token(Dot),
            b'-' => return self.make_token(Minus),
            b'+' => return self.make_token(Plus),
            b'/' => return self.make_token(Slash),
            b'*' => return self.make_token(Star),
            b'!' => 
            {
                let type_of = match self.match_type('=')
                {
                    true => BangEqual,
                    false => Bang,
                };
                return self.make_token(type_of);
            }
            b'=' => 
            {
                let type_of = match self.match_type('=')
                {
                    true => EqualEqual,
                    false => Equal,
                };
                return self.make_token(type_of);
            }
            b'<' => 
            {
                let type_of = match self.match_type('=')
                {
                    true => LessEqual,
                    false => Less,
                };
                return self.make_token(type_of);
            }
            b'>' => 
            {
                let type_of = match self.match_type('=')
                {
                    true => GreaterEqual,
                    false => Greater,
                };
                return self.make_token(type_of);
            }

            b'"' => return self.string(),

            b'\0' => return self.make_token(EOF),

            _ => return self.error_token("Unexpected Character.".to_string()),
        }
    }

    fn peek(&self) -> char
    {
        self.char_at(self.current).unwrap()
    }

    fn peek_next(&self) -> char
    {
        println!("[peek_next]");
        if self.is_at_end() { return '\0'; }
        self.char_at(self.current + 1).unwrap()
    }

    fn skip_whitespace(&mut self)
    {
        loop
        {
            match self.peek()
            {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' =>
                {
                    self.line += 1;
                    self.advance();
                }
                '/' =>
                {
                    if self.peek_next() == '/'
                    {
                        while self.peek() != '\n' && !self.is_at_end()
                        {
                            self.advance();
                        }
                    }
                    else
                    {
                        return;
                    }
                }
                _ => return,
            }
        }
    }

    fn string(&mut self) -> Token
    {
        while self.peek() != '"' && !self.is_at_end()
        {
            if self.peek() == '\n' { self.line += 1; }
            self.advance();
        }

        if self.is_at_end() { return self.error_token("Unterminated string.".to_string()); }

        self.advance();
        self.make_token(TokenType::String)
    }

    fn number(&mut self) -> Token
    {
        while Scanner::is_digit(self.peek()) { self.advance(); }

        if self.peek() == '.' && Scanner::is_digit(self.peek_next())
        {
            self.advance();
            
            while Scanner::is_digit(self.peek()) { self.advance(); }
        }

        self.make_token(TokenType::Number)
    }

    fn identifier(&mut self) -> Token
    {
        while Scanner::is_alpha(self.peek()) || Scanner::is_digit(self.peek())
        {
            self.advance();
        }

        self.make_token(self.identifier_type())
    }

    fn identifier_type(&self) -> TokenType
    {
        use TokenType::*;
        match self.char_at(self.start).unwrap()
        {
            'a' => return self.check_keyword(1, 2, "nd".to_string(), And),
            'c' => return self.check_keyword(1, 4, "lass".to_string(), Class),
            'e' => return self.check_keyword(1, 3, "lse".to_string(), Else),
            'f' =>
            {
                if self.current - self.start > 1
                {
                    match self.char_at(self.start + 1).unwrap()
                    {
                        'a' => return self.check_keyword(2, 3, "lse".to_string(), False),
                        'o' => return self.check_keyword(2, 1, "r".to_string(), For),
                        'n' => return self.check_keyword(2, 1, "c".to_string(), Func),
                        _ => return Identifier,                    
                    }
                }
                else
                {
                    // Should not get here
                    return Identifier;
                }
            }
            'i' => return self.check_keyword(1, 1, "f".to_string(), If),
            'n' => return self.check_keyword(1, 3, "ull".to_string(), Null),
            'o' => return self.check_keyword(1, 1, "r".to_string(), Or),
            'p' => return self.check_keyword(1, 4, "rint".to_string(), Print),
            'r' => return self.check_keyword(1, 5, "eturn".to_string(), Return),
            's' => return self.check_keyword(1, 4, "uper".to_string(), Super),
            't' => 
            {
                if self.current - self.start > 1
                {
                    match self.char_at(self.start + 1).unwrap()
                    {
                        'h' => return self.check_keyword(2, 2, "is".to_string(), This),
                        'r' => return self.check_keyword(2, 2, "ue".to_string(), True),
                        _ => return Identifier,                    
                    }
                }
                else
                {
                    // Should not get here
                    return Identifier;
                }
            }
            'l' => return self.check_keyword(1, 2, "et".to_string(), Var),
            'w' => return self.check_keyword(1, 4, "hile".to_string(), While),
            _ => return Identifier,
        }
    }

    fn check_keyword(&self, start: usize, length: usize, 
        rest: String, type_of: TokenType) -> TokenType
    {
        if self.current - self.start == start + length &&
            self.substr(start, start + length) == rest
        {
            return type_of;
        }

        TokenType::Identifier
    }

    fn match_type(&mut self, character: char) -> bool
    {
        if self.is_at_end() { return false; }
        if self.char_at(self.current).unwrap() != character { return false; }

        self.current += 1;
        true
    }

    fn char_at(&self, pos: usize) -> Option<char>
    {
        self.source.chars().nth(pos)
    }

    pub fn substr(&self, start: usize, end: usize) -> String
    {
        self.source[start as usize..end as usize].to_string()
    }

    fn advance(&mut self) -> char
    {
        self.current += 1;
        self.char_at(self.current - 1).unwrap()
    }

    fn is_at_end(&self) -> bool
    {
        //? Maybe match on the option
        self.current >= self.source.len()
    }

    fn is_digit(character: char) -> bool
    {
        character >= '0' && character <= '9'
    }

    fn is_alpha(character: char) -> bool
    {
        character >= 'a' && character <= 'z' ||
        character >= 'A' && character <= 'Z' ||
        character == '_'
    }

    fn make_token(&self, type_of: TokenType) -> Token
    {
        Token
        {
            type_of,
            start: self.start,
            length: self.current - self.start,
            line: self.line,
        }
    }

    fn error_token(&self, message: String) -> Token
    {
        Token
        {
            type_of: TokenType::Error,
            start: self.start,
            length: message.len(),
            line: self.line,
        }
    }
}