/*
    100 DAYS OF CODE: Day 003
    Occurance counter

    An attempt at a word / symbol occurance counter,
    which can process strings or files and output all
    words and the amount of times they occur within the
    string / file.

    I fought the borrow system a little on this, but it 
    wasn't too difficult to fix.
*/
use std::{
    io,
    fs,
    env,
    collections::HashMap,
};

struct Config
{
    buffer: String,
    file: bool,
}

// Process word / symnbol occurance
fn process_words(buffer: String) -> HashMap<String, i32>
{
    let mut map: HashMap<String, i32> = HashMap::new();

    for word in buffer.split_whitespace()
    {
        let count = map.entry(word.to_string()).or_insert(0);
        *count += 1;
    }

    map
}

/// A super duper terrible way to parse arguments :P
/// I only need an 'f' flag to specify a file, but
/// I should do this much better than it is.
/// I'm also using '=f' since cargo thinks its a flag
/// for it, instead of my program.
fn extract_args() -> Result<Config, &'static str>
{
    // Fetch all arguments
    let arguments: Vec<String> = env::args().collect();
    let file: bool;
    let buffer: String;

    match arguments.len()
    {
        1 => return Err("Program requires arguments. eg 'Text to process'"),
        2 => {
            if arguments[1] != "=f"
            {
                file = false;
                buffer = arguments[1].clone();
            }
            else
            {
                return Err("Incorrect argument provided");
            }
        }
        3 => {
            if arguments[1] == "=f"
            {
                file = true;
                buffer = arguments[2].clone();
            }
            else
            {
                return Err("Incorrect argument provided. Might be in the incorrect order.");
            }
        }
        _ => return Err("Too many arguments"),
    }

    // Program collected arguments
    Ok(Config
    {
        buffer,
        file,
    })
}

// Easy read from file
fn read_from_file(config: Config) -> io::Result<String>
{
    fs::read_to_string(config.buffer)
}

fn main()
{
    // Collect a configuration from arguments
    let conf: Config = match extract_args()
    {
        Ok(c) => c,
        Err(e) => panic!("Error: {}", e),
    };

    let map: HashMap<String, i32>;

    // Check if file is provided
    if conf.file
    {
        map = match read_from_file(conf)
        {
            Ok(s) => process_words(s),
            Err(e) => panic!("Error: {}", e),
        };
    }
    else
    {
        map = process_words(conf.buffer);
    }

    // Prettier than printing the hashmap
    for (word, count) in map
    {
        println!("{} - {}", word, count);
    }
}