use std::{fs::File, io::{self, Error, Read}};

fn read_input(p: &str) -> Result<Vec<String>, Error> {
    let mut f = File::open(p)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    Ok(s.lines().map(|x| x.to_string()).collect())
}

fn is_valid(s: &str) -> bool {
    let bad_chars = ['i', 'o', 'l'];
    let mut num_pairs = 0;
    let mut straight_found = false;
    let mut prev_pair_char = None;
    let chars: Vec<char> = s.chars().collect();

    for i in 0..chars.len() {
        let ch = chars[i];

        // Check for bad characters
        if bad_chars.contains(&ch) {
            return false;
        }

        // Check for pairs
        if i < chars.len() - 1 && chars[i + 1] == ch && prev_pair_char != Some(ch) {
            num_pairs += 1;
            prev_pair_char = Some(ch);
        }

        // Check for a straight of three consecutive characters
        if i < chars.len() - 2 && chars[i] as u8 + 1 == chars[i + 1] as u8 && chars[i + 1] as u8 + 1 == chars[i + 2] as u8 {
            straight_found = true;
        }
    }

    // The string is valid if there is at least one straight and two pairs
    straight_found && num_pairs >= 2
}

fn new_password(p: &str) -> String {
    let mut chars: Vec<char> = p.chars().collect();
    let mut i = chars.len();

    while i > 0 {
        i -= 1;

        if chars[i] == 'z' {
            chars[i] = 'a';
        } else {
            chars[i] = (chars[i] as u8 + 1) as char;
            break;
        }
    }

    chars.iter().collect()
}

fn main() -> io::Result<()> {
    let input = read_input("../input")?;
    println!("Input: {:?}", input);

    let mut password = input[0].clone();
    let mut valid = false;

    while !valid {
        password = new_password(&password);
        valid = is_valid(&password);
    }

    println!("Valid password: {:?}", password);

    password = new_password(&password);
    valid = is_valid(&password);

    while !valid {
        password = new_password(&password);
        valid = is_valid(&password);
    }

    println!("Valid password: {:?}", password);

    Ok(())
}
