fn read_input(p: &str) -> Vec<String> {
	std::fs::read_to_string(p).unwrap()
		.lines()
		.map(String::from)
		.collect()
}

fn get_neighbors(w: i32, x: i32, y: i32) -> Vec<i32> {
	let neighbors = [
		(x-1, y+1),
		(x, y+1),
		(x+1, y+1),
		(x-1, y),
		(x+1, y),
		(x-1, y-1),
		(x, y-1),
		(x+1, y-1),
	];

	neighbors.into_iter()
		.filter(|x| {
			x.0 >= 0 && x.1 >= 0 && x.0 < w && x.1 < w
		})
		.map(|(x, y)| {
			coords_to_i(w, x, y)
		})
		.collect()
}

fn coords_to_i(n_col: i32, x: i32, y: i32) -> i32 {
	(n_col) * y + x
}

fn i_to_coords(n_col: i32, i: i32) -> (i32, i32) {
	(i % n_col, i / n_col)
}

fn check_neighbors(g: &Vec<char>, w: i32, i: i32) -> i32 {
	let mut s = 0;

	let coords = i_to_coords(w, i);

	let ns = get_neighbors(w, coords.0, coords.1);

	for n in ns {
		if g[n as usize] == '@' {
			s += 1
		}
	}

	s
}

fn main() {
	let input = read_input("../input");
	let w = input[0].len() as i32;

	let mut grid = Vec::new();
	for i in input {
		for j in i.chars() {
			if j == '.' || j == '@' {
				grid.push(j)
			}
		}
	}

	let mut ans = 0;

	for (i, g) in grid.iter().enumerate() {
		if *g == '@' {
			let n = check_neighbors(&grid, w, i as i32);
			if n < 4 {
				ans += 1;
			}
		}
	}

  println!("{:#?}", ans);
}
