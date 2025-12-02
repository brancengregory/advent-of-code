fn read_input(p: &str) -> Vec<String> {
    std::fs::read_to_string(p).unwrap()
        .lines()
        .map(String::from)
        .collect()
}

type Screen = [[i32; 50]; 6];

fn follow_command(s: &mut Screen, c: &str) {
    let splits: Vec<&str> = c.split_whitespace().collect();
    let mut splits_iter = splits.into_iter();
    let command = splits_iter.next().unwrap();

    match command {
        "rect" => {
            let mut v = splits_iter.next().unwrap().split("x");
            let a = v.next().unwrap().parse::<i32>().unwrap();
            let b = v.next().unwrap().parse::<i32>().unwrap();

            follow_rect(s, a, b);
        },
        "rotate" => {
            let margin = splits_iter.next().unwrap();
            let a = splits_iter.next().unwrap().split("=").nth(1).unwrap().parse::<i32>().unwrap();
            let b = splits_iter.nth(1).unwrap().parse::<i32>().unwrap();
            
            match margin {
                "row" => {
                    follow_rotate_row(s, a, b);
                },
                "column" => {
                    follow_rotate_col(s, a, b);
                },
                _ => panic!("Unexpected rotate margin")
            }
        },
        _ => panic!("Unexpected command"),
    }
}

fn follow_rect(s: &mut Screen, a: i32, b: i32) {
    for r in 0..(b as usize) {
        for c in 0..(a as usize) {
            s[r][c] = 1;
        }
    }
}

fn follow_rotate_row(s: &mut Screen, a: i32, b: i32) {
    s[a as usize].rotate_right(b as usize);
}

fn follow_rotate_col(s: &mut Screen, a: i32, b: i32) {
    let mut c: Vec<i32> = s.iter().flat_map(|x| x.iter().nth(a as usize).cloned()).collect();
    c.rotate_right(b as usize);

    c.iter().enumerate()
        .for_each(|(i, &x)| {
            s[i][a as usize] = x;
        })
}

fn main() {
    let input = read_input("../input");

    let mut screen = [[0; 50]; 6];

    input.iter()
        .for_each(|x| follow_command(&mut screen, x));

    let n_lit: i32 = screen.iter()
        .fold(
            0,
            |acc, x| {
                acc + x.iter().sum::<i32>()
            }
        ); 

    println!("{:?}", n_lit);

    for r in screen {
        println!("{:?}", r)
    }
}
