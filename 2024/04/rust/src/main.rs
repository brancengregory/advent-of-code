use std::fs::read_to_string;

fn search_grid(grid: &[Vec<char>], word: &str) -> usize {
    let word_len = word.len();
    let word_chars: Vec<char> = word.chars().collect();
    let rows = grid.len();
    let cols = grid[0].len();

    // Directions: (row_delta, col_delta)
    let directions = [
        (0, 1),   // Right
        (0, -1),  // Left
        (1, 0),   // Down
        (-1, 0),  // Up
        (1, 1),   // Down-right
        (1, -1),  // Down-left
        (-1, 1),  // Up-right
        (-1, -1), // Up-left
    ];

    let mut count = 0;

    for r in 0..rows {
        for c in 0..cols {
            for &(dr, dc) in &directions {
                let mut match_found = true;

                for i in 0..word_len {
                    let nr = r as isize + dr * i as isize;
                    let nc = c as isize + dc * i as isize;

                    if nr < 0 || nc < 0 || nr >= rows as isize || nc >= cols as isize {
                        match_found = false;
                        break;
                    }

                    if grid[nr as usize][nc as usize] != word_chars[i] {
                        match_found = false;
                        break;
                    }
                }

                if match_found {
                    count += 1;
                }
            }
        }
    }

    count
}

fn count_xmas(grid: &[Vec<char>]) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut count = 0;

    for r in 1..rows - 1 {
        for c in 1..cols - 1 {
            // Check for X-MAS pattern
            let top_left = grid[r - 1][c - 1];
            let top_right = grid[r - 1][c + 1];
            let middle = grid[r][c];
            let bottom_left = grid[r + 1][c - 1];
            let bottom_right = grid[r + 1][c + 1];

            // Check for the diagonal X pattern with MAS/SAM
            if (top_left == 'M' && middle == 'A' && bottom_right == 'S') || 
               (top_left == 'S' && middle == 'A' && bottom_right == 'M') {
                if (top_right == 'M' && middle == 'A' && bottom_left == 'S') || 
                   (top_right == 'S' && middle == 'A' && bottom_left == 'M') {
                    count += 1;
                }
            }
        }
    }

    count
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string("../input")?; 

    // Parse input into a grid of characters
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // Search for "XMAS"
    let count = search_grid(&grid, "XMAS");
    println!("Part 1: {:?}", count);

    let count = count_xmas(&grid);
    println!("Part 2: {:?}", count);
    Ok(())
}

