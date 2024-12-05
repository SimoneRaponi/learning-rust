use std::fs::File;
use std::io::{self, BufRead};

// Constants for file path, maximum grid size, direction vectors, and target word 'XMAS'
const PATH: &str = "./puzzle_input.txt";
const MAX: usize = 255;

// Directions for all 8 possible directions
const DIRS: [[i32; 2]; 8] = [
    [0, 1],    // Right
    [1, 0],    // Down
    [0, -1],   // Left
    [-1, 0],   // Up
    [-1, -1],  // Up-Left
    [-1, 1],   // Up-Right
    [1, -1],   // Down-Left
    [1, 1],    // Down-Right
];

// Diagonal directions
const DIAGONALS: [[i32; 2]; 2] = [
    [1, 1],   // Down-Right diagonal
    [1, -1],  // Down-Left diagonal
];

// The byte representation of the word "XMAS"
const XMAS: [u8; 4] = ['X' as u8, 'M' as u8, 'A' as u8, 'S' as u8];

fn main() -> io::Result<()> {
    // Open the input file and create a buffered reader
    let file = File::open(PATH)?;
    let reader = io::BufReader::new(file);

    // Initialize the grid with a maximum size
    let mut grid: Vec<Vec<u8>> = vec![vec![0; MAX]; MAX];
    let mut h = 0;  // Track the number of rows read from the file

    // Read the lines from the file and populate the grid
    for line in reader.lines() {
        let line = line?;
        grid[h] = line.bytes().collect();
        h += 1;
    }

    let w = grid[0].len();  // Get the number of columns (width of the grid)
    let mut total = 0;      // Initialize the count for occurrences of "XMAS"

    // Iterate over the grid and count the number of occurrences of "XMAS"
    for y in 0..h {
        for x in 0..w {
            // Skip if the current cell is not 'X'
            if grid[y][x] != XMAS[0] {
                continue;
            }
            // Check if "XMAS" can be formed starting from (x, y)
            total += get_word_count_at(x, y, &grid, h, w);
        }
    }
    println!("{}", total); // Print the total count of "XMAS"

    let mut total_x = 0;  // Initialize the count for valid "XMAS" diagonal patterns

    // Iterate again to check for specific "XMAS" diagonals starting with 'A'
    for y in 0..h {
        for x in 0..w {
            // Skip if the current cell is not 'A'
            if grid[y][x] != b'A' {
                continue;
            }
            // Check if the specific "XMAS" pattern is valid at (x, y)
            if has_x_mas_at(x, y, &grid, h, w) {
                total_x += 1;
            }
        }
    }
    println!("{}", total_x); // Print the count of valid "XMAS" diagonal patterns

    Ok(())
}

/// Counts how many times the word "XMAS" can be formed starting from (ox, oy)
fn get_word_count_at(ox: usize, oy: usize, board: &[Vec<u8>], h: usize, w: usize) -> i32 {
    let mut count = 0;

    // Check all 8 possible directions for the word "XMAS"
    for &d in &DIRS {
        for i in 1..=3 {
            let x = ox as i32 + d[0] * i;
            let y = oy as i32 + d[1] * i;

            // If out of bounds, break the loop
            if !is_on_board(x, y, h, w) {
                break;
            }

            // If the letter does not match the current "XMAS" character, break the loop
            if board[y as usize][x as usize] != XMAS[i as usize] {
                break;
            }

            // If we've matched all characters of "XMAS", increment the count
            if i == 3 {
                count += 1;
            }
        }
    }
    count
}

/// Checks if the "XMAS" diagonal pattern can be formed at (ox, oy)
fn has_x_mas_at(ox: usize, oy: usize, board: &[Vec<u8>], h: usize, w: usize) -> bool {
    let r = [-1, 1];  // Check both directions (up and down) along the diagonal

    // Check both diagonals for the specific "XMAS" pattern
    for &d in &DIAGONALS {
        let mut s = 0;

        // Check both directions (positive and negative) on the diagonal
        for &i in &r {
            let x = ox as i32 + d[0] * i;
            let y = oy as i32 + d[1] * i;

            // If out of bounds, return false
            if !is_on_board(x, y, h, w) {
                return false;
            }

            // Add the byte value to the sum for checking 'M' + 'S' = 168
            s += board[y as usize][x as usize] as i32;
        }

        // If the sum does not equal the byte values of 'M' + 'S', return false
        if s != ('M' as i32 + 'S' as i32) {
            return false;
        }
    }

    true  // If the diagonal pattern is valid, return true
}

/// Checks if the given coordinates (x, y) are within the bounds of the grid
fn is_on_board(x: i32, y: i32, h: usize, w: usize) -> bool {
    x >= 0 && x < w as i32 && y >= 0 && y < h as i32
}
