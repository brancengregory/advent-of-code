use std::collections::HashMap;

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

fn main() {
    let input = read_input("../input");

    let ans = get_last_key_idx(&input);

    println!("{:#?}", ans);
}
