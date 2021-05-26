/* 
    100 DAYS OF CODE : Day 6
    TO-DO List

    This ended up going through a bit of a re-write / re-design
    around the 1-2 hour mark. It went from a terminal based menu
    with input, to a command line interface. This came from my 
    lack of knowledge about Rust's std in/out. This is my 
    first project using crates and I picked up serde and clap.

    Once I had both of those, it made the project easier to handle
    and work with. I don't know if I'm really using it right or not,
    but it works for now.

    This also took a bit of googling to get everything I wanted out 
    of the end result.
*/
extern crate clap;
extern crate serde;
extern crate serde_json;

use clap::{App, Arg, ArgMatches};
use serde::{Serialize, Deserialize};
use std::{fs::File, io::{Read, Result, Write}, path::Path};


#[derive(Serialize, Deserialize, Debug)]
struct TodoList
{
    not_completed: Vec<String>,
    completed: Vec<String>,
}

fn save_list<P: AsRef<Path>>(path: P, list: &TodoList) -> Result<()>
{
    // Create / open a file and write serialised list to it
    let mut file = File::create(path)?;
    let buf = serde_json::to_vec(&list)?;
    file.write_all(&buf[..])?;

    Ok(())
}

fn load_list<P: AsRef<Path>>(path: P) -> TodoList
{
    // File was found / could open
    if let Ok(mut file) = File::open(path)
    {
        let mut buf = vec![];

        if file.read_to_end(&mut buf).is_ok()
        {
            // Serde could deserialise data
            if let Ok(list) = serde_json::from_slice(&buf[..])
            {
                return list;
            }
        }
    }

    // Create a default list
    TodoList { not_completed: vec![], completed: vec![] }
}

/* 
    Basically makes a vec like a hashmap :P but a hashmap would
    not be suitable, since it requires a pair and I only use a String.
*/
fn find_name_index(name: &String, array: &mut Vec<String>) -> Option<usize>
{
    array.sort_unstable();

    match array.binary_search(name)
    {
        Ok(index) => Some(index),
        Err(_) => None,
    }
}


fn task_insert(todo_list: &mut TodoList, task_name: String)
{
    // Check for task
    if let Some(_) = find_name_index(&task_name, &mut todo_list.not_completed)
    {
        println!("A task with a similar name already exists");
        return;
    }

    todo_list.not_completed.push(task_name);
    save_list("misc/todo", &todo_list).unwrap();
}

fn task_remove(todo_list: &mut TodoList, task_name: String)
{
    // Check for task
    if let Some(index) = find_name_index(&task_name, &mut todo_list.not_completed)
    {
        todo_list.not_completed.remove(index);
        save_list("misc/todo", &todo_list).unwrap();
    }
}

fn task_complete(todo_list: &mut TodoList, task_name: String)
{
    if let Some(index) = find_name_index(&task_name, &mut todo_list.not_completed)
    {
        let item = todo_list.not_completed.remove(index);
        todo_list.completed.push(item);

        save_list("misc/todo", &todo_list).unwrap();
    }
}

fn task_destroy_all(todo_list: &mut TodoList)
{
    todo_list.not_completed.clear();
    todo_list.completed.clear();

    save_list("misc/todo", &todo_list).unwrap();
}

fn task_view(todo_list: &TodoList)
{
    if todo_list.not_completed.len() > 0
    {
        println!("=== Tasks to Complete ===");
        for (ind, item) in todo_list.not_completed.iter().enumerate()
        {
            println!("{}. {}", ind+1, item);
        }
    }

    if todo_list.completed.len() > 0
    {
        if todo_list.not_completed.len() > 0
        {
            print!("\n\n");
        }

        println!("=== Tasks Completed ===");
        for (ind, item) in todo_list.completed.iter().enumerate()
        {
            println!("{}. {}", ind+1, item);
        }
    }
}

fn main()
{
    // Create or load our todo list
    let mut todo_list: TodoList = load_list("misc/todo");

    // Setup clap
    let matches: ArgMatches = App::new("Todo List")
        .version("1.0")
        .author("Caleb O.")
        .arg(Arg::with_name("insert")
            .short("i")
            .long("insert")
            .value_name("TASK")
            .takes_value(true))
        .arg(Arg::with_name("remove")
            .short("r")
            .long("remove")
            .value_name("TASK")
            .takes_value(true))
        .arg(Arg::with_name("complete")
            .short("c")
            .long("complete")
            .value_name("TASK")
            .takes_value(true))
        .arg(Arg::with_name("view")
            .short("v")
            .long("view"))
        .arg(Arg::with_name("destroy")
            .short("d")
            .long("destroy"))
        .get_matches();

    
    // Insert a new task
    if let Some(task_name) = matches.value_of("insert") {
        task_insert(&mut todo_list, String::from(task_name));
    }

    // Remove a task
    if let Some(task_name) = matches.value_of("remove") {
        task_remove(&mut todo_list, String::from(task_name));
    }

    // Complete a new task
    if let Some(task_name) = matches.value_of("complete") {
        task_complete(&mut todo_list, String::from(task_name));
    }

    if matches.occurrences_of("view") > 0
    {
        task_view(&todo_list);
    }

    if matches.occurrences_of("destroy") > 0
    {
        task_destroy_all(&mut todo_list);
    }
}
