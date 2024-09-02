use std::{collections::HashMap, fs::File, io::{Error, Read}};
use regex::Regex;
use itertools::Itertools;

fn read_input(p: &str) -> Result<Vec<String>, Error> {
    let mut f = File::open(p)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    Ok(s.lines().map(|x| x.to_string()).collect())
}

#[derive(Debug)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32 
}

impl From::<String> for Ingredient {
    fn from(value: String) -> Self {
        let re = Regex::new("(\\w+): capacity (-?\\d+), durability (-?\\d+), flavor (-?\\d+), texture (-?\\d+), calories (\\d+)").unwrap();
        let captures = re.captures(&value).unwrap();
        Ingredient {
            capacity: captures[2].parse().unwrap(),
            durability: captures[3].parse().unwrap(),
            flavor: captures[4].parse().unwrap(),
            texture: captures[5].parse().unwrap(),
            calories: captures[6].parse().unwrap(),
        }
    }
}

fn score_cookie(ingredients: &Vec<Ingredient>, quantities: &Vec<i32>) -> (i32, i32) {
    let mut scores: [i32; 4] = [0; 4];
    let mut calories: i32 = 0;

    ingredients.iter().enumerate().for_each(|(i, x)| {
        scores[0] += x.capacity * quantities[i];
        scores[1] += x.durability * quantities[i];
        scores[2] += x.flavor * quantities[i];
        scores[3] += x.texture * quantities[i];
        calories += x.calories * quantities[i]
    });

    let total_score = scores.into_iter().map(|v| 0.max(v)).reduce(|agg, v| agg * v).unwrap();

    (0.max(total_score), calories)
}

fn main() -> Result<(), Error>{
    let input = read_input("../input")?;

    let mut ingredients: Vec<Ingredient> = Vec::new();
    input
      .into_iter()
      .for_each(|x| ingredients.push(Ingredient::from(x)));

    let total_tsp = 100;
    let num_ingredients = ingredients.len();
    let remaining_tsp = total_tsp - num_ingredients; // Subtract one tsp per ingredient

    // Generate valid combinations ensuring each ingredient gets at least 1 tsp
    let possible_ratios = (0..=remaining_tsp).combinations_with_replacement(num_ingredients - 1)
        .map(|mut combo| {
            combo.push(0);
            combo.push(remaining_tsp);
            combo.sort();
            combo.windows(2).map(|window| (window[1] - window[0] + 1) as i32).collect::<Vec<i32>>()
        })
        .collect::<Vec<_>>();

    let mut scores: HashMap<Vec<i32>, (i32, i32)> = HashMap::new();

    possible_ratios.into_iter().for_each(|c| {
        scores.insert(c.clone(), score_cookie(&ingredients, &c));
    });

    let (max_key, max_value) = scores.clone()
      .into_iter()
      .max_by_key(|&(_, v)| v)
      .unwrap();

    println!("Problem 1: {:?} - {:?}", max_key, max_value);

    let (max_key, max_value) = scores
    .into_iter()
    .filter(|(_, (_, calories))| {
        *calories == 500
    })
    .max_by_key(|&(_, v)| v)
    .unwrap();

    println!("Problem 2: {:?} - {:?}", max_key, max_value);

    Ok(())
}
