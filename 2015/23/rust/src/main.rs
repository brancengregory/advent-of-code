use std::fs::read_to_string;

fn read_input(p: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
	let content = read_to_string(p)?;

	Ok(
		content
			.lines()
			.map(|x| x.to_string())
			.collect::<Vec<String>>()
	)
}

fn parse_input(v: Vec<String>) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
	let mut im: Vec<(String, String)> = Vec::new();

	v.iter().for_each(|s| {
		if let Some((k, v)) = s.split_once(' ') {
			im.push((k.to_string(), v.to_string()))
		}
	});

	Ok(im)
}

fn compute(v: Vec<(String, String)>, mut a: i32, mut b: i32) -> (i32, i32) {
	let mut over: bool = false;
	let mut current_instruction: isize = 0;

	while !over {
		// Check whether to end compute
		if current_instruction as usize >= v.len() || current_instruction < 0 {
			over = true;
			break
		}

		match v[current_instruction as usize].0.as_str() {
			"hlf" => {
				match v[current_instruction as usize].1.as_str() {
					"a" => a = (a / 2).max(0),
					"b" => b = (b / 2).max(0),
					&_ => ()
				}
				current_instruction += 1;
			},
			"tpl" => {
				match v[current_instruction as usize].1.as_str() {
					"a" => a = a * 3,
					"b" => b = b * 3,
					&_ => ()
				}
				current_instruction += 1;
			},
			"inc" => {
				match v[current_instruction as usize].1.as_str() {
					"a" => a += 1,
					"b" => b += 1,
					&_ => ()
				}
				current_instruction += 1;
			},
			"jmp" => {
				current_instruction += v[current_instruction as usize].1.parse::<isize>().unwrap();
			},
			"jie" => {
				let t: Vec<&str> = v[current_instruction as usize].1.split(", ").collect();
				match t[0] {
					"a" => if a % 2 == 0 {
						current_instruction += t[1].parse::<isize>().unwrap();
					} else {
						current_instruction += 1;
					},
					"b" => if b % 2 == 0 {
						current_instruction += t[1].parse::<isize>().unwrap();
					} else {
						current_instruction += 1;
					},
					&_ => ()
				}
			},
			"jio" => {
				let t: Vec<&str> = v[current_instruction as usize].1.split(", ").collect();
				match t[0] {
					"a" => if a == 1 {
						current_instruction += t[1].parse::<isize>().unwrap();
					} else {
						current_instruction += 1;
					},
					"b" => if b == 1 {
						current_instruction += t[1].parse::<isize>().unwrap();
					} else {
						current_instruction += 1;
					},
					&_ => ()
				}
			},
			&_ => ()
		}
	}

	(a, b)
}

fn main() {
	let input = read_input("../input").unwrap();
	let p = parse_input(input).unwrap();
	println!("{:?}", p);

	let mut a: i32 = 0;
	let mut b: i32 = 0;

	(a, b) = compute(p.clone(), a, b);

	println!("a: {:?} b: {:?}", a, b);

	a = 1;
	b = 0;

	(a, b) = compute(p.clone(), a, b);

	println!("a: {:?} b: {:?}", a, b);
}
