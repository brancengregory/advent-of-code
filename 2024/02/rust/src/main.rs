use std::fs::read_to_string;


fn is_safe(r: &[i32]) -> bool {
    let mode = (r[1] - r[0]).signum();

    if mode == 0 {return false};

    r.windows(2).all(|pair| {
        let d = pair[1] - pair[0];
        d.signum() == mode && (1..=3).contains(&d.abs())
    })
}

fn is_almost_safe(r: &[i32]) -> bool {
    if is_safe(r) { return true };

    for i in 0..r.len() {
        let mut n = r.to_owned();
        n.remove(i);
        
        if is_safe(&n) { return true }
    }

    false
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string("../input")?;
    
    let reports: Vec<Vec<i32>> = input.lines()
        .map(|x| 
            x.split_whitespace()
                .map(|v| v.parse::<i32>().expect("Couldn't parse to i32"))
                .collect::<Vec<i32>>()
        )
        .collect();
    
    let ans = reports.iter()
        .filter(|r| is_safe(r))
        .count();
    
    println!("Part 1: {:?}", ans);
    
    let ans = reports.iter()
        .filter(|r| is_almost_safe(r))
        .count();

    println!("Part 2: {:?}", ans);

    Ok(())
}
