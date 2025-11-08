use std::{fs::read_to_string, vec, collections::HashMap};

fn is_vowel(c: &char) -> bool {
    match c {
      'a' | 'e' | 'i' | 'o' | 'u' => true,
      _ => false
    }
}

fn has_three_vowels(s: &str) -> bool {
    let num_vowels = s.chars().filter(is_vowel).count();
    num_vowels >= 3
}

fn has_double_letter(s: &str) -> bool {
    s.chars()
        .zip(s.chars().skip(1))
        .any(|(a, b)| a == b)
}

fn has_double_letter_anywhere(s: &str) -> bool {
    let mut pairs_seen: HashMap<(char, char), usize> = HashMap::new();
    let s_chars: Vec<char> = s.chars().collect();
    
    for (i, window) in s_chars.windows(2).enumerate() {
        let pair = (window[0], window[1]);

        if let Some(&last_index) = pairs_seen.get(&pair) {
            if i > last_index + 1 {
                return true; // Found a non-overlapping pair
            }
        } else {
            pairs_seen.insert(pair, i);
        }
    }

    false
}

fn has_double_letter_skip(s: &str) -> bool {
    let c: Vec<char> = s.chars().collect();
    c.windows(3).any(|x| x[0] == x[2])
}

fn has_no_bad_strings(s: &str) -> bool {
    let bad_strings = ["ab", "cd", "pq", "xy"];
    
    !bad_strings
        .iter()
        .any(|&x| s.contains(x))
}

fn is_nice(s: &str) -> bool {
    has_no_bad_strings(s) &&
        has_double_letter(s) &&
        has_three_vowels(s)
}

fn is_new_nice(s: &str) -> bool {
    has_double_letter_anywhere(s) &&
        has_double_letter_skip(s)
}

fn read_input(p: &str) -> Vec<String> {
    let s = read_to_string(p).unwrap();

    s
        .lines()
        .map(String::from)
        .collect()
}

fn main() {
    let input = read_input("../input");

    let ans1 = input
        .iter()
        .filter(|s| is_nice(s))
        .count();

    println!("Problem 1: {}", ans1);

    let ans2 = input
        .iter()
        .filter(|s| is_new_nice(s))
        .count();

    println!("Problem 2: {}", ans2)
}

#[test]
fn test_has_double_letter_anywhere() {
    let s1 = "xyxy";
    let s2 = "aabcdefgaa";
    let s3 = "aaa";
    let s4 = "aaaa";

    assert_eq!(has_double_letter_anywhere(s1), true);
    assert_eq!(has_double_letter_anywhere(s2), true);
    assert_eq!(has_double_letter_anywhere(s3), false);
    assert_eq!(has_double_letter_anywhere(s4), true);
}
