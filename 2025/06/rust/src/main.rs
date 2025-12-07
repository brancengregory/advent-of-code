fn read_input(p: &str) -> Vec<Vec<String>> {
	std::fs::read_to_string(p).unwrap()
		.lines()
		.map(|x| x.split_whitespace().map(String::from).collect())
		.collect()
}

fn main() {
	let input = read_input("../input");

	let operations = input.last().unwrap()[0].clone();
	let data = &input[0..input.len()-1];
	let mut ans: Vec<u32> = Vec::new();
	println!("{:?}", ans);

	for (i, x) in operations.chars().enumerate() {
		println!("x: {:?}", x);
		for cols in data[i].iter() {
			for v in cols.chars() {
				println!("{}", v);
			}
		}
	}

  println!("{:?}", ans);
}
