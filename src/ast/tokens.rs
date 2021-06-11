#[derive(Debug)]
pub enum TokenType
{
    // Single-character tokens.
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals.
    Identifier, String, Number,

    // Keywords.
    And, Class, Else, False, Func, For, If, Null, Or,
    Print, Return, Super, This, True, Var, While,

    EOF
}


pub struct Token
{
    pub type_of: TokenType,
    pub lexeme: String,
    pub literal: String,
    pub line: i32,
}

impl Token
{
    pub fn new(type_of: TokenType, lexeme: String, literal: String, line: i32) -> Token
    {
        Token
        {
            type_of,
            lexeme,
            literal,
            line
        }
    }

    pub fn to_string(&self) -> String
    {
        format!("{:?} {} {}", self.type_of, self.lexeme, self.literal)
    }
}