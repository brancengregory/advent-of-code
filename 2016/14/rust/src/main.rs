use std::collections::{HashMap, VecDeque};

fn read_input(p: &str) -> String {
    std::fs::read_to_string(p).unwrap().trim().to_string()
}

fn hash(s: &str, i: u32) -> String {
    let salt = format!("{}{}", s, i);
    let digest = md5::compute(salt);
    format!("{:x}", digest)
}

fn get_hash(hm: &mut HashMap<u32, String>, s: &str, i: u32) -> String {
    hm.entry(i)
        .or_insert_with(|| hash(s, i))
        .to_owned()
}

fn get_triple(s: &str) -> Option<char> {
    s.as_bytes()
        .windows(3)
        .find(|x| {
            x[0] == x[1] && x[1] == x[2]
        })
        .map(|w| w[0] as char)
}

fn has_quintuple(s: &str, c: char) -> bool {
    let pattern = c.to_string().repeat(5);
    s.contains(&pattern)
}

fn get_last_key_idx(input: &str) -> u32 {
    let mut visited: HashMap<u32, String> = HashMap::new();
    let mut keys: Vec<u32> = Vec::new();
    let mut i: u32 = 0;

    loop {
        if keys.len() == 64 { break; }

        let current_hash = get_hash(&mut visited, &input, i);
        let triple_char_opt = get_triple(&current_hash);

        if let Some(c) = triple_char_opt {
            let has_peer = ((i + 1)..=(i + 1000))
                .any(|x| {
                    let hash_str = get_hash(&mut visited, &input, x);
                    has_quintuple(&hash_str, c)
                });

            if has_peer {
                keys.push(i)
            }
        }

        i += 1;
    }

    keys.last().unwrap().to_owned()
}

fn stretch_hash(s: &str, i: u32) -> String {
		let salt = format!("{}{}", s, i);
		let digest = md5::compute(salt);
		let mut hash = format!("{:x}", digest);

		for _ in 0..2016 {
				let d = md5::compute(hash);
				hash = format!("{:x}", d);
		}

		hash
}

fn get_last_stretch_key_idx(input: &str) -> u32 {
    let mut keys: Vec<u32> = Vec::new();
    let mut pending: VecDeque<(char, u32)> = VecDeque::new();
    let mut i: u32 = 0;

    loop {
        let current_hash = stretch_hash(input, i);

        let mut satisfied_indices = Vec::new();

        // Check pending against hash
        for (pending_idx, (c, source_idx)) in pending.iter().enumerate() {
            if has_quintuple(&current_hash, *c) {
                if i <= source_idx + 1000 {
                    keys.push(*source_idx);
                    satisfied_indices.push(pending_idx);
                }
            }
        }

        // Remove satisfied
        satisfied_indices.sort_unstable_by(|a, b| b.cmp(a));
        for idx in satisfied_indices {
            pending.remove(idx);
        }

        // Check if hash creates new pending
        if let Some(c) = get_triple(&current_hash) {
            pending.push_back((c, i));
        }

        // Cleanup expired candidates
        while let Some((_, source_idx)) = pending.front() {
            if *source_idx + 1000 < i {
                pending.pop_front();
            } else {
                break;
            }
        }

        if keys.len() >= 64 {
            keys.sort_unstable();
            let key_64 = keys[63];

            let oldest_pending_idx = pending.front().map(|x| x.1).unwrap_or(u32::MAX);

            if oldest_pending_idx > key_64 {
                return key_64;
            }
        }
        
        i += 1;
    }
}

fn main() {
    let input = read_input("../input");

    let ans = get_last_key_idx(&input);
    println!("{:#?}", ans);

		let ans2 = get_last_stretch_key_idx(&input);
    println!("{:#?}", ans2);
}

