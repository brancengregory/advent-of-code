use regex::Regex;
use std::{
    fs::File,
    io::{Error, Read, Result},
    ops::Add,
};

fn read_input(p: &str) -> Result<Vec<String>> {
    let mut f = File::open(p)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    Ok(s.lines().map(|x| x.to_string()).collect())
}

#[derive(Debug)]
struct Record {
    items: Vec<(String, i8)>,
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

        Record { items }
    }
}

#[derive(Debug, Clone, Copy)]
struct Sue {
    children: Option<i8>,
    cats: Option<i8>,
    samoyeds: Option<i8>,
    pomeranians: Option<i8>,
    akitas: Option<i8>,
    vizslas: Option<i8>,
    goldfish: Option<i8>,
    trees: Option<i8>,
    cars: Option<i8>,
    perfumes: Option<i8>,
}

impl Sue {
    fn new() -> Self {
        Sue {
            children: None,
            cats: None,
            samoyeds: None,
            pomeranians: None,
            akitas: None,
            vizslas: None,
            goldfish: None,
            trees: None,
            cars: None,
            perfumes: None,
        }
    }

    fn from(r: Record) -> Self {
        let mut s = Sue::new();
        r.items
            .iter()
            .for_each(|(name, value)| match name.as_str() {
                "children" => s.children = Some(*value),
                "cats" => s.cats = Some(*value),
                "samoyeds" => s.samoyeds = Some(*value),
                "pomeranians" => s.pomeranians = Some(*value),
                "akitas" => s.akitas = Some(*value),
                "vizslas" => s.vizslas = Some(*value),
                "goldfish" => s.goldfish = Some(*value),
                "trees" => s.trees = Some(*value),
                "cars" => s.cars = Some(*value),
                "perfumes" => s.perfumes = Some(*value),
                _ => panic!("Bad field"),
            });

        s
    }

    fn matches(&self, other: &Sue) -> bool {
        (self.children.is_none() || self.children == other.children)
            && (self.cats.is_none() || self.cats == other.cats)
            && (self.samoyeds.is_none() || self.samoyeds == other.samoyeds)
            && (self.pomeranians.is_none() || self.pomeranians == other.pomeranians)
            && (self.akitas.is_none() || self.akitas == other.akitas)
            && (self.vizslas.is_none() || self.vizslas == other.vizslas)
            && (self.goldfish.is_none() || self.goldfish == other.goldfish)
            && (self.trees.is_none() || self.trees == other.trees)
            && (self.cars.is_none() || self.cars == other.cars)
            && (self.perfumes.is_none() || self.perfumes == other.perfumes)
    }

    fn ranged_matches(&self, other: &Sue) -> bool {
        (self.children.is_none() || self.children == other.children)
            && (self.samoyeds.is_none() || self.samoyeds == other.samoyeds)
            && (self.akitas.is_none() || self.akitas == other.akitas)
            && (self.vizslas.is_none() || self.vizslas == other.vizslas)
            && (self.cars.is_none() || self.cars == other.cars)
            && (self.perfumes.is_none() || self.perfumes == other.perfumes)
            && (self.cats.is_none() || self.cats > other.cats)
            && (self.trees.is_none() || self.trees > other.trees)
            && (self.pomeranians.is_none() || self.pomeranians < other.pomeranians)
            && (self.goldfish.is_none() || self.goldfish < other.goldfish)
    }
}

fn main() -> Result<()> {
    let input: Vec<String> = read_input("../input")?;
    let aunts: Vec<Sue> = input
        .into_iter()
        .map(|s| {
            let r = Record::from(s);
            Sue::from(r)
        })
        .collect();

    let gift_sue = Sue {
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
    };

    let which_sue: usize = aunts
        .iter()
        .enumerate()
        .position(|(_, s)| s.matches(&gift_sue))
        .expect("No compatible Sue found!")
        .add(1);

    println!("{:?}", which_sue);

    let which_sue: usize = aunts
        .iter()
        .enumerate()
        .position(|(_, s)| s.ranged_matches(&gift_sue))
        .expect("No compatible Sue found!")
        .add(1);

    println!("{:?}", which_sue);

    Ok(())
}
