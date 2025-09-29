use std::fs::read_to_string;

fn read_input(p: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let content = read_to_string(p)?;
    
    Ok(
        content
            .lines()
            .next()
            .unwrap()
            .parse::<i32>()
            .unwrap()
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input("../input")?;
    
    let n_houses = 1_000_000;

    let mut houses: Vec<i32> = vec![0; n_houses];

    for elf_number in 1..houses.len() {
        for house_number in (elf_number..houses.len()).step_by(elf_number) {
            houses[house_number] += elf_number as i32 * 10;
        }
    }

    for (house_number, presents) in houses.iter().enumerate() {
        if *presents >= input {
            println!("Part One: House {} gets {} presents", house_number, presents);
            break;
        }
    }

    // Part 2
    let mut houses: Vec<i32> = vec![0; n_houses];

    for elf_number in 1..houses.len() {
        let mut n_visited = 0;
        for house_number in (elf_number..houses.len()).step_by(elf_number) {
            if n_visited >= 50 { break };
            houses[house_number] += elf_number as i32 * 11;
            n_visited += 1;
        }
    }

    for (house_number, presents) in houses.iter().enumerate() {
        if *presents >= input {
            println!("Part Two: House {} gets {} presents", house_number, presents);
            break;
        }
    }
    
    Ok(())
}
