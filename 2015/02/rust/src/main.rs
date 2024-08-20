use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn surface_area(l: u32, w: u32, h: u32) -> u32 {
    (2*l*w) + (2*w*h) + (2*h*l)
}

fn smallest_face_area(l: u32, w: u32, h: u32) -> u32 {
    let mut a = [l, w, h];
    a.sort();
    
    a[0] * a[1]
}

fn smallest_face_perimeter(l: u32, w: u32, h: u32) -> u32 {
    let mut a = [l, w, h];
    a.sort();
    
    (2 * a[0]) + (2 * a[1])
}

fn volume(l: u32, w: u32, h: u32) -> u32 {
    l * w * h
}

fn read_input(p: &str) -> Vec<Vec<u32>> {
    let f = File::open(p).unwrap();
    let mut r = BufReader::new(f);
    
    let mut s = String::new();
    let _ = r.read_to_string(&mut s);

    let x = s
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.split("x").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    return x
}

fn main() {
    let input = read_input("../input");

    let ans1: u32 = input
        .clone()
        .into_iter()
        .map(|x| {
            surface_area(x[0], x[1], x[2]) + smallest_face_area(x[0], x[1], x[2])
        })
        .reduce(|acc, e| acc + e)
        .unwrap();

    println!("Answer 1: {:?}", ans1);

    let ans2: u32 = input
        .into_iter()
        .map(|x| {
            smallest_face_perimeter(x[0], x[1], x[2]) + volume(x[0], x[1], x[2])
        })
        .reduce(|acc, e| acc + e)
        .unwrap();

    println!("Answer 2: {:?}", ans2);
}
