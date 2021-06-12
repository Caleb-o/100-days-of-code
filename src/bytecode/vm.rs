#![allow(dead_code)]

use super::{
    debug,
    compiler::Parser,
    chunk::{Chunk, OpCode},
    value::{self, Value},
};


const STACK_MAX: usize = 256;


pub struct VM
{
    pub(super) chunk: Chunk,
    pub(super) ip: usize,
    stack: Vec<Value>,
}

pub enum InterpretResult
{
    Okay,
    CompilerError,
    RuntimeError,
}

enum BinaryOp
{
    ADD,
    SUB,
    MUL,
    DIV,
}

impl VM
{
    pub fn new() -> VM
    {
        VM
        {
            chunk: Chunk::new(),
            ip: 0,
            stack: Vec::new(),
        }
    }

    pub fn init(&mut self)
    {
        self.reset_stack();
    }

    pub fn free(&self)
    {

    }

    pub fn reset_stack(&mut self)
    {
        self.stack.clear();
        self.stack.reserve(STACK_MAX);
    }

    pub fn interpret(&mut self, source: String) -> InterpretResult
    {
        let mut parser = Parser::new();
        let mut chunk = Chunk::new();

        if !parser.compile(source, chunk)
        {
            chunk.free();
            return InterpretResult::CompilerError;
        }

        self.init();
        self.chunk = chunk;

        let result = self.run();
        
        self.free();
        result
    }

    fn push(&mut self, value: Value)
    {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Option<Value>
    {
        self.stack.pop()
    }

    fn read_byte(&mut self) -> OpCode
    {
        self.ip += 1;
        OpCode::from(self.chunk.code[self.ip - 1])
    }

    fn read_constant(&mut self) -> Value
    {
        let pos = self.read_byte() as u8;
        self.chunk.constants.values[pos as usize]
    }

    // Can probably turn this into a macro
    fn binary_op(&mut self, op: BinaryOp)
    {
        use BinaryOp::*;
        match op
        {
            ADD => {
                let b = self.pop().unwrap();
                let a = self.pop().unwrap();
                self.push(a + b);
            }
            SUB => {
                let b = self.pop().unwrap();
                let a = self.pop().unwrap();
                self.push(a - b);
            }
            MUL => {
                let b = self.pop().unwrap();
                let a = self.pop().unwrap();
                self.push(a * b);
            }
            DIV => {
                let b = self.pop().unwrap();
                let a = self.pop().unwrap();
                self.push(a / b);
            }
        }
    }

    fn run(&mut self) -> InterpretResult
    {
        use OpCode::*;
        loop
        {
            if cfg!(trace_exec = "true")
            {
                // Show stack
                print!("          ");
                for value in &self.stack
                {
                    print!("[ ");
                    value::print_value(*value);
                    print!(" ]");
                }
                println!();

                // Show instruction
                debug::disassemble_instruction(&self.chunk, self.ip);
            }

            let instruction = self.read_byte();

            match instruction
            {
                Constant =>
                {
                    let constant = self.read_constant();
                    self.push(constant);
                }
                Negate =>
                {
                    if let Some(val) = self.pop()
                    {
                        self.push(-val);
                    }
                }

                Add => self.binary_op(BinaryOp::ADD),
                Subtract => self.binary_op(BinaryOp::SUB),
                Multiply => self.binary_op(BinaryOp::MUL),
                Divide => self.binary_op(BinaryOp::DIV),

                Return => 
                {
                    if let Some(val) = self.pop()
                    {
                        value::print_value(val);
                        println!();
                    }
                    return InterpretResult::Okay;
                }
                _ => {}
            }
        }
    }
}