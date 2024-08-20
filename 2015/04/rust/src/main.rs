use std::{fmt::format, fs::File, io::Read};
use md5::{compute, Digest};

fn read_input(p: &str) -> String {
    let mut f = File::open(p).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s);
    
    return s.trim().to_owned();
}

fn main() {
    let input = read_input("../input").trim().to_owned();
    
    let mut i = 0;
    let mut s = String::new();
    let mut h: Digest;
    let mut done: bool = false;
    let mut hex_string = String::new();

    while !done {
        i += 1;
        s = input.clone() + &i.to_string();
        h = compute(&s);
        hex_string = format!("{:x}", h);
        done = hex_string.starts_with("00000");
    }

    println!("Answer 1: {:?}", i);

    i = 0;
    done = false;

    while !done {
        i += 1;
        s = input.clone() + &i.to_string();
        h = compute(&s);
        hex_string = format!("{:x}", h);
        done = hex_string.starts_with("000000");
    }

    println!("Answer 2: {:?}", i)
}
