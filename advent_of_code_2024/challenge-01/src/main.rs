
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    // Vectors to hold the split parts
    let mut first_values = Vec::new();
    let mut second_values = Vec::new();

    // Read the input file line by line
    if let Ok(lines) = read_lines("./puzzle_input.txt") {
        for line in lines.flatten() {
            // Split each lines into parts and parse them into integers       
            let parts: Vec<&str> = line.split_whitespace().collect();
            
            // Ensure the line has at least two parts
            if parts.len() >= 2 {
                first_values.push(parts[0].parse::<i32>().unwrap());
                second_values.push(parts[1].parse::<i32>().unwrap());
            }
        }
    }

    // Sort both lists in ascending order
    first_values.sort_unstable();
    second_values.sort_unstable();

    // Calculate the total absolute difference between corresponding elements
    let total_difference: i32 = first_values.iter()
        .zip(second_values.iter()) // Pair up elements from both lists
        .map(|(first, second)| (first - second).abs()) // Compute absolute difference
        .sum(); // Sum up the differences 

    println!("Total: {}", total_difference);

    // Calculate the similarity score: product of each element and its occurrence count in the other list
    let similarity: i32 = first_values.iter()
    .map(|&first| {
        let count = count_occurrences(&second_values, first); // Count occurrences of `first` in `second_values`
        first * count as i32 // Multiply the value by its occurrence count
    })
    .sum(); // Sum up all similarity contributions

    println!("Similarity: {}", similarity);

}

/// Reads lines from a file and returns an iterator over them.
/// If the file cannot be opened, an error is returned.
///
/// # Arguments
/// * `filename` - Path to the file to be read.
///
/// # Returns
/// * An iterator over the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Counts the number of occurrences of a target number in a vector of integers.
///
/// # Arguments
/// * `vec` - A reference to a vector of integers.
/// * `target` - The integer to count occurrences of.
///
/// # Returns
/// * The number of times `target` appears in `vec`.
fn count_occurrences(vec: &[i32], target: i32) -> usize {
    vec.iter().filter(|&&x| x == target).count()
}