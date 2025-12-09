use std::collections::VecDeque;

const INPUT: &str = "njfxhljp";

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    x: i32,
    y: i32,
    path: String,
}

fn hash_path(s: &str) -> String {
    let salt = format!("{}{}", INPUT, s);
    let digest = md5::compute(salt);

    let mut digest_str = format!("{:x}", digest);
    digest_str.truncate(4);
    
    digest_str
}

fn is_open(c: char) -> bool {
    if c.is_digit(10) || c == 'a' {
        return false 
    }

    true 
}

fn in_bounds(x: i32, y: i32) -> bool {
    x >= 0 && y >= 0 && x <= 3 && y <= 3
}

fn get_neighbors(s: &State) -> Vec<State> {
    let mut neighbors = Vec::new();
    
    let directions = [
        ('U', 0, -1),
        ('D', 0, 1),
        ('L', -1, 0),
        ('R', 1, 0)
    ];

    let neighbors_str = hash_path(&s.path);

    for (i, (d, dx, dy)) in directions.iter().enumerate() {
        let c = neighbors_str.chars().nth(i).unwrap(); 
        if is_open(c) {
            let new_x = s.x + dx;
            let new_y = s.y + dy;

            if in_bounds(new_x, new_y) {
                let mut new_path = s.path.clone();
                new_path.push(*d);
            
                let new_state = State {
                    x: new_x,
                    y: new_y,
                    path: new_path,
                };

                neighbors.push(new_state)
            }
        }
    }

    neighbors
}

fn find_shortest_path(start: &State, target: (i32, i32)) -> Option<State> {
    let mut queue = VecDeque::new();
    queue.push_back(start.clone());
    
    while let Some(current) = queue.pop_front() {
        if (current.x, current.y) == target { return Some(current) }

        for neighbor in get_neighbors(&current) {
            queue.push_back(neighbor);
        }
    }

    None
}

fn find_longest_path(start: &State, target: (i32, i32)) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back(start.clone());

    let mut max_len = 0;
    
    while let Some(current) = queue.pop_back() {
        if (current.x, current.y) == target {
            max_len = max_len.max(current.path.len());
            continue;
        }

        for neighbor in get_neighbors(&current) {
            queue.push_back(neighbor);
        }
    }

    max_len
}

fn main() {
    let start = State {
        x: 0,
        y: 0,
        path: "".to_string(),
    };

    let target = (3, 3);

    let shortest_path = find_shortest_path(&start, target);

    if shortest_path.is_some() {
        println!("{:?}", shortest_path.unwrap().path);
    }

    let longest_path = find_longest_path(&start, target);

    println!("{:?}", longest_path);
}
