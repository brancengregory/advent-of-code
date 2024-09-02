use std::{collections::{HashMap, HashSet}, fs::File, i32, io::{Error, Read, Result}, ops::Add};
use regex::Regex;
use itertools::Itertools;

fn read_input(p: &str) -> std::io::Result<Vec<String>> {
    let mut f = File::open(p)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    Ok(s.lines().map(|x| x.to_string()).collect())
}

#[derive(Debug)]
struct Rule {
    subject: String,
    method: String,
    happiness: i32,
    target: String
}

fn total_happiness(p: &Vec<&String>, h: &HashMap<(String, String), i32>) -> i32 {
    let mut total_happiness = 0;
    for i in 0..p.len() {
        let p1 = p[i];
        let p2 = p[(i + 1) % p.len()];
        total_happiness += h.get(&(p1.clone(), p2.clone())).unwrap();
        total_happiness += h.get(&(p2.clone(), p1.clone())).unwrap();
    }

    return total_happiness
}

fn main() -> Result<()> {
    let input = read_input("../input")?;

    let re = Regex::new("^(\\w+) would (\\w+) (\\d+) happiness units by sitting next to (\\w+)\\.$").unwrap();

    let rules: Vec<Rule> = input
      .iter()
      .map(|x| {
        let c = re.captures(x).unwrap();
        Rule {
            subject: c.get(1).map_or(String::new(), |m| m.as_str().to_string()),
            method: c.get(2).map_or(String::new(), |m| m.as_str().to_string()),
            happiness: c.get(3).map_or(0, |m| m.as_str().parse::<i32>().unwrap()),
            target: c.get(4).map_or(String::new(), |m| m.as_str().to_string())
        }
      })
      .collect();

    let mut hm: HashMap<(String, String), i32> = HashMap::new();
    rules.iter().for_each(|x| {
        let h = if x.method == "gain" {
            x.happiness
        } else if x.method == "lose" {
            -x.happiness
        } else {
            0
        };
        hm.insert((x.subject.clone(), x.target.clone()), h);
    });

    let mut people: Vec<String> = hm.keys()
        .flat_map(|(subject, target)| vec![subject.clone(), target.clone()])
        .collect::<HashSet<String>>()
        .into_iter()
        .collect();

    people.sort();

    let mut max_happiness = i32::MIN;

    for p in people.iter().permutations(people.len()) {
        max_happiness = max_happiness.max(total_happiness(&p, &hm));
    }

    println!("Problem 1: {:?}", max_happiness);

    for p in people.clone() {
        hm.insert(("Brancen".to_string(), p.clone()), 0);
        hm.insert((p, "Brancen".to_string()), 0);
    }

    people.push("Brancen".to_string());
    people.sort();

    max_happiness = i32::MIN;

    for p in people.iter().permutations(people.len()) {
        max_happiness = max_happiness.max(total_happiness(&p, &hm));
    }

    println!("Problem 2: {:?}", max_happiness);

    Ok(())
}