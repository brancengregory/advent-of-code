# Advent of Code Repository

Welcome to my **Advent of Code** repository! This project is my attempt to solve [Advent of Code](https://adventofcode.com/) puzzles across multiple years, primarily using Rust, with some solutions written in R or other languages. This repository reflects my journey through problem-solving, learning, and coding.


## 📂 Directory Structure

The repository is organized by year, then by day. Each day\'s folder contains:

- `puzzle.md`: The puzzle description for that day \(copied for reference\).
- Language-specific subdirectories \(e.g., `rust`, `r`\) for solutions.

For example:
. ├── 2015 │ ├── 01 │ │ ├── puzzle.md │ │ └── rust │ ├── 02 │ │ └── rust │ └── ... ├── 2023 │ ├── 01 │ │ ├── puzzle.md │ │ ├── rust │ │ └── R │ └── ... └── 2024 ├── 01 ├── puzzle.md └── rust


## 🛠️ Tools Used

### **Rust**
- I use the \[`aoc\`](https://github.com/gobanos/cargo-aoc) CLI tool for scaffolding, fetching inputs \(not pushed to Git due to Advent of Code\'s license\), and running solutions.
- Most solutions are implemented in Rust, leveraging its speed and safety for complex calculations.

### **R**
- Occasionally, I solve puzzles in R, especially for puzzles that require extensive data manipulation or statistical computation.
- Solutions are in the \`r\` or \`R\` subdirectories for respective days.

### **Other Languages**
- While rare, I might explore solutions in other languages for variety or to experiment with different approaches.


## ⚠️ Input Files

**Note:** Puzzle input files are not included in this repository due to Advent of Code\'s licensing policy. If you\'d like to run the solutions locally, use the \`aoc\` CLI tool or download your inputs directly from the Advent of Code website after logging in.


## 🎯 How to Use

1. **Clone the Repository:**
    ```
    git clone https://github.com/<your-username>/advent-of-code.git
    cd advent-of-code
    ```
2. **Install the `aoc` CLI Tool** 
    ```
    cargo install aoc
    ```

3. **Run a Solution:** Navigate to the year and day of interest, then run:
    ```
    cd 2015/01/rust
    cargo run
    ```

4. **R Solutions:** Open the R script in your favorite R environment and run it interactively or as a script.


## 🌟 Credits

Advent of Code for the amazing puzzles.
`aoc` CLI Tool for simplifying Rust-based AoC workflows.


Happy coding! 🎄✨

