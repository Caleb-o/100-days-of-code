use super::{
    debug,
    value::Value,
    chunk::{Chunk, OpCode},
    scanner::{Scanner, Token, TokenType},
};

#[derive(Debug, Copy, Clone)]
enum Precedence
{
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

#[derive(Copy, Clone)]
struct ParseRule<F>
    where F: for<'r> Fn(&'r mut Parser)
{
    prefix: Option<F>,
    infix: Option<F>,
    precedence: Precedence,
}

pub struct Parser
{
    current: Token,
    previous: Token,
    chunk: Chunk,
    had_error: bool,
    panic_mode: bool,
    scanner: Scanner,
    rules: Vec<ParseRule<Box<dyn Fn(&mut Parser)>>>,
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
            scanner: Scanner::new(),
            rules: vec![
                /* LeftParen */     ParseRule { prefix: Some(Box::new(Self::grouping)), infix: None, precedence: Precedence::None },
                /* RightParen */    ParseRule { prefix: None, infix: None, precedence: Precedence::None },
                /* LeftBrace */     ParseRule { prefix: None, infix: None, precedence: Precedence::None },
                /* RightBrace */    ParseRule { prefix: None, infix: None, precedence: Precedence::None },
                /* Comma */         ParseRule { prefix: None, infix: None, precedence: Precedence::None },
                /* Dot */           ParseRule { prefix: None, infix: None, precedence: Precedence::None },
                /* Minus */         ParseRule { prefix: Some(Box::new(Self::unary)), infix: Some(Box::new(Self::binary)), precedence: Precedence::Term },
                /* Plus */          ParseRule { prefix: None, infix: Some(Box::new(Self::binary)), precedence: Precedence::Term  },
                /* Semicolon */     ParseRule { prefix: None, infix: None, precedence: Precedence::None },
                /* Slash */         ParseRule { prefix: None, infix: Some(Box::new(Self::binary)), precedence: Precedence::Factor  },
                /* Star */          ParseRule { prefix: None, infix: Some(Box::new(Self::binary)), precedence: Precedence::Factor  },
                /* Bang */          ParseRule { prefix: None, infix: None, precedence: Precedence::None },
                /* BangEqual */     ParseRule { prefix: None, infix: None, precedence: Precedence::None },
                /* Equal */         ParseRule { prefix: None, infix: None, precedence: Precedence::None },
                /* EqualEqual */    ParseRule { prefix: None, infix: None, precedence: Precedence::None },
                /* Greater */       ParseRule { prefix: None, infix: None, precedence: Precedence::None },
                /* GreaterEqaul */  ParseRule { prefix: None, infix: None, precedence: Precedence::None },
                /* Less */          ParseRule { prefix: None, infix: None, precedence: Precedence::None },
                /* LessEqual */     ParseRule { prefix: None, infix: None, precedence: Precedence::None },
                /* Identifier */    ParseRule { prefix: None, infix: None, precedence: Precedence::None },
                /* String */        ParseRule { prefix: None, infix: None, precedence: Precedence::None },
                /* Number */        ParseRule { prefix: Some(Box::new(Self::number)), infix: None, precedence: Precedence::None },
                /* And */           ParseRule { prefix: None, infix: None, precedence: Precedence::None },
                /* Class */         ParseRule { prefix: None, infix: None, precedence: Precedence::None },
                /* Else */          ParseRule { prefix: None, infix: None, precedence: Precedence::None },
                /* False */         ParseRule { prefix: None, infix: None, precedence: Precedence::None },
                /* For */           ParseRule { prefix: None, infix: None, precedence: Precedence::None },
                /* Func */          ParseRule { prefix: None, infix: None, precedence: Precedence::None },
                /* If */            ParseRule { prefix: None, infix: None, precedence: Precedence::None },
                /* Null */          ParseRule { prefix: None, infix: None, precedence: Precedence::None },
                /* Or */            ParseRule { prefix: None, infix: None, precedence: Precedence::None },
                /* Print */         ParseRule { prefix: None, infix: None, precedence: Precedence::None },
                /* Return */        ParseRule { prefix: None, infix: None, precedence: Precedence::None },
                /* Super */         ParseRule { prefix: None, infix: None, precedence: Precedence::None },
                /* This */          ParseRule { prefix: None, infix: None, precedence: Precedence::None },
                /* True */          ParseRule { prefix: None, infix: None, precedence: Precedence::None },
                /* Var */           ParseRule { prefix: None, infix: None, precedence: Precedence::None },
                /* While */         ParseRule { prefix: None, infix: None, precedence: Precedence::None },
                /* Error */         ParseRule { prefix: None, infix: None, precedence: Precedence::None },
                /* EOF */           ParseRule { prefix: None, infix: None, precedence: Precedence::None },
            ],
        }
    }

    pub fn advance(&mut self)
    {
        self.previous = self.current;

        loop
        {
            self.current = self.scanner.scan_token();
            if self.current.type_of != TokenType::Error { break; }

            self.error_at_current(format!("{}", self.current.start));
        }
    }

    pub fn compile(&mut self, source: String, chunk: Chunk) -> bool
    {
        self.scanner.init(source);

        self.chunk = chunk;

        self.had_error = false;
        self.panic_mode = false;

        self.advance();
        self.expression();
        self.consume(TokenType::EOF, "Expect end of expression.".to_string());

        self.end_compiler();
        !self.had_error
    }

    fn end_compiler(&mut self)
    {
        self.emit_return();

        if cfg!(print_code = "true")
        {
            if !self.had_error
            {
                debug::disassemble(&self.chunk, "code".to_string());
            }
        }
    }

    fn binary(&mut self)
    {
        let operator_type = self.previous.type_of;
        let precedence = self.get_rule_precedence(operator_type);

        self.parse_precedence(match precedence
        {
            Precedence::None => Precedence::Assignment,
            Precedence::Assignment => Precedence::Or,
            Precedence::Or => Precedence::And,
            Precedence::And => Precedence::Equality,
            Precedence::Equality => Precedence::Comparison,
            Precedence::Comparison => Precedence::Term,
            Precedence::Term => Precedence::Factor,
            Precedence::Factor => Precedence::Unary,
            Precedence::Unary => Precedence::Call,
            Precedence::Call => Precedence::Primary,
            Precedence::Primary => Precedence::None,
        });

        use TokenType::*;
        match operator_type
        {
            Plus => self.emit_byte(OpCode::Add),
            Minus => self.emit_byte(OpCode::Subtract),
            Star => self.emit_byte(OpCode::Multiply),
            Slash => self.emit_byte(OpCode::Divide),
            _ => unimplemented!(), // Unreachable
        }
    }

    fn expression(&mut self)
    {
        self.parse_precedence(Precedence::Assignment);
    }

    fn parse_precedence(&mut self, precedence: Precedence)
    {
        self.advance();

        //* Borrows occur and we need to not, prefix and infix */
        //* calls are also not recommended in stable. */

        let type_of = self.previous.type_of;
        if let Some(prefix) = self.get_rule(type_of).prefix
        {
            //? Find better fix, this is unsafe
            prefix.call((self,));

            while precedence as usize <= self.get_rule_precedence(self.previous.type_of) as usize
            {
                self.advance();
                
                if let Some(infix) = self.get_rule_infix(type_of)
                {
                    infix.call((self,));
                }
            }
        }
    }

    fn get_rule(&self, type_of: TokenType) -> &ParseRule<Box<dyn Fn(&mut Parser)>>
    {
        &self.rules[type_of as usize]
    }

    fn get_rule_prefix(&self, type_of: TokenType) -> &Option<Box<dyn Fn(&mut Parser)>>
    {
        &self.get_rule(type_of).prefix
    }

    fn get_rule_infix(&self, type_of: TokenType) -> &Option<Box<dyn Fn(&mut Parser)>>
    {
        &self.get_rule(type_of).infix
    }

    fn get_rule_precedence(&self, type_of: TokenType) -> Precedence
    {
        let rule = self.get_rule(type_of);
        rule.precedence
    }

    fn grouping(&mut self)
    {
        self.expression();
        self.consume(TokenType::RightParen, "Expect ')' after expression.".to_string());
    }

    fn number(&mut self)
    {
        let tok = &self.previous;
        let value: Value = self.scanner.substr(tok.start, tok.start + tok.length)
                            .parse()
                            .unwrap();
        self.emit_constant(value);
    }

    fn unary(&mut self)
    {
        let operator_type: TokenType = self.previous.type_of;

        // Compile the operand
        self.parse_precedence(Precedence::Unary);

        use TokenType::*;
        match operator_type
        {
            Minus => self.emit_byte(OpCode::Negate),
            _ => unimplemented!(), // Unreachable
        }
    }

    fn consume(&mut self, type_of: TokenType, message: String)
    {
        if self.current.type_of == type_of
        {
            self.advance();
            return;
        }

        self.error_at_current(message);
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

    fn emit_constant(&mut self, value: Value)
    {
        let constant = self.make_constant(value);
        self.emit_bytes(OpCode::Constant, OpCode::from(constant));
    }

    fn make_constant(&mut self, value: Value) -> u8
    {
        let constant: usize = self.chunk.add_constant(value);

        if constant as u8 > u8::MAX
        {
            self.error("Too many constants in one chunk.".to_string());
            return 0;
        }

        constant as u8
    }

    fn error_at_current(&mut self, message: String)
    {
        self.error_at(self.current, message);
    }

    fn error(&mut self, message: String)
    {
        self.error_at(self.previous, message);
    }

    fn error_at(&mut self, token: Token, message: String)
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
            print!(" at {}", self.scanner.substr(token.start, token.start + token.length));
        }

        println!(": {}", message);
        self.had_error = false;
    }
}