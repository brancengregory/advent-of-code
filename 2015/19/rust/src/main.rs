use std::fs::read_to_string;
use std::collections::HashMap;
use regex::Regex;

fn read_input(p: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let content = read_to_string(p)?;
    
    Ok(
        content
            .lines()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    )
}

fn parse_input(input: Vec<String>) -> Result<(HashMap<String, Vec<String>>, String), Box<dyn std::error::Error>> {
    let re = Regex::new(r"^(\w+) => (\w+)$")?;
    
    let mut subs: HashMap<String, Vec<String>> = HashMap::new();
    let molecule = input.last().unwrap().to_string();

    for i in 0..(input.len() - 2) {
        let caps = re.captures(&input[i]).unwrap();
        subs
            .entry(caps.get(1).unwrap().as_str().to_string())
            .or_default()
            .push(caps.get(2).unwrap().as_str().to_string()); 
    };

    Ok((subs, molecule))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_input("../input")?;
    
    let (subs, molecule) = parse_input(input)?;
    
    println!("{}", molecule);
    
    Ok(())
}
