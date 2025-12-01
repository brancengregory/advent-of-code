const N_TICKS: i32 = 100;

fn read_input(p: &str) -> Vec<String> {
    std::fs::read_to_string(p).unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn watch_dial(d: &mut i32, p1: &mut i32, p2: &mut i32, s: &str) {
    let start_d = *d; 
    let mut chars = s.chars();
    let direction = chars.next().unwrap();
    let distance = chars.collect::<String>().parse::<i32>().unwrap();

    let calc_start_d = if start_d == 0 && direction == 'L' {
        N_TICKS
    } else {
        start_d
    };

    let raw_distance = if direction == 'L' { -distance } else { distance };

    let raw_d = calc_start_d + raw_distance;

    let new_d = raw_d.rem_euclid(N_TICKS); 

    if new_d == 0 {
        *p1 += 1
    }
    
    let mut zero_crossings = raw_d.div_euclid(N_TICKS).abs(); 

    if raw_d <= 0 && new_d == 0 {
        zero_crossings += 1;
    }

    *p2 += zero_crossings;

    *d = new_d;
}

fn main() {
    let input = read_input("../input");

    let mut password1 = 0;
    let mut password2 = 0;
    let mut dial = 50;
    
    input.iter()
        .for_each(|x| {
           watch_dial(&mut dial, &mut password1, &mut password2, x) 
        });

    println!("password 1: {}\ndial: {}", password1, dial);
    println!("password 2: {}\ndial: {}", password2, dial);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem() {
        let input = vec![
            "L68",
            "L30",
            "R48",
            "L5",
            "R60",
            "L55",
            "L1",
            "L99",
            "R14",
            "L82",
        ];
        let mut password1 = 0;
        let mut password2 = 0;
        let mut dial = 50;

        input.iter()
            .for_each(|x| {
                watch_dial(&mut dial, &mut password1, &mut password2, x);
                println!("{} {}", dial, password2);
            }); 
        
        assert_eq!(password1, 3);
        assert_eq!(password2, 6);
    }
}

