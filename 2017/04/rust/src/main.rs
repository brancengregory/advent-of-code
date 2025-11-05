use std::fs::read_to_string;

fn read_input(p: &str) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
	let content = read_to_string(p)?;

	Ok(
		content
			.lines()
			.map(|x| {
				x.split_whitespace().map(String::from).collect()
			})
			.collect()
	)
}

fn main() {
	let input = read_input("../input").unwrap();

	let res: usize = input
		.clone()
		.into_iter()
		.filter(|p| {
			let mut d = p.clone();

			d.sort_unstable();
			d.dedup();

			d.len() == p.len()
		})
		.collect::<Vec<Vec<String>>>()
		.len();

  println!("{:?}", res);


	let new_res: usize = input
		.into_iter()
		.filter(|p| {
			let mut d = p.clone();

			d = d.into_iter()
				.map(|dd| {
					let mut c: Vec<char> = dd.chars().collect();
					c.sort_unstable();
					c.iter().collect()
				})
				.collect();

			d.sort_unstable();
			d.dedup();

			d.len() == p.len()
		})
		.collect::<Vec<Vec<String>>>()
		.len();


  println!("{:?}", new_res);
}
