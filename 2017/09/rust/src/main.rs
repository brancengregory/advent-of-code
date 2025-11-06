use std::fs::read_to_string;

fn read_input(p: &str) -> String {
	read_to_string(p).unwrap().trim().to_string()
}

fn main() {
	let input = read_input("../input");
  println!("{:?}", input.chars());

	let mut c: Vec<String> = input.chars().map(String::from).collect();

	for (i, x) in c.clone().into_iter().enumerate() {
		if x == "!" {
			c.remove(i);
			c.remove(i+1);
		}
		println!("{:?}: {:?}", i, x);
	}
}
