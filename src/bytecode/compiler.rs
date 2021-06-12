use super::{
    chunk::{Chunk, OpCode},
    scanner::{Scanner, Token, TokenType},
};

pub struct Parser
{
    current: Token,
    previous: Token,
    chunk: Chunk,
    had_error: bool,
    panic_mode: bool,
}

impl Parser
{
    pub fn new() -> Parser
    {
        Parser
        {
            current: Token { type_of: TokenType::EOF, start: 0, length: 0, line: 0 },
            previous: Token { type_of: TokenType::EOF, start: 0, length: 0, line: 0 },
            chunk: Chunk::new(),
            had_error: false,
            panic_mode: false,
        }
    }

    pub fn advance(&mut self, scanner: &Scanner)
    {
        self.previous = self.current;

        loop
        {
            self.current = scanner.scan_token();
            if self.current.type_of != TokenType::Error { break; }

            self.error_at_current(&scanner, format!("{}", self.current.start));
        }
    }

    pub fn compile(&mut self, source: String, chunk: Chunk) -> bool
    {
        let mut scanner = Scanner::new();
        scanner.init(source);

        self.chunk = chunk;

        self.had_error = false;
        self.panic_mode = false;

        self.advance(&scanner);
        self.expression();
        self.consume(&scanner, TokenType::EOF, "Expect end of expression.".to_string());

        self.end_compiler();
        !self.had_error
    }

    fn end_compiler(&mut self)
    {
        self.emit_return();
    }

    fn number(&mut self)
    {
        //TODO: convert to number
    }

    fn consume(&mut self, scanner: &Scanner, type_of: TokenType, message: String)
    {
        if self.current.type_of == type_of
        {
            self.advance(&scanner);
            return;
        }

        self.error_at_current(&scanner, message);
    }

    fn emit_return(&mut self)
    {
        self.emit_byte(OpCode::Return);
    }

    fn emit_byte(&mut self, byte: OpCode)
    {
        self.chunk.write(byte, self.previous.line);
    }

    fn emit_bytes(&mut self, byte1: OpCode, byte2: OpCode)
    {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn error_at_current(&mut self, scanner: &Scanner, message: String)
    {
        self.error_at(&scanner, self.current, message);
    }

    fn error(&mut self, scanner: &Scanner, message: String)
    {
        self.error_at(&scanner, self.previous, message);
    }

    fn error_at(&mut self, scanner: &Scanner, token: Token, message: String)
    {
        if self.panic_mode { return; }
        self.panic_mode = true;

        print!("[line {}] Error", token.line);

        if token.type_of == TokenType::EOF
        {
            print!(" at end");
        }
        else if token.type_of == TokenType::Error
        {
            // Nothing
        }
        else
        {
            print!(" at {}", scanner.substr(token.start, token.start + token.length));
        }

        println!(": {}", message);
        self.had_error = false;
    }
}