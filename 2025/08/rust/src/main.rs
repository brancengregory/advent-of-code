use std::collections::HashMap;

type Coord = (i64, i64, i64);

struct CircuitManager {
	parents: HashMap<Coord, Coord>,
}

impl CircuitManager {
	fn new() -> Self {
		Self { parents: HashMap::new() }
	}

	fn find(&mut self, node: Coord) -> Coord {
		if let Some(&parent) = self.parents.get(&node) {
			if parent == node {
				node
			} else {
				let root = self.find(parent);
				self.parents.insert(node, root);
				root
			}
		} else {
			self.parents.insert(node, node);
			node
		}
	}

	fn union(&mut self, c1: Coord, c2: Coord) {
		let root1 = self.find(c1);
		let root2 = self.find(c2);

		if root1 != root2 {
			self.parents.insert(root1, root2);
		}
	}
}

fn euclidean_distance(c1: &Coord, c2: &Coord) -> f64 {
	let s: f64 = ((c2.0 - c1.0).pow(2) + (c2.1 - c1.1).pow(2) + (c2.2 - c1.2).pow(2)) as f64;
	s.sqrt()
}

fn read_input(p: &str) -> Vec<Coord> {
	std::fs::read_to_string(p).unwrap()
		.lines()
		.map(|x| {
			let v: Vec<i64> = x.split(",").map(|y| y.parse::<i64>().unwrap()).collect();
			(v[0], v[1], v[2])
		})
		.collect()
}

fn main() {
	let input = read_input("../input");

	let circuits: Vec<Vec<Coord>> = Vec::new();

	let mut edges: Vec<(Coord, Coord, f64)> = Vec::new();

	for i in 0..input.len() {
		for j in (i+1)..input.len() {
			let dist = euclidean_distance(&input[i], &input[j]);
			edges.push((input[i], input[j], dist));
		}
	}

	edges.sort_by(|a, b| a.2.total_cmp(&b.2));

	let mut manager = CircuitManager::new();

	for (c1, c2, dist) in edges.iter().take(1000) {
		manager.union(*c1, *c2);
	}

	let mut clusters: HashMap<Coord, Vec<Coord>> = HashMap::new();

	for coord in &input {
		let root = manager.find(*coord);
		clusters.entry(root).or_default().push(*coord);
	}

	let mut sizes: Vec<usize> = clusters.values().map(|v| v.len()).collect();

	sizes.sort_unstable_by(|a, b| b.cmp(a));

	let result: usize = sizes.iter().take(3).product();

	println!("{:?}", result);

	let mut manager = CircuitManager::new();

	let mut ans: i64 = 0;

	let mut distinct_groups = input.len();

	for (c1, c2, _dist) in edges {
		let root1 = manager.find(c1);
		let root2 = manager.find(c2);

		if root1 != root2 {
			manager.union(c1, c2);

			distinct_groups -= 1;

			if distinct_groups == 1 {
				ans = c1.0 * c2.0;
			}
		}
	}

	println!("{:?}", ans);
}
