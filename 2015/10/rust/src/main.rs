use std::{fs::File, io::{self, Read, Result}, str::Chars};

fn read_input(p: &str) -> Result<Vec<String>> {
    let mut f: File = File::open(p)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    Ok(s.lines().map(|x| x.to_string()).collect())
}

fn look_and_say(mut x: String, n: usize) -> usize {
    for _ in 0..n {
        let mut new_x = String::new();
        let mut chars = x.chars().peekable();

        while let Some(ch) = chars.next() {
            let mut count = 1;
    
            while let Some(&next) = chars.peek() {
                if ch == next {
                    count += 1;
                    chars.next();
                } else {
                    break;
                }
            }
    
            new_x.push_str(&count.to_string());
            new_x.push(ch);
    
        }

        x = new_x;
    }

    return x.len()
}

fn main() -> Result<()> {
    let input = read_input("../input")?;

    let ans1 = look_and_say(input[0].clone(), 40);
    let ans2 = look_and_say(input[0].clone(), 50);

    println!("Input: {:?}", input);
    println!("Answer 1: {:?}", ans1);
    println!("Answer 2: {:?}", ans2);

    Ok(())
}
