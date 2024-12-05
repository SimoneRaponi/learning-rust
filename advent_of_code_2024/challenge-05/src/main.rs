use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    // Open the input file
    let path = Path::new("puzzle_input.txt");
    let file = File::open(path)?;

    // Create a buffered reader for efficient reading of the file
    let reader = io::BufReader::new(file);

    // HashMap to track which numbers must appear before others
    let mut goes_after: HashMap<i32, HashSet<i32>> = HashMap::new();
    // Vector to store sequences of numbers
    let mut arrays: Vec<Vec<i32>> = Vec::new();

    // Process each line in the file
    for line in reader.lines() {
        let line = line?;

        // Skip empty lines to avoid unnecessary processing
        if line.trim().is_empty() {
            continue;
        }

        // If the line contains a pipe character ('|'), it's a dependency pair (a, b)
        if line.contains('|') {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 2 {
                // Try to parse the numbers and store the pair (b -> a)
                if let (Ok(a), Ok(b)) = (parts[0].trim().parse(), parts[1].trim().parse()) {
                    // b must appear before a, so store this relationship
                    goes_after.entry(b).or_insert_with(HashSet::new).insert(a);
                }
            }
        }
        // If the line contains commas, it represents a sequence of numbers
        else if line.contains(',') {
            let numbers: Vec<i32> = line.split(',')
                                        .filter_map(|s| s.trim().parse().ok())
                                        .collect();
            arrays.push(numbers);
        }
    }

    // Variable to store the total of the middle values of valid sequences
    let mut total: i32 = 0;

    // Validate each sequence
    for sequence in arrays.iter() {
        if validate_sequence(sequence, &goes_after) {
            // If valid, add the middle element of the sequence to the total
            let middle_value = sequence.get(sequence.len() / 2).copied().unwrap_or(0);
            total += middle_value;
        }
    }

    // Output the final total
    print!("Total: {}", total);

    Ok(())
}

// Function to validate a sequence based on dependency rules
fn validate_sequence(sequence: &Vec<i32>, goes_after: &HashMap<i32, HashSet<i32>>) -> bool {
    // HashSet to track the values that have already been seen in the sequence
    let mut seen: HashSet<i32> = HashSet::new();
    // HashSet of the current sequence to check if any required values are missing
    let sequence_set: HashSet<_> = sequence.iter().cloned().collect();

    // Iterate through each value in the sequence
    for val in sequence.iter() {
        // Mark the value as seen
        seen.insert(*val);

        // Check if there are any values that must appear before the current value
        match goes_after.get(val) {
            Some(v) => {
                // For each value that must precede the current value, verify it exists in the sequence
                for value in v.iter() {
                    // If a required value is missing but is part of the sequence, it's invalid
                    if !seen.contains(value) && sequence_set.contains(value) {
                        return false;
                    }
                }
            }
            None => continue, // No dependencies, so continue
        }
    }

    // If all dependencies are satisfied, return true
    return true;
}
