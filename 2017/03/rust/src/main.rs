use std::collections::HashMap;

enum Dir {
	Left,
	Right,
	Up,
	Down
}

fn turn(d: &Dir) -> Dir {
	match d {
		Dir::Right => Dir::Up,
		Dir::Up => Dir::Left,
		Dir::Left => Dir::Down,
		Dir::Down => Dir::Right
	}
}

fn step(mut c: (i32, i32), d: &Dir) -> (i32, i32) {
	match d {
		Dir::Left => c.0 -= 1,
		Dir::Right => c.0 += 1,
		Dir::Up => c.1 += 1,
		Dir::Down => c.1 -= 1
	}

	c
}

fn sum_neighbors(hm: &HashMap<(i32, i32), i32>, c: &(i32, i32)) -> i32 {
	let neighbors: Vec<(i32, i32)> = [
		(c.0 + 1, c.1),
		(c.0 + 1, c.1 + 1),
		(c.0, c.1 + 1),
		(c.0 - 1, c.1 + 1),
		(c.0 - 1, c.1),
		(c.0 - 1, c.1 - 1),
		(c.0, c.1 - 1),
		(c.0 + 1, c.1 - 1)
	].into();

	neighbors.iter().filter_map(|n| hm.get(n)).sum()
}

fn main() {
	let val = 312051;
	let mut ring: i32 = 0;
	let mut odd: i32 = 1;
	let mut number: i32 = odd.pow(2);

	while number < val {
		ring += 1;
		odd += 2;
		number = odd.pow(2);
	}

	let mut coords: (i32, i32) = (ring, ring);
	let side_length = ring * 2;

	let diff = number - val;

	if !(diff < side_length) {
		panic!("diff ({}) not less than side length ({})", diff, side_length)
	};

	coords.1 -= diff;
	println!("distance: {:?}", coords.0.abs() + coords.1.abs());

	let mut grid: HashMap<(i32, i32), i32> = HashMap::new();
	let mut coords: (i32, i32) = (0, 0);
	let mut num: i32 = 1;

	grid.insert(coords, num);

	let mut dir = Dir::Right;

	while num < val {
		// Move forward
		coords = step(coords, &dir);

		// Change directions if needed
		if (!matches!(dir, Dir::Right) && (coords.0.abs() == coords.1.abs())) || (matches!(dir, Dir::Right) && coords.1 == -coords.0 + 1) {
			dir = turn(&dir);
		}

		// Get neighbors values and sum
		num = sum_neighbors(&grid, &coords);

		// Insert to HashMap
		grid.insert(coords, num);
	}

	println!("{:?}", num);
}

