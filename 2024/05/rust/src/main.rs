use std::fs::read_to_string;
use std::collections::HashMap;

fn in_order(pages: &[&str], rules: &HashMap<&str, Vec<&str>>) -> bool {
    pages.iter().for_each(|p| {
       let p_rules = rules.get(p).unwrap();
       
    });

    true
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let input = read_to_string("../input")?;
    
    let rules: Vec<&str> = input.lines()
        .take_while(|&x| !x.is_empty())
        .collect();

    let mut ruleset: HashMap<&str, Vec<&str>> = HashMap::new();
    
    for r in rules {
        let parts: Vec<&str> = r.split("|").collect();
        if ruleset.contains_key(parts[0]) {
            let v = ruleset.get_mut(parts[0]).unwrap();
            v.push(parts[1]);
        } else {
            ruleset.insert(parts[0], vec![parts[1]]);
        }
    }
    
    let updates: Vec<Vec<&str>> = input.lines()
        .skip_while(|x| !x.is_empty())
        .map(|x| x.split(",").collect())
        .collect();

    let ans = updates.iter()
        .filter(|x| in_order(x, &ruleset))
        .count();
    
    println!("Part 1: {:?}", ans);

    Ok(())
}
