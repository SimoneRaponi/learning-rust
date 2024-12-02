
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Struct to represent a row of numbers in the report.
struct Row {
    numbers: Vec<i32>, // List of numbers in the row.
}

impl Row {
    
    /// Checks if the row of numbers is "safe" without any removals.
    /// 
    /// A row is considered safe if:
    /// 1. The difference between any two consecutive numbers is no more than 3.
    /// 2. The numbers are either strictly increasing or strictly decreasing, but not both.
    fn is_safe(&self) -> bool {

        // We must have at least two numbers to check the differences.
        if self.numbers.len() < 2 {
            return false; // A row with less than two numbers cannot be evaluated
        }

        // Flags to track if the sequence is increasing or decreasing
        let mut is_increasing = self.numbers[0] < self.numbers[1];
        let mut is_decreasing = self.numbers[0] > self.numbers[1];

        // Iterate through the numbers to check the conditions
        for i in 0..self.numbers.len() - 1 {
            let current = self.numbers[i];
            let next = self.numbers[i + 1];

            // Check if the absolute difference between current and next exceeds 3
            // or if they are equal (which is not allowed).
            if (current - next).abs() > 3 || current == next{
                return false;
            }

            // Update the flags based on whether the sequence is increasing or decreasing
            if current < next {
                is_increasing = true;
            } else if current > next {
                is_decreasing = true;
            }

            // If both increasing and decreasing flags are true, the row is unsafe
            if is_increasing && is_decreasing {
                return false;
            }
        }

        true
    }

    /// Checks if the row of numbers can be "safe" by removing one element.
    ///
    /// A row can be made safe if removing one element results in a safe sequence.
    fn is_safe_with_one_removal(&self) -> bool {
        // If the row is already safe, no need to check for removal
        if self.is_safe() {
            return true;
        }

        // Try removing each number one by one and check if the modified row is safe
        for i in 0..self.numbers.len() {
            let mut modified_numbers = self.numbers.clone();
            modified_numbers.remove(i);
            let modified_row = Row { numbers: modified_numbers };

            // If removing one element makes the row safe, return true
            if modified_row.is_safe() {
                return true
            }
        }

        false // Return false if no removal results in a safe row
    }

}

/// A processor that manages multiple rows of data and performs operations on them.
struct Processor {
    rows: Vec<Row>, // A collection of rows.
}


impl Processor {
    /// Counts how many rows are safe (either as is or with one removal).
    fn count_safe(&self) -> i32 {
        self.rows.iter().filter(|row| row.is_safe_with_one_removal()).count() as i32
    }

    /// Adds a new row to the processor's collection.
    fn add_row(&mut self, row: Row) {
        self.rows.push(row);
    }
}

fn main() {
    
    let mut processor = Processor {
        rows: Vec::new()
    };

    // Read the lines from the input file and process each one
    if let Ok(lines) = read_lines("./puzzle_input.txt") {
        for line in lines.flatten() {

            // Parse the line into a vector of integers
            let numbers: Vec<i32> = line.split_whitespace()
                .filter_map(|x|x.parse::<i32>().ok())
                .collect();

            // Create a new Row struct and add it to the processor
            let row = Row { numbers };
            processor.add_row(row);
        }
    }

    // Print the total number of safe rows
    println!("Safe rows: {}", processor.count_safe());
    
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