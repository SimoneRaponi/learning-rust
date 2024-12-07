use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

/// Represents the four possible directions the guard can face
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    /// Rotates the guard's direction 90 degrees to the right (clockwise).
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

/// Represents the guard's current state: position and direction.
struct Guard {
    x: isize,             // Current x-coordinate of the guard
    y: isize,             // Current y-coordinate of the guard
    direction: Direction, // Current direction the guard is facing
}

impl Guard {
    /// Constructs a new Guard starting at (x, y) facing a given direction.
    fn new(x: usize, y: usize, direction: Direction) -> Self {
        Guard {
            x: x as isize,
            y: y as isize,
            direction,
        }
    }

    /// Determines the next direction for the guard based on the environment.
    ///
    /// The guard checks the position in front of it:
    /// - If there is an obstacle or the position is out of bounds, the guard will turn right.
    /// - Otherwise, the guard continues forward in the same direction.
    fn next_direction(&self, grid: &Vec<Vec<char>>) -> Direction {
        // Calculate the movement offset based on the current direction
        let (dx, dy) = match self.direction {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        };

        let next_x = self.x + dx;
        let next_y = self.y + dy;

        // Check boundaries before indexing
        if next_x < 0 || next_y < 0 {
            // If next position is negative, it's out of bounds
            return self.direction;
        }

        let nx = next_x as usize;
        let ny = next_y as usize;

        // Check if out of bounds in the positive direction
        if ny >= grid.len() || nx >= grid[0].len() {
            // Out of bounds
            return self.direction;
        }

        // Check for obstacle
        if grid[ny][nx] == '#' {
            // If an obstacle is encountered, turn right
            self.direction.turn_right()
        } else {
            // No obstacle: continue in the current direction
            self.direction
        }
    }

    /// Moves the guard one step forward based on the current direction.
    fn move_forward(&mut self) {
        match self.direction {
            Direction::Up => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
        }
    }
}

/// Runs a simulation of the guard's patrol on the given grid.
///
/// Returns a tuple:
/// (number_of_visited_positions, leaves_grid, in_loop)
/// - number_of_visited_positions: Count of distinct positions visited (including the start)
/// - leaves_grid: true if the guard eventually leaves the grid
/// - in_loop: true if the guard gets stuck in a loop
fn simulate_patrol(grid: &Vec<Vec<char>>, start_x: usize, start_y: usize) -> (usize, bool, bool) {
    let mut guard = Guard::new(start_x, start_y, Direction::Up);

    // Keep track of visited positions to count distinct visits
    let mut visited_positions = HashSet::new();

    // Each state: (x, y, direction). Used to detect loops.
    let mut states = HashSet::new();

    // Record the initial state and position
    states.insert((guard.x, guard.y, guard.direction));
    visited_positions.insert((guard.x, guard.y));

    loop {
        let next_direction = guard.next_direction(grid);

        // If the direction remains unchanged, move forward
        if next_direction == guard.direction {
            guard.move_forward();
        }
        guard.direction = next_direction;

        // Check if guard leaves the grid after moving
        if guard.x < 0 || guard.y < 0 ||
            guard.y as usize >= grid.len() || guard.x as usize >= grid[0].len() {
            // Guard left the mapped area
            return (visited_positions.len(), true, false);
        }

        // Mark the new position as visited
        visited_positions.insert((guard.x, guard.y));

        let state = (guard.x, guard.y, guard.direction);
        if states.contains(&state) {
            // We have encountered this state before -> loop detected
            return (visited_positions.len(), false, true);
        } else {
            states.insert(state);
        }
    }
}

/// Reads the grid from the specified file and returns it as a vector of vectors of characters.
fn read_grid_from_file(filename: &str) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut grid = Vec::new();

    // Each line of the file is converted into a vector of characters
    for line in reader.lines() {
        let line = line?;
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }

    Ok(grid)
}

/// Part 2 logic:
///
/// Tries placing a single new obstruction in different positions and checks if it causes the guard to get stuck in a loop.
///
/// Now includes a simple progress bar that updates during the process.
fn count_obstructions_that_cause_loop(grid: &Vec<Vec<char>>, x_guard: usize, y_guard: usize) -> usize {
    let mut loop_count = 0;
    let mut grid_modified = grid.clone();

    // Calculate total number of cells to try (for the progress bar)
    let total_positions = grid.len() * grid[0].len();
    let mut processed = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, _ch) in row.iter().enumerate() {
            processed += 1; // Increment the number of processed positions

            // Print progress every 100 positions
            if processed % 100 == 0 || processed == total_positions {
                let progress = (processed as f64 / total_positions as f64) * 100.0;
                print!("\rProcessing: {:.2}% ", progress);
                io::Write::flush(&mut io::stdout()).unwrap();
            }

            // Conditions for placing the new obstruction:
            // - Can't place where the guard started (x_guard, y_guard)
            // - Can't place where there's already an obstruction or the guard
            if (x, y) == (x_guard, y_guard) {
                continue;
            }
            if grid[y][x] == '#' || grid[y][x] == '^' {
                continue;
            }

            // Temporarily place the obstruction
            let original = grid_modified[y][x];
            grid_modified[y][x] = '#';

            // Simulate with the new obstruction in place
            let (_, left_grid, in_loop) = simulate_patrol(&grid_modified, x_guard, y_guard);

            // If guard doesn't leave and gets stuck in a loop, count this obstruction
            if !left_grid && in_loop {
                loop_count += 1;
            }

            // Remove the obstruction and restore the original character
            grid_modified[y][x] = original;
        }
    }

    // Print a newline after finishing progress
    println!();

    loop_count
}

fn main() -> io::Result<()> {
    let filename = "puzzle_input.txt"; // Specify the input file containing the grid
    let grid = read_grid_from_file(filename)?; // Read the grid from the file

    // Locate the guard's initial position in the grid (marked as '^')
    let mut x_guard = 0;
    let mut y_guard = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if *ch == '^' {
                x_guard = j;
                y_guard = i;
            }
        }
    }

    // Part 1: Simulate the guard's initial patrol without modifications
    let (visited_count, _, _) = simulate_patrol(&grid, x_guard, y_guard);
    println!("Total visited positions without modification: {}", visited_count);

    // Part 2: Count how many positions can cause the guard to loop if obstructed
    let loop_count = count_obstructions_that_cause_loop(&grid, x_guard, y_guard);
    println!("Number of positions that cause guard to get stuck in a loop: {}", loop_count);

    Ok(())
}
