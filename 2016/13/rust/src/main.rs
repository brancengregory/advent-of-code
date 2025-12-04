use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32
}

impl From<(i32, i32)> for Point {
    fn from(item: (i32, i32)) -> Self {
        Point {
            x: item.0,
            y: item.1
        }
    }
}

fn read_input(p: &str) -> i32 {
    std::fs::read_to_string(p).unwrap().trim().parse::<i32>().unwrap()
}

fn is_wall(p: &Point, input: i32) -> bool {
    let mut ans = (p.x * p.x) + (3 * p.x) + (2 * p.x * p.y) + p.y + (p.y * p.y);
    ans += input;
    let n_ones = ans.count_ones();

    n_ones % 2 != 0
}

fn neighbors(p: &Point, input: &i32) -> Vec<Point> {
    [
        (p.x + 1, p.y),
        (p.x, p.y + 1),
        (p.x - 1, p.y),
        (p.x, p.y - 1)
    ]
        .into_iter()
        .filter(|p| {
            p.0 >= 0 && p.1 >= 0 && !is_wall(&Point::from(*p), *input)
        })
        .map(Point::from)
        .collect()
}

fn build_path(paths: HashMap<Point, Point>, init: Point, dest: Point) -> Vec<Point> {
    let mut path = Vec::new();
    let mut current = dest; 

    while current != init {
        path.push(current);
        current = *paths.get(&current).unwrap();
    }

    path.push(init);
    path.reverse();
    
    path
}

fn search_shortest_path(input: &i32, init: &Point, dest: &Point) -> Option<Vec<Point>> {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut queue: VecDeque<Point> = VecDeque::new();
    let mut paths: HashMap<Point, Point> = HashMap::new();

    visited.insert(*init);
    queue.push_back(*init);

    while let Some(p) = queue.pop_front() {
        if p == *dest {
            return Some(build_path(paths, *init, *dest))
        }

        let ns = neighbors(&p, input);

        if !ns.is_empty() {
            for n in ns {
                if !visited.contains(&n) {
                    visited.insert(n);
                    paths.insert(n, p);
                    queue.push_back(n);
                }
            }
        }
    }

    None 
}

fn search_radius(input: &i32, init: &Point, steps: u32) -> Option<HashSet<Point>> {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut queue: VecDeque<(Point, u32)> = VecDeque::new();
    let mut paths: HashMap<Point, u32> = HashMap::new();

    visited.insert(*init);
    queue.push_back((*init, 0));

    while let Some((p, i)) = queue.pop_front() {
        if i == steps {
            return Some(visited)
        }

        let ns = neighbors(&p, input);

        if !ns.is_empty() {
            for n in ns {
                if !visited.contains(&n) {
                    visited.insert(n);
                    paths.insert(n, i + 1);
                    queue.push_back((n, i + 1));
                }
            }
        }
    }

    None 
}

fn main() {
    let input = read_input("../input");

    let init: Point = (1, 1).into();
    let dest: Point = (31, 39).into();

    let path = search_shortest_path(&input, &init, &dest);

    let min_steps = path.unwrap().len() - 1;
    println!("{:#?}", min_steps);

    let visited = search_radius(&input, &init, 50);
    let n_visited = visited.unwrap().len();
    println!("{:#?}", n_visited);
}
