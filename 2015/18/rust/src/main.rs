use std::fs::read_to_string;
use std::io::Result;

const NROWS: usize = 100;
const NCOLS: usize = 100;


fn read_input(p: &str) -> Result<Vec<String>> {
    let content = read_to_string(p)?;
    Ok(content.lines().map(|s| s.to_string()).collect())
}

type Grid = [[u8; NCOLS]; NROWS];

fn get_neighbors(p: (usize, usize), g: &Grid) {

}

fn main() -> Result<()> {
    let input = read_input("../input")?;
    
    let max_lights = 10_000;
    
    let mut grid: Grid = [[0; NCOLS]; NROWS];

    assert!(
        input.len() == NROWS && input.iter().all(|line| line.len() == NCOLS),
        "Input dimensions must be {}x{}!",
        NROWS,
        NCOLS
    );

    for (row, line) in input.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            grid[row][col] = match c {
                '#' => 1,
                '.' => 0,
                _ => panic!("Unexpected character found: {}", c)
            }
        }
    }
   

    println!("{:?}", grid);
    
    Ok(())
}
