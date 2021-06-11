/*
    100 DAYS OF CODE : Day 18
    AST Interpreter - craftinginterpreters

    Current:
    http://craftinginterpreters.com/scanning.html
*/

use std::{
    fs,
    env,
};
use one_hundred_days_of_code::ast::{
    tokens::{Token},
    scanner::{Scanner},
};

// Execution of code
fn run(source: String) -> Result<(), String>
{
    let mut scanner = Scanner::new(source);
    let tokens: &Vec<Token> = scanner.scan_tokens().unwrap();

    for tok in tokens
    {
        println!("Token ({:?})", tok.to_string());
    }

    Ok(())
}

// Read a file and run
fn run_file(file_path: String) -> Result<(), String>
{
    // Read source code from file
    match fs::read_to_string(file_path)
    {
        Ok(s) => run(s),
        Err(e) => Err(format!("Error: {}", e)),
    }
}

// Run as a prompt
fn run_prompt() -> Result<(), String>
{
    // TODO
    Ok(())
}


// Entry
fn main() -> Result<(), String>
{
    let args: Vec<_> = env::args().collect();

    let status: Result<(), String> = match args.len()
    {
        1 => run_prompt(),
        2 => run_file(args[1].clone()),
        _ => {
            Err(String::from("Usage: rlox [script]"))
        }
    };

    status
}