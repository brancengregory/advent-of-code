use std::fs::read_to_string;

fn read_input(p: &str) -> Vec<Vec<String>> {
    read_to_string(p).unwrap()
        .lines()
        .map(|x| x.chars().map(String::from).collect())
        .collect()
}

fn num_from_pos(p: (i32, i32)) -> Option<i32> {
    match p {
        (-1, 1) => Some(1),
        (0, 1) => Some(2),
        (1, 1) => Some(3),
        (-1, 0) => Some(4),
        (0, 0) => Some(5),
        (1, 0) => Some(6),
        (-1, -1) => Some(7),
        (0, -1) => Some(8),
        (1, -1) => Some(9),
        _ => None
    } 
}

fn num_from_weird_pos(p: &(i32, i32)) -> Option<&str> {
    match p {
        (0, 2) => Some("1"),
        (-1, 1) => Some("2"),
        (0, 1) => Some("3"),
        (1, 1) => Some("4"),
        (-2, 0) => Some("5"),
        (-1, 0) => Some("6"),
        (0, 0) => Some("7"),
        (1, 0) => Some("8"),
        (2, 0) => Some("9"),
        (-1, -1) => Some("A"),
        (0, -1) => Some("B"),
        (1, -1) => Some("C"),
        (0, -2) => Some("D"),
        _ => None
    } 
}

fn main() {
    let input = read_input("../input");

    let mut pos = (0, 0);

    for n in &input {
        for d in n {
            let c = match d.as_str() {
                "L" => (pos.0 - 1, pos.1),
                "R" => (pos.0 + 1, pos.1),
                "U" => (pos.0, pos.1 + 1),
                "D" => (pos.0, pos.1 - 1),
                _ => pos,
            };

            if num_from_pos(c).is_some() {
                pos = c;
            }
        }

        println!("{}", num_from_pos(pos).unwrap());
    }

    println!();

    pos = (0, 0);

    for n in &input {
        for d in n {
            let c = match d.as_str() {
                "L" => (pos.0 - 1, pos.1),
                "R" => (pos.0 + 1, pos.1),
                "U" => (pos.0, pos.1 + 1),
                "D" => (pos.0, pos.1 - 1),
                _ => pos,
            };

            if num_from_weird_pos(&c).is_some() {
                pos = c;
            }
        }

        println!("{}", num_from_weird_pos(&pos).unwrap());
    }
}
