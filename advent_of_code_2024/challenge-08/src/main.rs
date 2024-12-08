use std::collections::{HashMap, HashSet};
use std::fs;

/// Type alias for positions on the grid (row, col)
/// It is used to represent the coordinates of antennas and antinodes
type Position = (isize, isize);

/// Subtracts two positions
/// Returns a new position representing the vector difference of the input positions.
fn subtract_positions(a: Position, b: Position) -> Position {
    (a.0 - b.0, a.1 - b.1)
}

/// Add two positions
/// Returns a new position by adding the coordinates of the two input positions.
fn add_positions(a: Position, b: Position) -> Position {
    (a.0 + b.0, a.1 + b.1)
}

/// Parses the input grid into a HashMap of positions and their corresponding frequencies
/// Input: A vector of strings, where each character represents either an antenna (frequency) or an empty space
/// Output: A HashMap mapping grid positions to their respective characters (frequencies)
fn parse_input(input: &[String]) -> HashMap<Position, char> {
    let mut grid = HashMap::new();
    for (row, line) in input.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            grid.insert((row as isize, col as isize), ch); // Convert grid indices to isize for flexibility
        }
    }
    grid
}

/// Generate all unique pairs of items from a list
/// Input: A list of items
/// Output: A vector of tuples, each containing a unique pair of items from the list
fn generate_pairs<T: Clone>(list: &[T]) -> Vec<(T, T)> {
    if list.len() < 2 {
        vec![] // Return an empty vector if there are fewer than two items.
    } else {
        let head = &list[0];
        let tail = &list[1..];
        let mut result = tail
            .iter()
            .map(|item| (head.clone(), item.clone()))
            .collect::<Vec<_>>(); // Create pairs with the first item and all remaining items.
        result.extend(generate_pairs(tail)); // Recursively generate pairs from the tail.
        result
    }
}

/// Finds antinodes based on the positions of the two antennas
/// Logic: For each pair of antennas, calculate potential antinode positions based on their relative positions
/// Filters antinodes to include only valid grid positions
fn find_antinodes_part1(
    grid: &HashMap<Position, char>,
    antenna_a: Position,
    antenna_b: Position,
) -> Vec<Position> {
    vec![
        add_positions(antenna_a, subtract_positions(antenna_a, antenna_b)), // First antinode
        add_positions(antenna_b, subtract_positions(antenna_b, antenna_a)), // Second antinode
    ]
    .into_iter()
    .filter(|pos| grid.contains_key(pos)) // Keep only positions that exist in the grid
    .collect()
}

/// Solves the problem using the given antinode finding function
/// Input:
/// - The parsed input grid as a vector of strings
/// - A function to determine the antinodes for a pair of antennas
/// Process:
/// - Groups antennas by frequency
/// - For each frequency group, generates all pairs of antennas
/// - Computes antinodes for each pair and adds them to a unique set
/// Output: The total count of unique antinode positions
fn solve(
    input: &[String],
    find_antinodes: fn(&HashMap<Position, char>, Position, Position) -> Vec<Position>,
) -> usize {
    // Parse the input into a grid of positions and their respective characters (frequencies)
    let grid = parse_input(input);

    // Group all antenna positions by their frequency
    let grouped_by_frequency = grid
        .iter()
        .filter(|&(_, &freq)| freq != '.') // Ignore empty cells
        .fold(HashMap::new(), |mut acc, (&position, &freq)| {
            acc.entry(freq).or_insert_with(Vec::new).push(position);
            acc
        });

    // Use a HashSet to keep track of unique antinode positions
    let mut unique_antinode_positions = HashSet::new();

    // Process each frequency group.
    for antennas in grouped_by_frequency.values() {
        let antenna_pairs = generate_pairs(antennas); // Generate all unique pairs of antennas in the group
        for (antenna_a, antenna_b) in antenna_pairs {
            unique_antinode_positions.extend(find_antinodes(&grid, antenna_a, antenna_b)); // Find and collect antinodes
        }
    }

    unique_antinode_positions.len()
}

/// Reads the puzzle input file and returns the grid as a vector of strings
/// Input: The file path to the puzzle input file
/// Output: A vector of strings, each representing a row of the grid
fn read_input(file_path: &str) -> Vec<String> {
    fs::read_to_string(file_path) // Read the file contents as a single string
        .expect("Failed to read input file") // Panic if the file cannot be read
        .lines() // Split the string into lines
        .map(String::from) // Convert each line to a String
        .collect() // Collect the lines into a vector
}

fn main() {
    // Read the input file containing the grid representation
    let input = read_input("puzzle_input.txt");

    // Solve the problem for Part 1 using the specific antinode calculation function
    let unique_locations = solve(&input, find_antinodes_part1);

    println!("Unique Locations: {}", unique_locations);
}