use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    // Rotates the guard's direction 90 degrees to the right (clockwise)
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

struct Guard {
    x: usize,              // Current x-coordinate of the guard
    y: usize,              // Current y-coordinate of the guard
    direction: Direction,  // Current direction the guard is facing
}

impl Guard {
    // Constructs a new Guard with the given position and direction
    fn new(x: usize, y: usize, direction: Direction) -> Self {
        Guard { x, y, direction }
    }

    // Determines the next direction for the guard, considering obstacles in the grid
    fn next_direction(&self, grid: &Vec<Vec<char>>) -> Direction {
        // Calculate the movement offset based on the current direction
        let (dx, dy) = match self.direction {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        };

        // Compute the coordinates of the next position
        let next_x = self.x as isize + dx;
        let next_y = self.y as isize + dy;

        // Check for obstacles or boundaries
        if next_x < 0 || next_y < 0 || next_x >= grid.len() as isize || next_y >= grid[0].len() as isize {
            // Out of bounds: continue facing the same direction
            self.direction
        } else {
            let next_x = next_x as usize;
            let next_y = next_y as usize;
            if grid[next_y][next_x] == '#' {
                // If an obstacle is encountered, turn right
                self.direction.turn_right()
            } else {
                // No obstacle: continue in the current direction
                self.direction
            }
        }
    }

    // Moves the guard one step forward based on the current direction
    fn move_forward(&mut self) {
        match self.direction {
            Direction::Up => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
        }
    }
}

// Simulates the guard's patrol and returns the visited grid and the count of visited positions
fn simulate_patrol(grid: &Vec<Vec<char>>, start_x: usize, start_y: usize) -> (Vec<Vec<char>>, i32) {
    let mut guard = Guard::new(start_x, start_y, Direction::Up);
    let mut visited_positions = grid.clone();
    let mut visited_count: i32 = 0;

    loop {
        // Mark the current position as visited
        if visited_positions[guard.y][guard.x] != '#' {
            // Avoid double counting already visited positions
            if visited_positions[guard.y][guard.x] != 'X' {
                visited_count += 1;
            }
            visited_positions[guard.y][guard.x] = 'X'; // Mark position as visited
        }

        // Determine the next direction based on obstacles
        let next_direction = guard.next_direction(grid);

        // If the direction remains unchanged, move forward
        if next_direction == guard.direction {
            guard.move_forward();
        }

        // Stop the simulation if the guard exits the grid boundaries
        if guard.x >= grid[0].len() || guard.y >= grid.len() {
            break;
        }

        // Update the guard's direction
        guard.direction = next_direction;
    }

    (visited_positions, visited_count)
}

fn main() -> io::Result<()> {
    let filename = "puzzle_input.txt"; // Specify the input file containing the grid
    let grid = read_grid_from_file(filename)?; // Read the grid from the file

    let mut x_guard = 0;
    let mut y_guard = 0;

    // Locate the guard's initial position in the grid (marked as '^')
    for (i, row) in grid.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if *ch == '^' {
                x_guard = j; 
                y_guard = i;
            }
        }
    }

    // Simulate the guard's patrol
    let (visited, visited_count) = simulate_patrol(&grid, x_guard, y_guard);

    for row in visited {
        println!("{}", row.iter().collect::<String>());
    }

    println!("Total visited positions: {}", visited_count);

    Ok(())
}

// Reads the grid from the specified file and returns it as a vector of vectors of characters
fn read_grid_from_file(filename: &str) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(filename)?; 
    let reader = io::BufReader::new(file);
    let mut grid = Vec::new();

    // Read each line of the file and convert it into a vector of characters
    for line in reader.lines() {
        let line = line?;
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }

    Ok(grid)
}
