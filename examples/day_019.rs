/*
    100 DAYS OF CODE : Day 19
    Bytecode Interpreter - crafting interpreters
*/
use one_hundred_days_of_code::bytecode::{
    vm::{VM, InterpretResult},
};
use std::{
    fs,
    env,
    io::{self, Write},
};

// Read a file and run
fn run_file(file_path: String) -> Result<(), String>
{
    let vm = VM::new();
    vm.init();

    use InterpretResult::*;

    // Read source code from file
    match fs::read_to_string(file_path)
    {
        Ok(s) => {
            match vm.interpret(s)
            {
                Okay => {},
                _ => return Err("Error occured".to_string()),
            }
        }
        Err(e) => return Err(format!("Error: {}", e)),
    }

    vm.free();
    Ok(())
}

// Run as a prompt
fn run_prompt() -> Result<(), String>
{
    println!("=== Rusty Lox Repl ===");

    let mut code = String::new();
    let vm = VM::new();
    vm.init();

    use InterpretResult::*;

    loop
    {
        print!("> ");
        
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut code).unwrap();

        if code.len() == 0 | 1
        {
            continue;
        }

        match vm.interpret(code.clone())
        {
            Okay => {},
            CompilerError | RuntimeError => return Err("Error occured".to_string()),
        }
    }
}

fn main()
{
    let args: Vec<_> = env::args().collect();

    let status = match args.len()
    {
        1 => run_prompt(),
        2 => run_file(args[1]),
        _ => {
            panic!("Usage: rlox [path]");
        }
    };
}