use std::io;
use std::collections::BTreeMap;
use std::{thread, time};

fn main() {
    let mut todo = BTreeMap::new();
    
    let mut finished = false;
    let mut counter = 1; 
    let default_sleep = time::Duration::from_secs(5);
    println!("Enter todo:");
    while finished != true {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input: String = input.trim().parse().expect("Empty");
        if input == String::from("q") {
            finished = true;
        } else {
            todo.insert(counter, input);
            counter +=1;
        }
    }
    for item in todo.values() {
        let mut complete = false;
        println!("TODO: {}", item);
        while complete != true {
            thread::sleep(default_sleep);
            print!("\x07");
            print!("\x07");
            print!("\x07");
            println!("Is the task complete?");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let input: String = input.trim().parse().expect("Empty");
            if input == "y" { 
                complete = true;
            } else {
                println!("Restarting timer");
            }
            
            }
    }
}
