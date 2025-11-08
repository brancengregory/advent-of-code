use std::fs::read_to_string;

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
    let s = read_to_string(p).unwrap();

    let x = s
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| x.split('x').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    x
}

fn main() {
    let input = read_input("../input");

    let ans1: u32 = input
        .iter()
        .map(|x| {
            surface_area(x[0], x[1], x[2]) + smallest_face_area(x[0], x[1], x[2])
        })
        .sum();

    println!("Answer 1: {}", ans1);

    let ans2: u32 = input
        .iter()
        .map(|x| {
            smallest_face_perimeter(x[0], x[1], x[2]) + volume(x[0], x[1], x[2])
        })
        .sum();

    println!("Answer 2: {}", ans2);
}
