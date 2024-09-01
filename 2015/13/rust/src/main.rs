use std::{collections::{HashMap, HashSet}, fs::File, io::{Error, Read, Result}, ops::Add};
use regex::Regex;

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
    happiness: i64,
    target: String
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
            happiness: c.get(3).map_or(0, |m| m.as_str().parse::<i64>().unwrap()),
            target: c.get(4).map_or(String::new(), |m| m.as_str().to_string())
        }
      })
      .collect();

    let mut hm: HashMap<(String, String), i64> = HashMap::new();
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

    let mut unique_people: Vec<String> = hm.keys()
        .flat_map(|(subject, target)| vec![subject.clone(), target.clone()])
        .collect::<HashSet<String>>()
        .into_iter()
        .collect();

    unique_people.sort();

    println!("HashMap: {:?}", hm);
    println!("People: {:?}", unique_people);
    
    Ok(())
}