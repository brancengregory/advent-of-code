use std::collections::{HashSet, VecDeque};

// (gen_floor, chip_floor)
type Pair = (u8, u8);

// (elevator_floor, [(gen_floor, chip_floor), ...])
type State<const N: usize> = (u8, [Pair; N]);

fn is_valid<const N: usize>((_, pairs): &State<N>) -> bool {
    pairs.iter().all(|&(gen_floor, chip_floor)| {
        if chip_floor == gen_floor {
            return true;
        }

        let gens_on_floor = pairs.iter()
            .any(|&(gen_floor, _)| gen_floor == chip_floor);
        
        !gens_on_floor
    })
}

fn next_states<const N: usize>((elevator, pairs): &State<N>) -> impl Iterator<Item = State<N>> {
    let mut moves: Vec<State<N>> = Vec::new();

    let mut on_floor: Vec<(usize, bool)> = Vec::new();
    for (i, &(gen_floor, chip_floor)) in pairs.iter().enumerate() {
        if gen_floor == *elevator { on_floor.push((i, true)); }
        if chip_floor == *elevator { on_floor.push((i, false)); }
    }

    let directions = [-1, 1];

    for dir in directions {
        let next_floor = *elevator as i8 + dir;

        if next_floor < 0 || next_floor > 3 { continue; }
        let next_floor = next_floor as u8;

        let mut try_add_move = |indices: &[(usize, bool)]| {
            let mut new_pairs = *pairs;

            for &(idx, is_gen) in indices {
                if is_gen {
                    new_pairs[idx].0 = next_floor;
                } else {
                    new_pairs[idx].1 = next_floor;
                }
            }

            new_pairs.sort_unstable();

            let new_state = (next_floor, new_pairs);

            if is_valid(&new_state) {
                moves.push(new_state);
            }
        };

        for &item in &on_floor {
            try_add_move(&[item]);
        }

        for i in 0..on_floor.len() {
            for j in (i+1)..on_floor.len() {
                try_add_move(&[on_floor[i], on_floor[j]]);
            }
        }
    }

    moves.into_iter()
}

fn is_goal<const N: usize>((_, pairs): &State<N>) -> bool {
    pairs.iter().all(|&(gen_floor, chip_floor)| gen_floor == 3 && chip_floor == 3)
}

fn solve<const N: usize>(init: State<N>) -> Result<usize, String> {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    queue.push_back((0, init));
    seen.insert(init);
    
    while let Some((depth, current_state)) = queue.pop_front() {
        if is_goal(&current_state) {
            return Ok(depth);
        }

        for next in next_states(&current_state) {
            if seen.insert(next) {
                queue.push_back((depth + 1, next));
            }
        }
    }

    return Err("Failed to solve".to_string())
}

fn main() {
    let init: State<5> = (0, [(0, 0), (0, 1), (0, 1), (2, 2), (2, 2)]);

    let ans = solve(init).unwrap();
    println!("{:#?}", ans);

    let init: State<7> = (0, [(0, 0), (0, 1), (0, 1), (2, 2), (2, 2), (0, 0), (0, 0)]);

    let ans = solve(init).unwrap();
    println!("{:#?}", ans);
}

