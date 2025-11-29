use std::fs::read_to_string;

fn read_input(p: &str) -> Vec<Vec<u32>> {
    read_to_string(p).unwrap()
        .lines()
        .map(|l| {
            l.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect()
        })
        .collect()
}

fn main() {
    let input = read_input("../input");

    let triangles: Vec<Vec<u32>> = input.clone()
        .into_iter()
        .filter(|x| {
            (x[0] + x[1] > x[2]) && (x[1] + x[2] > x[0]) && (x[0] + x[2] > x[1])
        })
        .collect();

    println!("{:?}", triangles.len());

    let mut vc: Vec<Vec<u32>> = vec![vec![0; input.len()]; 3];

    for i in (0..input.len()).step_by(3) {
        let c1 = [input[i][0], input[i+1][0], input[i+2][0]];
        let c2 = [input[i][1], input[i+1][1], input[i+2][1]];
        let c3 = [input[i][2], input[i+1][2], input[i+2][2]];
        vc.push(c1.to_vec());
        vc.push(c2.to_vec());
        vc.push(c3.to_vec());
    }

    let triangles: Vec<Vec<u32>> = vc
        .into_iter()
        .filter(|x| {
            (x[0] + x[1] > x[2]) && (x[1] + x[2] > x[0]) && (x[0] + x[2] > x[1])
        })
        .collect();

    println!("{:?}", triangles.len());
}
