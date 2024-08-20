use std::fs::File;
use std::io::{BufReader, Read};

fn read_input(p: &str) -> Vec<u8> {
    let f = File::open(p).unwrap();
    let mut r = BufReader::new(f);
    let mut s = Vec::new();

    r.read_to_end(&mut s).unwrap();

    return s
}

fn next_house(c: (i32, i32), i: u8) -> (i32, i32) {
    match i {
        60 => (c.0 - 1, c.1),
        62 => (c.0 + 1, c.1),
        94 => (c.0, c.1 + 1),
        118 => (c.0, c.1 - 1),
        _ => panic!()
    }
}

fn main() {
    let input = read_input("../input");

    let mut coords: Vec<(i32, i32)> = Vec::new();
    coords.push((0, 0));

    for x in input.clone() {
        let last_coords = coords.last().unwrap();
        let new_coords = next_house(*last_coords, x);
        coords.push(new_coords)
    }

    coords.sort();
    coords.dedup();

    let ans1 = coords.len();

    println!("Answer 1: {:?}", ans1);

    coords.clear();
    coords.push((0, 0));
    coords.push((0, 0));

    for (i, x) in input.into_iter().enumerate() {
        let last_coords = coords[i];
        let new_coords = next_house(last_coords, x);
        coords.push(new_coords)
    }

    coords.sort();
    coords.dedup();

    let ans2 = coords.len();

    println!("Answer 1: {:?}", ans2);
}
