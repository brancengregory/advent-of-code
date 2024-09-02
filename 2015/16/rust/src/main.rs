use std::{fs::File, io::{Error, Read, Result}, ops::Add};
use regex::Regex;

fn read_input(p: &str) -> Result<Vec<String>> {
    let mut f = File::open(p)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    Ok(s.lines().map(|x| x.to_string()).collect())
}

#[derive(Debug)]
struct Record {
    items: Vec<(String, i8)>
}

impl Record {
    fn from(s: String) -> Self {
        let re = Regex::new("Sue \\d+: (.*)").unwrap();
        let captures = re.captures(&s).unwrap();
        let items: Vec<(String, i8)> = captures[1]
            .split(",")
            .map(|s| {
                let parts: Vec<&str> = s.trim().split(':').collect();
                let name = parts[0].trim().to_string();
                let value = parts[1].trim().parse::<i8>().unwrap();
                (name, value)
            })
            .collect();

        Record {
            items
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Sue {
    children: i8,
    cats: i8,
    samoyeds: i8,
    pomeranians: i8,
    akitas: i8,
    vizslas: i8,
    goldfish: i8,
    trees: i8,
    cars: i8,
    perfumes: i8
}

impl Sue {
    fn new() -> Self {
        Sue {
            children: 0,
            cats: 0,
            samoyeds: 0,
            pomeranians: 0,
            akitas: 0,
            vizslas: 0,
            goldfish: 0,
            trees: 0,
            cars: 0,
            perfumes: 0
        }
    }

    fn from(r: Record) -> Self {
        let mut s = Sue::new();
        r.items.iter().for_each(|(name, value)| {
            match name.as_str() {
                "children" => s.children = *value,
                "cats" => s.cats = *value,
                "samoyeds" => s.samoyeds = *value,
                "pomeranians" => s.pomeranians = *value,
                "akitas" => s.akitas = *value,
                "vizslas" => s.vizslas = *value,
                "goldfish" => s.goldfish = *value,
                "trees" => s.trees = *value,
                "cars" => s.cars = *value,
                "perfumes" => s.perfumes = *value,
                _ => panic!("Bad field"),
            }
        });

        s
    }

    fn matches(&self, other: &Sue) -> bool {
        (self.children == 0 || self.children == other.children) &&
        (self.cats == 0 || self.cats == other.cats) &&
        (self.samoyeds == 0 || self.samoyeds == other.samoyeds) &&
        (self.pomeranians == 0 || self.pomeranians == other.pomeranians) &&
        (self.akitas == 0 || self.akitas == other.akitas) &&
        (self.vizslas == 0 || self.vizslas == other.vizslas) &&
        (self.goldfish == 0 || self.goldfish == other.goldfish) &&
        (self.trees == 0 || self.trees == other.trees) &&
        (self.cars == 0 || self.cars == other.cars) &&
        (self.perfumes == 0 || self.perfumes == other.perfumes)
    }

    fn newly_matches(&self, gift_sue: &Sue) -> bool {
        let exact_match = |sue_val, gift_val| {
            if gift_val == 0 {
                true // If `gift_sue`'s value is 0, allow any value in `Sue`
            } else {
                sue_val == gift_val // Otherwise, must be an exact match
            }
        };

        exact_match(self.children, gift_sue.children) &&
        exact_match(self.samoyeds, gift_sue.samoyeds) &&
        exact_match(self.akitas, gift_sue.akitas) &&
        exact_match(self.vizslas, gift_sue.vizslas) &&
        exact_match(self.cars, gift_sue.cars) &&
        exact_match(self.perfumes, gift_sue.perfumes) &&
        (gift_sue.cats == 0 || self.cats > gift_sue.cats) &&  // Cats must be greater than in gift_sue
        (gift_sue.trees == 0 || self.trees > gift_sue.trees) &&  // Trees must be greater than in gift_sue
        (gift_sue.pomeranians == 0 || self.pomeranians < gift_sue.pomeranians) &&  // Pomeranians must be less than in gift_sue
        (gift_sue.goldfish == 0 || self.goldfish < gift_sue.goldfish) // Goldfish must be less than in gift_sue
    }
}

fn main() -> Result<()>{
    let input: Vec<String> = read_input("../input")?;
    let aunts: Vec<Sue> = input.into_iter()
        .map(|s| {
            let r = Record::from(s);
            Sue::from(r)
        })
        .collect();

    let gift_sue = Sue {
        children: 3,
        cats: 7,
        samoyeds: 2,
        pomeranians: 3,
        akitas: 0,
        vizslas: 0,
        goldfish: 5,
        trees: 3,
        cars: 2,
        perfumes: 1
    };

    let which_sue: usize = aunts.iter().enumerate()
        .position(|(_, s)| s.matches(&gift_sue))
        .unwrap()
        .add(1);

    println!("{:?}", which_sue);

/*     for (i, sue) in aunts.iter().enumerate() {
        let matches = sue.newly_matches(&gift_sue);
        println!("Sue #{} matches: {} - {:?}", i + 1, matches, sue);
    }

    let which_sue_new = aunts.iter().enumerate()
        .position(|(_, s)| s.newly_matches(&gift_sue))
        .map(|i| i + 1);  // Convert zero-based index to one-based

    match which_sue_new {
        Some(index) => println!("Newly matching Sue found at index: {}", index),
        None => println!("No newly matching Sue found"),
    }
 */
    let test: Vec<&Sue> = aunts.iter()
        .filter(|&x| x.children == 3 && x.perfumes == 1)
        .collect();
    println!("{:?}", test);
    
    Ok(())
}
