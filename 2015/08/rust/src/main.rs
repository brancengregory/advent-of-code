use std::{fs::File, io::Read};

fn evaluate(s: &str) -> (i64, i64) {
    let sv: Vec<&str> = s.split("").filter(|x| !x.is_empty()).collect();
    let code_len = sv.len(); 
    println!("Length: {:?}\nValue: {:?}", sv.len(), sv);

    let mem_len: i64 = 0;
    for (i, x) in sv.into_iter().enumerate() {

    }
    (code_len as i64, 2)
}

fn read_input(p: &str) -> Vec<String> {
    let mut f = File::open(p).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    
    s.split_whitespace().map(|x| x.to_string()).collect::<Vec<String>>()
}

fn main() {
    let input = read_input("../input");
    println!("Input: {:?}", input);

    let ans1 = evaluate(input[0].as_str());
    println!("Problem 1: {:?}", ans1);
}
