/*
 * 100 DAYS OF CODE : Day 005
 * BrainF*ck Interpreter
*/

use std::{env, fs, io};


const CELLS: usize = 30_000;


fn read_code(file_name: String) -> io::Result<String>
{
    fs::read_to_string(file_name)
}

fn execute(code: String) -> Result<(), String>
{
    let code: &[u8] = code.as_bytes();
    let mut ip: usize = 0;
    let size: usize = code.len();

    let mut memory = [0u8; CELLS];
    let mut mp: usize = 0;

    loop {
        match code[ip]
        {
            b'+' => memory[mp] += 1,
            b'-' => memory[mp] -= 1,
            b'<' => {
                mp -= 1;
                if mp >= CELLS
                {
                    mp = CELLS - 1;
                }
            }
            b'>' => {
                mp += 1;

                if mp == CELLS
                {
                    mp = 0;
                }
            }
            b',' => unimplemented!(),
            b'.' => print!("{}", memory[mp] as char),
            b'[' => {
                if memory[mp] == 0
                {
                    let mut cntr = 0;
                    
                    loop
                    {
                        ip += 1;

                        if code[ip] == b'[' { cntr += 1; }
                        if code[ip] == b']'
                        {
                            if cntr == 0
                            {
                                break;
                            }
                            cntr -= 1;
                        }
                    }
                }
            }
            b']' => {
                if memory[mp] != 0
                {
                    let mut cntr = 0;

                    loop
                    {
                        ip -= 1;

                        if code[ip] == b']' { cntr += 1; }
                        if code[ip] == b'['
                        {
                            if cntr == 0
                            {
                                break;
                            }

                            cntr -= 1;
                        }
                    }
                }
            }
            _ => {},
        }

        ip += 1;
        if ip as usize == size
        {
            break;
        }
    }

    Ok(())
}

fn main()
{
    let args: Vec<_> = env::args().collect();

    if args.len() == 2
    {
        let code: io::Result<String> = read_code(args[1].clone());
        
        match execute(code.unwrap())
        {
            Ok(_) => println!("Executed successfully!"),
            Err(e) => eprintln!("Error occured: {}", e),
        }
    }
}
