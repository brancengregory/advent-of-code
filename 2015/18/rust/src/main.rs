use std::fs::read_to_string;
use std::io::Result;

const NROWS: usize = 100;
const NCOLS: usize = 100;


fn read_input(p: &str) -> Result<Vec<String>> {
    let content = read_to_string(p)?;
    Ok(content.lines().map(|s| s.to_string()).collect())
}

type Grid = [[i8; NCOLS]; NROWS];

fn init_grid(input: &Vec<String>) -> Grid {
    let mut grid = [[0; NCOLS]; NROWS];
    for (row, line) in input.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            grid[row][col] = match c {
                '#' => 1,
                '.' => 0,
                _ => panic!("Unexpected character found: {}", c)
            }
        }
    }

    // Assert that corners are "on" for Part 2
    assert!(
        grid[0][0] == 1 && grid[0][NCOLS - 1] == 1 && grid[NROWS - 1][0] == 1 && grid[NROWS - 1][NCOLS - 1] == 1,
        "Corners must be set to 'on' (#) in the input for Part 2!"
    );
    
    grid
}

fn init_grid_locked(input: &Vec<String>) -> Grid {
    let mut grid = [[0; NCOLS]; NROWS];
    for (row, line) in input.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            grid[row][col] = match c {
                '#' => 1,
                '.' => 0,
                _ => panic!("Unexpected character found: {}", c)
            }
        }
    }
    
    grid

}

fn get_neighbor_coords(p: (i8, i8)) -> Vec<(i8, i8)> {
    let offsets: [(i8, i8); 8] = [
        (-1, -1), (-1,  0), (-1,  1),
        ( 0,  1),           ( 1,  1),
        ( 1,  0), ( 1, -1), ( 0, -1)
    ];

    offsets
        .iter()
        .map(|x| {
            (x.0 + p.0, x.1 + p.1)
        })
        .filter(|x| (x.0 >= 0) && (x.0 < NROWS as i8) && (x.1 >= 0) && (x.1 < NCOLS as i8))
        .collect::<Vec<(i8, i8)>>()
}

fn get_neighbor_vals(p: (i8, i8), g: &Grid) -> Vec<i8> {
    let neighbor_coords = get_neighbor_coords((p.0, p.1));
    
    neighbor_coords
        .into_iter()
        .map(|x| g[x.0 as usize][x.1 as usize])
        .collect::<Vec<i8>>()
}

fn update_grid(g: &mut Grid) {
    let mut ug = [[0; NCOLS]; NROWS];

    for i in 0..NROWS {
        for j in 0..NCOLS {
            let init_val = g[i][j];
            let n: i8 = get_neighbor_vals((i as i8, j as i8), g)
                .iter()
                .sum();
            
            ug[i][j] = match init_val {
                0 => if n == 3 { 1 } else { 0 },
                1 => if n == 2 || n == 3 { 1 } else { 0 },
                _ => panic!("Unexpected value found: {}", init_val)
            }
        }
    }
    
    *g = ug;
}

fn update_grid_locked(g: &mut Grid) {
    let mut ug = [[0; NCOLS]; NROWS];

    for i in 0..NROWS {
        for j in 0..NCOLS {
            let init_val = g[i][j];
            let n: i8 = get_neighbor_vals((i as i8, j as i8), g)
                .into_iter()
                .sum();
            
            ug[i][j] = match init_val {
                0 => if n == 3 { 1 } else { 0 },
                1 => if n == 2 || n == 3 { 1 } else { 0 },
                _ => panic!("Unexpected value found: {}", init_val)
            }
        }
    }

    // Lock corners "on"
    ug[0][0] = 1;
    ug[0][NCOLS - 1] = 1;
    ug[NROWS - 1][0] = 1;
    ug[NROWS - 1][NCOLS - 1] = 1;
    
    *g = ug;
}


fn main() -> Result<()> {
    let input = read_input("../input")?;
    
    let _max_lights = 10_000;
    
    let mut grid: Grid = init_grid(&input);

    assert!(
        input.len() == NROWS && input.iter().all(|line| line.len() == NCOLS),
        "Input dimensions must be {}x{}!",
        NROWS,
        NCOLS
    );
   
    for _ in 0..100 {
        update_grid(&mut grid);
    }

    let ans: i32 = grid
        .iter()
        .flatten()
        .map(|&val| val as i32)
        .sum();

    println!("Part 1: {:?}", ans);

    let mut grid = init_grid_locked(&input);
    
    for _ in 0..100 {
        update_grid_locked(&mut grid)
    }

    let ans: i32 = grid
        .iter()
        .flatten()
        .map(|&val| val as i32)
        .sum();

    println!("Part 2: {:?}", ans);
    
    Ok(())
}
