use std::fs::read;

fn next_house(c: (i32, i32), i: u8) -> (i32, i32) {
    match i {
        b'<' => (c.0 - 1, c.1),
        b'>' => (c.0 + 1, c.1),
        b'^' => (c.0, c.1 + 1),
        b'v' => (c.0, c.1 - 1),
        _ => panic!()
    }
}

fn main() {
    let input = read("../input").unwrap();

    let mut coords: Vec<(i32, i32)> = Vec::new();
    coords.push((0, 0));

    for x in input.iter() {
        let last_coords = coords.last().unwrap();
        let new_coords = next_house(*last_coords, *x);
        coords.push(new_coords)
    }

    coords.sort();
    coords.dedup();

    let ans1 = coords.len();

    println!("Answer 1: {:?}", ans1);

    coords.clear();
    coords.push((0, 0));
    coords.push((0, 0));

    for (i, x) in input.iter().enumerate() {
        let last_coords = coords[i];
        let new_coords = next_house(last_coords, *x);
        coords.push(new_coords)
    }

    coords.sort();
    coords.dedup();

    let ans2 = coords.len();

    println!("Answer 1: {:?}", ans2);
}
