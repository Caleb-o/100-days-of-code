use super::{
    value,
    chunk::{Chunk, OpCode},
};

pub fn disassemble(chunk: &Chunk, title: String)
{
    println!("== {} ==", title);

    let mut offset: usize = 0;

    while offset < chunk.code.len()
    {
        offset = disassemble_instruction(chunk, offset);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize
{
    print!("{:04} ", offset);

    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1]
    {
        print!("     | ");
    }
    else
    {
        print!("{:04} ", chunk.lines[offset]);
    }

    let instruction = OpCode::from(chunk.code[offset]);
    
    use OpCode::*;
    match instruction
    {
        Constant => return constant_instruction("OP_CONSTANT".to_string(), &chunk, offset),
        Negate => return simple_instruction("OP_NEGATE".to_string(), offset),
        Add => return simple_instruction("OP_ADD".to_string(), offset),
        Subtract => return simple_instruction("OP_SUBTRACT".to_string(), offset),
        Multiply => return simple_instruction("OP_MULTIPLY".to_string(), offset),
        Divide => return simple_instruction("OP_DIVIDE".to_string(), offset),
        Return => return simple_instruction("OP_RETURN".to_string(), offset),
       
        _ => {
            println!("Unknown opcode: {:?}", instruction);
            return  offset + 1
        }
    }
}

fn constant_instruction(name: String, chunk: &Chunk, offset: usize) -> usize
{
    let constant = chunk.code[offset + 1];
    print!("{:16} {:04} '", name, constant);
    value::print_value(chunk.constants.values[constant as usize]);
    println!("'");

    offset + 2
}

fn simple_instruction(name: String, offset: usize) -> usize
{
    println!("{}", name);
    offset + 1
}