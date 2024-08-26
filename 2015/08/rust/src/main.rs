use std::{fs::File, io::Read};

fn mem_count(s: &str) -> (i64, i64) {
    let code_len = s.len() as i64;

    let mut mem_len: i64 = 0;
    let mut chars = s.chars().peekable();
    
    while let Some(c) = chars.next() {
        match c {
            '"' => {}, // Ignore quotes in memory length
            '\\' => match chars.peek() {
                Some('\\') | Some('"') => {
                    mem_len += 1;
                    chars.next(); // Skip the next character
                },
                Some('x') => {
                    mem_len += 1;
                    chars.next(); // Skip 'x'
                    chars.next(); // Skip two hex digits
                    chars.next();
                },
                _ => mem_len += 1,
            },
            _ => mem_len += 1,
        }
    }

    (code_len, mem_len)
}

fn encode_count(s: &str) -> (i64, i64) {
    let code_len = s.len() as i64;

    let mut enc_len: i64 = 2; // Start with 2 for the added quotes
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '"' => {
                enc_len += 2; // " becomes \"
            },
            '\\' => {
                enc_len += 2; // \ becomes \\
            },
            _ => enc_len += 1,
        }
    }

    (code_len, enc_len)
}

fn read_input(p: &str) -> Vec<String> {
    let mut f = File::open(p).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    
    s.split_whitespace().map(|x| x.to_string()).collect::<Vec<String>>()
}

fn main() {
    let input = read_input("../input");

    let ans1: (i64, i64) = input.clone()
        .into_iter()
        .map(|x| mem_count(&x))
        .reduce(|acc, x| {
            (acc.0 + x.0, acc.1 + x.1)
        })
        .unwrap();

    println!("Problem 1: {:?}", ans1.0 - ans1.1);

    let ans2: (i64, i64) = input
        .into_iter()
        .map(|x| encode_count(&x))
        .reduce(|acc, x| {
            (acc.0 + x.0, acc.1 + x.1)
        })
        .unwrap();

    println!("Problem 2: {:?}", ans2.1 - ans2.0);
}

#[test]
fn test() {
    assert!(encode_count("\"\\x27\"").eq(&(6, 11)));
}
