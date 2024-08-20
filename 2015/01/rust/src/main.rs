use std::any::Any;
use std::io::{BufReader, Read};
use std::fs::File;
use std::ops::Add;
use std::path::Path;

fn read_input<'a>(p: &str, s: &mut Vec<u8>) {
    let mut f = File::open(p).unwrap();
  
    f.read_to_end(s).unwrap();
}

fn main() {
    let mut s: Vec<u8> = Vec::new();

    read_input("../input", &mut s);

    let mut sum = 0;
    let mut basement = 0;

    for (index, e) in s.clone().into_iter().enumerate() {
      let b = match e {
        40 => 1,
        41 => -1,
        _ => 0
      };

      sum += b;

      if (basement == 0) & (sum < 0) { basement = index };
    };

    println!("Part One: {:?}", sum);
    println!("Part Two: {:?}", basement + 1);
}
