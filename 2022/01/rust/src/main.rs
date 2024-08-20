
// Read input file
use std::fs;

struct Elf {
    inventory: Vec<i32>,
}

fn main() {

    let input = fs::read_to_string("../input/day_1.txt")
        .expect("Something went wrong reading the file");

    let mut elves: Vec<Elf> = Vec::new();

    for line in input.lines() {
        match line.parse::<i32>() {
            Ok(id) => {
                let elf = Elf {
                    inventory: Vec::new(),
                };
                elves.push(elf);
            },
            Err(_) => {
                println!("Error parsing line: {}", line);
            }
        }
    }

    println!("Hello, world!");
}
