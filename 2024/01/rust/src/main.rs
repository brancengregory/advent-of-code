use std::fs::read_to_string;
use std::collections::HashMap;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string("../input")?;

    let parsed_input: Vec<Vec<&str>> = input
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.split_whitespace().collect())
        .collect();

    let mut x1: Vec<u32> = Vec::new();
    let mut x2: Vec<u32> = Vec::new();
    
    parsed_input.iter().for_each(|x| {
        x1.push(x[0].parse::<u32>().expect("Couldn't parse string to number"));
        x2.push(x[1].parse::<u32>().expect("Couldn't parse string to number"));
    });
   
    assert!(x1.len() == x2.len());
    
    x1.sort();
    x2.sort();

    let res: u32 = x2.iter()
        .zip(x1.iter())
        .map(|(v2, v1)| {
            v2.max(v1) - v2.min(v1)
        })
        .sum();
    
    println!("Part 1: {:?}", res);
  
    let mut tally: HashMap<u32, usize> = HashMap::new();
    let mut res = 0;
    
    x1.iter().for_each(|v| {
        let c = match tally.get(v) {
            Some(&c) => c,
            _ => {
                let nc = x2.iter().filter(|&&v2| v2 == *v).count();
                tally.insert(*v, nc);
                nc
        }};
        res += v * c as u32
    });

    println!("Part 2: {:?}", res);
    
    Ok(())
}
