type Discs<const N: usize> = [usize; N];

fn disc_position<const N: usize>(i: usize, t: usize, mods: &Discs<N>, positions: &Discs<N>) -> usize {
    (positions[i] + t) % mods[i]
}

fn get_positions<const N: usize>(drop: usize, mods: &Discs<N>, positions: &Discs<N>) -> [usize; N] {
    let mut res = [0; N];
    
    for i in 0..N {
        res[i] = disc_position(i, drop + (i + 1), mods, positions);
    }

    res
}

fn main() {
    let mods: Discs<6> = [7, 13, 3, 5, 17, 19];
    let positions: Discs<6> = [0, 0, 2, 2, 0, 7];

    let target = [0, 0, 0, 0, 0, 0];
    let mut i: usize = 0;

    loop {
        let current_positions = get_positions(i, &mods, &positions);

        if current_positions == target {
            break
        };

        i += 1;
    }

    println!("{:?}", i);

    let mods: Discs<7> = [7, 13, 3, 5, 17, 19, 11];
    let positions: Discs<7> = [0, 0, 2, 2, 0, 7, 0];

    let target = [0, 0, 0, 0, 0, 0, 0];
    let mut i: usize = 0;

    loop {
        let current_positions = get_positions(i, &mods, &positions);

        if current_positions == target {
            break
        };

        i += 1;
    }

    println!("{:?}", i);
}
