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

    reverse_subs.sort_by(|(to_a, from_a), (to_b, from_b)| {
        to_b.len().cmp(&to_a.len()).then_with(|| from_a.len().cmp(&from_b.len()))
    });

    let mut queue: BinaryHeap<Reverse<(usize, usize, String)>> = BinaryHeap::new();
    let mut visited: HashSet<String> = HashSet::new();

    queue.push(Reverse((0, molecule.len(), molecule.clone())));
    visited.insert(molecule.clone());
    
    while let Some(Reverse((steps, _, current))) = queue.pop() {
        if current == "e" {
            println!("Part 2: {:?}", steps);
            break;
        }

        for (to, from) in &reverse_subs {
            let mut pos = 0;
            while let Some(start) = current[pos..].find(to) {
                let start = start + pos;
                let m = format!(
                    "{}{}{}",
                    &current[..start],
                    from,
                    &current[start + to.len()..]
                );

                if visited.insert(m.clone()) && m.len() <= current.len() {
                    queue.push(Reverse((steps + 1, m.len(), m)));
                }

                pos = start + 1;
            }
        } 
    }
    
    Ok(())
}
