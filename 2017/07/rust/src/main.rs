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

fn get_weight(p: &str, vm: &HashMap<String, u32>, tm: &HashMap<String, Vec<String>>) -> u32 {
	let mut weight = vm.get(p).unwrap().clone();

	if let Some(children) = tm.get(p) {
		children.iter().for_each(|c| {
			weight += get_weight(c, vm, tm);
		})
	};

	weight
}

fn find_odd_one_out<'a>(
	children: &'a [String],
	weights: &'a [u32]
) -> Option<(&'a String, u32, u32)> {
	if weights.len() < 3 { return None; }

	let mut counts: HashMap<&u32, u32> = HashMap::new();
	for w in weights { *counts.entry(w).or_insert(0) += 1; }

	let odd_weight = *counts.iter().find(|(_, c)| **c == 1).map(|(w, _)| *w)?;
	let good_weight = *counts.iter().find(|(_, c)| **c > 1).map(|(w, _)| *w)?;
	let (odd_name, _) = children.iter()
		.zip(weights.iter())
		.find(|(_, w)| **w == odd_weight).unwrap();

	Some((odd_name, odd_weight, good_weight))
}

fn find_correction(
	node: &str,
	target_weight: u32,
	vm: &HashMap<String, u32>,
	tm: &HashMap<String, Vec<String>>,
) -> u32 {
	let children_names = tm.get(node);

	let mut children_weights: Vec<u32> = Vec::new();

	if let Some(names) = children_names {
		children_weights = names
			.iter()
			.map(|c| get_weight(c, vm, tm))
			.collect();
	}

	if let Some((odd_child_name, _, good_child_weight)) =
		find_odd_one_out(children_names.unwrap(), &children_weights)
	{
		return find_correction(odd_child_name, good_child_weight, vm, tm);
	} else {
		let my_current_total_weight = get_weight(node, vm, tm);
		let my_individual_weight = vm.get(node).unwrap();
		let diff = target_weight as i32 - my_current_total_weight as i32;
		return (*my_individual_weight as i32 + diff) as u32;
	}
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

	let branch_roots: &Vec<String> = treemap.get(root_node).unwrap();
	let branch_weights: Vec<u32> = branch_roots
		.iter()
		.map(|b| {
			get_weight(b, &valmap, &treemap)
		})
		.collect();

	let (heavy_branch, heavy_branch_weight, good_branch_weight) = find_odd_one_out(branch_roots, &branch_weights).unwrap();

	let answer = find_correction(
		heavy_branch,
		good_branch_weight,
		&valmap,
		&treemap
  );

	println!("{:?}", answer);
}
