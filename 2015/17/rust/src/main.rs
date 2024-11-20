use std::{
    fs::read_to_string,
    io::Result,
};
use itertools::Itertools;

fn read_input(path: &str) -> Result<Vec<i32>> {
    let content = read_to_string(path)?;
    Ok(
        content
            .lines()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    )
}

fn main() -> Result<()> {
    let input: Vec<i32> = read_input("../input")?;

    // Generate the powerset of container indices to preserve uniqueness
    let powerset = (0..input.len()).powerset();

    let matches: Vec<Vec<usize>> = powerset
        .filter_map(|indices| {
            let sum: i32 = indices.iter().map(|&i| input[i]).sum();
            if sum == 150 {
                Some(indices)
            } else {
                None
            }
        })
        .collect();

    let ans = matches.len();
    
    println!("Part 1: {:?}", ans);

    let min_len = matches
        .iter()
        .map(|v| v.len())
        .min()
        .unwrap();
    
    let ans = matches
        .iter()
        .filter(|v| v.len() == min_len)
        .count();
    
    println!("Part 2: {:?}", ans);

    Ok(())
}

