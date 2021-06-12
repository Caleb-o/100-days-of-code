use super::value::{Value, ValueArray};

#[derive(Debug)]
pub enum OpCode
{
    Constant,
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
    Return,

    Unknown,
}

impl From<u8> for OpCode
{
    fn from(orig: u8) -> Self
    {
        match orig
        {
            0 => Self::Constant,
            1 => Self::Negate,
            2 => Self::Add,
            3 => Self::Subtract,
            4 => Self::Multiply,
            5 => Self::Divide,
            6 => Self::Return,
            _ => Self::Unknown,
        }
    }
}

impl From<OpCode> for u8
{
    fn from(orig: OpCode) -> Self
    {
        use OpCode::*;
        match orig
        {
            Constant => 0,
            Negate => 1,
            Add => 2,
            Subtract => 3,
            Multiply => 4,
            Divide => 5,
            Return => 6,
            _ => 7,
        }
    }
}

pub struct Chunk
{
    pub(super) code: Vec<u8>,
    pub(super) lines: Vec<u32>,
    pub(super) constants: ValueArray,
}

impl Chunk
{
    pub fn new() -> Chunk
    {
        Chunk
        {
            code: Vec::new(),
            lines: Vec::new(),
            constants: ValueArray::new(),
        }
    }

    pub fn write(&mut self, byte: OpCode, line: u32)
    {
        self.code.push(byte as u8);
        self.lines.push(line);
    }

    pub fn write_constant(&mut self, byte: usize, line: u32)
    {
        self.code.push(byte as u8);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> usize
    {
        self.constants.write(value);
        self.constants.values.len() - 1
    }

    pub fn free(&mut self)
    {
        self.code.clear();
        self.lines.clear();
        self.constants.free();
    }
}