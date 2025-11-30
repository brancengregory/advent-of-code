use std::fs::read_to_string;
use regex::{Regex};
use std::collections::HashMap;

fn read_input(p: &str) -> Vec<(String, u32, String)> {
    let content: Vec<String> = read_to_string(p).unwrap()
        .lines()
        .map(String::from)
        .collect();

    let re = Regex::new(r"^(.*)-(.*)\[(.*)\]$").unwrap();

    content.into_iter()
        .map(|x| {
            let m = re.captures(&x).unwrap();
            (
                m[1].split("-").collect::<String>(),
                m[2].parse::<u32>().unwrap(),
                m[3].to_string()
            )
        })
        .collect()
}

fn count_chars(s: &str) -> HashMap<char, u32> {
    let mut hm: HashMap<char, u32> = HashMap::new();

    s.chars().for_each(|c| {
        hm
            .entry(c)
            .and_modify(|n| *n += 1)
            .or_insert(1);
    });

    hm
}

fn get_code(hm: &HashMap<char, u32>) -> String {
    let mut v: Vec<(&char, &u32)> = hm.iter().collect();
    v.sort_by(|a, b| {
        b.1.cmp(a.1)
            .then(a.0.cmp(b.0)) 
    });

    v.into_iter()
        .take(5)
        .map(|(c, _)| *c)
        .collect()
}


fn rotate_char(c: char, n: u32) -> char {
    let first_letter = 'a' as u32;
    let last_letter = 'z' as u32;

    let range_size = last_letter - first_letter + 1;

    let c_u32 = c as u32;
    let wrapped_pos = (c_u32 - first_letter + n) % range_size;
    std::char::from_u32(first_letter + wrapped_pos).unwrap()
}

fn rotate_string(s: &str, n: u32) -> String {
    s.chars()
        .map(|c| {
           rotate_char(c, n) 
        })
        .collect()
}

fn main() {
    let input = read_input("../input");

    let res: Vec<HashMap<char, u32>> = input.iter().map(|v| count_chars(&v.0)).collect();

    let real_rooms = input.iter().zip(res)
        .filter(|(x, y)| {
            let c = get_code(y);
            c == x.2
        });

    let ans = real_rooms
        .clone()
        .fold(
            0,
            |acc, (x, _)| {
                acc + x.1
            }
        );

    println!("{:?}", ans);

    let real_name_id: Vec<u32> = real_rooms
        .filter_map(|(x, _)| {
            let c = rotate_string(&x.0, x.1);
            if c.contains("north") {
                Some(x.1) 
            } else {
                None
            }
        })
        .collect::<Vec<u32>>();
    
    println!("{:?}", real_name_id)
}
