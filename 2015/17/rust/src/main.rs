use std::{
    fs::read_to_string,
    io::Result
};
use itertools::Itertools;

fn read_input(path: &str) -> Result<Vec<String>> {
    let content = read_to_string(path)?;

    Ok(content.lines().map(|s| s.to_string()).collect())
}

fn main() -> Result<()> {
    let input: Vec<String> = read_input("../input")?;

    let powerset: Vec<_> = input.iter().powerset().collect();
    
    let matches = powerset
        .into_iter()
        .filter_map(|v| {
            let v_num: Option<Vec<i32>> = v
                .iter()
                .map(|s| s.parse::<i32>().ok())
                .collect();

            v_num.map(|v| if v.iter().sum::<i32>() == 150 { v } else { vec![] })
        })
        .filter(|v| !v.is_empty())
        .collect::<Vec<_>>();

    let ans: usize = matches.len();
    
    println!("Part 1: {:?}", ans);
    
    Ok(())
}
