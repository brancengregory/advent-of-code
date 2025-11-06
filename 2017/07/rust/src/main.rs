use std::fs::read_to_string;
use std::collections::HashMap;

fn read_input(p: &str) -> Vec<Vec<String>> {
	let content = read_to_string(p).unwrap();

	content
		.lines()
		.map(|x| {
			x.split("->").map(String::from).collect()
		})
		.collect()
}

fn main() {
	let input = read_input("../input");

	let mut valmap: HashMap<String, u32> = HashMap::new();
	let mut treemap: HashMap<String, Vec<String>> = HashMap::new();
	input
		.iter()
		.for_each(|x| {
			let mut splits = x[0].split_whitespace();
			let k = splits.next().unwrap().to_string();
			let v = splits.next().unwrap()
				.strip_prefix("(").unwrap()
				.strip_suffix(")").unwrap()
				.parse::<u32>().unwrap();
			valmap.insert(k.clone(), v);

			if x.len() > 1 {
				let children: Vec<String> = x[1]
					.split(",")
					.map(|x| x.trim().to_string())
					.collect();

				treemap.insert(k, children);
			}
		});

	let all_nodes: Vec<&String> = valmap.keys().collect();
	let all_children: Vec<&String> = treemap.values().flatten().collect();

	let root_node: &String = all_nodes.into_iter().filter(|x| !all_children.contains(x)).next().unwrap();
	println!("{:?}", root_node);
}
