use std::fs::read_to_string;
use std::collections::HashSet;

fn read_input(p: &str) -> Vec<String> {
    read_to_string(p)
        .unwrap()
        .trim()
        .split(", ")
        .map(String::from)
        .collect::<Vec<String>>()
}

fn main() {
    let input = read_input("../input");

    let init_coords = (0, 0);
    let mut coords: (i32, i32) = (0, 0);
    let mut all_coords: HashSet<(i32, i32)> = HashSet::new();
    all_coords.insert(init_coords);
    let mut finding: bool = true;

    let mut dir: i32 = 0;
    
    for s in input {
        let mut chars = s.chars();
        let d = chars.next().unwrap();
        let n: i32 = chars.collect::<String>().parse::<i32>().unwrap();
        
        match d {
            'L' => {
                dir = (dir - 1).rem_euclid(4);
            },
            'R' => {
                dir = (dir + 1).rem_euclid(4);
            },
            _ => continue,
        }

        for _ in 0..n {
            match dir {
                0 => coords.1 += 1,
                1 => coords.0 += 1,
                2 => coords.1 -= 1,
                3 => coords.0 -= 1,
                _ => {},
            }

            if finding && !all_coords.insert(coords) {
                println!("{:?}", coords.0.abs() + coords.1.abs());
                finding = false;
            };
        }
    }

    println!("{:?}", coords);

    let dist = coords.0.abs() + coords.1.abs();
    println!("{:?}", dist);
}
