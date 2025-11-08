use std::fs::read_to_string;
use std::fmt::Write;
use md5::compute;


fn find_hash(input: &str, prefix: &str) -> u32 {
    let mut i = 0;
    let mut s = input.to_string();
    let base_len = s.len();
    let mut hex_string = String::new();

    loop {
        i += 1;
        write!(&mut s, "{}", i).unwrap();
        let h = compute(s.as_bytes());
        write!(&mut hex_string, "{:x}", h).unwrap();
        if hex_string.starts_with(prefix) {
            break;
        };
        s.truncate(base_len);
        hex_string.clear();
    }

    i
}

fn main() {
    let input = read_to_string("../input").unwrap().trim().to_string();
   
    let ans1 = find_hash(&input, "00000");
    println!("Answer 1: {}", ans1);

    let ans2 = find_hash(&input, "000000");
    println!("Answer 2: {}", ans2);
}
