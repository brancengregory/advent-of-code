use std::{fs::File, io::Read, vec, collections::HashMap};

fn is_vowel(c: &char) -> bool {
    let vowels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];
    
    let v: bool = vowels
        .into_iter()
        .map(|x| x == *c)
        .reduce(|acc, e| acc || e)
        .unwrap();

    return v
}

fn has_three_vowels(s: &String) -> bool {
    let num_vowels = s.chars().filter(is_vowel).count();
    if num_vowels >= 3 { return true }
    return false
}

fn has_double_letter(s: &String) -> bool {
    let mut last_character: char = Default::default();

    for x in s.chars() {
        if x == last_character {
            return true
        }

        last_character = x;
    }

    return false
}

fn has_double_letter_anywhere(s: &String) -> bool {
    let mut pairs_seen: HashMap<(char, char), usize> = HashMap::new();
    
    for (i, window) in s.chars().collect::<Vec<_>>().windows(2).enumerate() {
        let pair = (window[0], window[1]);

        if let Some(&last_index) = pairs_seen.get(&pair) {
            if i > last_index + 1 {
                return true; // Found a non-overlapping pair
            }
        }

        pairs_seen.insert(pair, i);
    }

    return false
}

fn has_double_letter_skip(s: &String) -> bool {
    let c: Vec<char> = s.chars().collect();
    let b: Vec<bool> = c.windows(3).map(|x| x[0] == x[2]).collect();
    
    return b.into_iter().reduce(|acc, e| acc || e).unwrap()
}

fn has_no_bad_strings(s: &String) -> bool {
    let bad_strings = vec!["ab", "cd", "pq", "xy"];
    
    let bs: bool = bad_strings
        .into_iter()
        .map(|x| s.contains(x))
        .reduce(|acc, e| acc || e)
        .unwrap();

    return !bs
}

fn is_nice(s: &String) -> bool {
    has_no_bad_strings(s) &&
        has_double_letter(s) &&
        has_three_vowels(s)
}

fn is_new_nice(s: &String) -> bool {
    has_double_letter_anywhere(s) &&
        has_double_letter_skip(s)
}

fn read_input(p: &str) -> Vec<String> {
    let mut f = File::open(p).unwrap();
    let mut s = String::new();

    f.read_to_string(&mut s).unwrap();

    let x = s
        .trim()
        .split("\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    return x
}

fn main() {
    let input = read_input("../input");

    let ans1 = input.clone()
        .into_iter()
        .filter(is_nice)
        .count();

    println!("Problem 1: {:?}", ans1);

    let ans2 = input
        .into_iter()
        .filter(is_new_nice)
        .count();

    println!("Problem 2: {:?}", ans2)
}

#[test]
fn test_has_double_letter_anywhere() {
    let s1 = &String::from("xyxy");
    let s2 = &String::from("aabcdefgaa");
    let s3 = &String::from("aaa");

    let ts: Vec<&String> = vec![s1, s2, s3];
    let b: Vec<bool> = ts.into_iter().map(has_double_letter_anywhere).collect();

    assert_eq!(
        b,
        vec![true, true, false]
    )
}
