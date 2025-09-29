use std::fs::read_to_string;
use std::collections::{BinaryHeap, HashMap, HashSet};
use regex::Regex;
use std::cmp::Reverse;

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
    
    let mut molset: HashSet<String> = HashSet::new();
    
    for (key, replacements) in &subs {
        for (start, matched) in molecule.match_indices(key) {
            for replacement in replacements {
                let m = format!(
                    "{}{}{}",
                    &molecule[..start],
                    replacement,
                    &molecule[start + matched.len()..]
                );
                molset.insert(m);
            }; 
        };
    };
    
    println!("Part 1: {:?}", molset.len());
    
    let mut reverse_subs: Vec<(String, String)> = subs
        .iter()
        .flat_map(|(from, to_vec)| to_vec.iter().map(|to| (to.clone(), from.clone())))
        .collect();

    reverse_subs.sort_by_key(|(to, _)| std::cmp::Reverse(to.len()));

    let mut current_molecule = molecule.clone();
    let mut steps = 0;

    // Keep reducing the molecule until it becomes "e".
    while current_molecule != "e" {
        let mut replaced_in_pass = false;
        // Find the first (and therefore longest) possible replacement.
        for (to, from) in &reverse_subs {
            if let Some(pos) = current_molecule.find(to) {
                // Perform the replacement.
                current_molecule.replace_range(pos..pos + to.len(), from);
                steps += 1;
                replaced_in_pass = true;
                // Important: Break and restart the scan with the new, smaller molecule.
                break;
            }
        }
        // Failsafe in case the molecule can't be reduced.
        if !replaced_in_pass {
            panic!("Could not reduce the molecule further.");
        }
    }

    println!("Part 2: {}", steps);
    
    Ok(())
}
