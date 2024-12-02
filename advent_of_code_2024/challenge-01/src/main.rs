    
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// A struct to hold and process two lists of integers for comparison and scoring
struct IntegerPairProcessor {
    list_a: Vec<i32>, // Represents the first list of integers
    list_b: Vec<i32>, // Represents the second list of integers

}

impl IntegerPairProcessor {
    /// Sorts both lists in ascending order for ordered comparisons
    fn sort_lists(&mut self) {
        self.list_a.sort_unstable();
        self.list_b.sort_unstable();
    }

    /// Calculates the total sum of absolute differences between corresponding elements in both lists
    /// Assumes both lists are sorted and of the same length.
    fn total_absolute_difference(&self) -> i32 {
        self.list_a.iter()
        .zip(self.list_b.iter()) // Pair up elements from both lists
        .map(|(first, second)| (first - second).abs()) // Compute absolute difference
        .sum() // Sum up the differences 
    }

    /// Calculates a similarity score based on the frequency of elements in one list matching elements in the other
    fn similarity_score(&self) -> i32 {
       self.list_a.iter()
            .map(|&first| {
                let count = count_occurrences(&self.list_b, first); // Count occurrences of `first` in `list_b`
                first * count as i32 // Multiply the value by its occurrence count
            })
            .sum() // Sum up all similarity contributions
    }
}

fn main() {

    let mut processor = IntegerPairProcessor {
        list_a: Vec::new(),
        list_b: Vec::new(),
    };

    // Read the input file line by line
    if let Ok(lines) = read_lines("./puzzle_input.txt") {
        for line in lines.flatten() {
            // Split each lines into parts and parse them into integers       
            let parts: Vec<&str> = line.split_whitespace().collect();
            
            // Ensure the line has at least two parts
            if parts.len() >= 2 {
                // Parse integers and add to respective lists.
                processor.list_a.push(parts[0].parse::<i32>().unwrap());
                processor.list_b.push(parts[1].parse::<i32>().unwrap());
            }
        }
    }

    // Sort the lists before performing operations.
    processor.sort_lists();

    // Print results of the operations
    println!("Total Absolute Difference: {}", processor.total_absolute_difference());
    println!("Similarity Score: {}", processor.similarity_score());

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