use std::{collections::{HashMap, HashSet}, fs::File, i64, io::{self, Read}};
use regex::Regex;

type DistanceLookup = HashMap<(String, String), i64>;
type DPTable = HashMap<(u64, usize), i64>;

fn read_input(p: &str) -> io::Result<Vec<String>> {
    let mut f = File::open(p)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    Ok(s.lines().map(|x| x.to_string()).collect())
}

fn parse_input(s: &[String], d: &mut DistanceLookup) {
    let re = Regex::new(r"^(\w+) to (\w+) = (\d+)$").unwrap();

    s.iter().for_each(|x| {
        if let Some(caps) = re.captures(x) {
            let from = caps.get(1).unwrap().as_str().to_string();
            let to = caps.get(2).unwrap().as_str().to_string();
            let distance: i64 = caps.get(3).unwrap().as_str().parse().unwrap();

            d.insert((from.clone(), to.clone()), distance);
            d.insert((to, from), distance);
        }
    });
}

fn get_unique_cities(hm: &DistanceLookup) -> Vec<String> {
    let mut cl: HashSet<String> = HashSet::new();

    hm.keys().for_each(|(from, to)| {
        cl.insert(from.clone());
        cl.insert(to.clone());
    });

    let mut cities: Vec<String> = cl.into_iter().collect();
    cities.sort();

    return cities;
}

enum FillMethod {
    MinDistance,
    MaxDistance
}

fn fill(dpt: &mut DPTable, dl: &DistanceLookup, c: &Vec<String>, f: &FillMethod) {
    let num_cities = c.len();

    // Initialize base cases: starting from each city with only that city visited
    for i in 0..num_cities {
        dpt.insert((1 << i, i), 0);
    }

    // Iterate over all subsets of cities
    for mask in 0..(1 << num_cities) {
        for u in 0..num_cities {
            // If city u is in the subset represented by mask
            if mask & (1 << u) != 0 {
                if let Some(&current_dist_to_u) = dpt.get(&(mask, u)) {
                    for v in 0..num_cities {
                        // If city v is not in the subset represented by mask
                        if mask & (1 << v) == 0 {
                            let next_mask = mask | (1 << v);
                            if let Some(&distance_uv) = dl.get(&(c[u].clone(), c[v].clone())) {
                                let new_dist = current_dist_to_u + distance_uv;
                                match f {
                                    FillMethod::MinDistance => {
                                        let current_dist = dpt.entry((next_mask, v)).or_insert(i64::MAX);
                                        *current_dist = (*current_dist).min(new_dist)
                                    },
                                    FillMethod::MaxDistance => {
                                        let current_dist = dpt.entry((next_mask, v)).or_insert(i64::MIN);
                                        *current_dist = (*current_dist).max(new_dist)
                                    }
                                }
                            } else {
                                panic!("No distance found between {} and {}", c[u], c[v]);
                            }
                        }
                    }
                } else {
                    panic!("No entry found for mask: {:08b}, u: {} ({})", mask, u, c[u]);
                }
            }
        }
    }
}

fn evaluate(m: &FillMethod, dl: &DistanceLookup, c: &Vec<String>) -> i64 {
    let mut dp_table: DPTable = HashMap::new();
    fill(&mut dp_table, &dl, &c, m);

    let ans_mask: u64 = (1 << c.len()) - 1;
    let mut dist: i64 = match m {
        FillMethod::MaxDistance => i64::MIN,
        FillMethod::MinDistance => i64::MAX
    };

    for u in 0..c.len() {
        if let Some(&cached_dist) = dp_table.get(&(ans_mask, u)) {
          match m {
            FillMethod::MinDistance => dist = dist.min(cached_dist),
            FillMethod::MaxDistance => dist = dist.max(cached_dist)
          }
        } else {
          panic!(
            "No DP entry found for final mask: {:08b}, u: {} ({})",
            ans_mask, u, c[u]
          );
        }
      }

    return dist
}

fn main() -> io::Result<()> {
    let input = read_input("../input")?;

    let mut dl = DistanceLookup::new();
    parse_input(&input, &mut dl);

    let cities = get_unique_cities(&dl);

    let ans1: i64 = evaluate(&FillMethod::MinDistance, &dl, &cities);
    println!("Minimum distance to visit all cities: {}", ans1);

    let ans2: i64 = evaluate(&FillMethod::MaxDistance, &dl, &cities);
    println!("Maximum distance to visit all cities: {}", ans2);

    Ok(())
}
