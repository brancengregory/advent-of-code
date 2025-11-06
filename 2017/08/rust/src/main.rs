use std::fs::read_to_string;
use std::collections::HashMap;

fn read_input(p: &str) -> Vec<Vec<String>> {
	read_to_string(p)
		.unwrap()
		.lines()
		.map(|x| {
			x
				.split(" if ")
				.map(String::from)
				.collect()
		})
		.collect()
}


fn main() {
	let input = read_input("../input");

	let mut registers: HashMap<&str, i32> = HashMap::new();
	input.iter().for_each(|x| {
		registers.insert(x[0].split_whitespace().next().unwrap(), 0);
		registers.insert(x[1].split_whitespace().next().unwrap(), 0);
	});

	input.iter().for_each(|x| {
		let mut c_splits = x[1].split_whitespace();
		let v1: &str = c_splits.next().unwrap();
		let op: &str = c_splits.next().unwrap();
		let v2: &str = c_splits.next().unwrap();

		let res: bool = match op {
			">" => v1 > v2,
			"<" => v1 < v2,
			">=" => v1 >= v2,
			"<=" => v1 <= v2,
			"==" => v1 == v2,
			"!=" => v1 != v2,
			_ => panic!()
		};

		if res {
			let mut o_splits = x[0].split_whitespace();
			let r: &str = o_splits.next().unwrap();
			let d: &str = o_splits.next().unwrap();
			let n: i32 = o_splits.next().unwrap().parse::<i32>().unwrap();

			match d {
				"inc" => registers.entry(r).and_modify(|x| *x += n),
				"dec" => registers.entry(r).and_modify(|x| *x -= n),
				_ => panic!()
			};
		}
	});

	let max_val = registers.values().max().unwrap();
	println!("{:?}", max_val);
}
