use std::{cmp::min, fs::File, io::{Error, Read}, mem::zeroed, u32};
use regex::Regex;

fn read_input(p: &str) -> Result<Vec<String>, Error> {
    let mut f = File::open(p)?;
    let mut s: String = String::new();
    f.read_to_string(&mut s)?;

    Ok(s.lines().map(|x| x.to_string()).collect())
}

#[derive(Debug)]
struct Reindeer {
    name: String,
    flight_speed: u32,
    flight_time: u32,
    rest_time: u32 
}

#[derive(Debug)]
struct RaceResult {
    distance: u32,
    points: u32
}

fn find_max_and_indices(vec: &Vec<u32>) -> (u32, Vec<usize>) {
    let mut max_value = u32::MIN;
    let mut indices = Vec::new();

    for (i, &value) in vec.iter().enumerate() {
        if value > max_value {
            max_value = value;
            indices.clear();
            indices.push(i);
        } else if value == max_value {
            indices.push(i);
        }
    }

    (max_value, indices)
}

fn race(reindeer: &Vec<Reindeer>, s: u32) -> (u32, Vec<usize>) {
    let mut distances: Vec<u32> = vec![0; reindeer.len()];

    for (i, r) in reindeer.iter().enumerate() {
        let mut d = r.flight_speed * r.flight_time * (s / (r.flight_time + r.rest_time));
        let remainder = s % (r.flight_time + r.rest_time);
        if remainder != 0 {
            if remainder >= r.flight_time {
                d += r.flight_speed * r.flight_time
            } else {
                d += r.flight_speed * remainder
            }
        }

        distances[i] = d
    }

    find_max_and_indices(&distances)
}

fn stepwise_race(reindeer: &Vec<Reindeer>, s: u32) -> (u32, Vec<usize>) {
    let mut distances: Vec<u32> = vec![0; reindeer.len()];
    let mut scoreboard: Vec<u32> = vec![0; reindeer.len()];
    
    for t in 1..s {
        for (i, r) in reindeer.iter().enumerate() {
            let mut d = r.flight_speed * r.flight_time * (t / (r.flight_time + r.rest_time));
            let remainder = t % (r.flight_time + r.rest_time);
            if remainder != 0 {
                if remainder >= r.flight_time {
                    d += r.flight_speed * r.flight_time
                } else {
                    d += r.flight_speed * remainder
                }
            }
            distances[i] = d
        }
        let (_, inds) = find_max_and_indices(&distances);
        inds.into_iter().for_each(|i| scoreboard[i] += 1);
    }

    find_max_and_indices(&scoreboard)
}

fn main() -> Result<(), Error> {
    let input = read_input("../input")?;
    let mut reindeer: Vec<Reindeer> = Vec::new();
    let re = Regex::new("(\\w+) can fly (\\d+) km/s for (\\d+) seconds, but then must rest for (\\d+) seconds.").unwrap();

    for s in input {
        if let Some(captures) = re.captures(&s) {
            let r = Reindeer {
                name: captures[1].to_string(),
                flight_speed: captures[2].parse::<u32>().unwrap(),
                flight_time: captures[3].parse::<u32>().unwrap(),
                rest_time: captures[4].parse::<u32>().unwrap()
            };
            reindeer.push(r);
        }
    }

    let res = race(&reindeer, 2503);
    println!("Problem 1: {:?}", res);

    let res2 = stepwise_race(&reindeer, 2503);
    println!("Problem 2: {:?}", res2);

    Ok(())
}
